use std::env;
use std::io::{self, IsTerminal};
use std::process::{Command, Stdio};

fn is_kitty() -> bool {
    env::var_os("KITTY_WINDOW_ID").is_some()
}

fn has_kitty_socket() -> bool {
    env::var_os("KITTY_LISTEN_ON").is_some()
}

pub fn should_set_tab_title() -> bool {
    if !is_kitty() || !io::stderr().is_terminal() {
        return false;
    }
    if !has_kitty_socket() {
        eprintln!(
            "wordcast: tab 标题已禁用（请在 ~/.config/kitty/kitty.conf 添加 `allow_remote_control yes` 后重启 kitty）"
        );
        return false;
    }
    let ok = Command::new("kitten")
        .args(["@", "ls"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if !ok {
        eprintln!("wordcast: tab 标题已禁用（kitten 不可用或 socket 不可达）");
    }
    ok
}

pub fn set_tab_title(title: &str) {
    let sanitized = sanitize(title);
    let _ = Command::new("kitten")
        .args(["@", "set-tab-title", &sanitized])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}

pub fn clear_tab_title() {
    set_tab_title("");
}

fn sanitize(s: &str) -> String {
    s.chars().filter(|c| !c.is_control()).take(256).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_strips_escapes_and_bells() {
        assert_eq!(sanitize("hello\x1b]0;evil\x07world"), "hello]0;evilworld");
        assert_eq!(sanitize("a\nb\tc"), "abc");
    }

    #[test]
    fn sanitize_keeps_unicode() {
        assert_eq!(sanitize("안녕하세요"), "안녕하세요");
        assert_eq!(sanitize("ありがとう"), "ありがとう");
    }

    #[test]
    fn sanitize_caps_length() {
        let long: String = "a".repeat(1000);
        assert_eq!(sanitize(&long).chars().count(), 256);
    }
}
