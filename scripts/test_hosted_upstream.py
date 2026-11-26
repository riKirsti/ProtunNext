from contextlib import chdir
import os
import importlib.util
import json
from pathlib import Path
from tempfile import TemporaryDirectory
import subprocess
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location("hosted", Path(__file__).with_name("hosted-upstream.py"))
hosted = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(hosted)


class HostedUpstream(unittest.TestCase):
    def test_agent_allows_all_tools_with_the_pinned_model(self):
        config = hosted.opencode_ci.configuration()
        self.assertEqual(config["model"], "opencode/deepseek-v4.1-flash")
        self.assertEqual(config["permissions"], [{"action": "*", "resource": "*", "effect": "allow"}])
        self.assertNotIn("policies", config)
        self.assertEqual(config["websearch"], {"provider": "random"})

    def test_generated_patch_excludes_workflow_and_publication_paths(self):
        for path in (".github/workflows/release.yml", "scripts/hosted-upstream.py",
                     "src/../.github/config", "tests/fixtures/private-PUBLIC-TEST-ONLY.der"):
            self.assertFalse(hosted.allowed(path), path)
        for path in ("src/packet.rs", "tests/transport_test.rs", "Cargo.lock"):
            self.assertTrue(hosted.allowed(path), path)

    def test_agent_verifies_model_and_removes_configuration_after_exit(self):
        def complete(argv, **options):
            self.assertIn("--standalone", argv)
            self.assertNotIn("GH_TOKEN", options["env"])
            if argv[1] == "run":
                prompt = Path(argv[argv.index("--file") + 1]).read_text(encoding="utf-8")
                self.assertIn("noninteractive GitHub Actions", prompt)
                self.assertTrue(prompt.endswith("Synthetic task"))
                self.assertEqual(options["env"]["OPENCODE_API_KEY"], "synthetic")
                output = {"type": "step_finish", "sessionID": "fixture"}
            else:
                self.assertNotIn("OPENCODE_API_KEY", options["env"])
                output = {"info": {"id": "fixture"}, "messages": [{
                    "type": "assistant",
                    "model": {"providerID": "opencode", "id": model[0], "variant": "default"},
                }]}
            return subprocess.CompletedProcess(argv, 0, json.dumps(output))

        model = ["deepseek-v4.1-flash"]
        with TemporaryDirectory() as folder, chdir(folder):
            with (patch.dict(os.environ, {"GITHUB_ACTIONS": "true"}),
                  patch.object(hosted.subprocess, "run", side_effect=complete)):
                self.assertTrue(hosted.run_agent("synthetic", "Synthetic task"))
                model[0] = "wrong-model"
                self.assertFalse(hosted.run_agent("synthetic", "Synthetic task"))
            self.assertFalse(Path("opencode.json").exists())

    def test_draft_adopts_only_the_release_selected_by_the_monitor(self):
        baseline = {"tag": "v99.1.1", "commit": "a" * 40}
        target = {"tag": "v99.1.2", "commit": "b" * 40}
        with (patch.object(hosted, "adopted_release", return_value=baseline) as release,
              patch.object(hosted, "check_versions"),
              patch.object(hosted, "validate_checkout") as coverage):
            self.assertFalse(hosted.check_proposal_version(target, baseline))
            release.return_value = target
            self.assertTrue(hosted.check_proposal_version(target, baseline))
            coverage.assert_called_once()
            release.return_value = {**target, "commit": "c" * 40}
            with self.assertRaisesRegex(ValueError, "exact upstream release"):
                hosted.check_proposal_version(target, baseline)
