# LightCraft 简体中文界面

LightCraft 支持简体中文、日语和英语界面。
语言设置保存在 `ui.json` 的 `language` 字段中。

- 通过"编辑 → 语言"或"设置 → 通用 → 语言"在简体中文、日本語和 English 之间切换，选择会在下次启动时保留。
- 已翻译：菜单、照片编辑、蒙版、裁剪、设置、导入、导出和主要进度提示。
- 中文界面使用 SIL OFL 许可的 Noto Sans CJK SC。该字体不在本仓库，而在
  [storytold/craft-fonts](https://github.com/storytold/craft-fonts) 中；以可选构建输入
  `CRAFT_FONTS_DIR=../craft-fonts` 构建（所有正式发布都如此）时会嵌入。不指定也能构建、
  测试和运行，但中文字符没有字形（网页版构建只嵌入日语界面字体，中文需用带
  `CRAFT_FONTS_DIR` 的本地构建）。
- 翻译只作用于显示层：操作命令的 ID、照片文件名、你输入的元数据不会被改动。
- 未翻译的技术性错误、附加信息和发行说明仍以英语显示。

## 维护翻译

静态文案在 `crates/ui-egui/locales/zh.json`，含变量的文案在
`crates/ui-egui/locales/zh-formats.json`，均以英文原文为键管理。
含变量文案的每种语言在构建时都接受 Rust 格式检查。
省略英文复数词尾时，把对应的字符串参数用 `{:.0}` 置空。
`LIGHTCRAFT_LANGUAGE=zh lightcraft-cli snapshot ...` 可以渲染中文界面截图。

显示、字体、语言切换、设置保存与命令 ID 保持不变由
`cargo test -p lightcraft-ui-egui i18n::tests` 验证（字形检查仅在指定 `CRAFT_FONTS_DIR`
构建时执行）。
