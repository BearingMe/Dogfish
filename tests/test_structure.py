#!/usr/bin/env python3
"""
tests/test_structure.py
Automated Structural Invariant Assertions for Conquest of Skyrim v0.3.9
Part of Dogfish Specification Recovery Program (Phase 10: M37)

Usage:
    python -I -m unittest tests/test_structure.py
"""

import sys
import unittest
from pathlib import Path

# Add project root to sys.path
root_dir = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(root_dir))

from scripts.verify_structure import verify_files, verify_esp

class TestStructureInvariants(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.data_dir = root_dir / "references/local/conquest-of-skyrim/Data"
        cls.esp_path = cls.data_dir / "conquerskyrim.esp"

        if not cls.data_dir.exists() or not cls.esp_path.exists():
            raise unittest.SkipTest("Original mod Data/ directory not available locally.")

        cls.file_metrics = verify_files(cls.data_dir)
        cls.esp_metrics = verify_esp(cls.esp_path)

    def test_total_file_count(self):
        """Assert exact total package file count matches audited 3,779 files."""
        self.assertEqual(self.file_metrics["total_files"], 3779)

    def test_script_counts_and_correspondence(self):
        """Assert 1,539 compiled scripts and 1,539 source scripts match 1:1."""
        self.assertEqual(self.file_metrics["compiled_scripts_count"], 1539)
        self.assertEqual(self.file_metrics["source_scripts_count"], 1539)
        self.assertTrue(self.file_metrics["scripts_matching_1_to_1"])

    def test_esp_header_invariants(self):
        """Assert ESP Form Version 44 and stated record count 22,690."""
        self.assertEqual(self.esp_metrics["form_version"], 44)
        self.assertEqual(self.esp_metrics["hedr_records"], 22690)
        self.assertEqual(self.esp_metrics["file_size"], 4991035)

    def test_master_dependencies(self):
        """Assert exactly 5 official master dependencies in load order."""
        expected_masters = [
            "Skyrim.esm",
            "Update.esm",
            "Dawnguard.esm",
            "HearthFires.esm",
            "Dragonborn.esm"
        ]
        self.assertEqual(self.esp_metrics["masters"], expected_masters)

    def test_top_level_groups(self):
        """Assert 47 top-level GRUP record signatures."""
        self.assertEqual(self.esp_metrics["top_level_groups_count"], 47)
        self.assertIn("QUST", self.esp_metrics["top_level_groups"])
        self.assertIn("GLOB", self.esp_metrics["top_level_groups"])
        self.assertIn("FLST", self.esp_metrics["top_level_groups"])
        self.assertIn("FACT", self.esp_metrics["top_level_groups"])

    def test_vanilla_overrides_count(self):
        """Assert exactly 83 overridden vanilla records from Skyrim.esm."""
        self.assertEqual(self.esp_metrics["total_vanilla_overrides"], 83)
        self.assertEqual(self.esp_metrics["vanilla_overrides_by_type"]["QUST"], 1)
        self.assertEqual(self.esp_metrics["vanilla_overrides_by_type"]["LCTN"], 57)

if __name__ == "__main__":
    unittest.main()
