#!/usr/bin/env python3
"""
Dogfish Integration Test Package Staging Utility (Milestone M96).

Stages the compiled release DLL and uncompiled Papyrus source scripts into a standard
Skyrim SE Data/ directory tree ready for deployment via Mod Organizer 2 or manual installation.
"""

import os
import shutil
import hashlib
import sys

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
INTEG_ROOT = os.path.abspath(os.path.join(SCRIPT_DIR, ".."))
DEPLOY_ROOT = os.path.join(INTEG_ROOT, "deploy", "package")

DLL_SRC = os.path.join(INTEG_ROOT, "target", "release", "dogfish_integration.dll")
PSC_BRIDGE = os.path.join(INTEG_ROOT, "scripts", "source", "DogfishNativeBridge.psc")
PSC_BANNER = os.path.join(INTEG_ROOT, "scripts", "source", "DogfishClaimBannerScript.psc")

def compute_sha256(path):
    with open(path, "rb") as f:
        return hashlib.sha256(f.read()).hexdigest()

def stage_package():
    print("=" * 60)
    print("Staging Dogfish Skyrim Integration Test Package")
    print("=" * 60)

    if not os.path.exists(DLL_SRC):
        print(f"ERROR: Compiled DLL not found at: {DLL_SRC}")
        print("Please run: cargo build --manifest-path integration/Cargo.toml --release --offline")
        sys.exit(1)

    skse_dir = os.path.join(DEPLOY_ROOT, "Data", "SKSE", "Plugins")
    scripts_dir = os.path.join(DEPLOY_ROOT, "Data", "Source", "Scripts")

    os.makedirs(skse_dir, exist_ok=True)
    os.makedirs(scripts_dir, exist_ok=True)

    # 1. Copy DLL
    dll_dst = os.path.join(skse_dir, "dogfish_integration.dll")
    shutil.copy2(DLL_SRC, dll_dst)
    dll_sha = compute_sha256(dll_dst)
    print(f"[OK] Staged DLL: {dll_dst}")
    print(f"     SHA256: {dll_sha} ({os.path.getsize(dll_dst):,} bytes)")

    # 2. Copy Papyrus Sources
    shutil.copy2(PSC_BRIDGE, os.path.join(scripts_dir, "DogfishNativeBridge.psc"))
    shutil.copy2(PSC_BANNER, os.path.join(scripts_dir, "DogfishClaimBannerScript.psc"))
    print(f"[OK] Staged Papyrus Sources: {scripts_dir}")

    print("-" * 60)
    print(f"Staging complete in: {DEPLOY_ROOT}")
    print("=" * 60)

if __name__ == "__main__":
    stage_package()
