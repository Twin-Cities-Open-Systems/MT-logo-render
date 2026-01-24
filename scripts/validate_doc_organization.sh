#!/bin/bash

# validate_doc_organization.sh
# Validates that agent-focused documentation is properly organized
# Agent docs should be in prompts/, human docs should be in docs/

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if rg (ripgrep) is installed
if ! command -v rg &> /dev/null; then
    echo -e "${YELLOW}⚠ ripgrep (rg) not found. Pattern matching will be limited.${NC}"
    echo -e "${BLUE}Installation instructions:${NC}"

    # Detect OS and provide installation instructions
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo -e "${BLUE}Linux (Debian/Ubuntu):${NC} sudo apt-get install ripgrep"
        echo -e "${BLUE}Linux (Fedora/RHEL):${NC} sudo dnf install ripgrep"
        echo -e "${BLUE}Linux (Arch):${NC} sudo pacman -S ripgrep"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo -e "${BLUE}macOS (Homebrew):${NC} brew install ripgrep"
        echo -e "${BLUE}macOS (MacPorts):${NC} sudo port install ripgrep"
    elif [[ "$OSTYPE" == "cygwin" ]] || [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
        echo -e "${BLUE}Windows (Chocolatey):${NC} choco install ripgrep"
        echo -e "${BLUE}Windows (Scoop):${NC} scoop install ripgrep"
    else
        echo -e "${BLUE}Download from:${NC} https://github.com/BurntSushi/ripgrep/releases"
    fi

    echo -e "${BLUE}Or use: cargo install ripgrep (if you have Rust/Cargo)${NC}"
    echo ""

    # Set flag to use basic grep instead
    USE_BASIC_GREP=true
fi

echo -e "${YELLOW}Validating documentation organization...${NC}"

# Check if we're in the right directory
if [ ! -d "docs" ] || [ ! -d "prompts" ]; then
    echo -e "${RED}Error: Must run from project root directory${NC}"
    exit 1
fi

# Agent-focused documentation that should be in prompts/
AGENT_DOCS=(
    "STATE_CAPSULE_GUIDE.md"
    "AGENT_STATE_HANDOFF.md"
)

# Check that agent docs are in prompts/ directory
echo -e "${YELLOW}Checking agent-focused documentation...${NC}"
AGENT_DOCS_OK=true

for doc in "${AGENT_DOCS[@]}"; do
    if [ -f "prompts/$doc" ]; then
        echo -e "${GREEN}✓ $doc correctly located in prompts/${NC}"
    elif [ -f "docs/$doc" ]; then
        echo -e "${RED}✗ $doc incorrectly located in docs/ (should be in prompts/)${NC}"
        AGENT_DOCS_OK=false
    else
        echo -e "${YELLOW}⚠ $doc not found in either location${NC}"
    fi
done

# Check for agent instructions in human docs
echo -e "${YELLOW}Checking for agent instructions in human documentation...${NC}"

if [ "${USE_BASIC_GREP:-false}" = true ]; then
    # Fallback to basic grep when rg is not available
    echo -e "${YELLOW}Using basic grep (limited pattern matching)${NC}"
    AGENT_INSTRUCTIONS=$(grep -rn --include="*.md" \
        -E "(^|\s)(MUST|SHALL|DO NOT|DON'T|NEVER|ALWAYS|CHECKLIST|RUNBOOK|STEP[- ]BY[- ]STEP|FOLLOW THESE STEPS|START SESSION|END SESSION)" \
        docs/ 2>/dev/null || true)
else
    # Use ripgrep for better pattern matching
    AGENT_INSTRUCTIONS=$(rg -n --hidden --no-ignore-vcs \
        "(^|\s)(MUST|SHALL|DO NOT|DON'T|NEVER|ALWAYS|CHECKLIST|RUNBOOK|STEP[- ]BY[- ]STEP|FOLLOW THESE STEPS|START SESSION|END SESSION)\b" \
        docs/*.md docs/**/*.md 2>/dev/null || true)
fi

if [ -z "$AGENT_INSTRUCTIONS" ]; then
    echo -e "${GREEN}✓ No agent instructions found in human documentation${NC}"
else
    echo -e "${RED}✗ Found agent instructions in human documentation:${NC}"
    echo "$AGENT_INSTRUCTIONS"
    AGENT_DOCS_OK=false
fi

# Check for direct agent references in human docs
echo -e "${YELLOW}Checking for direct agent references in human documentation...${NC}"

if [ "${USE_BASIC_GREP:-false}" = true ]; then
    # Fallback to basic grep when rg is not available
    AGENT_REFERENCES=$(grep -rn --include="*.md" \
        -E "\b(you are an agent|as an agent)\b" \
        docs/ 2>/dev/null || true)
else
    # Use ripgrep for better pattern matching
    AGENT_REFERENCES=$(rg -n --hidden --no-ignore-vcs \
        "\b(you are an agent|as an agent)\b" \
        docs/*.md docs/**/*.md 2>/dev/null || true)
fi

if [ -z "$AGENT_REFERENCES" ]; then
    echo -e "${GREEN}✓ No direct agent references found in human documentation${NC}"
else
    echo -e "${RED}✗ Found direct agent references in human documentation:${NC}"
    echo "$AGENT_REFERENCES"
    AGENT_DOCS_OK=false
fi

# Final result
echo -e "${YELLOW}Validation complete${NC}"
if [ "$AGENT_DOCS_OK" = true ]; then
    echo -e "${GREEN}✓ Documentation organization is correct${NC}"
    exit 0
else
    echo -e "${RED}✗ Documentation organization issues found${NC}"
    echo -e "${YELLOW}Run: ./scripts/validate_doc_organization.sh for details${NC}"
    exit 1
fi
