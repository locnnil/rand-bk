use clap::Parser;
use log::{LevelFilter, error, info, warn};
use rand::Rng;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::string::String;
use systemd_journal_logger::JournalLog;

// Embed the colors file at compile time as fallback
const EMBEDDED_COLORS: &str = include_str!("../colors");

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    index: Option<String>,

    #[arg(short, long)]
    color: Option<String>,
}

/// Attempts to load colors from external file, falls back to embedded colors
fn load_colors() -> Result<String, Box<dyn std::error::Error>> {
    // Try multiple locations for the colors file
    let possible_paths = get_possible_colors_paths();

    for path in possible_paths {
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    info!("Loaded colors from: {}", path.display());
                    return Ok(content);
                }
                Err(e) => {
                    warn!("Failed to read colors from {}: {}", path.display(), e);
                }
            }
        }
    }

    // Fallback to embedded colors
    info!("Using embedded colors as fallback");
    Ok(EMBEDDED_COLORS.to_string())
}

/// Returns a list of possible paths where the colors file might be located
fn get_possible_colors_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    // 1. Same directory as the executable
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            paths.push(exe_dir.join("colors"));
        }
    }

    // 2. Current working directory
    paths.push(PathBuf::from("colors"));

    // 3. XDG config directory (Linux/Unix)
    if let Ok(home) = std::env::var("HOME") {
        paths.push(
            PathBuf::from(home)
                .join(".config")
                .join("rand-bk")
                .join("colors"),
        );
    }

    // 4. User's home directory
    if let Ok(home) = std::env::var("HOME") {
        paths.push(PathBuf::from(home).join(".rand-bk-colors"));
    }

    // 5. System-wide config (Linux/Unix)
    paths.push(PathBuf::from("/etc/rand-bk/colors"));

    paths
}

/// Parse colors from the loaded content
fn parse_colors(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && line.starts_with('#'))
        .map(|line| line.to_string())
        .collect()
}

fn main() -> std::io::Result<()> {
    JournalLog::new()
        .unwrap()
        .with_syslog_identifier("rand-alacritty".to_string())
        .install()
        .unwrap();
    log::set_max_level(LevelFilter::Debug);

    let args = Args::parse();

    // Load colors using the new fallback mechanism
    let colors = match load_colors() {
        Ok(content) => {
            let parsed_colors = parse_colors(&content);
            if parsed_colors.is_empty() {
                error!("No valid colors found in colors file");
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "No valid colors found",
                ));
            }
            parsed_colors
        }
        Err(e) => {
            error!("Failed to load colors: {e}");
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Failed to load colors: {e}"),
            ));
        }
    };

    info!("Loaded {} colors", colors.len());

    let mut color: String = String::new();
    let index: usize;

    if let Some(idx) = args.index {
        index = idx.parse::<usize>().unwrap_or(0);
        if let Some(clr) = colors.get(index) {
            color = clr.clone();
        } else {
            warn!("Index {} is out of bounds (max: {})", idx, colors.len() - 1);
        }
    } else {
        index = 0;
    }

    if let Some(clr) = args.color {
        color = clr
    }

    let chosen_color = if color.is_empty() {
        let mut rng = rand::rng();
        let random_index = rng.random_range(0..colors.len());
        info!("Chosen randomly at index: {random_index}");
        colors[random_index].clone()
    } else {
        info!("Chosen from index: {index}");
        color
    };

    info!("Chosen color: {chosen_color}");

    // alacritty --option="colors.primary.background='${customcolor}'"
    Command::new("alacritty")
        .arg(format!(
            "--option=colors.primary.background='{chosen_color}'",
        ))
        .spawn()
        .expect("Failed to launch Alacritty terminal emulator")
        .wait()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedded_colors_fallback() {
        // This should always work since we have embedded colors
        let colors = load_colors().unwrap();
        assert!(!colors.is_empty());
    }

    #[test]
    fn test_parse_colors() {
        let test_content = "#ff0000\n#00ff00\n\n#0000ff\ninvalid_line\n#ffffff";
        let colors = parse_colors(test_content);
        assert_eq!(colors.len(), 4);
        assert_eq!(colors[0], "#ff0000");
        assert_eq!(colors[1], "#00ff00");
        assert_eq!(colors[2], "#0000ff");
        assert_eq!(colors[3], "#ffffff");
    }
}
