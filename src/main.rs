mod config;
mod player;
mod source;
mod terminal;
mod voice;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::config::Mode;

#[derive(Parser)]
#[command(
    name = "caw",
    version,
    about = "Read bilingual vocabulary aloud using macOS `say` (wordcast)"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    List,
    Play {
        /// Source name (filename in sources/ without .json)
        name: String,

        /// Repeat each word N times
        #[arg(short = 'r', long)]
        repeat: Option<u32>,

        /// Reading mode (which language plays first)
        #[arg(short = 'm', long, value_enum)]
        mode: Option<Mode>,

        /// Speech rate (words per minute)
        #[arg(long)]
        rate: Option<u32>,

        /// Pause between words in milliseconds (half this between CN and foreign)
        #[arg(long)]
        pause_ms: Option<u64>,

        /// Cycle the source forever (Ctrl+C to stop)
        #[arg(short = 'c', long)]
        cycle: bool,

        /// Play words in reverse order
        #[arg(long)]
        reverse: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let outcome = config::load_or_init()?;

    if outcome.freshly_initialized {
        eprintln!(
            "Initialized wordcast at {}",
            config::config_dir()?.display()
        );
        eprintln!("Created example source: sources/korean-basic.json\n");
    }

    match cli.command {
        Command::List => cmd_list(),
        Command::Play {
            name,
            repeat,
            mode,
            rate,
            pause_ms,
            cycle,
            reverse,
        } => cmd_play(
            &outcome.config,
            &name,
            repeat,
            mode,
            rate,
            pause_ms,
            cycle,
            reverse,
        ),
    }
}

fn cmd_list() -> Result<()> {
    let names = source::list()?;
    let dir = config::sources_dir()?;
    if names.is_empty() {
        println!("No sources in {}", dir.display());
        println!("Drop JSON files like {{\"你好\":\"안녕하세요\"}} into that directory.");
        return Ok(());
    }
    println!("Sources in {}:\n", dir.display());
    for name in &names {
        match source::load(name) {
            Ok(src) => println!("  {:<28}  {} words", name, src.words.len()),
            Err(e) => println!("  {:<28}  (error: {})", name, e),
        }
    }
    println!("\nUse: caw play <name>");
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn cmd_play(
    cfg: &config::Config,
    name: &str,
    repeat: Option<u32>,
    mode: Option<Mode>,
    rate: Option<u32>,
    pause_ms: Option<u64>,
    cycle: bool,
    reverse: bool,
) -> Result<()> {
    let mode = mode.unwrap_or(cfg.playback.mode);
    let repeat = repeat.unwrap_or(cfg.playback.repeat);
    let rate = rate.unwrap_or(cfg.playback.rate);
    let pause_ms = pause_ms.unwrap_or(cfg.playback.pause_ms);
    let cycle = cycle || cfg.playback.cycle;
    let reverse = reverse || cfg.playback.reverse;

    let src = source::load(name)?;
    let mut words: Vec<(usize, (String, String))> = src.words.into_iter().enumerate().collect();
    if reverse {
        words.reverse();
    }

    println!(
        "Playing {} ({} words) — mode: {:?}, repeat: {}, rate: {} wpm, pause: {}ms, cycle: {}, reverse: {}",
        name,
        words.len(),
        mode,
        repeat,
        rate,
        pause_ms,
        cycle,
        reverse,
    );

    let opts = player::PlaybackOptions {
        voices: cfg.voice.clone(),
        mode,
        rate,
        pause_ms,
        repeat,
        cycle,
        tab_title: cfg.terminal.tab_title,
    };
    player::play(&words, &opts)?;
    Ok(())
}
