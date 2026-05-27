#!/usr/bin/env bash
# install-ide-skills.sh — Generate IDE-specific skill discovery files from .agents/skills/
# This script reads all Harness skills and generates the appropriate format
# for each supported IDE so that skill names appear in their native UI.
#
# Usage:
#   scripts/install-ide-skills.sh [--tool <tool>] [--target <path>]
#
# Supported tools: kiro, cursor, windsurf, claude-code, copilot, all
# Default: all (generates for every detected IDE config)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
SKILLS_DIR="$REPO_ROOT/.agents/skills"

TOOL="all"
TARGET_DIR="$REPO_ROOT"

while [ $# -gt 0 ]; do
  case "$1" in
    --tool)   TOOL="$2";       shift 2 ;;
    --target) TARGET_DIR="$2"; shift 2 ;;
    -h|--help)
      echo "Usage: install-ide-skills.sh [--tool <tool>] [--target <path>]"
      echo "Tools: kiro, cursor, windsurf, claude-code, copilot, all"
      exit 0
      ;;
    *) echo "Unknown option: $1" >&2; exit 1 ;;
  esac
done

# ── Parse skill metadata ───────────────────────────────────────────
parse_skill_description() {
  local skill_md="$1"
  local desc=""
  local in_fm=0
  while IFS= read -r line; do
    if [ "$line" = "---" ]; then
      if [ "$in_fm" -eq 0 ]; then
        in_fm=1; continue
      else
        break
      fi
    fi
    if [ "$in_fm" -eq 1 ]; then
      case "$line" in
        description:*)
          desc="${line#description:}"
          desc="${desc#"${desc%%[![:space:]]*}"}"
          desc="${desc#\"}"
          desc="${desc%\"}"
          desc="${desc#\'}"
          desc="${desc%\'}"
          ;;
      esac
    fi
  done < "$skill_md"
  printf '%s' "$desc"
}

# ── Collect all skills ─────────────────────────────────────────────
declare -a SKILL_NAMES=()
declare -a SKILL_DESCS=()

for skill_dir in "$SKILLS_DIR"/*/; do
  [ -d "$skill_dir" ] || continue
  name="$(basename "$skill_dir")"
  skill_md="$skill_dir/SKILL.md"
  if [ -f "$skill_md" ]; then
    desc="$(parse_skill_description "$skill_md")"
  else
    desc="Workflow for $name"
  fi
  SKILL_NAMES+=("$name")
  SKILL_DESCS+=("$desc")
done

echo "Found ${#SKILL_NAMES[@]} skills in .agents/skills/"

# ── Kiro: .kiro/steering/*.md with inclusion: manual ───────────────
generate_kiro() {
  local kiro_dir="$TARGET_DIR/.kiro/steering"
  mkdir -p "$kiro_dir"

  # Always-on rules file
  cat > "$kiro_dir/harness-rules.md" << 'RULES_EOF'
---
inclusion: auto
---

# Harness Operating Rules

This repository uses **Harness** — a durable, SQLite-backed operating framework for human-agent pair programming.

## Quick Commands

```bash
./scripts/harness query stats       # Summary counts
./scripts/harness query matrix      # Test verification status
./scripts/harness query traces      # Agent execution logs
./scripts/harness query friction    # Development blockages
./scripts/harness query backlog     # Improvement list
./scripts/harness skill list        # Available skills
```

## Task Loop

1. Classify: `./scripts/harness intake --type <type> --summary "<text>" --lane <lane>`
2. Implement within lane constraints
3. Trace (MANDATORY): `./scripts/harness trace --summary "<task>" --outcome <outcome> --agent kiro`

## Invoke Skills

Use `./scripts/harness skill list` to see all available skills.
Skills are invoked by following the instructions in `.agents/skills/<name>/SKILL.md`.
RULES_EOF

  local i
  for i in "${!SKILL_NAMES[@]}"; do
    local name="${SKILL_NAMES[$i]}"
    local desc="${SKILL_DESCS[$i]}"
    cat > "$kiro_dir/$name.md" << EOF
---
inclusion: manual
---

# Skill: $name

$desc

## How to Invoke

Read and follow the full skill instructions at:
\`.agents/skills/$name/SKILL.md\`

## CLI Reference

\`\`\`bash
./scripts/harness skill list              # See all skills
./scripts/harness skill run $name         # Run if wrapper exists
\`\`\`
EOF
  done

  echo "  Kiro: generated $((${#SKILL_NAMES[@]} + 1)) steering files in .kiro/steering/"
}

# ── Cursor: .cursor/rules/*.mdc ───────────────────────────────────
generate_cursor() {
  local cursor_dir="$TARGET_DIR/.cursor/rules"
  mkdir -p "$cursor_dir"

  local i
  for i in "${!SKILL_NAMES[@]}"; do
    local name="${SKILL_NAMES[$i]}"
    local desc="${SKILL_DESCS[$i]}"
    cat > "$cursor_dir/$name.mdc" << EOF
---
description: "$desc"
globs: 
alwaysApply: false
---

# Skill: $name

$desc

## How to Invoke

Read and follow the full skill instructions at:
\`.agents/skills/$name/SKILL.md\`

## CLI Reference

\`\`\`bash
./scripts/harness skill list
./scripts/harness skill run $name
\`\`\`
EOF
  done

  echo "  Cursor: generated ${#SKILL_NAMES[@]} rule files in .cursor/rules/"
}

# ── Windsurf: .windsurfrules (single consolidated file) ────────────
generate_windsurf() {
  local rules_file="$TARGET_DIR/.windsurfrules"

  {
    echo "# Harness Skills"
    echo ""
    echo "This repository uses Harness. Available skills can be invoked by reading"
    echo "the SKILL.md in .agents/skills/<name>/."
    echo ""
    echo "## Available Skills"
    echo ""
    local i
    for i in "${!SKILL_NAMES[@]}"; do
      echo "### ${SKILL_NAMES[$i]}"
      echo "${SKILL_DESCS[$i]}"
      echo "Invoke: Read \`.agents/skills/${SKILL_NAMES[$i]}/SKILL.md\`"
      echo ""
    done
    echo "## CLI Quick Reference"
    echo ""
    echo "\`\`\`bash"
    echo "./scripts/harness skill list"
    echo "./scripts/harness query stats"
    echo "./scripts/harness query matrix"
    echo "\`\`\`"
  } > "$rules_file"

  echo "  Windsurf: generated .windsurfrules"
}

# ── Claude Code / GitHub Copilot: relies on AGENTS.md (native .md) ─
generate_claude_copilot() {
  # Claude Code and Copilot read AGENTS.md natively.
  # We append a skill catalog section if not already present.
  local agents_file="$TARGET_DIR/AGENTS.md"
  [ -f "$agents_file" ] || return 0

  local marker="<!-- HARNESS-SKILLS:BEGIN -->"
  local end_marker="<!-- HARNESS-SKILLS:END -->"

  # Build the skill catalog block into a temp file
  local block_file
  block_file="$(mktemp)"
  {
    echo "$marker"
    echo ""
    echo "## Available Harness Skills"
    echo ""
    echo "Invoke a skill by reading its instructions: \`.agents/skills/<name>/SKILL.md\`"
    echo "Or run directly: \`./scripts/harness skill run <name>\`"
    echo ""
    echo "| Skill | Description |"
    echo "| --- | --- |"
    local i
    for i in "${!SKILL_NAMES[@]}"; do
      echo "| ${SKILL_NAMES[$i]} | ${SKILL_DESCS[$i]} |"
    done
    echo ""
    echo "$end_marker"
  } > "$block_file"

  if grep -Fq "$marker" "$agents_file" && grep -Fq "$end_marker" "$agents_file"; then
    # Replace existing block
    local tmp
    tmp="$(mktemp)"
    awk -v bf="$block_file" '
      /<!-- HARNESS-SKILLS:BEGIN -->/ { skip=1; while ((getline line < bf) > 0) print line; next }
      /<!-- HARNESS-SKILLS:END -->/ { skip=0; next }
      !skip { print }
    ' "$agents_file" > "$tmp"
    mv "$tmp" "$agents_file"
    echo "  Claude/Copilot: updated skill catalog in AGENTS.md"
  else
    # Append before HARNESS:END if it exists, otherwise at end
    if grep -Fq "<!-- HARNESS:END -->" "$agents_file"; then
      local tmp
      tmp="$(mktemp)"
      awk -v bf="$block_file" '
        /<!-- HARNESS:END -->/ { while ((getline line < bf) > 0) print line; print ""; }
        { print }
      ' "$agents_file" > "$tmp"
      mv "$tmp" "$agents_file"
    else
      printf '\n' >> "$agents_file"
      cat "$block_file" >> "$agents_file"
    fi
    echo "  Claude/Copilot: appended skill catalog to AGENTS.md"
  fi

  rm -f "$block_file"
}

# ── Main dispatch ──────────────────────────────────────────────────
echo "Generating IDE skill discovery files..."

case "$TOOL" in
  kiro)
    generate_kiro
    ;;
  cursor)
    generate_cursor
    ;;
  windsurf)
    generate_windsurf
    ;;
  claude-code|copilot|claude|github-copilot)
    generate_claude_copilot
    ;;
  all)
    generate_kiro
    generate_cursor
    generate_windsurf
    generate_claude_copilot
    ;;
  *)
    echo "Unknown tool: $TOOL" >&2
    echo "Supported: kiro, cursor, windsurf, claude-code, copilot, all" >&2
    exit 1
    ;;
esac

echo ""
echo "Done. All IDEs can now discover Harness skills in their native UI."
echo "  - Kiro:         Type # in chat to see skill names"
echo "  - Cursor:       Type @ or see rules panel"
echo "  - Windsurf:     Skills in .windsurfrules"
echo "  - Claude Code:  Skills listed in AGENTS.md"
echo "  - Copilot:      Skills listed in AGENTS.md"
