use clap::{Arg, Command};
use prettytable::Cell;
use prettytable::Row;
use prettytable::Table;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;
use std::path::PathBuf;
use std::{fs, os::windows::fs::MetadataExt};
use walkdir::{DirEntry, WalkDir};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Debug)]
struct ProjectFile {
    hash: String,
    projects: Vec<Project>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    path: String,
    package_name: Option<String>,
}

fn main() -> std::io::Result<()> {
    let matches = Command::new("cargo-project-finder")
        .version(VERSION)
        .author("Ahmet Özcan")
        .about("Finds Cargo projects in directories recursively")
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .num_args(0..=1)
                .default_missing_value("cargo_projects")
                .help("Write output to JSON file (defaults to cargo_projects)"),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("DIR")
                .help("Directory to search (defaults to home directory)"),
        )
        .arg(
            Arg::new("noskip")
                .short('n')
                .long("noskip")
                .help("Don't skip hidden files and directories (will take some time)")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let search_path: PathBuf = matches
        .get_one::<String>("path")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            std::env::var("HOME")
                .or_else(|_| std::env::var("USERPROFILE"))
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."))
        });

    let skip_hidden = !matches.get_flag("noskip");

    if !search_path.exists() {
        eprintln!("Search path does not exist: {}", search_path.display());
        return Ok(());
    }

    let projects = find_cargo_projects(&search_path, skip_hidden)?;
    let current_hash = calculate_hash(&projects);

    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new("Project Name"), Cell::new("Path")]));

    for project in &projects {
        table.add_row(Row::new(vec![
            Cell::new(project.package_name.as_deref().unwrap_or("Unknown")),
            Cell::new(&project.path),
        ]));
    }

    table.printstd();

    if let Some(output_file) = matches.get_one::<String>("output") {
        let output_file = format!("{}.json", output_file);
        let project_file = ProjectFile {
            hash: current_hash,
            projects,
        };

        fs::write(
            &output_file,
            serde_json::to_string_pretty(&project_file).unwrap(),
        )?;

        println!("\nOutput written to: {}", output_file);
    }

    Ok(())
}

fn calculate_hash(projects: &[Project]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(serde_json::to_string(projects).unwrap().as_bytes());
    format!("{:x}", hasher.finalize())
}

fn find_cargo_projects(root: &Path, skip_hidden: bool) -> std::io::Result<Vec<Project>> {
    let mut projects = Vec::new();

    let walker = WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !skip_hidden || !should_skip(e))
        .filter_map(Result::ok);

    for entry in walker {
        if entry.file_name() == "Cargo.toml" {
            if let Some(parent) = entry.path().parent() {
                match fs::read_to_string(entry.path()) {
                    Ok(contents) => {
                        if contents.contains("[package]") {
                            projects.push(Project {
                                path: parent.display().to_string(),
                                package_name: extract_package_name(&contents),
                            });
                        }
                    }
                    Err(_) => continue,
                }
            }
        }
    }

    Ok(projects)
}

#[cfg(windows)]
fn should_skip(entry: &DirEntry) -> bool {
    // Check if file is hidden using Windows metadata
    if let Ok(metadata) = entry.metadata() {
        let attributes = metadata.file_attributes();
        // FILE_ATTRIBUTE_HIDDEN is 0x2
        if attributes & 0x2 != 0 {
            return true;
        }
    }

    // TODO: Make more readable
    // ?: Return block
    entry
        .path()
        .components()
        .any(|c| c.as_os_str().to_string_lossy().starts_with('.'))
}

#[cfg(not(windows))]
fn should_skip(entry: &DirEntry) -> bool {
    // Unix-like systems hidden file check
    entry
        .path()
        .components()
        .any(|c| c.as_os_str().to_string_lossy().starts_with('.'))
}

fn extract_package_name(contents: &str) -> Option<String> {
    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with("name") {
            return line
                .split('=')
                .nth(1)
                .map(|s| s.trim().trim_matches('"').to_string());
        }
    }
    None
}
