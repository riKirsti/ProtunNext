"""Finish approved-by-policy bot changes through GitHub's normal PR and release APIs."""

import json
import os
import re
import subprocess
import sys
import time
import tomllib
from urllib.error import HTTPError
from urllib.parse import quote, urlencode
from urllib.request import Request, urlopen

from compatibility import INDEX, validate_versions
import upstream_coverage

CHECKS = ("ci.yml", "security.yml")
MERGE_STATUS = "protunnext/verified-merge"
ACTIONS_APP = 15368
BOT = "github-actions[bot]"
SHA = r"[0-9a-f]{40}"
UPSTREAM_BRANCH = rf"upstream/v[0-9]+\.[0-9]+\.[0-9]+-{SHA}"
MAINTENANCE_BRANCH = (
    r"maintenance/review/\d{4}-\d{2}-\d{2}/(performance|refactor|cleanup)-[0-9a-f]{12}"
)


class WorkflowFailed(RuntimeError):
    """An exact-commit check completed without qualifying its candidate."""


def api(path, body=None, method="GET", missing=False):
    url = f"https://api.github.com/repos/{os.environ['GITHUB_REPOSITORY']}/{path}"
    request = Request(url, method=method, data=None if body is None else json.dumps(body).encode(),
                      headers={"Authorization": f"Bearer {os.environ['GH_TOKEN']}",
                               "Accept": "application/vnd.github+json",
                               "Content-Type": "application/json",
                               "X-GitHub-Api-Version": "2026-03-10"})
    try:
        with urlopen(request, timeout=30) as response:
            data = response.read()
            return json.loads(data) if data else None
    except HTTPError as error:
        if error.code == 404 and missing:
            return None
        raise RuntimeError(f"GitHub API {method} failed with HTTP {error.code}") from None


def git(*args):
    return subprocess.check_output(["git", *args], text=True, encoding="utf-8", timeout=60).strip()


def main_sha():
    return api("git/ref/heads/main")["object"]["sha"]


def release_identity(ref):
    if not re.fullmatch(SHA, ref):
        raise ValueError("invalid commit identity")
    upstream = json.loads(git("show", f"{ref}:{INDEX.as_posix()}"))["upstream"]
    package = tomllib.loads(git("show", f"{ref}:Cargo.toml"))["package"]
    lock = tomllib.loads(git("show", f"{ref}:Cargo.lock"))["package"]
    validate_versions(upstream, package, lock)
    return upstream


def owned(pr, pattern):
    repo = os.environ["GITHUB_REPOSITORY"]
    return (pr["state"] == "open" and pr["user"]["login"] == BOT
            and pr["base"]["ref"] == "main" and pr["base"]["repo"]["full_name"] == repo
            and (pr["head"]["repo"] or {}).get("full_name") == repo
            and re.fullmatch(pattern, pr["head"]["ref"]) is not None
            and re.fullmatch(SHA, pr["head"]["sha"]) is not None)


def open_pulls(**filters):
    for page in range(1, 101):
        query = urlencode(dict(state="open", per_page=100, page=page, **filters))
        batch = api(f"pulls?{query}")
        yield from batch
        if len(batch) < 100:
            return
    raise RuntimeError("pull request inventory exceeded the page limit")


def dispatch(workflow, branch, inputs=None):
    body = {"ref": branch, "inputs": inputs or {}}
    run = api(f"actions/workflows/{workflow}/dispatches", body, "POST")
    if not run or type(run.get("workflow_run_id")) is not int:
        raise RuntimeError("GitHub did not return the dispatched run ID")
    return run["workflow_run_id"]


def wait_for_runs(runs, branch, expected_sha):
    deadline = time.monotonic() + 3600
    pending = set(runs)
    while pending and time.monotonic() < deadline:
        for run_id in tuple(pending):
            run = api(f"actions/runs/{run_id}", missing=True)
            if run is None:
                continue  # The run ID may precede its visibility in the list endpoint.
            if (run["head_sha"] != expected_sha or run["head_branch"] != branch
                    or run["event"] != "workflow_dispatch"):
                raise RuntimeError("dispatched run does not match the expected branch and commit")
            if run["status"] == "completed":
                if run["conclusion"] != "success":
                    raise WorkflowFailed(f"workflow run {run_id} concluded {run['conclusion']}")
                pending.remove(run_id)
        if pending:
            time.sleep(20)
    if pending:
        raise TimeoutError("timed out waiting for workflow checks")


def check(branch, sha):
    runs = [dispatch(workflow, branch) for workflow in CHECKS]
    wait_for_runs(runs, branch, sha)


def fetch(branch):
    git("fetch", "--no-tags", "origin", f"refs/heads/{branch}")
    return git("rev-parse", "FETCH_HEAD")


def changed_paths(base, head):
    return set(git("diff", "--name-only", "--no-renames", "-z", base, head).strip("\0").split("\0")) - {""}


def require_merge_rule():
    rules = api("rules/branches/main")
    checks = any(
        rule["type"] == "required_status_checks"
        and rule["parameters"]["strict_required_status_checks_policy"] is True
        and {"context": MERGE_STATUS, "integration_id": ACTIONS_APP}
        in rule["parameters"]["required_status_checks"]
        for rule in rules
    )
    if not checks or not any(rule["type"] == "pull_request" for rule in rules):
        raise RuntimeError("main requires an active, strict GitHub Actions merge status and pull requests")


def verification_status(head, state, description):
    api(f"statuses/{head}", {"state": state, "context": MERGE_STATUS, "description": description,
                            "target_url": f"https://github.com/{os.environ['GITHUB_REPOSITORY']}"
                                          f"/actions/runs/{os.environ['GITHUB_RUN_ID']}"}, "POST")


def pr_verification():
    with open(os.environ["GITHUB_EVENT_PATH"], encoding="utf-8") as file:
        trigger = json.load(file)["workflow_run"]
    head, branch = trigger["head_sha"], trigger["head_branch"]
    source = trigger["head_repository"]
    if trigger["event"] != "pull_request" or not source or not re.fullmatch(SHA, head):
        return
    prs = [pr for pr in open_pulls(head=f"{source['owner']['login']}:{branch}", base="main")
           if pr["user"]["login"] != BOT and pr["head"]["sha"] == head
           and (pr["head"]["repo"] or {}).get("id") == source["id"]]
    if len(prs) != 1:
        return
    pr = prs[0]
    base = main_sha()
    if api(f"compare/{base}...{head}")["merge_base_commit"]["sha"] != base:
        return
    query = urlencode(dict(event="pull_request", head_sha=head, per_page=30))
    conclusions = []
    for workflow in CHECKS:
        runs = api(f"actions/workflows/{workflow}/runs?{query}")["workflow_runs"]
        run = next((row for row in runs if row["head_sha"] == head
                    and row["head_branch"] == branch
                    and (row["head_repository"] or {}).get("id") == source["id"]
                    and (not row["pull_requests"] or any(item["number"] == pr["number"]
                                                         for item in row["pull_requests"]))), None)
        if run is None or run["status"] != "completed":
            return
        conclusions.append(run["conclusion"])
    if main_sha() == base and api(f"pulls/{pr['number']}")["head"]["sha"] == head:
        success = all(value == "success" for value in conclusions)
        verification_status(head, "success" if success else "failure",
                            "Platform and linked erasure checks passed" if success
                            else "Platform or linked erasure verification failed")


def merge(pr, base, pattern):
    require_merge_rule()
    branch, head, number = pr["head"]["ref"], pr["head"]["sha"], pr["number"]
    if fetch(branch) != head or main_sha() != base or git("merge-base", base, head) != base:
        raise RuntimeError("proposal or main changed before validation")
    verification_status(head, "pending", "Checking the exact proposal")
    try:
        check(branch, head)
    except WorkflowFailed:
        verification_status(head, "failure", "Platform or linked erasure verification failed")
        raise
    current = api(f"pulls/{number}")
    if not owned(current, pattern) or current["head"]["sha"] != head or main_sha() != base:
        raise RuntimeError("proposal or main changed during validation")
    if current["draft"]:
        subprocess.run(["gh", "pr", "ready", str(number), "--repo", os.environ["GITHUB_REPOSITORY"]],
                       check=True, timeout=60)
    if main_sha() != base:
        raise RuntimeError("main changed before merge")
    verification_status(head, "success", "Platform and linked erasure checks passed")
    merged = api(f"pulls/{number}/merge", {"sha": head, "merge_method": "merge"}, "PUT")
    if not merged.get("merged"):
        raise RuntimeError(f"GitHub did not merge PR #{number}")
    merge_sha = merged["sha"]
    print(f"Merged automated PR #{number} at {merge_sha}.")
    if fetch("main") != merge_sha:
        raise RuntimeError("main moved after automation merged; exact-head release deferred")
    if (git("rev-parse", f"{merge_sha}^1") != base
            or git("rev-parse", f"{merge_sha}^{{tree}}") != git("rev-parse", f"{head}^{{tree}}")):
        # A concurrent human merge can land between the last read and the API merge.
        check("main", merge_sha)
    return merge_sha


def release_main():
    sha = fetch("main")
    upstream = release_identity(sha)
    tag = upstream["tag"]
    if api(f"releases/tags/{quote(tag, safe='')}", missing=True) is not None:
        return
    verify_release_coverage(sha, upstream)
    check("main", sha)
    if main_sha() != sha:
        raise RuntimeError("main changed after release checks")
    run = dispatch("release.yml", "main", {"publish": True})
    wait_for_runs([run], "main", sha)
    release = api(f"releases/tags/{quote(tag, safe='')}")
    if release["tag_name"] != tag or release["draft"] or release["prerelease"]:
        raise RuntimeError("GitHub did not publish the expected stable release")
    print(f"Published {tag}.")


def verify_release_coverage(sha, upstream):
    adoption = git("log", "--first-parent", "--diff-merges=first-parent", "--no-patch", "-1", "--format=%H",
                   f"-S{upstream['commit']}", sha, "--", INDEX.as_posix())
    if not adoption or upstream_coverage.read_record(sha) != upstream_coverage.read_record(adoption):
        raise RuntimeError("release commit does not preserve its upstream adoption record")
    base = git("rev-parse", f"{adoption}^1")
    previous = release_identity(base)["commit"]
    expected = upstream_coverage.upstream_files(previous, upstream["commit"])
    if upstream_coverage.problems(previous, upstream["commit"], expected, base, adoption):
        raise RuntimeError("merged release commit lacks complete upstream coverage")


def finish_upstream(update):
    # Also recover publication if an earlier run merged but could not publish.
    release_main()
    if not update:
        return
    branch = f"upstream/{update['tag']}-{update['commit']}"
    if not re.fullmatch(UPSTREAM_BRANCH, branch):
        raise ValueError("invalid upstream proposal identity")
    owner = os.environ["GITHUB_REPOSITORY"].split("/")[0]
    for pr in open_pulls(head=f"{owner}:{branch}", base="main"):
        if not owned(pr, UPSTREAM_BRANCH):
            continue
        base = fetch("main")
        head = fetch(branch)
        if head != pr["head"]["sha"]:
            raise RuntimeError("proposal branch changed")
        if release_identity(head) != {**release_identity(base), "tag": update["tag"], "commit": update["commit"]}:
            print("The upstream port remains partial. It will be picked up on a later monitor run.")
            return
        if git("merge-base", base, head) != base:
            print("The upstream proposal is based on older main. The next proposal run will refresh it.")
            return
        if release_identity(base)["commit"] != update["base"]:
            raise RuntimeError("main adopted a different upstream baseline")
        expected = upstream_coverage.upstream_files(update["base"], update["commit"])
        outstanding = upstream_coverage.problems(update["base"], update["commit"], expected, base, head)
        if outstanding:
            print("Upstream coverage is pending: " + "; ".join(dict.fromkeys(outstanding)))
            return
        merge(pr, base, UPSTREAM_BRANCH)
        release_main()


if __name__ == "__main__":
    if sys.argv[1:] == ["pr-verification"]:
        pr_verification()
    else:
        finish_upstream(json.loads(os.environ["UPDATE"]) if os.environ.get("UPDATE") else None)
