import importlib.util
from pathlib import Path
import unittest
from unittest.mock import Mock, patch
from urllib.error import HTTPError
from urllib.request import Request, urlopen

SPEC = importlib.util.spec_from_file_location("hosted", Path(__file__).with_name("hosted-upstream.py"))
hosted = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(hosted)


class HostedUpstream(unittest.TestCase):
    def test_only_free_requests_forwarded_and_budget_stops_before_api_call(self):
        connection = Mock()
        response = connection.getresponse.return_value
        response.status = 200
        response.getheader.return_value = "text/event-stream"
        response.read1.side_effect = [b"data: [DONE]\n\n", b""]
        with patch.object(hosted, "REQUEST_LIMIT", 1), patch.object(
            hosted.http.client, "HTTPSConnection", return_value=connection
        ), hosted.free_gateway("test-fixture") as port:
            request = Request(f"http://127.0.0.1:{port}/v1/chat/completions",
                              data=b'{"model":"paid","models":["paid"],"messages":[]}')
            with urlopen(request, timeout=3) as actual:
                self.assertEqual(actual.read(), b"data: [DONE]\n\n")
            with self.assertRaises(HTTPError) as blocked:
                urlopen(request, timeout=3)
            self.assertEqual(blocked.exception.code, 429)
            self.assertEqual(connection.request.call_count, 1)
            args = connection.request.call_args.args
            self.assertIn('"model": "openrouter/free"', args[2])
            self.assertNotIn('"models"', args[2])

    def test_generated_patch_excludes_workflow_and_publication_paths(self):
        for path in (".github/workflows/release.yml", "scripts/hosted-upstream.py",
                     "src/../.github/config", "tests/fixtures/private-PUBLIC-TEST-ONLY.der"):
            self.assertFalse(hosted.allowed(path), path)
        for path in ("src/packet.rs", "tests/transport_test.rs", "Cargo.lock"):
            self.assertTrue(hosted.allowed(path), path)
