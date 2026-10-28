#!/usr/bin/env python3
"""
scripts/verify_papyrus.py
Reproducible Papyrus Script and Symbol Verification Utility for Conquest of Skyrim v0.3.9
Part of Dogfish Specification Recovery Program (Phase 10: M38)

Usage:
    python -I scripts/verify_papyrus.py [--source-dir PATH] [--output JSON_PATH]
"""

import argparse
import json
import re
import sys
from pathlib import Path

DEFAULT_SOURCE_DIR = Path("references/local/conquest-of-skyrim/Data/scripts/source")

# Critical symbols and structures to verify across the core controller scripts
CORE_SYMBOL_SPEC = {
    "cs_govscriptfunctions.psc": {
        "parent": "Quest",
        "properties": ["Steward", "ASIC", "Blacksmith", "CourtMage", "Throne", "ChoppingBlock", "AllLocations", "AllHoldings"],
        "functions": ["SetUpFactionOwners"]
    },
    "cs_transferownershipscript.psc": {
        "parent": "Quest",
        "properties": ["AllHoldings", "AllHoldingsFort", "AllLocations", "CrimeFactions", "PlayerMarkers", "FortMarkersPlayer"],
        "functions": ["TransferOwnershipCity", "TransferOwnershipFort", "TransferOwnershipVillage", "MakeCitizen", "RemoveGovernment"]
    },
    "cs_attackcityscript.psc": {
        "parent": "Quest",
        "properties": ["CS_BattleSize", "CS_BattleSpeed", "TroopsRemainingDefenders", "RetreatFaction", "BatteringRamEnabled", "AllHoldings"],
        "functions": ["RetreatDefenders"]
    },
    "cs_meetingscript.psc": {
        "parent": "Quest",
        "properties": ["Throne", "Steward", "Jarls", "MootVoteFor", "MootVoteAgainst", "AllLocations", "LoyaltyCheck"],
        "functions": ["GetJarls", "StartMeeting", "StartMoot", "StartMootVote", "JarlNextVote", "FindJarlNominator", "EndMoot"]
    },
    "cs_coronationscript.psc": {
        "parent": "Quest",
        "properties": ["IsHighKing", "HighKingCrown", "HighQueenCrown", "JarlPledgeScene", "LoyaltyCheck"],
        "functions": ["SetupThrone", "GetKeyPersonnel", "StartJarlPledges", "NextJarlPledge", "CitizensKneel", "JarlsKneel", "AllCheer", "AddLoyalty"]
    },
    "cs_negotiationscript.psc": {
        "parent": "Quest",
        "properties": ["WarScore", "TruceImp", "TruceSons", "Reparations", "HostOffer", "GuestCounterOffer", "TotalTroopCount"],
        "functions": ["EvaluateNegotiatingPackages", "MoveGuestsToTable", "MakeFriends", "UnMakeFriends", "CalculateSuccess", "CalculateArmySize"]
    },
    "cs_holdingeconomyupdaterscript.psc": {
        "parent": "Quest",
        "properties": ["holdingnum", "AllowUpdate"],
        "functions": ["UpdateHoldingResources"]
    },
    "cs_troopupgradescript.psc": {
        "parent": "Quest",
        "properties": ["CostGold", "CostMetal", "Treasury", "Metal", "Level", "LevelNext"],
        "functions": ["SetUp", "MenuStart", "MenuUpgrade", "UpgradeTroop"]
    },
    "cs_bountyscript.psc": {
        "parent": "Quest",
        "properties": ["Commander", "TroopCountCertain", "PlayerDecision"],
        "functions": ["SimulateCasualties", "PlayerChangedLocation"]
    },
    "cs_checkversion.psc": {
        "parent": "ReferenceAlias",
        "properties": ["TMImps", "TMSons", "CourtScript"],
        "events": ["OnPlayerLoadGame"]
    },
    "cs_patrolholdscript.psc": {
        "parent": "Quest",
        "properties": ["TaskBarrelFollow", "TaskBarrelHunt", "TaskBarrelNoOrder", "TaskBarrelsAttackCity", "TaskBarrelsDefendFort"],
        "functions": []
    }
}

def parse_args():
    parser = argparse.ArgumentParser(description="Verify Papyrus script symbols and properties.")
    parser.add_argument(
        "--source-dir",
        type=Path,
        default=DEFAULT_SOURCE_DIR,
        help="Path to uncompiled Papyrus .psc source directory."
    )
    parser.add_argument(
        "--output",
        type=Path,
        default=None,
        help="Optional path to write JSON symbol verification report."
    )
    return parser.parse_args()

def verify_script(script_path: Path, spec: dict):
    if not script_path.exists():
        return {
            "exists": False,
            "error": "File does not exist"
        }

    text = script_path.read_text(encoding="utf-8", errors="replace")

    # 1. Class name and parent class
    class_match = re.search(r'^\s*Scriptname\s+(\w+)\s+extends\s+(\w+)', text, re.IGNORECASE | re.MULTILINE)
    parent_actual = class_match.group(2) if class_match else None
    parent_ok = parent_actual.lower() == spec["parent"].lower() if parent_actual else False

    # 2. Properties
    found_props = []
    missing_props = []
    for prop in spec.get("properties", []):
        prop_pattern = rf'\bProperty\s+{prop}\b'
        if re.search(prop_pattern, text, re.IGNORECASE):
            found_props.append(prop)
        else:
            missing_props.append(prop)

    # 3. Functions
    found_funcs = []
    missing_funcs = []
    for func in spec.get("functions", []):
        func_pattern = rf'^\s*Function\s+{func}\s*\('
        if re.search(func_pattern, text, re.IGNORECASE | re.MULTILINE):
            found_funcs.append(func)
        else:
            missing_funcs.append(func)

    # 4. Events
    found_events = []
    missing_events = []
    for ev in spec.get("events", []):
        ev_pattern = rf'^\s*Event\s+{ev}\s*\('
        if re.search(ev_pattern, text, re.IGNORECASE | re.MULTILINE):
            found_events.append(ev)
        else:
            missing_events.append(ev)

    is_valid = (
        parent_ok and
        len(missing_props) == 0 and
        len(missing_funcs) == 0 and
        len(missing_events) == 0
    )

    return {
        "exists": True,
        "parent_expected": spec["parent"],
        "parent_actual": parent_actual,
        "parent_ok": parent_ok,
        "found_properties": found_props,
        "missing_properties": missing_props,
        "found_functions": found_funcs,
        "missing_functions": missing_funcs,
        "found_events": found_events,
        "missing_events": missing_events,
        "valid": is_valid
    }

def main():
    args = parse_args()
    print("=================================================================")
    print(" Dogfish Technical Verification: Papyrus Symbol & Claim Audit   ")
    print("=================================================================")
    print(f"Source Directory: {args.source_dir.resolve()}\n")

    if not args.source_dir.exists():
        print(f"ERROR: Source directory not found: {args.source_dir}")
        return 1

    all_passed = True
    results = {}

    for script_name, spec in CORE_SYMBOL_SPEC.items():
        script_path = args.source_dir / script_name
        res = verify_script(script_path, spec)
        results[script_name] = res

        status_str = "PASS" if res.get("valid") else "FAIL"
        if not res.get("valid"):
            all_passed = False

        print(f"[{status_str}] {script_name}")
        print(f"       Extends: {res.get('parent_actual')} (Expected: {res.get('parent_expected')})")
        print(f"       Properties: {len(res.get('found_properties', []))}/{len(spec.get('properties', []))} verified")
        if res.get("missing_properties"):
            print(f"       MISSING PROPS: {res['missing_properties']}")
        if spec.get("functions"):
            print(f"       Functions:  {len(res.get('found_functions', []))}/{len(spec.get('functions', []))} verified")
            if res.get("missing_functions"):
                print(f"       MISSING FUNCS: {res['missing_functions']}")
        if spec.get("events"):
            print(f"       Events:     {len(res.get('found_events', []))}/{len(spec.get('events', []))} verified")

    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        with open(args.output, "w", encoding="utf-8") as f:
            json.dump(results, f, indent=2)
        print(f"\n[OUTPUT] Written to: {args.output.resolve()}")

    if all_passed:
        print("\nOVERALL VERIFICATION RESULT: PASS - All core Papyrus symbols, inheritance, and functions verified.")
        return 0
    else:
        print("\nOVERALL VERIFICATION RESULT: FAIL - Discrepancies detected.")
        return 1

if __name__ == "__main__":
    sys.exit(main())
