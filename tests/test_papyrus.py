#!/usr/bin/env python3
"""
tests/test_papyrus.py
Automated Papyrus Symbol & Function Assertions for Conquest of Skyrim v0.3.9
Part of Dogfish Specification Recovery Program (Phase 10: M38)

Usage:
    python -I tests/test_papyrus.py
"""

import sys
import unittest
from pathlib import Path

# Add project root to sys.path
root_dir = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(root_dir))

from scripts.verify_papyrus import verify_script, CORE_SYMBOL_SPEC, DEFAULT_SOURCE_DIR

class TestPapyrusInvariants(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.source_dir = root_dir / DEFAULT_SOURCE_DIR
        if not cls.source_dir.exists():
            raise unittest.SkipTest("Papyrus source directory not available locally.")

    def test_all_core_script_symbols(self):
        """Assert all 11 core controller script specs pass symbol and inheritance verification."""
        for script_name, spec in CORE_SYMBOL_SPEC.items():
            with self.subTest(script=script_name):
                script_path = self.source_dir / script_name
                res = verify_script(script_path, spec)
                self.assertTrue(res["exists"], f"{script_name} must exist on disk")
                self.assertTrue(res["parent_ok"], f"{script_name} must extend {spec['parent']}")
                self.assertEqual(res["missing_properties"], [], f"{script_name} missing properties")
                self.assertEqual(res["missing_functions"], [], f"{script_name} missing functions")
                self.assertEqual(res["missing_events"], [], f"{script_name} missing events")
                self.assertTrue(res["valid"])

    def test_task_barrel_proxies_exist(self):
        """Assert Task Barrel properties are defined in cs_patrolholdscript.psc."""
        path = self.source_dir / "cs_patrolholdscript.psc"
        text = path.read_text(encoding="utf-8", errors="replace")
        for barrel in ["TaskBarrelFollow", "TaskBarrelHunt", "TaskBarrelNoOrder", "TaskBarrelsAttackCity"]:
            self.assertIn(barrel, text)

    def test_moot_and_coronation_functions(self):
        """Assert The Moot and Coronation function signatures exist in respective scripts."""
        meeting = (self.source_dir / "cs_meetingscript.psc").read_text(encoding="utf-8", errors="replace")
        coronation = (self.source_dir / "cs_coronationscript.psc").read_text(encoding="utf-8", errors="replace")
        self.assertIn("Function StartMootVote", meeting)
        self.assertIn("Function JarlNextVote", meeting)
        self.assertIn("Function CitizensKneel", coronation)
        self.assertIn("Function AllCheer", coronation)

if __name__ == "__main__":
    unittest.main()
