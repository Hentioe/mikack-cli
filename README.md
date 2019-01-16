# MANGA

Smaller, faster and less dependent [manga](https://github.com/Hentioe/manga).

## 说明

此程序是对 [manga](https://github.com/Hentioe/manga) 项目的重新实现，目前正处于开发阶段，特别是对上游平台的支持很不全面。

### 使用演示

![演示动画](https://raw.githubusercontent.com/Hentioe/manga-rs/master/.github/manga.gif)

### 目的

本项目的初衷是为了方便我个人将在线的漫画资源导入阅读器设备，但也有扩展到更大的想法（类似 youtube-dl 不仅仅只是下载 YouTube 视频），我想支持的平台没有上限。  
使用 Rust 重新实现是为了减少体积、依赖项以及兼容到更多的平台上，如手机（Android）甚至是路由器（OpenWrt）。

### 当前状态

目前已经过了最基础的开发阶段，就像演示 GIF 中的那样一个完整的基本功能是通顺的。但仍然处在初级阶段，主要表现在上游平台的支持不全面以及缺乏 CLI 交互能力。

## TODO(1.0)

- [x] 基于交互式终端模式
  - [ ] 选择平台 -> 选择漫画 -> 保存
  - [ ] 漫画索引支持查看更多
  - [x] 漫画保存列表支持多选
- [ ] 更多的导出格式支持
  - [ ] 基于 epub 转换的 mobi
  - [ ] 基于 epub 转换的 azw3
  - [ ] 基于 epub 转换的 pdf
  - [ ] 基于参数或终端交互定义输出格式
- [ ] 更多的资源来源支持
  - [ ] www.dm5.com (动漫屋)
  - [ ] www.cartoonmad.com (动漫狂)
  - [ ] manhua.fzdm.com (风之动漫)
  - [ ] www.gufengmh.com (古风漫画网)
  - [x] www.hhmmoo.com (汗汗漫画)
  - [ ] comic.kukudm.com (KuKu动漫)
  - [ ] www.manhuagui.com (动漫柜)
  - [ ] www.manhuaren.com (漫画人)
  - [ ] www.manhuatai.com (漫画台)
  - [ ] www.verydm.com (非常爱漫)
  - [ ] www.177mh.net (新新漫画网)
- [ ] 附加功能
  - [ ] 清理缓存资源
  - [ ] 清空原始资源
  - [ ] 指定输出目录
