"""Two isolated archives and the same corrected, fixed-endpoint probe."""
import argparse
import hashlib
import json
import os
import pathlib
import shutil
import subprocess
import tempfile

ROOT = pathlib.Path(__file__).resolve().parents[2]
OUT = ROOT / "target/erasure-audit"
BASELINE = "f7bf03f5b4dc3981eaadedbf3a7841c5ed5ee5c1"


def prepare(candidate="HEAD", baseline=BASELINE):
    OUT.mkdir(parents=True, exist_ok=True)
    state = {}
    for name, ref in {"main": baseline, "erase": candidate}.items():
        sha = subprocess.check_output(["git", "rev-parse", f"{ref}^{{commit}}"], cwd=ROOT, text=True).strip()
        dest = pathlib.Path(tempfile.mkdtemp(prefix=name + "-", dir=OUT))
        archive = subprocess.check_output(["git", "archive", sha], cwd=ROOT)
        subprocess.run(["tar", "-x", "-C", str(dest)], input=archive, check=True)
        prototype = None
        if name == "erase" and os.environ.get("NEXT_RECEIVE_PROTOTYPE", "0") == "1":
            patch = ROOT / "experiments/receive/stage4-pinned.patch"
            prototype = hashlib.sha256(patch.read_bytes()).hexdigest()
            assert prototype == "dd5f493c21ac34fe2dddff41d117affcd0a96968ffb748fbfaba59de8a3f0d38", "prototype identity changed"
            # Windows archive export can apply working-tree CRLF filters.
            # The patch uses stored LF for ordinary sources; vendor bytes retain
            # their declared CRLF contract. Fingerprints already normalize EOLs.
            for file in (dest / "src").rglob("*.rs"):
                file.write_bytes(file.read_bytes().replace(b"\r\n", b"\n"))
            # Give only this disposable archive its own working-tree boundary.
            # The checked patch modifies src/ and the declared vendor surface;
            # the user's checkout and production build are never patched.
            subprocess.run(["git", "init", "--quiet", str(dest)], check=True)
            subprocess.run(["git", "-c", "core.autocrlf=false", "apply", "--whitespace=nowarn", str(patch)], cwd=dest, check=True)
        runtime = hashlib.sha256()
        for file in sorted([*dest.joinpath("src").rglob("*.rs"), *dest.joinpath("vendor").rglob("*.rs"), dest / "Cargo.toml", dest / "Cargo.lock"]):
            runtime.update(file.relative_to(dest).as_posix().encode() + b"\0")
            runtime.update(file.read_bytes().replace(b"\r\n", b"\n") + b"\0")
        shutil.copyfile(ROOT / "experiments/audit/worker_probe.rs", dest / "src/worker_probe.rs")
        shutil.copyfile(ROOT / "experiments/download/probe.rs", dest / "src/download_probe.rs")
        # Diagnostic-only overlay, identical on both variants. Never log keys or
        # packet bodies. Rejected packets invalidate a sample; classify them
        # before interpreting performance. Included in effective source identity.
        session = dest / "src/session.rs"
        text = session.read_text(encoding="utf-8")
        original = 'TunnResult::Err(e) => Err(Error::Protocol(format!("WireGuard: {e:?}"))),'
        assert text.count(original) == 1
        session.write_text(text.replace(original, 'TunnResult::Err(e) => { eprintln!("DOWNLOAD_SESSION_ERROR {e:?}"); Err(Error::Protocol(format!("WireGuard: {e:?}"))) },'), encoding="utf-8")
        text = session.read_text(encoding="utf-8")
        original = 'let result = Self::output(self.tunnel.decapsulate(None, packet, output));'
        assert text.count(original) == 1
        session.write_text(text.replace(original, '''if packet.first().is_some_and(|kind| *kind != 4) {
            eprintln!("DOWNLOAD_CONTROL pid={} len={} header={:?}", std::process::id(), packet.len(), packet.get(..packet.len().min(12)));
        }
        let result = Self::output(self.tunnel.decapsulate(None, packet, output));'''), encoding="utf-8")
        text = session.read_text(encoding="utf-8")
        original = 'TunnResult::WriteToNetwork(p) => Ok(Output::Network(p.len())),'
        assert text.count(original) == 1
        session.write_text(text.replace(original, '''TunnResult::WriteToNetwork(p) => {
            if p.first().is_some_and(|kind| *kind != 4) {
                eprintln!("DOWNLOAD_CONTROL_SEND pid={} len={} header={:?}", std::process::id(), p.len(), p.get(..p.len().min(12)));
            }
            Ok(Output::Network(p.len()))
        },'''), encoding="utf-8")
        with (dest / "src/lib.rs").open("a", encoding="utf-8") as f:
            f.write('\n#[cfg(target_os="linux")] pub mod worker_probe;\n')
        (dest / "src/bin/erasure-probe.rs").write_text("fn main(){protun_next::worker_probe::run();}\n", encoding="utf-8")
        with (dest / "Cargo.toml").open("a", encoding="utf-8") as f:
            f.write('\n[[bin]]\nname="erasure-probe"\npath="src/bin/erasure-probe.rs"\n')
        digest = hashlib.sha256()
        for file in sorted([*dest.rglob("*.rs"), *dest.rglob("Cargo.toml"), *dest.rglob("Cargo.lock")]):
            digest.update(file.relative_to(dest).as_posix().encode() + b"\0")
            digest.update(file.read_bytes().replace(b"\r\n", b"\n") + b"\0")
        state[name] = dict(sha=sha, path=str(dest), archive_sha256=hashlib.sha256(archive).hexdigest(), effective_sources_sha256=digest.hexdigest(), runtime_sources_sha256=runtime.hexdigest())
        if prototype:
            state[name].update(prototype_patch_sha256=prototype, prototype_source_commit="543675241b948a6d3b8c656bb39feb7450b9470a")
    # Equal packet sources are a legitimate A/A characterization, not a speedup.
    (OUT / "builds.json").write_text(json.dumps(state, indent=2), encoding="utf-8")
    return state


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--candidate", default="HEAD")
    parser.add_argument("--baseline", default=BASELINE)
    args = parser.parse_args()
    print(json.dumps(prepare(args.candidate, args.baseline)))
