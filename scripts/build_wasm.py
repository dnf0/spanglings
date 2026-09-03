"""WebAssembly Build Pipeline for Spanglings.

This script compiles the Spanglings WebAssembly engine using `wasm-pack`,
outputs artifacts into `docs/assets/playground/pkg/`, cleans any generated
`.gitignore` so artifacts are trackable for GitHub Pages deployment, synchronizes
the fallback JSON bundle, and validates generated binary sizes.
"""

from __future__ import annotations

import shutil
import subprocess
import sys
from pathlib import Path


def find_wasm_pack_cmd() -> list[str]:
    """Determine whether native `wasm-pack` or `npx wasm-pack` should be used.

    Returns:
        List containing the base command and arguments.

    Raises:
        RuntimeError: If neither wasm-pack nor npx is available.
    """
    if shutil.which("wasm-pack"):
        return ["wasm-pack"]
    if shutil.which("npx"):
        return ["npx", "--yes", "wasm-pack"]
    raise RuntimeError(
        "Neither 'wasm-pack' nor 'npx' was found in system PATH. Please install wasm-pack."
    )


def build_wasm(repo_root: Path | None = None) -> Path:
    """Build the WebAssembly module and synchronize playground assets.

    Args:
        repo_root: Optional repository root path.

    Returns:
        Path to the output `pkg` directory.

    Raises:
        RuntimeError: If wasm-pack build fails or required artifacts are missing.
    """
    if repo_root is None:
        repo_root = Path(__file__).resolve().parent.parent

    pkg_dir = repo_root / "docs" / "assets" / "playground" / "pkg"
    pkg_dir.mkdir(parents=True, exist_ok=True)

    wasm_pack_cmd = find_wasm_pack_cmd()
    build_args = [
        *wasm_pack_cmd,
        "build",
        "--target",
        "web",
        "--out-dir",
        str(pkg_dir),
        "--release",
        "--features",
        "wasm",
    ]

    print(f"⚙ Executing WebAssembly build: {' '.join(build_args)}")
    res = subprocess.run(build_args, cwd=repo_root, check=False)
    if res.returncode != 0:
        raise RuntimeError(f"wasm-pack build failed with exit code {res.returncode}")

    # Remove generated .gitignore so git tracks pkg for GitHub Pages deployment
    pkg_gitignore = pkg_dir / ".gitignore"
    if pkg_gitignore.exists():
        pkg_gitignore.unlink()
        print("✓ Removed generated docs/assets/playground/pkg/.gitignore")

    # Synchronize fallback playground bundle via subprocess
    print("⚙ Synchronizing fallback playground JSON bundle...")
    bundle_script = repo_root / "scripts" / "build_playground_bundle.py"
    res_bundle = subprocess.run(
        [sys.executable, str(bundle_script)],
        cwd=repo_root,
        check=False,
    )
    if res_bundle.returncode != 0:
        raise RuntimeError(
            f"build_playground_bundle.py failed with exit code {res_bundle.returncode}"
        )

    # Validate generated artifacts
    js_file = pkg_dir / "spanglings.js"
    wasm_file = pkg_dir / "spanglings_bg.wasm"
    dts_file = pkg_dir / "spanglings.d.ts"

    if not js_file.exists() or js_file.stat().st_size == 0:
        raise RuntimeError(f"Build failed: {js_file} is missing or empty")
    if not wasm_file.exists() or wasm_file.stat().st_size == 0:
        raise RuntimeError(f"Build failed: {wasm_file} is missing or empty")

    js_size_kb = js_file.stat().st_size / 1024
    wasm_size_kb = wasm_file.stat().st_size / 1024
    dts_size_kb = dts_file.stat().st_size / 1024 if dts_file.exists() else 0.0

    print("==================================================")
    print("✓ WebAssembly build successful!")
    print(f"  • JavaScript glue:     {js_file.name} ({js_size_kb:.2f} KB)")
    print(f"  • WebAssembly binary:  {wasm_file.name} ({wasm_size_kb:.2f} KB)")
    if dts_file.exists():
        print(f"  • TypeScript types:    {dts_file.name} ({dts_size_kb:.2f} KB)")
    print(f"  • Output Directory:    {pkg_dir}")
    print("==================================================")

    return pkg_dir


if __name__ == "__main__":
    try:
        build_wasm()
    except (subprocess.SubprocessError, RuntimeError, OSError) as exc:
        print(f"✗ Build error: {exc}", file=sys.stderr)
        sys.exit(1)
