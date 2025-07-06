# 🦀 Rust File Organizer CLI

A powerful command-line tool built with Rust that automatically organizes files in your directories based on their types, extensions, or custom rules.

## 🚀 Features

- **Smart File Organization**: Automatically sort files by extension, file type, or date
- **Custom Rules**: Define your own organization patterns
- **Safe Operations**: Preview changes before applying them
- **Undo Functionality**: Revert organization operations
- **Multiple Organization Modes**:
  - By file extension (images, documents, videos, etc.)
  - By file size (small, medium, large)
  - By creation/modification date
  - By custom patterns
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Fast & Memory Efficient**: Built with Rust's performance in mind

## 🛠️ Installation

### From Source
```bash
git clone https://github.com/Janmesh23/rust-file-organizer.git
cd rust-file-organizer
cargo build --release
cargo install --path .
```

### Using Cargo
```bash
cargo install file-organizer
```

## 📋 Usage

### Basic Commands

```bash
# Organize current directory by file extension
file-organizer organize

# Preview organization without making changes
file-organizer organize --dry-run

# Organize specific directory
file-organizer organize /path/to/directory

# Organize by file size
file-organizer organize --mode size

# Organize by date
file-organizer organize --mode date

# Undo last organization
file-organizer undo

# Show help
file-organizer --help
```

### Organization Modes

#### By Extension (Default)
```bash
file-organizer organize
# Creates folders: Images/, Documents/, Videos/, Audio/, Archives/, etc.
```

#### By Size
```bash
file-organizer organize --mode size
# Creates folders: Small/, Medium/, Large/
```

#### By Date
```bash
file-organizer organize --mode date
# Creates folders: 2024-01/, 2024-02/, etc.
```

#### Custom Rules
```bash
file-organizer organize --config custom-rules.toml
```

## 📁 File Type Categories

| Category | Extensions |
|----------|------------|
| **Images** | `.jpg`, `.jpeg`, `.png`, `.gif`, `.bmp`, `.svg`, `.webp` |
| **Documents** | `.pdf`, `.doc`, `.docx`, `.txt`, `.md`, `.rtf` |
| **Videos** | `.mp4`, `.avi`, `.mkv`, `.mov`, `.wmv`, `.flv` |
| **Audio** | `.mp3`, `.wav`, `.flac`, `.aac`, `.ogg`, `.m4a` |
| **Archives** | `.zip`, `.rar`, `.7z`, `.tar`, `.gz`, `.bz2` |
| **Code** | `.rs`, `.js`, `.py`, `.java`, `.cpp`, `.c`, `.go` |
| **Spreadsheets** | `.xlsx`, `.xls`, `.csv`, `.ods` |

## ⚙️ Configuration

Create a `config.toml` file to customize organization rules:

```toml
[general]
create_date_folders = true
backup_enabled = true
max_folder_depth = 3

[extensions]
# Custom extension mappings
[extensions.my_images]
folder = "MyImages"
extensions = ["jpg", "png", "custom_ext"]

[extensions.work_docs]
folder = "WorkDocuments"
extensions = ["docx", "xlsx", "pptx"]

[size_limits]
small = "10MB"
medium = "100MB"
# Files larger than medium go to "Large"

[ignore]
# Files/folders to ignore
patterns = [".git", "node_modules", "*.tmp"]
```

## 📚 Documentation

Comprehensive documentation is available in the `docs/` directory:

- **[📖 Code Walkthrough](docs/CODE_WALKTHROUGH.md)** - Line-by-line explanation of every file
- **[🔧 Build & Run Guide](docs/BUILD_AND_RUN.md)** - Complete guide to building and running the project
- **[🦀 Rust Concepts](docs/RUST_CONCEPTS.md)** - Explanation of all Rust concepts used in the project

## 🔧 Quick Start

### Prerequisites
- Rust 1.70.0 or higher
- Cargo

### Building
```bash
git clone https://github.com/Janmesh23/rust-file-organizer.git
cd rust-file-organizer
cargo build --release
```

### Running
```bash
# Show help
cargo run -- --help

# Preview organization (safe)
cargo run -- organize --dry-run

# Organize by file type
cargo run -- organize ~/Downloads --dry-run

# Organize by file size
cargo run -- organize --mode size --dry-run
```

### Testing
```bash
cargo test
```

## 📖 Examples

### Organize Downloads Folder
```bash
# Preview what would happen
file-organizer organize ~/Downloads --dry-run

# Actually organize
file-organizer organize ~/Downloads

# Undo if needed
file-organizer undo ~/Downloads
```

### Custom Organization
```bash
# Organize only image files
file-organizer organize --filter images

# Organize files older than 30 days
file-organizer organize --older-than 30d

# Organize but keep original structure for specific types
file-organizer organize --preserve code,documents
```

## 🚧 Roadmap

- [ ] **Phase 1: Core Functionality**
  - [x] Basic file organization by extension
  - [ ] Preview mode (--dry-run)
  - [ ] Undo functionality
  - [ ] Configuration file support

- [ ] **Phase 2: Advanced Features**
  - [ ] Organization by file size
  - [ ] Organization by date
  - [ ] Custom rules engine
  - [ ] Duplicate file detection

- [ ] **Phase 3: UI & UX**
  - [ ] Interactive mode
  - [ ] Progress bars
  - [ ] Colored output
  - [ ] Watch mode (auto-organize)

- [ ] **Phase 4: Integrations**
  - [ ] Shell completions
  - [ ] GUI version
  - [ ] Cloud storage integration

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 🐛 Bug Reports

If you find a bug, please create an issue with:
- Rust version (`rustc --version`)
- Operating system
- Command that caused the issue
- Expected vs actual behavior

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with ❤️ using Rust
- Inspired by the need for better file organization
- Thanks to the Rust community for amazing crates

## 📊 Project Stats

- **Language**: Rust 🦀
- **Type**: CLI Application
- **License**: MIT
- **Status**: In Development

---

**Made with 🦀 Rust by [Janmesh23](https://github.com/Janmesh23)**
