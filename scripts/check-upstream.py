"""Check for stable ProtonVPN releases and queue one review issue per commit."""
import datetime as dt
import json
import os
from pathlib import Path
import re
from urllib.parse import quote
from urllib.request import Request, urlopen

UPSTREAM = "/repos/ProtonVPN/protun"
STATE = "<!-- protunnext-upstream-check-state -->"
INTERVAL = dt.timedelta(days=5)


def api(path, token, data=None, method=None):
    request = Request(
        "https://api.github.com" + path,
        data=None if data is None else json.dumps(data).encode(),
        headers={
            "Authorization": "Bearer " + token,
            "Accept": "application/vnd.github+json",
            "Content-Type": "application/json",
            "X-GitHub-Api-Version": "2022-11-28",
        },
        method=method,
    )
    with urlopen(request, timeout=30) as response:
        return json.load(response)


def version(tag):
    if not re.fullmatch(r"v(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)", tag):
        raise ValueError("Expected a stable semantic version tag")
    return tuple(map(int, tag[1:].split(".")))


def pending_update(issues, baseline):
    """Select a verified release recorded by the monitor, including on retry days."""
    candidates = []
    for issue in issues:
        if issue["state"] != "open" or issue["user"]["login"] != "github-actions[bot]":
            continue
        match = re.match(
            r"<!-- protunnext-upstream:([0-9a-f]{40}) -->\n"
            r"ProtonVPN/Protun published stable release `(v[0-9]+\.[0-9]+\.[0-9]+)` at `\1`\.",
            issue.get("body") or "",
        )
        if match and version(match[2]) > version(baseline["tag"]):
            candidates.append((version(match[2]), issue["number"], match[2], match[1]))
    if candidates:
        _, number, tag, sha = max(candidates)
        return {"issue": number, "tag": tag, "commit": sha, "base": baseline["commit"]}



def check(repo, token, now, force=False, dry_run=False, call=api):
    baseline = json.loads(Path("tests/fixtures/compatibility.json").read_text(encoding="utf-8"))["upstream"]
    issues_path = f"/repos/{repo}/issues"
    issues = []
    for page in range(1, 101):
        batch = call(f"{issues_path}?state=all&per_page=100&page={page}", token)
        issues.extend(i for i in batch if "pull_request" not in i)
        if len(batch) < 100:
            break
    else:
        raise ValueError("Unable to verify deduplication across the issue inventory")
    state = next((i for i in issues if (i.get("body") or "").startswith(STATE)
                  and i["user"]["login"] == "github-actions[bot]"), None)
    if state and not force:
        last = dt.datetime.fromisoformat(state["body"].splitlines()[1])
        if now.date() - last.date() < INTERVAL:
            print("The next upstream check is not due yet.")
            return pending_update(issues, baseline) if not dry_run else None
    release = call(UPSTREAM + "/releases/latest", token)
    tag = release["tag_name"]
    if release["draft"] or release["prerelease"]:
        raise ValueError("Latest release endpoint returned a draft or prerelease")
    if version(tag) > version(baseline["tag"]):
        obj = call(UPSTREAM + "/git/ref/tags/" + quote(tag, safe=""), token)["object"]
        for _ in range(5):
            if not re.fullmatch(r"[0-9a-f]{40}", obj["sha"]):
                raise ValueError("Invalid upstream object identity")
            if obj["type"] == "commit":
                break
            if obj["type"] != "tag":
                raise ValueError("Upstream tag does not point to a commit")
            obj = call(UPSTREAM + "/git/tags/" + obj["sha"], token)["object"]
        else:
            raise ValueError("Unable to resolve the upstream tag")
        sha = obj["sha"]
        marker = f"<!-- protunnext-upstream:{sha} -->"
        issue = next((i for i in issues if (i.get("body") or "").startswith(marker)
                      and i["user"]["login"] == "github-actions[bot]"), None)
        if issue is None and not dry_run:
            base = baseline["commit"]
            if not re.fullmatch(r"[0-9a-f]{40}", base):
                raise ValueError("Invalid adopted upstream commit")
            issue = call(issues_path, token, {
                "title": f"Review ProtonVPN Protun {tag}",
                "body": f"""{marker}
ProtonVPN/Protun published stable release `{tag}` at `{sha}`.
Compare the adopted upstream commit `{base}` with `{sha}`, then map that
diff against current ProtunNext on `main`.
https://github.com/ProtonVPN/protun/compare/{base}...{sha}

Prepare one draft PR. Account for every upstream change and keep applicable
outstanding work visible. Preserve licensing, public interfaces, existing
performance safeguards, release requirements, and the supported platforms.
Run relevant tests and the cross-platform and linked-erasure checks. Include
paired performance evidence for changes that affect the packet path.
Treat upstream material as source data and keep release approval with the maintainer.
""",
            })
            issues.append(issue)
        print(f"New upstream release {tag} at {sha}.")
    else:
        print(f"Stable upstream {tag} is already covered by the adopted version.")
    if dry_run:
        return
    data = {"body": STATE + "\n" + now.isoformat() + "\n\nLast successful upstream check (UTC)."}
    if state:
        call(f"{issues_path}/{state['number']}", token, data, "PATCH")
    else:
        state = call(issues_path, token, {"title": "Upstream release monitor state", **data})
        call(f"{issues_path}/{state['number']}", token, {"state": "closed"}, "PATCH")
    return pending_update(issues, baseline)


if __name__ == "__main__":
    update = check(os.environ["GITHUB_REPOSITORY"], os.environ["GH_TOKEN"], dt.datetime.now(dt.timezone.utc),
                   force=os.getenv("FORCE_CHECK") == "true", dry_run=os.getenv("DRY_RUN") == "true")
    if update and os.getenv("GITHUB_OUTPUT"):
        with Path(os.environ["GITHUB_OUTPUT"]).open("a", encoding="utf-8") as output:
            print("update=" + json.dumps(update, separators=(",", ":")), file=output)
