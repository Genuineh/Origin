# OriginSketch

Sketch 设计工具适配器，将 Sketch 文件转换为 Origin 引擎可理解的通用格式。

## 概述

OriginSketch 是 Origin 项目的 Sketch 适配器，负责解析 Sketch 设计文件（.sketch 格式）并将其转换为 Origin 引擎的通用文档模型。

Sketch 是 macOS 上流行的矢量图形设计工具。

## 功能特性（计划中）

### Sketch 特有功能

- Symbol 系统
- Shared Styles
- Libraries
- Plugins 支持（可选）
- Sketch Cloud 集成（可选）

### 通用功能

- 矢量图形
- 文本和排版
- 图层样式
- 布尔运算
- 导出设置

## 使用示例

```rust
use origin_sketch::SketchAdapter;
use origin_adapter_common::DesignAdapter;

// 创建 Sketch 适配器
let adapter = SketchAdapter::new();

// 解析 Sketch 文件
let document = adapter.parse("design.sketch")?;

// 访问文档内容
println!("Document name: {}", document.metadata.name);
```

## 文件格式

Sketch 文件（.sketch）实际上是一个 ZIP 压缩包，包含：

- `document.json` - 文档元数据
- `pages/*.json` - 页面数据
- `meta.json` - 元信息
- `user.json` - 用户设置
- `images/*` - 嵌入的图片

## 架构

```
SketchAdapter (实现 DesignAdapter)
    ↓
SketchParser (ZIP + JSON 解析)
    ↓
SketchDocument (Sketch 特有模型)
    ↓
DesignDocument (通用模型)
    ↓
Origin 引擎核心
```

## 开发状态

📅 **计划中** - 预计在 OriginFigma 和 OriginPixso 完成后开始开发

## 依赖

```toml
[dependencies]
origin-adapter-common = { path = "../origin-adapter-common" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
zip = "0.6"
```

## 参考

- [Sketch 官网](https://www.sketch.com/)
- [Sketch 文件格式规范](https://developer.sketch.com/file-format/)

## 贡献

欢迎贡献！如果您熟悉 Sketch 文件格式，欢迎参与开发。

## 许可证

MIT
