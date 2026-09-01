import json
import sys
from pathlib import Path


runtime_root = Path(sys.argv[1]).resolve()
metadata = json.load(sys.stdin)
expected = {
    "gpui-ce": runtime_root / "crates/gpui/Cargo.toml",
    "gpui_ce_platform": runtime_root / "crates/gpui_platform/Cargo.toml",
    "gpui_ce_web": runtime_root / "crates/gpui_web/Cargo.toml",
    "gpui_ce_macros": runtime_root / "crates/gpui_macros/Cargo.toml",
}

failures = []
for name, expected_manifest in expected.items():
    matches = [package for package in metadata["packages"] if package["name"] == name]
    manifests = {Path(package["manifest_path"]).resolve() for package in matches}
    if manifests != {expected_manifest.resolve()}:
        failures.append(f"{name}: expected {expected_manifest}, resolved {sorted(map(str, manifests))}")

if failures:
    raise SystemExit("GPUI-CE runtime patch was not applied exactly:\n" + "\n".join(failures))
