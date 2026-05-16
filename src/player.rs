use anyhow::{Context, Result};
use std::process::Command;
use std::thread;
use std::time::Duration;

use crate::config::{Mode, VoiceConfig};
use crate::voice::LangHint;

pub struct PlaybackOptions {
    pub voices: VoiceConfig,
    pub mode: Mode,
    pub rate: u32,
    pub pause_ms: u64,
    pub repeat: u32,
    pub cycle: bool,
}

pub fn play(words: &[(String, String)], opts: &PlaybackOptions) -> Result<()> {
    let total = words.len();
    let inter_step_pause = Duration::from_millis(opts.pause_ms / 2);
    let inter_word_pause = Duration::from_millis(opts.pause_ms);
    let voice_chinese = &opts.voices.chinese;

    let mut cycle_num: u64 = 0;
    loop {
        cycle_num += 1;
        if opts.cycle && cycle_num > 1 {
            println!("\n↻ Cycle {cycle_num} (Ctrl+C to stop)");
        }
        for (idx, (cn, foreign)) in words.iter().enumerate() {
            let voice_foreign = LangHint::detect(foreign).voice_for(&opts.voices);

            println!(
                "[{}/{total}] {cn}  ⇄  {foreign}  ({voice_foreign})",
                idx + 1
            );

            for _ in 0..opts.repeat.max(1) {
                let (first, first_voice, second, second_voice) = match opts.mode {
                    Mode::CnFirst => (cn, voice_chinese, foreign, &voice_foreign),
                    Mode::FlFirst => (foreign, &voice_foreign, cn, voice_chinese),
                };
                say(first, first_voice, opts.rate)?;
                thread::sleep(inter_step_pause);
                say(second, second_voice, opts.rate)?;
            }
            thread::sleep(inter_word_pause);
        }
        if !opts.cycle {
            break;
        }
    }
    Ok(())
}

fn say(text: &str, voice: &str, rate: u32) -> Result<()> {
    let status = Command::new("say")
        .args(["-v", voice, "-r", &rate.to_string(), text])
        .status()
        .context("Failed to spawn `say` (wordcast requires macOS)")?;
    if !status.success() {
        anyhow::bail!("`say` exited with status {status}");
    }
    Ok(())
}
