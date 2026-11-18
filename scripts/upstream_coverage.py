"""Check the per-file record for a proposed upstream release adoption."""

import json
from pathlib import Path
import re
import subprocess

from compatibility import UPSTREAM

REPORT = Path("tests/fixtures/upstream-adoption.json")
METADATA = {REPORT.as_posix(), "tests/fixtures/compatibility.json", "src/requirements.json",
            "Cargo.toml", "Cargo.lock"}
COMPLETE_STATUSES = {"ported", "already-covered", "not-applicable"}


def git(*args):
    return subprocess.check_output(["git", *args])


def names(*args):
    return set(git(*args).decode("utf-8").split("\0")) - {""}


def upstream_files(previous, upstream):
    if any(not re.fullmatch(r"[0-9a-f]{40}", commit) for commit in (previous, upstream)):
        raise ValueError("invalid upstream commit in adoption record")
    subprocess.run(["git", "fetch", "--no-tags", f"{UPSTREAM}.git", previous, upstream],
                   check=True, timeout=120)
    return changes(previous, upstream)


def changes(base, head=None):
    refs = ["--cached", base] if head is None else [base, head]
    return names("diff", "--name-only", "--no-renames", "-z", *refs)


def tracked(ref=None):
    if ref is None:
        return names("ls-files", "-z")
    return names("ls-tree", "-r", "--name-only", "-z", ref)


def validate(record, previous, upstream, expected, before, after, modified):
    if not isinstance(record, dict) or (record.get("from"), record.get("to")) != (previous, upstream):
        return ["adoption record does not identify the expected upstream comparison"]
    rows = record.get("files")
    if not isinstance(rows, list):
        return ["adoption record must contain a file list"]
    issues = []
    seen = set()
    for row in rows:
        path = row.get("path") if isinstance(row, dict) else None
        if not isinstance(path, str) or path not in expected or path in seen:
            issues.append("unknown or duplicate upstream path")
            continue
        seen.add(path)
        status = row.get("status")
        if not isinstance(status, str) or status not in COMPLETE_STATUSES:
            issues.append("upstream path has no completed disposition")
            continue
        reason, refs = row.get("reason"), row.get("evidence")
        if not isinstance(reason, str) or not reason.strip() or not isinstance(refs, list) or not refs:
            issues.append("completed entry requires a reason and evidence paths")
            continue
        if any(not isinstance(ref, str) for ref in refs):
            issues.append("evidence paths must be strings")
            continue
        evidence = set(refs)
        # Removed files can be evidence for an intentional downstream deletion.
        if not evidence <= (before | after) - METADATA:
            issues.append("evidence must refer to downstream files rather than version metadata")
        elif status == "ported" and not evidence & modified:
            issues.append("ported entry does not reference a downstream change")
        elif status == "already-covered" and not evidence <= before & after:
            issues.append("already-covered evidence must exist before and after adoption")
    if missing := expected - seen:
        issues.append(f"{len(missing)} upstream files are missing from the record")
    return issues


def read_record(ref=None):
    try:
        if ref is None:
            return json.loads(REPORT.read_text(encoding="utf-8"))
        payload = subprocess.run(["git", "show", f"{ref}:{REPORT.as_posix()}"],
                                 capture_output=True, check=False)
        if payload.returncode == 0:
            return json.loads(payload.stdout)
    except (OSError, ValueError):
        pass
    return None


def seed(previous, upstream, expected):
    record = read_record()
    if isinstance(record, dict) and (record.get("from"), record.get("to")) == (previous, upstream):
        return
    template = {"from": previous, "to": upstream,
                "files": [{"path": path, "status": "unresolved", "evidence": [], "reason": ""}
                          for path in sorted(expected)]}
    REPORT.write_text(json.dumps(template, indent=2) + "\n", encoding="utf-8")


def problems(previous, upstream, expected, base, head=None):
    return validate(read_record(head), previous, upstream, expected,
                    tracked(base), tracked(head), changes(base, head))
