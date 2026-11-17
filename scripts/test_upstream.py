import datetime as dt
import importlib.util
import json
from pathlib import Path
import unittest

SPEC = importlib.util.spec_from_file_location("upstream", Path(__file__).with_name("check-upstream.py"))
upstream = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(upstream)

NOW = dt.datetime(2026, 12, 2, 12, tzinfo=dt.timezone.utc)
BOT = {"login": "github-actions[bot]"}
ADOPTED = json.loads(Path("tests/fixtures/compatibility.json").read_text(encoding="utf-8"))["upstream"]["tag"]
MAJOR, MINOR, PATCH = upstream.version(ADOPTED)
NEXT = f"v{MAJOR}.{MINOR}.{PATCH + 1}"


class FakeGitHub:
    def __init__(self, tag=NEXT):
        self.tag = tag
        self.issues = []
        self.calls = []

    def __call__(self, path, token, data=None, method=None):
        self.calls.append((path, data, method))
        if path.endswith("/releases/latest"):
            return {"tag_name": self.tag, "draft": False, "prerelease": False}
        if "/git/ref/tags/" in path:
            return {"object": {"type": "tag", "sha": "a" * 40}}
        if "/git/tags/" in path:
            return {"object": {"type": "commit", "sha": "b" * 40}}
        if "?state=all" in path:
            return list(self.issues)
        if path.endswith("/assignees"):
            issue = self.issues[int(path.split("/")[-2]) - 1]
            issue["assignees"] = [{"login": login} for login in data["assignees"]]
            return issue
        if method == "PATCH":
            issue = self.issues[int(path.split("/")[-1]) - 1]
            issue.update(data)
            return issue
        issue = {"number": len(self.issues) + 1, "state": "open",
                 "user": BOT, "assignees": [], **data}
        self.issues.append(issue)
        return issue


class UpstreamMonitor(unittest.TestCase):
    def test_five_day_interval_across_month_boundary(self):
        api = FakeGitHub(ADOPTED)
        upstream.check("test/repo", "fixture", NOW, call=api)
        self.assertEqual(api.issues[0]["state"], "closed")
        api.calls.clear()
        upstream.check("test/repo", "fixture", NOW + dt.timedelta(days=4), call=api)
        self.assertEqual(len(api.calls), 1)
        upstream.check("test/repo", "fixture", NOW + dt.timedelta(days=5), call=api)
        self.assertTrue(any(path.endswith("/releases/latest") for path, _, _ in api.calls))
        self.assertEqual(len(api.issues), 1)

    def test_dry_run_then_annotated_release_and_idempotent_assignment(self):
        api = FakeGitHub()
        upstream.check("test/repo", "fixture", NOW, force=True, dry_run=True, call=api)
        self.assertTrue(all(data is None for _, data, _ in api.calls))
        for _ in range(2):
            upstream.check("test/repo", "fixture", NOW, force=True, copilot_token="fixture", call=api)
        self.assertEqual(len(api.issues), 2)
        self.assertIn("b" * 40, api.issues[0]["body"])
        self.assertEqual(sum(path.endswith("/assignees") for path, _, _ in api.calls), 1)

    def test_invalid_release_does_not_advance_schedule(self):
        api = FakeGitHub(NEXT + "-rc.1")
        with self.assertRaises(ValueError):
            upstream.check("test/repo", "fixture", NOW, call=api)
        self.assertEqual(api.issues, [])
