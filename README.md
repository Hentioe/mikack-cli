# manga-cli

一个高质量重构的 manga 项目分支，专注于为 CLI 提供功能。

## 说明

基于跨平台的 [manga-rs](https://github.com/Hentioe/manga-rs) 库，支持大量的在线漫画平台（详情请参考 manga-rs 项目说明）。

本工具是 manga-rs 周边项目的一员，它还包括有基于同一个核心库的桌面版和手机版。

## 使用方式

- 命令行帮助：

  ```
  manga-cli 0.1.0-dev
  Hentioe (绅士喵), <me@bluerain.io>
  A tool for exporting online comics

  USAGE:
      manga-cli [OPTIONS] [url]

  FLAGS:
      -h, --help       Prints help information
      -V, --version    Prints version information

  OPTIONS:
      -f, --format <save-format>    Saved format (eg: epub)

  ARGS:
      <url>    The address of the comic home page or reading page
  ```

- 基本使用：

  - 直接粘贴 URL：

    `manga-cli https://www.dm5.com/m136026/`

    支持漫画主页或具体的阅读页面，前者展示章节（话）列表，后者直接进行下载。

  - 指定保存格式：

    `manga-cli -f epub https://www.dm5.com/m136026/`

    若不指定，则仅仅将图片下载到目录。

无任何参数启动会进入交互模式，选择平台和漫画章节进行下载。章节支持多选。

## 格式说明

本项目当前主要支持 epub 格式，这是一种通用的现代电子图书格式。本工具生成的 epub 文件使用 Flexbox 布局，在任何标准的规范的阅读器上都能获得良好的阅读体验。

**推荐阅读器**：

- Calibre（Windows/Linux/Mac）
- Lithium（Android）

在旧版 manga 项目中支持了更多的格式，例如 mobi/azw3/zip。因为某些原因，暂且没有支持这些格式。顾及到 Kindle 用户，mobi/azw3 格式将会支持，而 zip 一类的压缩格式请自行使用压缩软件。
