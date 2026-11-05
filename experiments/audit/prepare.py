"""Two isolated archives and the same corrected, fixed-endpoint probe."""
import argparse
import hashlib
import json
import pathlib
import shutil
import subprocess
import tempfile

ROOT = pathlib.Path(__file__).resolve().parents[2]
OUT = ROOT / "target/erasure-audit"
BASELINE = "c4b2554398ffcbf35cdaa9c8797504d8df146bd9"


def prepare(candidate="HEAD"):
    OUT.mkdir(parents=True, exist_ok=True)
    resolved = subprocess.check_output(
        ["git", "rev-parse", "bench-pr3-main-20261105^{commit}"], cwd=ROOT, text=True
    ).strip()
    assert resolved == BASELINE, "baseline tag moved"
    state = {}
    for name, ref in {"main": BASELINE, "erase": candidate}.items():
        sha = subprocess.check_output(["git", "rev-parse", f"{ref}^{{commit}}"], cwd=ROOT, text=True).strip()
        dest = pathlib.Path(tempfile.mkdtemp(prefix=name + "-", dir=OUT))
        archive = subprocess.check_output(["git", "archive", sha], cwd=ROOT)
        subprocess.run(["tar", "-x", "-C", str(dest)], input=archive, check=True)
        shutil.copyfile(ROOT / "experiments/audit/worker_probe.rs", dest / "src/worker_probe.rs")
        with (dest / "src/lib.rs").open("a", encoding="utf-8") as f:
            f.write('\n#[cfg(target_os="linux")] pub mod worker_probe;\n')
        (dest / "src/bin/erasure-probe.rs").write_text("fn main(){protun_next::worker_probe::run();}\n", encoding="utf-8")
        with (dest / "Cargo.toml").open("a", encoding="utf-8") as f:
            f.write('\n[[bin]]\nname="erasure-probe"\npath="src/bin/erasure-probe.rs"\n')
        digest = hashlib.sha256()
        for file in sorted([*dest.rglob("*.rs"), *dest.rglob("Cargo.toml"), *dest.rglob("Cargo.lock")]):
            digest.update(file.relative_to(dest).as_posix().encode() + b"\0")
            digest.update(file.read_bytes().replace(b"\r\n", b"\n") + b"\0")
        state[name] = dict(sha=sha, path=str(dest), archive_sha256=hashlib.sha256(archive).hexdigest(), effective_sources_sha256=digest.hexdigest())
    assert state["main"]["effective_sources_sha256"] != state["erase"]["effective_sources_sha256"]
    (OUT / "builds.json").write_text(json.dumps(state, indent=2), encoding="utf-8")
    return state


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--candidate", default="HEAD")
    args = parser.parse_args()
    print(json.dumps(prepare(args.candidate)))
