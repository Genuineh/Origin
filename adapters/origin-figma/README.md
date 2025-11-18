# OriginFigma

Figma 设计工具适配器，将 Figma 文件转换为 Origin 引擎可理解的通用格式。

## 概述

OriginFigma 是 Origin 项目的 Figma 适配器，负责解析 Figma 设计文件并将其转换为 Origin 引擎的通用文档模型。

## 功能特性

### ✅ 支持的 Figma 功能

- **基础节点类型**
  - Document, Canvas, Frame
  - Group, Rectangle, Ellipse, Line
  - Polygon, Star, Vector
  - Text, Image

- **样式系统**
  - 实色填充
  - 渐变填充（线性、径向、角度、钻石）
  - 图片填充
  - 描边样式
  - 阴影和模糊效果
  - 混合模式

- **布局系统**
  - Auto Layout
  - 约束和锚点
  - 响应式尺寸

- **组件系统**
  - 组件定义和实例
  - 变体系统
  - 属性覆盖
  - 样式库

- **原型交互**
  - 原型连接
  - 页面导航
  - 交互触发器
  - 过渡动画

- **高级效果**
  - 布尔运算
  - 蒙版和裁剪
  - 混合模式
  - 高级视觉效果

## 使用示例

```rust
use origin_figma::FigmaAdapter;
use origin_adapter_common::DesignAdapter;

// 创建 Figma 适配器
let adapter = FigmaAdapter::new();

// 解析 Figma 文件
let document = adapter.parse("design.fig")?;

// 访问文档内容
println!("Document name: {}", document.metadata.name);
for node in document.root.children() {
    println!("  - {}", node.name());
}
```

## 文件格式支持

- ✅ Figma JSON 格式
- ✅ Figma REST API 响应格式
- 🚧 .fig 本地文件（计划中）

## 架构

```
FigmaAdapter (实现 DesignAdapter)
    ↓
FigmaParser (JSON 解析)
    ↓
FigmaDocument (Figma 特有模型)
    ↓
DesignDocument (通用模型)
    ↓
Origin 引擎核心
```

## 开发状态

### Week 11-12: 核心功能
- 🚧 基础解析器
- 🚧 基础节点类型
- 🚧 样式系统
- 🚧 文本和图片支持

### Week 13-14: 组件系统
- 📅 组件和实例
- 📅 变体系统
- 📅 样式库

### Week 15-16: 原型和高级功能
- 📅 原型交互
- 📅 高级效果
- 📅 混合模式

## 依赖

```toml
[dependencies]
origin-adapter-common = { path = "../origin-adapter-common" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 测试

```bash
# 运行测试
cargo test --package origin-figma

# 运行示例
cargo run --package origin-figma --example parse_figma
```

## 文档

- [Figma API 文档](https://www.figma.com/developers/api)
- [Figma 文件格式](https://www.figma.com/developers/api#files)

## 贡献

欢迎贡献！请参考 [贡献指南](../../CONTRIBUTING.md)。

## 许可证

MIT
