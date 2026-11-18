"""Exercise unattended release gates without mutating a GitHub repository."""

import importlib.util
from pathlib import Path
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location("finish", Path(__file__).with_name("finish-automation.py"))
finish = importlib.util.module_from_spec(spec)
spec.loader.exec_module(finish)


class Autocomplete(unittest.TestCase):
    def test_only_same_repository_bot_branches_are_eligible(self):
        repo = {"full_name": "owner/repo"}
        record = {"state": "open", "user": {"login": "github-actions[bot]"},
                  "base": {"ref": "main", "repo": repo},
                  "head": {"ref": "upstream/v99.1.2-" + "a" * 40, "sha": "b" * 40, "repo": repo}}
        with patch.dict(finish.os.environ, {"GITHUB_REPOSITORY": "owner/repo"}):
            self.assertTrue(finish.owned(record, finish.UPSTREAM_BRANCH))
            self.assertFalse(finish.owned({**record, "user": {"login": "someone"}},
                                          finish.UPSTREAM_BRANCH))
            record["head"]["repo"] = {"full_name": "someone/repo"}
            self.assertFalse(finish.owned(record, finish.UPSTREAM_BRANCH))

    def test_dispatched_check_rejects_wrong_identity_and_failed_result(self):
        sha = "a" * 40
        run = {"head_sha": sha, "head_branch": "main", "event": "workflow_dispatch",
               "status": "completed", "conclusion": "success"}
        for api_run, error in [({**run, "head_sha": "b" * 40}, "expected branch and commit"),
                               ({**run, "conclusion": "failure"}, "concluded failure")]:
            with (patch.object(finish, "api", return_value=api_run),
                  self.assertRaisesRegex(RuntimeError, error)):
                finish.wait_for_runs([1], "main", sha)
        with patch.object(finish, "api", return_value=run):
            finish.wait_for_runs([1], "main", sha)

    def test_existing_release_is_left_intact(self):
        with (patch.object(finish, "fetch", return_value="a" * 40),
              patch.object(finish, "release_identity", return_value={"tag": "v99.1.2"}),
              patch.object(finish, "api", return_value={"tag_name": "v99.1.2"}),
              patch.object(finish, "check") as check, patch.object(finish, "dispatch") as dispatch):
            finish.release_main()
            check.assert_not_called()
            dispatch.assert_not_called()

    def test_failed_proposal_checks_leave_draft_and_main_untouched(self):
        head, base = "a" * 40, "b" * 40
        pr = {"number": 1, "head": {"ref": "upstream/v99.1.2-" + head, "sha": head}}
        with (patch.object(finish, "fetch", return_value=head),
              patch.object(finish, "main_sha", return_value=base),
              patch.object(finish, "check", side_effect=finish.WorkflowFailed("rejected")),
              patch.object(finish, "api") as api, patch.object(finish.subprocess, "run") as run):
            with self.assertRaises(finish.WorkflowFailed):
                finish.merge(pr, base, finish.UPSTREAM_BRANCH)
            api.assert_not_called()
            run.assert_not_called()

    def test_unreleased_main_is_checked_before_publishing(self):
        sha = "a" * 40
        expected = {"tag_name": "v99.1.2", "draft": False, "prerelease": False}
        with (patch.object(finish, "fetch", return_value=sha),
              patch.object(finish, "release_identity", return_value={"tag": "v99.1.2"}),
              patch.object(finish, "api", side_effect=[None, expected]),
              patch.object(finish, "check") as check, patch.object(finish, "main_sha", return_value=sha),
              patch.object(finish, "dispatch", return_value=12) as dispatch,
              patch.object(finish, "wait_for_runs") as wait):
            finish.release_main()
            check.assert_called_once_with("main", sha)
            dispatch.assert_called_once_with("release.yml", "main", {"publish": True})
            wait.assert_called_once_with([12], "main", sha)


if __name__ == "__main__":
    unittest.main()
