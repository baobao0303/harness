#!/usr/bin/env bash
# ==============================================================================
# Harness - Demo Workflow Execution Script
# ==============================================================================
# This script demonstrates the full lifecycle of a feature using the Harness CLI:
# 1. Initialize the SQLite database (if not exists)
# 2. Record a Feature Intake
# 3. Add a Story to the Test Matrix
# 4. Update the Story with verification evidence
# 5. Record an Agent Trace (including dev friction)
# 6. Promote friction into a Harness Backlog item
# 7. Print the current state of the harness
# ==============================================================================

set -euo pipefail

# Ensure we are running from the project root
cd "$(dirname "$0")/.."

echo "=== Harness Workflow Demo ==="
echo ""

# 1. Initialize Database
if [ ! -f "harness.db" ]; then
  echo "1. Initializing harness database..."
  ./scripts/harness init
  echo ""
else
  echo "1. harness.db already exists, skipping init."
  echo ""
fi

# 2. Record Feature Intake
echo "2. Recording feature intake..."
./scripts/harness intake \
  --type "change_request" \
  --summary "Demo: Add user authentication feature" \
  --lane "normal"
echo ""

# 3. Add Story to Test Matrix
echo "3. Adding story to test matrix..."
./scripts/harness story add \
  --id "demo-001" \
  --title "Demo: JWT-based authentication" \
  --lane "normal"
echo ""

# 4. Query Initial Matrix State
echo "4. Current test matrix..."
./scripts/harness query matrix
echo ""

# 5. Simulate work (in real scenario, you would implement here)
echo "5. Simulating implementation work..."
echo "   - Read product docs"
echo "   - Create design"
echo "   - Implement feature"
echo "   - Write/run tests"
echo ""

# 6. Update Story with Verification Evidence
echo "6. Updating story with verification evidence..."
./scripts/harness story update \
  --id "demo-001" \
  --status "implemented" \
  --unit 1 \
  --integration 1 \
  --e2e 1 \
  --evidence "Demo: All tests pass - unit (3), integration (2), e2e (1)"
echo ""

# 7. Record Agent Trace
echo "7. Recording agent trace..."
./scripts/harness trace \
  --summary "Demo: Implemented JWT-based authentication" \
  --intake 1 \
  --story "demo-001" \
  --outcome "completed" \
  --actions "intake,story,implement,test" \
  --read "docs/, projects/" \
  --changed "src/auth/, tests/" \
  --friction "Demo mode - no real friction" \
  --agent "demo"
echo ""

# 8. Promote Friction to Backlog (demo friction)
echo "8. Adding demo friction to backlog..."
./scripts/harness backlog add \
  --title "Demo: Add proxy config for local CORS bypass" \
  --pain "Demo: Speech Recognition API blocked by CORS in local dev" \
  --suggestion "Demo: Add devServer proxy to angular.json" \
  --risk "tiny" \
  --predicted "Demo: Future agents can test Speech API locally without CORS issues"
echo ""

# 9. Print Final Harness State
echo "9. Final harness state..."
echo "--------------------------------------------------"
echo "📋 [TEST MATRIX]"
./scripts/harness query matrix
echo -e "\n⚠️ [FRICTION LOGS]"
./scripts/harness query friction
echo -e "\n🔧 [HARNESS BACKLOG]"
./scripts/harness query backlog
echo -e "\n📈 [STATS]"
./scripts/harness query stats
echo -e "\n📜 [TRACES]"
./scripts/harness query traces
echo "--------------------------------------------------"

echo ""
echo "=== Demo Complete ==="
echo "To clean up demo data, run:"
echo "  ./scripts/harness query sql \"DELETE FROM trace WHERE story_id = 'demo-001';\""
echo "  ./scripts/harness query sql \"DELETE FROM story WHERE id = 'demo-001';\""
echo "  ./scripts/harness query sql \"DELETE FROM backlog WHERE title LIKE '%Demo:%';\""
echo "  ./scripts/harness query sql \"DELETE FROM intake WHERE summary LIKE '%Demo:%';\""