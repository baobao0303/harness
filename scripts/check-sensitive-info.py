#!/usr/bin/env python3
"""
CI Security Guard: Scans repository for sensitive company information,
physical addresses, private keys, and corporate secrets.
Exits with code 1 if any forbidden pattern is detected.
"""

import sys
import re
from pathlib import Path

# Configurable forbidden regex patterns
FORBIDDEN_PATTERNS = [
    # Physical address & location patterns
    (
        r"(?i)\b(?:tân\s*bình|tân\s*phú|đường\s*19/5a|lô\s*[ivx0-9-]+|nhóm\s*cn\s*[ivx0-9-]*|khu\s*công\s*nghiệp)\b",
        "Physical address / Industrial park identifier"
    ),
    # Company specific internal domains
    (
        r"(?i)\b(?:tfs\.tmtco\.dev|tmt\s*solutions|tmtco\.org|tmtco\.dev)\b",
        "Internal corporate domain / entity name"
    ),
    # Private keys
    (
        r"-----BEGIN (?:RSA |EC |OPENSSH |DSA )?PRIVATE KEY-----",
        "Private encryption key"
    ),
]

EXCLUDED_DIRS = {
    ".git",
    "target",
    "dist",
    ".cargo",
    "__pycache__",
    ".idea",
    ".vscode",
}

def scan_repository(repo_root: Path) -> list[tuple[str, int, str, str]]:
    violations = []
    
    for file_path in repo_root.rglob("*"):
        if not file_path.is_file():
            continue
        if any(part in EXCLUDED_DIRS for part in file_path.parts):
            continue
        # Skip this check script itself
        if file_path.name == "check-sensitive-info.py":
            continue

        try:
            content = file_path.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            continue

        for pattern, rule_desc in FORBIDDEN_PATTERNS:
            for match in re.finditer(pattern, content):
                line_no = content[:match.start()].count("\n") + 1
                matched_snippet = match.group(0).strip()
                violations.append((
                    str(file_path.relative_to(repo_root)),
                    line_no,
                    rule_desc,
                    matched_snippet,
                ))

    return violations

def main() -> int:
    repo_root = Path(__file__).resolve().parent.parent
    violations = scan_repository(repo_root)

    print("=" * 65)
    print("🔒 HARNESS CI SENSITIVE DATA & ADDRESS SCANNER")
    print("=" * 65)

    if not violations:
        print("✅ PASS: Zero sensitive company names, addresses, or private keys found.")
        print("=" * 65)
        return 0

    print(f"❌ FAIL: Found {len(violations)} sensitive pattern violation(s):")
    for file_path, line_no, rule_desc, snippet in violations:
        print(f"  - [{rule_desc}] {file_path}:{line_no} -> '{snippet}'")
    print("=" * 65)
    print("Please sanitize these sensitive details before committing or merging.")
    return 1

if __name__ == "__main__":
    sys.exit(main())
