#!/usr/bin/env bash
# ==============================================================================
#            Harness Instant Project Bootstrapper & Setup Script
# ==============================================================================
# Drag this script into any new project directory and run it to instantly nạp
# the Harness operating rules, workflows, skills, and durable SQLite database!
# ==============================================================================
set -euo pipefail

# 1. Colors and Banners
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}"
cat <<'EOF'
  _    _                                 
 | |  | |                                
 | |__| | __ _ _ __ _ __   ___  ___ ___  
 |  __  |/ _` | '__| '_ \ / _ \/ __/ __| 
 | |  | | (_| | |  | | | |  __/\__ \__ \ 
 |_|  |_|\__,_|_|  |_| |_|\___||___/___/ 
         Instant Project Bootstrapper
==============================================
EOF
echo -e "${NC}"

# 2. Paths Configuration
# We locate the master Harness templates folder on the local machine
MASTER_SOURCE="/Users/bao312/Desktop/harness"
TARGET_DIR="$(pwd)"

if [ ! -d "$MASTER_SOURCE" ]; then
  echo -e "${RED}Error: Master Harness template directory not found at $MASTER_SOURCE${NC}"
  exit 1
fi

if [ "$TARGET_DIR" = "$MASTER_SOURCE" ]; then
  echo -e "${YELLOW}[!] You are running the bootstrapper directly inside the master template repository.${NC}"
  echo -e "${YELLOW}[i] Please drag this script into your new project directory instead.${NC}"
  exit 0
fi

echo -e "${BLUE}[i] Target project directory:${NC} $TARGET_DIR"
echo -e "${BLUE}[i] Copying core templates from master repo...${NC}"

# 3. Copy Operations
# Copy .agents workspace structure
mkdir -p "$TARGET_DIR/.agents"
cp -R "$MASTER_SOURCE/.agents/"* "$TARGET_DIR/.agents/"
echo -e "${GREEN}[+] Successfully copied .agents/ workflows, skills, and agents.${NC}"

# Copy AGENTS.md instructions
cp "$MASTER_SOURCE/AGENTS.md" "$TARGET_DIR/AGENTS.md"
echo -e "${GREEN}[+] Successfully copied AGENTS.md system instructions.${NC}"

# Copy stable scripts entrypoint and prebuilt CLI binaries
mkdir -p "$TARGET_DIR/scripts/bin"
cp "$MASTER_SOURCE/scripts/harness" "$TARGET_DIR/scripts/harness"
cp -R "$MASTER_SOURCE/scripts/bin/"* "$TARGET_DIR/scripts/bin/"
chmod +x "$TARGET_DIR/scripts/harness"
chmod +x "$TARGET_DIR/scripts/bin/"*
echo -e "${GREEN}[+] Successfully copied scripts/harness and prebuilt CLI binaries.${NC}"

# 4. Initialize Local SQLite database
echo -e "${BLUE}[i] Initializing local SQLite database...${NC}"
(
  cd "$TARGET_DIR"
  ./scripts/harness init >/dev/null 2>&1 || true
)

# 5. Done Confirmation
echo -e "\n${GREEN}==============================================${NC}"
echo -e "${GREEN}[✓] HARNESS INSTALLED SUCCESSFULLY!${NC}"
echo -e "${GREEN}==============================================${NC}\n"
echo -e "Your new project is now 100% powered by Harness!"
echo -e "You can now start a new chat session and command your AI companion:"
echo -e "  ${YELLOW}Hãy chạy Master Workflow .agents/workflow/harness-loop.md để bắt đầu!${NC}\n"
