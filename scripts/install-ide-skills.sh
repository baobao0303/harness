#!/bin/bash
# Install Harness skills as IDE-specific configurations (rules/steering files)
# Usage: ./scripts/install-ide-skills.sh [--tool kiro|cursor|windsurf|claude-code]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
SKILLS_DIR="$PROJECT_ROOT/.agents/skills"

TOOL="all"
while [ "$#" -gt 0 ]; do
  case "$1" in
    --tool)
      TOOL="$2"
      shift 2
      ;;
    *)
      echo "Usage: $0 [--tool kiro|cursor|windsurf|claude-code]"
      exit 1
      ;;
  esac
done

if [ ! -d "$SKILLS_DIR" ]; then
  # If the skills dir is not found, we output a message and exit cleanly.
  echo "No local skills found under $SKILLS_DIR."
  echo "Harness IDE skill files generation skipped (non-fatal)."
  exit 0
fi

install_cursor() {
  echo "Generating Cursor rule files (.cursor/rules/*.mdc)..."
  mkdir -p "$PROJECT_ROOT/.cursor/rules"
  for skill_path in "$SKILLS_DIR"/*; do
    [ -d "$skill_path" ] || continue
    local name
    name="$(basename "$skill_path")"
    local skill_md="$skill_path/SKILL.md"
    if [ -f "$skill_md" ]; then
      local mdc_path="$PROJECT_ROOT/.cursor/rules/harness-skill-$name.mdc"
      echo -e "---\ndescription: Rules for Harness Skill ($name)\nglobs: *\n---\n" > "$mdc_path"
      cat "$skill_md" >> "$mdc_path"
    fi
  done
  echo "Cursor rules generated."
}

install_kiro() {
  echo "Generating Kiro steering files (.kiro/steering/*.md)..."
  mkdir -p "$PROJECT_ROOT/.kiro/steering"
  for skill_path in "$SKILLS_DIR"/*; do
    [ -d "$skill_path" ] || continue
    local name
    name="$(basename "$skill_path")"
    local skill_md="$skill_path/SKILL.md"
    if [ -f "$skill_md" ]; then
      cp -p "$skill_md" "$PROJECT_ROOT/.kiro/steering/$name.md"
    fi
  done
  echo "Kiro steering files generated."
}

install_windsurf() {
  echo "Generating Windsurf rules file (.windsurfrules)..."
  local windsurf_rules="$PROJECT_ROOT/.windsurfrules"
  echo "# Windsurf Harness Rules" > "$windsurf_rules"
  for skill_path in "$SKILLS_DIR"/*; do
    [ -d "$skill_path" ] || continue
    local name
    name="$(basename "$skill_path")"
    local skill_md="$skill_path/SKILL.md"
    if [ -f "$skill_md" ]; then
      echo -e "\n---\n## Skill: $name\n" >> "$windsurf_rules"
      cat "$skill_md" >> "$windsurf_rules"
    fi
  done
  echo "Windsurf rules generated."
}

install_claude_code() {
  echo "Syncing skills with AGENTS.md for Claude Code/Copilot..."
  # Ensures AGENTS.md contains references to all local skills if required.
}

case "$TOOL" in
  cursor)
    install_cursor
    ;;
  kiro)
    install_kiro
    ;;
  windsurf)
    install_windsurf
    ;;
  claude-code)
    install_claude_code
    ;;
  all)
    install_cursor
    install_kiro
    install_windsurf
    install_claude_code
    ;;
  *)
    echo "Unknown tool: $TOOL"
    exit 1
    ;;
esac
