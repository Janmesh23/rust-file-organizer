pub mod file_types;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use colored::Colorize;
use walkdir::WalkDir;

use crate::OrganizeMode;
use file_types::{FileTypeClassifier, FileSizeCategory};

/// Represents a file operation to be performed
#[derive(Debug, Clone)]
pub struct FileOperation {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub operation_type: OperationType,
}

#[derive(Debug, Clone)]
pub enum OperationType {
    Move,
    Copy,
}

/// Main file organizer struct
pub struct FileOrganizer {
    classifier: FileTypeClassifier,
    operations: Vec<FileOperation>,
}

impl FileOrganizer {
    /// Create a new file organizer
    pub fn new() -> Self {
        Self {
            classifier: FileTypeClassifier::new(),
            operations: Vec::new(),
        }
    }

    /// Organize files in the specified directory
    pub fn organize(
        &mut self,
        target_dir: &Path,
        mode: &OrganizeMode,
        recursive: bool,
        filters: Option<&Vec<String>>,
        dry_run: bool,
    ) -> Result<OrganizationSummary> {
        println!("🔍 Scanning directory: {}", target_dir.display().to_string().cyan());
        
        // Collect all files to organize
        let files_to_organize = self.collect_files(target_dir, recursive)?;
        println!("📁 Found {} files to process", files_to_organize.len().to_string().yellow());
        
        // Filter files if filters are provided
        let filtered_files = if let Some(filter_list) = filters {
            self.filter_files(&files_to_organize, filter_list)
        } else {
            files_to_organize
        };

        if filtered_files.is_empty() {
            println!("ℹ️  No files to organize after filtering");
            return Ok(OrganizationSummary::new());
        }

        println!("🎯 Processing {} files after filtering", filtered_files.len().to_string().green());

        // Plan the organization
        let operations = self.plan_organization(&filtered_files, target_dir, mode)?;
        
        // Show preview
        self.show_preview(&operations, mode);
        
        if dry_run {
            println!("🔍 {} This was a dry run - no files were moved", "DRY RUN:".bold().yellow());
            return Ok(OrganizationSummary::from_operations(&operations));
        }

        // Execute the operations
        self.execute_operations(&operations)?;
        
        let summary = OrganizationSummary::from_operations(&operations);
        self.show_completion_summary(&summary);
        
        Ok(summary)
    }

    /// Collect all files in the directory
    fn collect_files(&self, target_dir: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        if recursive {
            for entry in WalkDir::new(target_dir)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() && !self.classifier.should_ignore(path) {
                    files.push(path.to_path_buf());
                }
            }
        } else {
            for entry in fs::read_dir(target_dir)
                .context("Failed to read directory")?
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() && !self.classifier.should_ignore(&path) {
                    files.push(path);
                }
            }
        }
        
        Ok(files)
    }

    /// Filter files based on provided filters
    fn filter_files(&self, files: &[PathBuf], filters: &[String]) -> Vec<PathBuf> {
        files
            .iter()
            .filter(|file| {
                if let Some(extension) = file.extension() {
                    if let Some(ext_str) = extension.to_str() {
                        return filters.contains(&ext_str.to_lowercase());
                    }
                }
                false
            })
            .cloned()
            .collect()
    }

    /// Plan the organization operations
    fn plan_organization(
        &self,
        files: &[PathBuf],
        target_dir: &Path,
        mode: &OrganizeMode,
    ) -> Result<Vec<FileOperation>> {
        let mut operations = Vec::new();
        let mut folder_counts: HashMap<String, usize> = HashMap::new();

        for file_path in files {
            let destination_folder = match mode {
                OrganizeMode::Extension => {
                    let category = self.classifier.classify(file_path);
                    format!("{} {}", category.emoji(), category.folder_name())
                }
                OrganizeMode::Size => {
                    let metadata = fs::metadata(file_path)
                        .context(format!("Failed to get metadata for {:?}", file_path))?;
                    let size_category = FileSizeCategory::from_size(metadata.len());
                    format!("{} {}", size_category.emoji(), size_category.folder_name())
                }
                OrganizeMode::Date => {
                    let metadata = fs::metadata(file_path)
                        .context(format!("Failed to get metadata for {:?}", file_path))?;
                    let created = metadata.created()
                        .or_else(|_| metadata.modified())
                        .context("Failed to get file creation/modification time")?;
                    
                    use chrono::{DateTime, Utc};
                    let datetime: DateTime<Utc> = created.into();
                    format!("📅 {}", datetime.format("%Y-%m"))
                }
                OrganizeMode::Modified => {
                    let metadata = fs::metadata(file_path)
                        .context(format!("Failed to get metadata for {:?}", file_path))?;
                    let modified = metadata.modified()
                        .context("Failed to get file modification time")?;
                    
                    use chrono::{DateTime, Utc};
                    let datetime: DateTime<Utc> = modified.into();
                    format!("🕒 {}", datetime.format("%Y-%m"))
                }
                OrganizeMode::Custom => {
                    // TODO: Implement custom rules from config
                    "📂 Custom".to_string()
                }
            };

            // Count files per folder for statistics
            *folder_counts.entry(destination_folder.clone()).or_insert(0) += 1;

            let destination_dir = target_dir.join(&destination_folder);
            let file_name = file_path.file_name()
                .context("Failed to get file name")?;
            let destination_path = destination_dir.join(file_name);

            operations.push(FileOperation {
                source: file_path.clone(),
                destination: destination_path,
                operation_type: OperationType::Move,
            });
        }

        Ok(operations)
    }

    /// Show preview of planned operations
    fn show_preview(&self, operations: &[FileOperation], mode: &OrganizeMode) {
        println!("\n{}", "📋 Organization Preview:".bold().blue());
        println!("Mode: {:?}", mode);
        
        // Group operations by destination folder
        let mut folder_groups: HashMap<String, Vec<&FileOperation>> = HashMap::new();
        
        for op in operations {
            if let Some(parent) = op.destination.parent() {
                if let Some(folder_name) = parent.file_name() {
                    if let Some(folder_str) = folder_name.to_str() {
                        folder_groups.entry(folder_str.to_string())
                            .or_insert_with(Vec::new)
                            .push(op);
                    }
                }
            }
        }

        for (folder_name, ops) in folder_groups {
            println!("\n📁 {} ({} files)", folder_name.green(), ops.len().to_string().yellow());
            
            // Show first few files as examples
            for op in ops.iter().take(3) {
                if let Some(file_name) = op.source.file_name() {
                    println!("   {} {}", "→".cyan(), file_name.to_string_lossy());
                }
            }
            
            if ops.len() > 3 {
                println!("   {} and {} more files...", "...".dimmed(), (ops.len() - 3).to_string().dimmed());
            }
        }
        
        println!("\n{} {} files will be organized", "Total:".bold(), operations.len().to_string().yellow());
    }

    /// Execute the planned operations
    fn execute_operations(&self, operations: &[FileOperation]) -> Result<()> {
        println!("\n{}", "🚀 Executing file operations...".bold().green());
        
        // Create all necessary directories first
        let mut dirs_created = 0;
        for op in operations {
            if let Some(parent) = op.destination.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)
                        .context(format!("Failed to create directory {:?}", parent))?;
                    dirs_created += 1;
                }
            }
        }
        
        if dirs_created > 0 {
            println!("📁 Created {} directories", dirs_created.to_string().cyan());
        }

        // Move files
        let mut moved_count = 0;
        let mut failed_count = 0;
        
        for op in operations {
            match fs::rename(&op.source, &op.destination) {
                Ok(_) => {
                    moved_count += 1;
                    if moved_count % 10 == 0 {
                        println!("📦 Moved {} files...", moved_count.to_string().green());
                    }
                }
                Err(e) => {
                    failed_count += 1;
                    eprintln!("❌ Failed to move {:?}: {}", op.source.file_name(), e);
                }
            }
        }

        println!("✅ Successfully moved {} files", moved_count.to_string().green());
        if failed_count > 0 {
            eprintln!("⚠️  {} files failed to move", failed_count.to_string().red());
        }

        Ok(())
    }

    /// Show completion summary
    fn show_completion_summary(&self, summary: &OrganizationSummary) {
        println!("\n{}", "🎉 Organization Complete!".bold().green());
        println!("📊 Summary:");
        println!("   Files processed: {}", summary.total_files.to_string().cyan());
        println!("   Folders created: {}", summary.folders_created.to_string().cyan());
        
        if !summary.categories.is_empty() {
            println!("   Categories:");
            for (category, count) in &summary.categories {
                println!("     {} {}: {}", "📁".cyan(), category, count.to_string().yellow());
            }
        }
    }
}

/// Summary of organization operation
#[derive(Debug)]
pub struct OrganizationSummary {
    pub total_files: usize,
    pub folders_created: usize,
    pub categories: HashMap<String, usize>,
}

impl OrganizationSummary {
    pub fn new() -> Self {
        Self {
            total_files: 0,
            folders_created: 0,
            categories: HashMap::new(),
        }
    }

    pub fn from_operations(operations: &[FileOperation]) -> Self {
        let mut categories = HashMap::new();
        let mut folders = std::collections::HashSet::new();

        for op in operations {
            if let Some(parent) = op.destination.parent() {
                if let Some(folder_name) = parent.file_name() {
                    if let Some(folder_str) = folder_name.to_str() {
                        *categories.entry(folder_str.to_string()).or_insert(0) += 1;
                        folders.insert(folder_str.to_string());
                    }
                }
            }
        }

        Self {
            total_files: operations.len(),
            folders_created: folders.len(),
            categories,
        }
    }
}
