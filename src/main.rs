use clap::Parser;
use log::{LevelFilter, info, warn};
use rand::Rng;
use std::fs;
use std::io::{self, BufRead};
use std::process::Command;
use std::string::String;
use systemd_journal_logger::JournalLog;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    index: Option<String>,

    #[arg(short, long)]
    color: Option<String>,
}

fn main() -> std::io::Result<()> {
    JournalLog::new()
        .unwrap()
        .with_syslog_identifier("rand-alacritty".to_string())
        .install()
        .unwrap();
    log::set_max_level(LevelFilter::Debug);

    let args = Args::parse();

    let curr_dir = std::env::current_exe()?
        .parent()
        .expect("Expected to find the parent directory of the executable")
        .to_path_buf();
    let colors_path = curr_dir.join("colors");
    let file = fs::File::open(&colors_path)?;
    let reader = io::BufReader::new(file);
    let colors: Vec<String> = reader.lines().map_while(Result::ok).collect();

    let mut color: String = String::new();
    let index: usize;
    if let Some(idx) = args.index {
        index = idx.parse::<usize>().unwrap_or(0);
        if let Some(clr) = colors.get(idx.parse::<usize>().unwrap_or(0)) {
            color = clr.clone();
        } else {
            warn!("Index {idx} is out of bounds");
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
            "--option=colors.primary.background='{chosen_color}'"
        ))
        .spawn()
        .expect("Failed to launch Alacritty terminal emulator")
        .wait()?;

    Ok(())
}
