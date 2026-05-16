# wordcast

> macOS 双语朗读 CLI：JSON 词表逐对朗读，晨读节奏，工作背景音。

**仅支持 macOS**（系统可用音色：`say -v '?'`）。二进制 `caw`，包名 `wordcast`。

## 功能特性

- **双语朗读**：JSON 词表逐对朗读中文 key + 外语 value，支持 `fl-first`（先外语）/ `cn-first`（先中文）两种顺序
- **多语种音色**：自动识别词汇语言选择对应音色
- **Terminal Tab 同步**：朗读时 tab 标题展示当前词汇方便工作中随时查看（当前仅支持Kitty，在 kitty.conf 打开远程控制 `allow_remote_control yes`）
- **节奏可调**：速率、重复次数、词对停顿、循环播放、倒序阅读等
- **零依赖**：底层走 macOS 自带 `say`（如 `say -v Yuna -r 160 안녕하세요`），不联网、不下模型、单一静态二进制
- 可配置

## Install

```bash
cargo install --git https://github.com/xiaoliyooo/wordcast.git
```

## Usage

```bash
caw list                                  # 列出所有词表
caw play korean-basic                     # 播放词表
caw play korean-basic --rate 220 --cycle  # 加速 + 循环
caw play --help                           # 全部参数
```

### CLI 参数（`caw play <name>`）

| 参数 | 默认 | 说明 |
|---|---|---|
| `<name>` | — | 词表名（即 `sources/<name>.json`，必需位置参数） |
| `-m, --mode <MODE>` | `fl-first` | 朗读顺序：`cn-first`（先中文）/ `fl-first`（先外语） |
| `-r, --repeat <N>` | `1` | 每个词重复朗读次数 |
| `--rate <WPM>` | `160` | 朗读速度（每分钟词数） |
| `--pause-ms <MS>` | `800` | 双语之间停顿间隔（毫秒） |
| `-c, --cycle` | `false` | 循环播放（Ctrl+C 退出） |
| `--reverse` | `false` | 倒序阅读词表 |
| `-h, --help` | — | 显示帮助 |
| `-V, --version` | — | 显示版本 |

> **CLI 参数覆盖 [`config.toml`](#配置)。**
>
> 配置路径默认 `~/.config/wordcast/`，可通过环境变量 `XDG_CONFIG_HOME` 改写。

## 配置

首次运行会在 `~/.config/wordcast/`（或 `$XDG_CONFIG_HOME/wordcast/`）自动生成：

```
~/.config/wordcast/
├── config.toml          # cli配置
└── sources/
    └── korean-basic.json
```

### 默认 `config.toml`

CLI 参数将覆盖配置的值，默认配置如下

```toml
# wordcast configuration

[playback]
# 阅读顺序：cn-first（先中文）| fl-first（先外语）
mode = "fl-first"
# 每组词重复朗读次数
repeat = 1
# 阅读速度（毫秒）
rate = 160
# 词对之间停顿（毫秒）
pause_ms = 800
# 是否循环播放
cycle = false
# 是否倒序阅读词表
reverse = false

[voice]
# 韩语朗读音色（`say -v ?` 查看系统可用音色）
korean = "Yuna"
# 英语朗读音色
english = "Samantha"
# 日语朗读音色
japanese = "Kyoko"
# 中文 key 朗读音色
chinese = "Tingting"
# 默认音色
default = "Samantha"

[terminal]
# 把 tab 标题设为当前外语词（仅 kitty；需 `allow_remote_control yes`）
tab_title = true
```

## 词表格式

`sources/<name>.json` —— key 是中文，value 是外语，按 JSON 顺序播放：

```json
{
  "你好": "안녕하세요",
  "谢谢": "ありがとう",
  "再见": "Goodbye"
}
```

文件名去掉 `.json` 后缀即词表名（`korean-basic.json` → `caw play korean-basic`）。

## License

MIT

## TODO

- [ ] 支持 Windows
- [ ] 支持多个播放模式
- [ ] 支持其他安装方式
- [ ] ...
