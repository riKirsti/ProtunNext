"""Check the actual AAR and executed tests after the AGP source-set migration."""
import io
from pathlib import Path
import xml.etree.ElementTree as xml
import zipfile

root = Path(__file__).resolve().parent.parent / "android" / "build"
packages = list((root / "outputs/aar").glob("*-release.aar"))
assert len(packages) == 1, "Expected one release AAR"
with zipfile.ZipFile(packages[0]) as package:
    assert package.read("jni/arm64-v8a/libprotun_next.so")[:4] == b"\x7fELF"
    with zipfile.ZipFile(io.BytesIO(package.read("classes.jar"))) as classes:
        prefix = "io/github/rikirsti/protunnext/"
        for name in ["android/ProtunNextCore.class", "android/ProtunVpnService.class", "generated/Connection.class"]:
            assert prefix + name in classes.namelist(), "Missing packaged class: " + name
reports = list((root / "test-results/testDebugUnitTest").glob("TEST-*.xml"))
assert reports and sum(int(xml.parse(p).getroot().get("tests", "0")) for p in reports) > 0, "No Android unit tests executed"
print("Release AAR contains service, core, generated connection and ARM64 ELF; Android tests executed")
