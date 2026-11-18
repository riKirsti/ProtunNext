"""Validate the adopted upstream release and its frozen external contracts."""
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys
import tomllib

ROOT = Path(__file__).resolve().parent.parent
INDEX = Path("tests/fixtures/compatibility.json")
UPSTREAM = "https://github.com/ProtonVPN/protun"
STABLE = r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)"


def validate(index, ledger, contract_text=None):
    upstream = index["upstream"]
    validate_upstream(upstream)
    actual = [item["id"] for item in ledger]
    mapped = [key for group in index["groups"] for key in group["requirements"]]
    if len(set(actual)) != len(actual) or len(set(mapped)) != len(mapped) or set(actual) != set(mapped):
        raise ValueError("missing, duplicate or unknown capability mapping")
    for item in ledger:
        if item["source"].startswith(f"{UPSTREAM}/") and not item["source"].startswith(
            f"{UPSTREAM}/blob/{upstream['commit']}/"
        ):
            raise ValueError("unpinned upstream requirement")
    paths = [index["contracts"]]
    for group in index["groups"]:
        if not group["implementation"] or not group["checks"]:
            raise ValueError("capability lacks implementation/check mapping")
        paths.extend(group["implementation"] + group["checks"])
    for path in paths:
        target = (ROOT / path).resolve()
        if not target.is_relative_to(ROOT) or not target.is_file():
            raise ValueError("missing or outside-repository mapping")
    if contract_text is None:
        contract_text = (ROOT / index["contracts"]).read_text(encoding="utf-8")
    if index.get("contracts_sha256") != contract_digest(contract_text):
        raise ValueError("frozen public contract changed; explicit compatibility review required")


def contract_digest(text):
    return hashlib.sha256(text.replace("\r\n", "\n").replace("\r", "\n").encode("utf-8")).hexdigest()


def validate_upstream(upstream):
    if (upstream["repository"] != UPSTREAM or not re.fullmatch("v" + STABLE, upstream["tag"])
            or not re.fullmatch(r"[0-9a-f]{40}", upstream["commit"])):
        raise ValueError("invalid adopted upstream release identity")


def validate_versions(upstream, package, lock_packages):
    validate_upstream(upstream)
    versions = [item["version"] for item in lock_packages
                if item["name"] == package["name"] and "source" not in item]
    if upstream["tag"] != "v" + package["version"] or versions != [package["version"]]:
        raise ValueError("adopted upstream tag, Cargo package and lockfile versions must agree")


def adopted_release(root=ROOT):
    return json.loads((root / INDEX).read_text(encoding="utf-8"))["upstream"]


def check_versions(upstream, root=ROOT):
    validate_versions(upstream, tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8"))["package"],
                      tomllib.loads((root / "Cargo.lock").read_text(encoding="utf-8"))["package"])


def verify_tag(upstream):
    ref = "refs/tags/" + upstream["tag"]
    output = subprocess.check_output(
        ["git", "ls-remote", UPSTREAM + ".git", ref, ref + "^{}"], text=True, timeout=30)
    refs = {name: sha for sha, name in map(str.split, output.splitlines())}
    if refs.get(ref + "^{}", refs.get(ref)) != upstream["commit"]:
        raise ValueError("adopted commit does not match the published upstream tag")


def validate_checkout():
    index = json.loads((ROOT / INDEX).read_text(encoding="utf-8"))
    validate(index, json.loads((ROOT / "src/requirements.json").read_text(encoding="utf-8")))
    check_versions(index["upstream"])
    return index["upstream"]


if __name__ == "__main__":
    release = validate_checkout()
    if sys.argv[1:] == ["--verify-tag"]:
        verify_tag(release)
    elif sys.argv[1:]:
        raise SystemExit("usage: compatibility.py [--verify-tag]")
