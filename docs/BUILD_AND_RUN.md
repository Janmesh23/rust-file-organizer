# 🔧 Build and Run Guide - Rust File Organizer CLI

This guide shows you **every possible way** to build and run the Rust File Organizer CLI project.

## 📋 Table of Contents

- [Prerequisites](#prerequisites)
- [Building the Project](#building-the-project)
- [Running the Project](#running-the-project)
- [Installation Methods](#installation-methods)
- [Example Commands](#example-commands)
- [Troubleshooting](#troubleshooting)

---

## 🛠️ Prerequisites

### **Rust Installation**
```bash
# Install Rust using rustup (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify installation
rustc --version   # Should show: rustc 1.70+ 
cargo --version   # Should show: cargo 1.70+
```

### **Git (for cloning)**
```bash
# macOS
brew install git

# Linux (Ubuntu/Debian)
sudo apt install git

# Verify
git --version
```

---

## 🏗️ Building the Project

### **1. Clone the Repository**
```bash
# Clone from GitHub
git clone https://github.com/Janmesh23/rust-file-organizer.git
cd rust-file-organizer

# Or if you already have it
cd path/to/rust-file-organizer
```

### **2. Development Build (Debug Mode)**
```bash
# Fast compilation, includes debug symbols
cargo build

# Output location
ls -la target/debug/file-organizer
```

**When to use:** During development, debugging, or testing changes.

**Pros:** 
- ✅ Fast compilation
- ✅ Debug symbols included
- ✅ Good for testing

**Cons:**
- ❌ Larger binary size
- ❌ Slower runtime performance

### **3. Release Build (Optimized)**
```bash
# Optimized compilation for production use
cargo build --release

# Output location
ls -la target/release/file-organizer
```

**When to use:** For production use, distribution, or maximum performance.

**Pros:**
- ✅ Optimized performance
- ✅ Smaller binary size
- ✅ Ready for distribution

**Cons:**
- ❌ Slower compilation
- ❌ No debug symbols

### **4. Check Compilation (No Binary)**
```bash
# Check if code compiles without creating binary
cargo check

# Faster than build, useful for development
```

**When to use:** Quick validation that code compiles without waiting for linking.

---

## 🚀 Running the Project

### **Method 1: `cargo run` (Recommended for Development)**

This is the **easiest** way during development:

```bash
# Show help
cargo run -- --help

# Organize current directory (dry run)
cargo run -- organize --dry-run

# Organize specific directory
cargo run -- organize ~/Downloads --dry-run

# Organize by file size
cargo run -- organize --mode size --dry-run

# Filter specific file types
cargo run -- organize --filter "jpg,png,mp4" --recursive --dry-run

# Actually organize files (remove --dry-run when ready)
cargo run -- organize test-folder

# Verbose mode
cargo run -- --verbose organize --dry-run
```

**Explanation:**
- `cargo run` compiles (if needed) and runs the program
- `--` separates cargo arguments from program arguments
- Everything after `--` goes to your program

### **Method 2: Run Compiled Binary Directly**

After building with `cargo build`:

```bash
# Debug version
./target/debug/file-organizer --help
./target/debug/file-organizer organize --dry-run

# Release version (after cargo build --release)
./target/release/file-organizer --help
./target/release/file-organizer organize ~/Downloads --dry-run
```

### **Method 3: Global Installation**

Install the binary globally to run from anywhere:

```bash
# Install from current directory
cargo install --path .

# Now run from anywhere
file-organizer --help
file-organizer organize ~/Downloads --dry-run

# Install from GitHub directly
cargo install --git https://github.com/Janmesh23/rust-file-organizer.git
```

**Installation location:** `~/.cargo/bin/file-organizer`

**Note:** Make sure `~/.cargo/bin` is in your PATH.

---

## 📦 Installation Methods

### **Method 1: From Source (Local)**
```bash
git clone https://github.com/Janmesh23/rust-file-organizer.git
cd rust-file-organizer
cargo install --path .
```

### **Method 2: From GitHub (Remote)**
```bash
cargo install --git https://github.com/Janmesh23/rust-file-organizer.git
```

### **Method 3: From crates.io (Future)**
```bash
# When published to crates.io
cargo install file-organizer
```

### **Method 4: Download Release Binary (Future)**
```bash
# Download from GitHub releases
curl -L https://github.com/Janmesh23/rust-file-organizer/releases/latest/download/file-organizer-macos -o file-organizer
chmod +x file-organizer
./file-organizer --help
```

---

## 💡 Example Commands

### **Getting Started**
```bash
# 1. Show help
cargo run -- --help

# 2. Show organize command help
cargo run -- organize --help

# 3. Preview organization of current directory
cargo run -- organize --dry-run

# 4. Preview with specific mode
cargo run -- organize --mode size --dry-run
```

### **File Organization Examples**

```bash
# Organize Downloads folder by file type (preview)
cargo run -- organize ~/Downloads --dry-run

# Actually organize Downloads folder
cargo run -- organize ~/Downloads

# Organize by file size
cargo run -- organize ~/Downloads --mode size

# Organize recursively (include subdirectories)
cargo run -- organize ~/Downloads --recursive

# Filter only image and video files
cargo run -- organize ~/Downloads --filter "jpg,png,mp4,avi" --dry-run

# Organize with backup (TODO: not implemented yet)
cargo run -- organize ~/Downloads --backup

# Organize by creation date
cargo run -- organize ~/Downloads --mode date

# Organize by modification date
cargo run -- organize ~/Downloads --mode modified
```

### **Other Commands**

```bash
# Show organization history (TODO: not implemented)
cargo run -- history ~/Downloads

# Undo last organization (TODO: not implemented)
cargo run -- undo ~/Downloads --dry-run

# Clean empty directories (TODO: not implemented)
cargo run -- clean ~/Downloads --dry-run

# Generate shell completions
cargo run -- completions zsh > _file-organizer
cargo run -- completions bash > file-organizer.bash
```

### **Advanced Usage**

```bash
# Verbose mode with custom config
cargo run -- --verbose --config my-rules.toml organize ~/Downloads

# Multiple filters with recursive organization
cargo run -- organize ~/Downloads --filter "pdf,doc,txt" --recursive --dry-run

# Force mode (skips confirmations)
cargo run -- organize ~/Downloads --force

# Combination of options
cargo run -- --verbose organize ~/Downloads --mode size --recursive --filter "jpg,png" --dry-run
```

---

## 🧪 Testing

### **Run Tests**
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_file_classification

# Run tests in release mode
cargo test --release
```

### **Test Different Scenarios**

```bash
# Create test directory with sample files
mkdir test-files
cd test-files
touch photo.jpg document.pdf video.mp4 song.mp3 archive.zip script.rs

# Test organization
cd ..
cargo run -- organize test-files --dry-run

# Test size-based organization
dd if=/dev/zero of=test-files/small.txt bs=1024 count=500   # 500KB
dd if=/dev/zero of=test-files/large.txt bs=1024 count=50000 # 50MB
cargo run -- organize test-files --mode size --dry-run

# Clean up
rm -rf test-files
```

---

## 🔍 Troubleshooting

### **Common Issues**

#### **1. "command not found: cargo"**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Or reload your shell
source ~/.bashrc  # or ~/.zshrc
```

#### **2. "could not find Cargo.toml"**
```bash
# Make sure you're in the project directory
cd rust-file-organizer
ls -la  # Should see Cargo.toml

# Or use absolute path
cd /path/to/rust-file-organizer
```

#### **3. Compilation Errors**
```bash
# Update Rust to latest version
rustup update

# Clean and rebuild
cargo clean
cargo build

# Check if specific dependencies are missing
cargo check
```

#### **4. Permission Denied**
```bash
# Make binary executable
chmod +x target/release/file-organizer

# Or for installed version
chmod +x ~/.cargo/bin/file-organizer
```

#### **5. "Directory does not exist" Error**
```bash
# Use absolute paths
cargo run -- organize /full/path/to/directory --dry-run

# Or create test directory first
mkdir test-dir
cargo run -- organize test-dir --dry-run
```

### **Debug Information**

```bash
# Show Rust version
rustc --version
cargo --version

# Show verbose cargo output
cargo build --verbose

# Run with debug logging
RUST_LOG=debug cargo run -- organize --dry-run

# Show binary information
file target/release/file-organizer
ldd target/release/file-organizer  # On Linux
otool -L target/release/file-organizer  # On macOS
```

### **Performance Tips**

```bash
# For fastest compilation during development
cargo build

# For fastest runtime performance
cargo build --release

# Parallel compilation (if you have multiple CPU cores)
export CARGO_BUILD_JOBS=4
cargo build --release

# Use cargo-watch for automatic rebuilds during development
cargo install cargo-watch
cargo watch -x "run -- organize --dry-run"
```

---

## 🎯 Quick Start Script

Save this as `quick-start.sh`:

```bash
#!/bin/bash

# Quick start script for Rust File Organizer CLI

echo "🦀 Rust File Organizer CLI - Quick Start"
echo "========================================"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Cargo.toml not found. Please run this from the project directory."
    exit 1
fi

echo "✅ Building project..."
cargo build --release

echo "✅ Running help command..."
cargo run -- --help

echo "✅ Creating test directory..."
mkdir -p test-files
cd test-files
touch photo.jpg document.pdf video.mp4 song.mp3 archive.zip script.rs
echo "Created test files: $(ls)"
cd ..

echo "✅ Running dry-run organization..."
cargo run -- organize test-files --dry-run

echo ""
echo "🎉 Quick start completed!"
echo "📖 Try these commands:"
echo "   cargo run -- organize test-files"
echo "   cargo run -- organize test-files --mode size --dry-run"
echo "   cargo run -- organize --help"
```

Make it executable and run:
```bash
chmod +x quick-start.sh
./quick-start.sh
```

---

This guide covers every way to build, run, and use the Rust File Organizer CLI! 🚀
