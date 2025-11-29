#!/bin/bash
# scripts/install_native_host.sh
# SynMem Native Host Installer for Linux/macOS
#
# This script registers the native messaging host with Chrome/Chromium/Firefox
#
# Usage:
#   ./install_native_host.sh --extension-id "your-extension-id"
#   ./install_native_host.sh --extension-id "your-extension-id" --uninstall
#   ./install_native_host.sh --extension-id "your-extension-id" --firefox

set -e

# Default values
EXTENSION_ID=""
UNINSTALL=false
FIREFOX=false
HOST_PATH=""

# Configuration
HOST_NAME="com.synmem.nativehost"
HOST_DESCRIPTION="SynMem Native Messaging Host"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --extension-id)
            EXTENSION_ID="$2"
            shift 2
            ;;
        --host-path)
            HOST_PATH="$2"
            shift 2
            ;;
        --uninstall)
            UNINSTALL=true
            shift
            ;;
        --firefox)
            FIREFOX=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 --extension-id <id> [options]"
            echo ""
            echo "Options:"
            echo "  --extension-id <id>   Chrome extension ID (required)"
            echo "  --host-path <path>    Path to native host executable"
            echo "  --uninstall           Uninstall the native host"
            echo "  --firefox             Install for Firefox instead of Chrome"
            echo "  -h, --help            Show this help"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

# Validate extension ID
if [ -z "$EXTENSION_ID" ]; then
    echo -e "${RED}Error: --extension-id is required${NC}"
    exit 1
fi

# Determine script and repo directories
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"

# Determine host path
if [ -z "$HOST_PATH" ]; then
    HOST_PATH="$REPO_ROOT/target/release/synmem-native-host"
    
    if [ ! -f "$HOST_PATH" ]; then
        HOST_PATH="$REPO_ROOT/target/debug/synmem-native-host"
    fi
fi

# Validate host exists (unless uninstalling)
if [ "$UNINSTALL" = false ] && [ ! -f "$HOST_PATH" ]; then
    echo -e "${RED}Error: Native host executable not found at: $HOST_PATH${NC}"
    echo "Please build the project first with: cargo build --release -p synmem-native-host"
    exit 1
fi

# Get absolute path
if [ "$UNINSTALL" = false ]; then
    HOST_PATH="$(cd "$(dirname "$HOST_PATH")" && pwd)/$(basename "$HOST_PATH")"
fi

echo -e "${CYAN}🧠 SynMem Native Host Installer${NC}"
echo -e "${CYAN}================================${NC}"
echo ""

if [ "$UNINSTALL" = true ]; then
    echo -e "${YELLOW}Uninstalling native host...${NC}"
else
    echo -e "${GREEN}Host Name:      $HOST_NAME${NC}"
    echo -e "${GREEN}Host Path:      $HOST_PATH${NC}"
    echo -e "${GREEN}Extension ID:   $EXTENSION_ID${NC}"
    echo ""
fi

# Function to get manifest directory
get_manifest_dir() {
    local browser=$1
    local os_type=$(uname -s)
    
    case $browser in
        "Chrome")
            if [ "$os_type" = "Darwin" ]; then
                echo "$HOME/Library/Application Support/Google/Chrome/NativeMessagingHosts"
            else
                echo "$HOME/.config/google-chrome/NativeMessagingHosts"
            fi
            ;;
        "Chromium")
            if [ "$os_type" = "Darwin" ]; then
                echo "$HOME/Library/Application Support/Chromium/NativeMessagingHosts"
            else
                echo "$HOME/.config/chromium/NativeMessagingHosts"
            fi
            ;;
        "Firefox")
            if [ "$os_type" = "Darwin" ]; then
                echo "$HOME/Library/Application Support/Mozilla/NativeMessagingHosts"
            else
                echo "$HOME/.mozilla/native-messaging-hosts"
            fi
            ;;
    esac
}

# Function to create manifest
create_manifest() {
    local browser=$1
    
    if [ "$FIREFOX" = true ]; then
        cat << EOF
{
    "name": "$HOST_NAME",
    "description": "$HOST_DESCRIPTION",
    "path": "$HOST_PATH",
    "type": "stdio",
    "allowed_extensions": ["$EXTENSION_ID@synmem.com"]
}
EOF
    else
        cat << EOF
{
    "name": "$HOST_NAME",
    "description": "$HOST_DESCRIPTION",
    "path": "$HOST_PATH",
    "type": "stdio",
    "allowed_origins": ["chrome-extension://$EXTENSION_ID/"]
}
EOF
    fi
}

# Function to install for a browser
install_native_host() {
    local browser=$1
    local manifest_dir=$(get_manifest_dir "$browser")
    local manifest_path="$manifest_dir/$HOST_NAME.json"
    
    echo -e "${CYAN}Installing for $browser...${NC}"
    
    if [ "$UNINSTALL" = true ]; then
        if [ -f "$manifest_path" ]; then
            rm -f "$manifest_path"
            echo -e "  ${GREEN}✓ Removed manifest: $manifest_path${NC}"
        else
            echo -e "  ${YELLOW}~ Manifest not found, skipping${NC}"
        fi
    else
        # Create directory if it doesn't exist
        if [ ! -d "$manifest_dir" ]; then
            mkdir -p "$manifest_dir"
            echo -e "  ${GREEN}✓ Created directory: $manifest_dir${NC}"
        fi
        
        # Write manifest
        create_manifest "$browser" > "$manifest_path"
        echo -e "  ${GREEN}✓ Created manifest: $manifest_path${NC}"
    fi
}

# Install for Chrome and Chromium (unless Firefox-only)
if [ "$FIREFOX" = false ]; then
    install_native_host "Chrome"
    install_native_host "Chromium"
fi

# Install for Firefox if requested
if [ "$FIREFOX" = true ]; then
    install_native_host "Firefox"
fi

echo ""
if [ "$UNINSTALL" = true ]; then
    echo -e "${GREEN}✅ Native host uninstalled successfully!${NC}"
else
    echo -e "${GREEN}✅ Native host installed successfully!${NC}"
    echo ""
    echo -e "${CYAN}Next steps:${NC}"
    echo "  1. Restart your browser"
    echo "  2. The extension should now be able to connect to the native host"
fi
