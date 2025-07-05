use std::collections::HashMap;
use std::path::Path;

/// File categories for organization
#[derive(Debug, Clone, PartialEq)]
pub enum FileCategory {
    Images,
    Documents,
    Videos,
    Audio,
    Archives,
    Code,
    Spreadsheets,
    Presentations,
    Executables,
    Fonts,
    Other,
}

impl FileCategory {
    /// Get the folder name for this category
    pub fn folder_name(&self) -> &'static str {
        match self {
            FileCategory::Images => "Images",
            FileCategory::Documents => "Documents",
            FileCategory::Videos => "Videos",
            FileCategory::Audio => "Audio",
            FileCategory::Archives => "Archives",
            FileCategory::Code => "Code",
            FileCategory::Spreadsheets => "Spreadsheets",
            FileCategory::Presentations => "Presentations",
            FileCategory::Executables => "Executables",
            FileCategory::Fonts => "Fonts",
            FileCategory::Other => "Other",
        }
    }

    /// Get emoji representation for this category
    pub fn emoji(&self) -> &'static str {
        match self {
            FileCategory::Images => "🖼️",
            FileCategory::Documents => "📄",
            FileCategory::Videos => "🎬",
            FileCategory::Audio => "🎵",
            FileCategory::Archives => "📦",
            FileCategory::Code => "💻",
            FileCategory::Spreadsheets => "📊",
            FileCategory::Presentations => "📈",
            FileCategory::Executables => "⚙️",
            FileCategory::Fonts => "🔤",
            FileCategory::Other => "📂",
        }
    }
}

/// File type classifier
pub struct FileTypeClassifier {
    extension_map: HashMap<String, FileCategory>,
}

impl FileTypeClassifier {
    /// Create a new file type classifier with default mappings
    pub fn new() -> Self {
        let mut extension_map = HashMap::new();

        // Images
        let image_extensions = vec![
            "jpg", "jpeg", "png", "gif", "bmp", "svg", "webp", "tiff", "tif",
            "ico", "raw", "cr2", "nef", "arw", "dng", "psd", "ai", "eps",
        ];
        for ext in image_extensions {
            extension_map.insert(ext.to_string(), FileCategory::Images);
        }

        // Documents
        let document_extensions = vec![
            "pdf", "doc", "docx", "txt", "md", "rtf", "odt", "pages", "tex",
            "wps", "wpd", "html", "htm", "xml", "json", "yaml", "yml", "toml",
        ];
        for ext in document_extensions {
            extension_map.insert(ext.to_string(), FileCategory::Documents);
        }

        // Videos
        let video_extensions = vec![
            "mp4", "avi", "mkv", "mov", "wmv", "flv", "webm", "m4v", "3gp",
            "mpg", "mpeg", "ogv", "f4v", "asf", "rm", "rmvb", "vob",
        ];
        for ext in video_extensions {
            extension_map.insert(ext.to_string(), FileCategory::Videos);
        }

        // Audio
        let audio_extensions = vec![
            "mp3", "wav", "flac", "aac", "ogg", "wma", "m4a", "opus", "aiff",
            "au", "ra", "3ga", "amr", "awb", "dss", "dvf", "m4b", "m4p", "mmf",
        ];
        for ext in audio_extensions {
            extension_map.insert(ext.to_string(), FileCategory::Audio);
        }

        // Archives
        let archive_extensions = vec![
            "zip", "rar", "7z", "tar", "gz", "bz2", "xz", "z", "lzma", "cab",
            "iso", "dmg", "pkg", "deb", "rpm", "msi", "exe", "jar", "war",
        ];
        for ext in archive_extensions {
            extension_map.insert(ext.to_string(), FileCategory::Archives);
        }

        // Code
        let code_extensions = vec![
            "rs", "py", "js", "ts", "java", "cpp", "c", "h", "hpp", "cs", "php",
            "rb", "go", "swift", "kt", "scala", "r", "m", "pl", "sh", "bash",
            "zsh", "fish", "ps1", "bat", "cmd", "sql", "css", "scss", "sass",
            "less", "vue", "jsx", "tsx", "svelte", "elm", "clj", "hs", "ml",
        ];
        for ext in code_extensions {
            extension_map.insert(ext.to_string(), FileCategory::Code);
        }

        // Spreadsheets
        let spreadsheet_extensions = vec!["xlsx", "xls", "csv", "ods", "numbers", "tsv"];
        for ext in spreadsheet_extensions {
            extension_map.insert(ext.to_string(), FileCategory::Spreadsheets);
        }

        // Presentations
        let presentation_extensions = vec!["pptx", "ppt", "odp", "key"];
        for ext in presentation_extensions {
            extension_map.insert(ext.to_string(), FileCategory::Presentations);
        }

        // Executables
        let executable_extensions = vec!["exe", "msi", "app", "deb", "rpm", "pkg", "dmg"];
        for ext in executable_extensions {
            extension_map.insert(ext.to_string(), FileCategory::Executables);
        }

        // Fonts
        let font_extensions = vec!["ttf", "otf", "woff", "woff2", "eot"];
        for ext in font_extensions {
            extension_map.insert(ext.to_string(), FileCategory::Fonts);
        }

        Self { extension_map }
    }

    /// Classify a file based on its extension
    pub fn classify(&self, file_path: &Path) -> FileCategory {
        if let Some(extension) = file_path.extension() {
            if let Some(ext_str) = extension.to_str() {
                let lowercase_ext = ext_str.to_lowercase();
                return self.extension_map
                    .get(&lowercase_ext)
                    .cloned()
                    .unwrap_or(FileCategory::Other);
            }
        }
        FileCategory::Other
    }

    /// Get all supported extensions for a category
    pub fn get_extensions_for_category(&self, category: &FileCategory) -> Vec<String> {
        self.extension_map
            .iter()
            .filter(|(_, cat)| *cat == category)
            .map(|(ext, _)| ext.clone())
            .collect()
    }

    /// Check if file should be ignored (system files, hidden files, etc.)
    pub fn should_ignore(&self, file_path: &Path) -> bool {
        if let Some(file_name) = file_path.file_name() {
            if let Some(name_str) = file_name.to_str() {
                // Ignore hidden files (starting with .)
                if name_str.starts_with('.') {
                    return true;
                }
                
                // Ignore common system files
                let system_files = vec![
                    "Thumbs.db", "Desktop.ini", ".DS_Store", "Icon\r",
                    "desktop.ini", "thumbs.db", "ehthumbs.db",
                ];
                
                if system_files.contains(&name_str) {
                    return true;
                }
            }
        }
        false
    }
}

/// File size categories for size-based organization
#[derive(Debug, Clone, PartialEq)]
pub enum FileSizeCategory {
    Tiny,    // < 1 MB
    Small,   // 1 MB - 10 MB
    Medium,  // 10 MB - 100 MB
    Large,   // 100 MB - 1 GB
    Huge,    // > 1 GB
}

impl FileSizeCategory {
    /// Get the folder name for this size category
    pub fn folder_name(&self) -> &'static str {
        match self {
            FileSizeCategory::Tiny => "Tiny (< 1MB)",
            FileSizeCategory::Small => "Small (1-10MB)",
            FileSizeCategory::Medium => "Medium (10-100MB)",
            FileSizeCategory::Large => "Large (100MB-1GB)",
            FileSizeCategory::Huge => "Huge (> 1GB)",
        }
    }

    /// Get emoji representation for this size category
    pub fn emoji(&self) -> &'static str {
        match self {
            FileSizeCategory::Tiny => "🔍",
            FileSizeCategory::Small => "📄",
            FileSizeCategory::Medium => "📁",
            FileSizeCategory::Large => "📦",
            FileSizeCategory::Huge => "🗃️",
        }
    }

    /// Classify file size into categories
    pub fn from_size(size_bytes: u64) -> Self {
        const MB: u64 = 1_024 * 1_024;
        const GB: u64 = 1_024 * MB;

        match size_bytes {
            0..=1_048_576 => FileSizeCategory::Tiny,           // < 1 MB
            1_048_577..=10_485_760 => FileSizeCategory::Small, // 1-10 MB
            10_485_761..=104_857_600 => FileSizeCategory::Medium, // 10-100 MB
            104_857_601..=1_073_741_824 => FileSizeCategory::Large, // 100 MB - 1 GB
            _ => FileSizeCategory::Huge,                       // > 1 GB
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_file_classification() {
        let classifier = FileTypeClassifier::new();
        
        assert_eq!(classifier.classify(&PathBuf::from("test.jpg")), FileCategory::Images);
        assert_eq!(classifier.classify(&PathBuf::from("document.pdf")), FileCategory::Documents);
        assert_eq!(classifier.classify(&PathBuf::from("video.mp4")), FileCategory::Videos);
        assert_eq!(classifier.classify(&PathBuf::from("song.mp3")), FileCategory::Audio);
        assert_eq!(classifier.classify(&PathBuf::from("archive.zip")), FileCategory::Archives);
        assert_eq!(classifier.classify(&PathBuf::from("script.rs")), FileCategory::Code);
        assert_eq!(classifier.classify(&PathBuf::from("unknown.xyz")), FileCategory::Other);
    }

    #[test]
    fn test_size_classification() {
        assert_eq!(FileSizeCategory::from_size(500_000), FileSizeCategory::Tiny);
        assert_eq!(FileSizeCategory::from_size(5_000_000), FileSizeCategory::Small);
        assert_eq!(FileSizeCategory::from_size(50_000_000), FileSizeCategory::Medium);
        assert_eq!(FileSizeCategory::from_size(500_000_000), FileSizeCategory::Large);
        assert_eq!(FileSizeCategory::from_size(2_000_000_000), FileSizeCategory::Huge);
    }

    #[test]
    fn test_should_ignore() {
        let classifier = FileTypeClassifier::new();
        
        assert!(classifier.should_ignore(&PathBuf::from(".hidden")));
        assert!(classifier.should_ignore(&PathBuf::from(".DS_Store")));
        assert!(classifier.should_ignore(&PathBuf::from("Thumbs.db")));
        assert!(!classifier.should_ignore(&PathBuf::from("normal_file.txt")));
    }
}
