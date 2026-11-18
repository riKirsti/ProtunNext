from contextlib import chdir
import importlib.util
from pathlib import Path
from tempfile import TemporaryDirectory
import subprocess
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location("hosted", Path(__file__).with_name("hosted-upstream.py"))
hosted = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(hosted)


class HostedUpstream(unittest.TestCase):
    def test_agent_uses_only_the_pinned_free_console_model(self):
        config = hosted.agent_config()
        self.assertEqual(config["enabled_providers"], ["opencode"])
        self.assertEqual(config["provider"], {"opencode": {"whitelist": ["deepseek-v4.1-flash-free"]}})
        for role in ("model", "small_model"):
            self.assertEqual(config[role], "opencode/deepseek-v4.1-flash-free")
        self.assertEqual(config["permission"]["*"], "deny")
        self.assertNotIn("agent", config)

    def test_generated_patch_excludes_workflow_and_publication_paths(self):
        for path in (".github/workflows/release.yml", "scripts/hosted-upstream.py",
                     "src/../.github/config", "tests/fixtures/private-PUBLIC-TEST-ONLY.der"):
            self.assertFalse(hosted.allowed(path), path)
        for path in ("src/packet.rs", "tests/transport_test.rs", "Cargo.lock"):
            self.assertTrue(hosted.allowed(path), path)

    def test_agent_uses_open_code_defaults_and_removes_prompt_after_exit(self):
        def complete(argv, **options):
            self.assertEqual(Path(argv[argv.index("--file") + 1]).read_text(), "Synthetic task")
            self.assertNotIn("timeout", options)
            return subprocess.CompletedProcess(argv, 0)

        with TemporaryDirectory() as folder, chdir(folder):
            with patch.object(hosted.subprocess, "run", side_effect=complete):
                self.assertTrue(hosted.run_agent("synthetic", "Synthetic task"))
            self.assertFalse(Path(".upstream-context.txt").exists())

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
