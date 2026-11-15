"""Shared v3.0.5 coverage and frozen external-contract validation."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PIN = "a498c38ae6ffb0e68e7190b2831ef985d73fa6da"


def validate(index, ledger, contract_text=None):
    if index["upstream"] != {
        "repository": "https://github.com/ProtonVPN/protun", "tag": "v3.0.5", "commit": PIN
    }:
        raise ValueError("upstream baseline changed")
    actual = [item["id"] for item in ledger]
    mapped = [key for group in index["groups"] for key in group["requirements"]]
    if len(set(actual)) != len(actual) or len(set(mapped)) != len(mapped) or set(actual) != set(mapped):
        raise ValueError("missing, duplicate or unknown capability mapping")
    for item in ledger:
        if item["source"].startswith("https://github.com/ProtonVPN/protun/") and f"/blob/{PIN}/" not in item["source"]:
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


def validate_checkout():
    validate(json.loads((ROOT / "tests/fixtures/compatibility.json").read_text(encoding="utf-8")),
             json.loads((ROOT / "src/requirements.json").read_text(encoding="utf-8")))
