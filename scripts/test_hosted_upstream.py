import importlib.util
from pathlib import Path
import unittest

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
        self.assertGreater(config["agent"][config["default_agent"]]["steps"], 0)

    def test_generated_patch_excludes_workflow_and_publication_paths(self):
        for path in (".github/workflows/release.yml", "scripts/hosted-upstream.py",
                     "src/../.github/config", "tests/fixtures/private-PUBLIC-TEST-ONLY.der"):
            self.assertFalse(hosted.allowed(path), path)
        for path in ("src/packet.rs", "tests/transport_test.rs", "Cargo.lock"):
            self.assertTrue(hosted.allowed(path), path)
