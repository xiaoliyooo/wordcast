# wordcast

macOS 双语词汇朗读 CLI。从 JSON 词表逐对朗读中文 key 与外语 value，回到上学晨读的时光，自动按字符块选音色（韩 / 日 / 英 / 兜底）。

二进制：`caw`，包名：`wordcast`。

## Install

```bash
git clone https://github.com/<you>/wordcast
cd wordcast
cargo install --path .
```

## Usage

```bash
caw list                                # 列出所有词表
caw play korean-basic                   # 播放词表
caw play korean-basic --rate 220 --cycle  # 加速 + 循环
caw play --help                         # 全部参数
```

CLI 参数永远覆盖配置文件。

## 配置文件

首次运行会在 `~/.config/wordcast/` 自动生成：

```
~/.config/wordcast/
├── config.toml          # 播放参数 + 音色（每行含注释，直接编辑）
└── sources/
    └── korean-basic.json
```

打开 `config.toml` 即可看到每个字段的说明与可选值。

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

查看系统可用音色：`say -v '?'`。

## License

MIT
