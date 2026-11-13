"""Check project-owned text for local identity leaks, artifacts and broken docs.

This is a repository hygiene guard, not a complete credential scanner or a
history purge. Upstream attribution is preserved and independently hash-checked.
"""
from pathlib import Path
import re
import subprocess
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parent.parent
EMAIL = re.compile(r"\b[A-Z0-9._%+-]+@([A-Z0-9.-]+\.[A-Z]{2,})\b", re.I)
HOME = re.compile(r"\b[A-Z]:[\\/](?:Users|Documents and Settings)[\\/][^\s\"<>]+|/(?:Users|home)/[^/\s\"<>]+", re.I)
PUBLIC_DOMAINS = {"example.com", "example.net", "example.org", "localhost"}
ARTIFACT_SUFFIXES = {".profraw", ".profdata", ".pcap", ".pcapng", ".p12", ".pfx", ".key", ".pem", ".zip"}


def privacy_findings(text):
    findings = []
    if HOME.search(text):
        findings.append("literal local home/profile path")
    if any(m[1].lower() not in PUBLIC_DOMAINS for m in EMAIL.finditer(text)):
        findings.append("non-example email address")
    return findings


def link_findings(path, text, root):
    findings = []
    # Fenced shell/code examples are not Markdown link targets.
    text = re.sub(r"```.*?```", "", text, flags=re.S)
    for target in re.findall(r"!?\[[^\]]*\]\(([^)]+)\)", text):
        target = target.strip().split(' "', 1)[0].strip("<>")
        parts = urlsplit(target)
        if parts.scheme or parts.netloc or not parts.path:
            continue
        resolved = (path.parent / unquote(parts.path)).resolve()
        if not resolved.is_relative_to(root) or not resolved.exists():
            findings.append("missing or outside-repository Markdown target")
    return findings


def main():
    names = subprocess.check_output(
        ["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard"], cwd=ROOT
    ).decode("utf-8").split("\0")
    errors = []
    checked = 0
    for name in sorted(set(filter(None, names))):
        path = ROOT / name
        if not path.is_file() or name.startswith("vendor/") or name == "LICENSE":
            continue
        if name.startswith(("experiments/", "docs/benchmarks/")) or path.suffix in ARTIFACT_SUFFIXES:
            errors.append((name, "research/private/generated artifact in tracked tree"))
        if path.suffix == ".der" and name.startswith("tests/fixtures/"):
            continue  # Intentionally public synthetic crypto fixtures.
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeError:
            errors.append((name, "unexpected non-text project file"))
            continue
        checked += 1
        errors.extend((name, reason) for reason in privacy_findings(text))
        if path.suffix == ".md":
            errors.extend((name, reason) for reason in link_findings(path, text, ROOT))
    for name, reason in errors:
        print(f"{name}: {reason}")  # Never print the matched personal content.
    if errors:
        raise SystemExit(1)
    print(f"Checked {checked} project text files; privacy/artifact and documentation checks passed.")


if __name__ == "__main__":
    main()
