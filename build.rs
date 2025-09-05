use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Tell Cargo to rerun if colors file changes
    println!("cargo:rerun-if-changed=colors");

    // Get the output directory
    let out_dir = env::var("OUT_DIR").unwrap();

    // Copy colors file to output directory (for potential installation)
    let dest_path = Path::new(&out_dir).join("colors");
    if let Err(e) = fs::copy("colors", &dest_path) {
        eprintln!("Warning: Failed to copy colors file: {}", e);
    }

    // Also make it available in the target directory for development
    if let Ok(target_dir) = env::var("CARGO_TARGET_DIR") {
        let target_colors = Path::new(&target_dir).join("colors");
        let _ = fs::copy("colors", target_colors);
    }
}
