# 📦 SynMem Installation Guide

This guide provides detailed installation instructions for SynMem, with a focus on Windows environments.

## Prerequisites

### Required

| Software | Version | Purpose |
|----------|---------|---------|
| [Rust](https://www.rust-lang.org/) | 1.70+ | Core runtime |
| [Git](https://git-scm.com/) | 2.x | Version control |
| [Chrome/Chromium](https://www.google.com/chrome/) | Latest | Browser automation |

### Recommended

| Software | Purpose |
|----------|---------|
| [GitHub CLI](https://cli.github.com/) | Issue management |
| [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/) | Windows compilation |

---

## Windows Installation

### Step 1: Install Rust

```powershell
# Option A: Using winget (Windows 10/11)
winget install Rustup.Rustup

# Option B: Manual installation
# Download from https://rustup.rs and run rustup-init.exe
```

After installation, restart your terminal and verify:

```powershell
rustc --version
cargo --version
```

### Step 2: Install Visual Studio Build Tools

Rust on Windows requires the MSVC toolchain:

```powershell
# Using winget
winget install Microsoft.VisualStudio.2022.BuildTools

# Or download from:
# https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
```

During installation, select:
- "Desktop development with C++"
- Windows 10/11 SDK

### Step 3: Clone and Build SynMem

```powershell
# Clone the repository
git clone https://github.com/iberi22/synmem.git
cd synmem

# Build in release mode
cargo build --release

# The binary will be at: target/release/synmem-mcp.exe
```

### Step 4: Add to PATH

```powershell
# Add to user PATH (run in elevated PowerShell)
$binPath = "$pwd\target\release"
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
[Environment]::SetEnvironmentVariable("Path", "$userPath;$binPath", "User")
```

### Step 5: Install Chrome Extension

1. Open Chrome and navigate to `chrome://extensions`
2. Enable "Developer mode" (top right toggle)
3. Click "Load unpacked"
4. Select the `extension/` folder from the SynMem directory

### Step 6: Register Native Messaging Host

```powershell
# Run the installation script
.\scripts\install_extension.ps1
```

This registers the native messaging host that allows the extension to communicate with SynMem.

---

## Linux Installation

### Ubuntu/Debian

```bash
# Install dependencies
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and build
git clone https://github.com/iberi22/synmem.git
cd synmem
cargo build --release

# Add to PATH
echo 'export PATH="$PATH:$HOME/synmem/target/release"' >> ~/.bashrc
source ~/.bashrc
```

### Fedora/RHEL

```bash
# Install dependencies
sudo dnf install -y gcc pkg-config openssl-devel

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and build
git clone https://github.com/iberi22/synmem.git
cd synmem
cargo build --release
```

### Arch Linux

```bash
# Install dependencies
sudo pacman -S base-devel openssl

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and build
git clone https://github.com/iberi22/synmem.git
cd synmem
cargo build --release
```

---

## macOS Installation

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and build
git clone https://github.com/iberi22/synmem.git
cd synmem
cargo build --release

# Add to PATH
echo 'export PATH="$PATH:$HOME/synmem/target/release"' >> ~/.zshrc
source ~/.zshrc
```

---

## Claude Desktop Configuration

### Windows

Edit `%APPDATA%\Claude\config.json`:

```json
{
  "mcpServers": {
    "synmem": {
      "command": "synmem-mcp.exe",
      "args": ["serve"]
    }
  }
}
```

### Linux/macOS

Edit `~/.config/claude/config.json`:

```json
{
  "mcpServers": {
    "synmem": {
      "command": "synmem-mcp",
      "args": ["serve"]
    }
  }
}
```

---

## Verification

### Test the Installation

```bash
# Check version
synmem-mcp --version

# Run health check
synmem-mcp health

# Start server (for testing)
synmem-mcp serve --verbose
```

### Test with Claude Desktop

1. Restart Claude Desktop
2. Open a new conversation
3. Ask: "List available SynMem tools"
4. Claude should show the available MCP tools

---

## Troubleshooting

### Common Issues

#### "Rust not found"

```powershell
# Ensure Rust is in PATH
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","User") + ";" + [System.Environment]::GetEnvironmentVariable("Path","Machine")
```

#### "linker 'link.exe' not found"

Install Visual Studio Build Tools with C++ workload.

#### "Chrome extension not connecting"

1. Check the native messaging host is registered:
   ```powershell
   # Windows
   reg query "HKCU\SOFTWARE\Google\Chrome\NativeMessagingHosts\com.synmem.host"
   ```

2. Check the manifest path is correct

#### "Permission denied" on Linux/macOS

```bash
chmod +x target/release/synmem-mcp
```

### Getting Help

If you encounter issues:

1. Check the [GitHub Issues](https://github.com/iberi22/synmem/issues)
2. Run with verbose logging: `synmem-mcp serve --verbose`
3. Open a new issue with:
   - OS and version
   - Rust version (`rustc --version`)
   - Full error message

---

## Updating

```bash
# Pull latest changes
cd synmem
git pull origin main

# Rebuild
cargo build --release
```

---

## Uninstallation

### Windows

```powershell
# Remove native messaging host
.\scripts\uninstall_extension.ps1

# Remove from PATH (manual)
# Remove the synmem folder
```

### Linux/macOS

```bash
# Remove binary from PATH
# Edit ~/.bashrc or ~/.zshrc and remove the PATH entry

# Remove the synmem folder
rm -rf ~/synmem
```
