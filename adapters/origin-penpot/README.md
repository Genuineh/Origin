# OriginPenpot

Penpot 设计工具适配器，将 Penpot 文件转换为 Origin 引擎可理解的通用格式。

## 概述

OriginPenpot 是 Origin 项目的 Penpot 适配器，负责解析 Penpot 设计文件并将其转换为 Origin 引擎的通用文档模型。

Penpot 是一款开源的设计和原型工具，专注于开放标准和 Web 平台。

## 功能特性（计划中）

### Penpot 特有功能

- 开源设计格式
- SVG 原生支持
- Flex Layout
- 组件系统
- 交互原型
- 协作功能

### 通用功能

- 矢量图形
- 文本和排版
- 图层样式
- 响应式设计
- 原型链接

## 使用示例

```rust
use origin_penpot::PenpotAdapter;
use origin_adapter_common::DesignAdapter;

// 创建 Penpot 适配器
let adapter = PenpotAdapter::new();

// 解析 Penpot 文件
let document = adapter.parse("design.penpot")?;

// 访问文档内容
println!("Document name: {}", document.metadata.name);
```

## 文件格式

Penpot 使用开放的 JSON 格式存储设计数据，基于 Web 标准。

## 架构

```
PenpotAdapter (实现 DesignAdapter)
    ↓
PenpotParser (JSON 解析)
    ↓
PenpotDocument (Penpot 特有模型)
    ↓
DesignDocument (通用模型)
    ↓
Origin 引擎核心
```

## 开发状态

📅 **计划中** - 预计在主要商业设计工具适配器完成后开始开发

## 依赖

```toml
[dependencies]
origin-adapter-common = { path = "../origin-adapter-common" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 参考

- [Penpot 官网](https://penpot.app/)
- [Penpot GitHub](https://github.com/penpot/penpot)
- [Penpot 文档](https://help.penpot.app/)

## 为什么支持 Penpot？

作为一个开源项目，我们相信支持开源设计工具是重要的。Penpot 代表了设计工具的未来方向：

- ✅ 开放标准
- ✅ 完全开源
- ✅ 无供应商锁定
- ✅ 社区驱动

## 贡献

特别欢迎 Penpot 社区的贡献者参与开发！

## 许可证

MIT
