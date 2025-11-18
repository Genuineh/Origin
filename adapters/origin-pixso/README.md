# OriginPixso

Pixso 设计工具适配器，将 Pixso 文件转换为 Origin 引擎可理解的通用格式。

## 概述

OriginPixso 是 Origin 项目的 Pixso 适配器，负责解析 Pixso 设计文件并将其转换为 Origin 引擎的通用文档模型。

Pixso 是一款国产的协作设计工具，兼容 Figma 文件格式，同时也有自己的特色功能。

## 功能特性（计划中）

### Pixso 特有功能

- 智能组件
- 协作批注
- 中文字体优化
- 资源管理
- AI 设计助手集成

### 通用功能

- 所有 Figma 兼容功能
- 节点类型支持
- 样式系统
- 组件系统
- 原型交互

## 使用示例

```rust
use origin_pixso::PixsoAdapter;
use origin_adapter_common::DesignAdapter;

// 创建 Pixso 适配器
let adapter = PixsoAdapter::new();

// 解析 Pixso 文件
let document = adapter.parse("design.pixso")?;

// 访问文档内容
println!("Document name: {}", document.metadata.name);
```

## 架构

```
PixsoAdapter (实现 DesignAdapter)
    ↓
PixsoParser (文件解析)
    ↓
PixsoDocument (Pixso 特有模型)
    ↓
DesignDocument (通用模型)
    ↓
Origin 引擎核心
```

## 开发状态

📅 **计划中** - 预计在 OriginFigma 完成后开始开发

## 依赖

```toml
[dependencies]
origin-adapter-common = { path = "../origin-adapter-common" }
origin-figma = { path = "../origin-figma" }  # 复用部分解析逻辑
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 参考

- [Pixso 官网](https://pixso.cn/)
- [Pixso 设计规范](https://pixso.cn/designskills/)

## 贡献

欢迎贡献！如果您熟悉 Pixso 文件格式，欢迎参与开发。

## 许可证

MIT
