"""Prove the release checker rejects a real binary with its Drop barrier removed.

Only an isolated git archive is mutated. Never deploy this insecure control.
"""
import hashlib
import json
import pathlib
import subprocess
import tempfile

from check_codegen import check

ROOT = pathlib.Path(__file__).resolve().parents[2]
OUT = ROOT / "target/erasure-codegen"


def run():
    parent = ROOT / "target/erasure-negative"
    parent.mkdir(parents=True, exist_ok=True)
    source = pathlib.Path(tempfile.mkdtemp(dir=parent, prefix="source-"))
    archive = subprocess.check_output(["git", "archive", "HEAD"], cwd=ROOT)
    subprocess.run(["tar", "-x", "-C", str(source)], input=archive, check=True)
    packet = source / "src/packet.rs"
    text = packet.read_text()
    needle = "zeroize::optimization_barrier(bytes);"
    assert text.count(needle) == 1
    packet.write_text(text.replace(needle, "// Intentionally insecure negative control: barrier removed."))
    with (OUT / "negative-build.log").open("w") as log:
        subprocess.run(["cargo", "build", "--locked", "--release", "--no-default-features", "--features", "unix",
                        "--example", "erasure_codegen", "--manifest-path", str(source / "Cargo.toml"),
                        "--target-dir", str(parent / "cargo")], stdout=log, stderr=subprocess.STDOUT, check=True)
    binary = parent / "cargo/release/examples/erasure_codegen"
    assembly = subprocess.check_output(["objdump", "-Cd", "--no-show-raw-insn", str(binary)], text=True)
    assembly += subprocess.check_output(["objdump", "-R", str(binary)], text=True)
    (OUT / "negative-disassembly.txt").write_text(assembly)
    try:
        check(assembly, roots=("audit_drop_packet",))
    except AssertionError as error:
        assert str(error) == "no recognized zero memset reachable from audit_drop_packet; inspect disassembly", str(error)
    else:
        raise AssertionError("checker incorrectly accepted the barrier-free Drop binary")
    report = dict(expected_rejection=True, binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
                  source_commit=subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
                  mutation="remove optimization_barrier from the packet erasure helper in an isolated archive")
    (OUT / "negative-control.json").write_text(json.dumps(report, indent=2))
    print(json.dumps(report))


if __name__ == "__main__":
    run()
