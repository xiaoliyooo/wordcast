use anyhow::{Context, Result};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub playback: PlaybackConfig,
    pub voice: VoiceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PlaybackConfig {
    pub mode: Mode,
    pub repeat: u32,
    pub rate: u32,
    pub pause_ms: u64,
    pub cycle: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    CnFirst,
    FlFirst,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct VoiceConfig {
    pub korean: String,
    pub english: String,
    pub japanese: String,
    pub chinese: String,
    pub default: String,
}

impl Default for PlaybackConfig {
    fn default() -> Self {
        Self {
            mode: Mode::FlFirst,
            repeat: 1,
            rate: 160,
            pause_ms: 800,
            cycle: false,
        }
    }
}

impl Default for VoiceConfig {
    fn default() -> Self {
        Self {
            korean: "Yuna".to_string(),
            english: "Samantha".to_string(),
            japanese: "Kyoko".to_string(),
            chinese: "Tingting".to_string(),
            default: "Samantha".to_string(),
        }
    }
}

pub fn config_dir() -> Result<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
        .context("Could not resolve $XDG_CONFIG_HOME or $HOME")?;
    Ok(base.join("wordcast"))
}

pub fn config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.toml"))
}

pub fn sources_dir() -> Result<PathBuf> {
    Ok(config_dir()?.join("sources"))
}

pub struct LoadOutcome {
    pub config: Config,
    pub freshly_initialized: bool,
}

pub fn load_or_init() -> Result<LoadOutcome> {
    let cfg_dir = config_dir()?;
    let cfg_path = config_path()?;
    let src_dir = sources_dir()?;

    let freshly_initialized = !cfg_dir.exists();

    if freshly_initialized {
        fs::create_dir_all(&cfg_dir)
            .with_context(|| format!("Failed to create {}", cfg_dir.display()))?;
        fs::create_dir_all(&src_dir)
            .with_context(|| format!("Failed to create {}", src_dir.display()))?;
        write_default_config(&cfg_path)?;
        write_example_source(&src_dir)?;
    } else {
        if !cfg_path.exists() {
            write_default_config(&cfg_path)?;
        }
        if !src_dir.exists() {
            fs::create_dir_all(&src_dir)?;
        }
    }

    let raw = fs::read_to_string(&cfg_path)
        .with_context(|| format!("Failed to read {}", cfg_path.display()))?;
    let config: Config =
        toml::from_str(&raw).with_context(|| format!("Failed to parse {}", cfg_path.display()))?;

    Ok(LoadOutcome {
        config,
        freshly_initialized,
    })
}

fn write_default_config(path: &Path) -> Result<()> {
    let template = r#"# wordcast configuration
# CLI 参数永远覆盖此处的值

[playback]
# 阅读顺序：cn-first（先中文）| fl-first（先外语）
mode = "fl-first"
# 每个词重复朗读次数
repeat = 1
# 阅读速度（每分钟词数）
rate = 160
# 词对之间停顿（毫秒）；同对内中↔外取一半
pause_ms = 800
# 读完所有词后是否循环（Ctrl+C 退出）
cycle = false

[voice]
# 韩语朗读音色（`say -v ?` 查看系统可用音色）
korean = "Yuna"
# 英语朗读音色
english = "Samantha"
# 日语朗读音色
japanese = "Kyoko"
# 中文 key 朗读音色
chinese = "Tingting"
# 兜底音色（未匹配 Hangul / Kana / ASCII 时使用，如纯汉字 value）
default = "Samantha"
"#;
    fs::write(path, template).with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

fn write_example_source(sources_dir: &Path) -> Result<()> {
    let path = sources_dir.join("korean-basic.json");
    let sample = r#"{
  "你好": "안녕하세요",
  "谢谢": "감사합니다",
  "对不起": "죄송합니다",
  "再见": "안녕히 가세요",
  "是": "네",
  "不是": "아니요"
}
"#;
    fs::write(&path, sample).with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_toml_uses_all_defaults() {
        let cfg: Config = toml::from_str("").unwrap();
        assert_eq!(cfg.playback.mode, Mode::FlFirst);
        assert_eq!(cfg.playback.repeat, 1);
        assert_eq!(cfg.playback.rate, 160);
        assert_eq!(cfg.playback.pause_ms, 800);
        assert!(!cfg.playback.cycle);
        assert_eq!(cfg.voice.korean, "Yuna");
        assert_eq!(cfg.voice.default, "Samantha");
    }

    #[test]
    fn partial_toml_overrides_only_present_fields() {
        let raw = r#"
[playback]
mode = "cn-first"
rate = 220
"#;
        let cfg: Config = toml::from_str(raw).unwrap();
        assert_eq!(cfg.playback.mode, Mode::CnFirst);
        assert_eq!(cfg.playback.rate, 220);
        assert_eq!(cfg.playback.repeat, 1);
        assert_eq!(cfg.playback.pause_ms, 800);
        assert!(!cfg.playback.cycle);
        assert_eq!(cfg.voice.korean, "Yuna");
        assert_eq!(cfg.voice.chinese, "Tingting");
    }
}
