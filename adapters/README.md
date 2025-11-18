# Origin 设计工具适配器

本目录包含 Origin 引擎支持的各种设计工具适配器。

## 架构概述

Origin 采用"引擎层 + 适配器层"的分离架构：

- **引擎层** (`crates/`) - 通用的渲染引擎，与具体设计工具无关
- **适配器层** (`adapters/`) - 针对各设计工具的文件格式解析和转换

## 可用适配器

| 适配器 | 设计工具 | 状态 | 优先级 | 文档 |
|--------|---------|------|--------|------|
| [origin-adapter-common](origin-adapter-common/) | 通用接口 | 🚧 开发中 | P0 | [README](origin-adapter-common/README.md) |
| [origin-figma](origin-figma/) | Figma | 🚧 开发中 | P0 | [README](origin-figma/README.md) |
| [origin-pixso](origin-pixso/) | Pixso | 📅 计划中 | P1 | [README](origin-pixso/README.md) |
| [origin-sketch](origin-sketch/) | Sketch | 📅 计划中 | P2 | [README](origin-sketch/README.md) |
| [origin-penpot](origin-penpot/) | Penpot | 📅 计划中 | P2 | [README](origin-penpot/README.md) |

### 状态说明

- ✅ **已完成** - 功能完整，可用于生产
- 🚧 **开发中** - 正在积极开发
- 📅 **计划中** - 已规划，等待开始
- 🔬 **实验性** - 实验性功能，不稳定

### 优先级说明

- **P0** - 核心功能，必须在 0.1.0 完成
- **P1** - 高优先级，计划在 0.2.0 完成
- **P2** - 中优先级，根据需求排期
- **P3** - 低优先级，社区驱动

## 统一接口

所有适配器都实现 `DesignAdapter` trait：

```rust
use origin_adapter_common::{DesignAdapter, DesignDocument};

pub trait DesignAdapter {
    /// 解析设计文件
    fn parse(&self, path: &Path) -> Result<DesignDocument>;
    
    /// 获取适配器元数据
    fn get_metadata(&self) -> AdapterMetadata;
    
    /// 支持的文件版本
    fn supported_versions(&self) -> Vec<String>;
}
```

## 使用示例

### 使用 Figma 适配器

```rust
use origin_figma::FigmaAdapter;
use origin_adapter_common::DesignAdapter;

let adapter = FigmaAdapter::new();
let document = adapter.parse("design.fig")?;

// 文档现在是通用的 DesignDocument 格式
// 可以传递给 Origin 引擎渲染
```

### 使用多个适配器

```rust
use origin_adapter_common::{DesignAdapter, DesignDocument};
use origin_figma::FigmaAdapter;
use origin_pixso::PixsoAdapter;

fn load_design(path: &Path) -> Result<DesignDocument> {
    let adapter: Box<dyn DesignAdapter> = match path.extension() {
        Some("fig") => Box::new(FigmaAdapter::new()),
        Some("pixso") => Box::new(PixsoAdapter::new()),
        _ => return Err("Unsupported file format"),
    };
    
    adapter.parse(path)
}
```

## 开发新适配器

### 1. 创建新的 crate

```bash
cd adapters
cargo new origin-yourtool
```

### 2. 添加依赖

```toml
[dependencies]
origin-adapter-common = { path = "../origin-adapter-common" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 3. 实现 DesignAdapter trait

```rust
use origin_adapter_common::{DesignAdapter, DesignDocument, AdapterMetadata};
use std::path::Path;

pub struct YourToolAdapter;

impl DesignAdapter for YourToolAdapter {
    fn parse(&self, path: &Path) -> Result<DesignDocument> {
        // 1. 读取文件
        // 2. 解析为工具特有的格式
        // 3. 转换为 DesignDocument
        todo!()
    }
    
    fn get_metadata(&self) -> AdapterMetadata {
        AdapterMetadata {
            name: "YourTool",
            version: env!("CARGO_PKG_VERSION"),
            author: "Your Name",
            description: "YourTool adapter for Origin",
        }
    }
    
    fn supported_versions(&self) -> Vec<String> {
        vec!["1.0".to_string(), "2.0".to_string()]
    }
}
```

### 4. 编写测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_basic_file() {
        let adapter = YourToolAdapter;
        let doc = adapter.parse("tests/fixtures/basic.yourtool").unwrap();
        assert_eq!(doc.metadata.name, "Basic Design");
    }
}
```

### 5. 更新文档

- 在 `adapters/origin-yourtool/README.md` 中添加文档
- 更新主 README.md 中的支持列表
- 添加示例程序

## 转换流程

```
设计文件 (.fig, .pixso, .sketch, etc.)
    ↓
特定适配器 (FigmaAdapter, PixsoAdapter, etc.)
    ↓ parse()
工具特有模型 (FigmaDocument, PixsoDocument, etc.)
    ↓ convert()
通用文档模型 (DesignDocument)
    ↓
Origin 引擎核心
    ↓
渲染输出 (120fps Native App)
```

## 设计原则

### 1. 职责分离
- **适配器层**：只负责文件解析和格式转换
- **引擎层**：只负责渲染和交互

### 2. 统一抽象
- 所有适配器输出相同的 `DesignDocument` 格式
- 引擎层无需知道原始设计工具

### 3. 可扩展性
- 添加新设计工具支持无需修改引擎核心
- 适配器可独立开发、测试、发布

### 4. 性能优化
- 延迟加载：只加载需要的适配器
- 增量解析：支持大文件的增量解析
- 缓存：解析结果可缓存

## 测试策略

### 单元测试
- 测试每个适配器的解析功能
- 测试格式转换的正确性

### 集成测试
- 测试适配器 + 引擎的完整流程
- 使用真实设计文件

### 兼容性测试
- 测试不同版本的设计文件
- 跨平台兼容性测试

## 性能基准

每个适配器应该包含性能基准测试：

```bash
cargo bench --package origin-figma
```

目标性能：
- 小文件（< 1MB）：< 10ms
- 中文件（1-10MB）：< 100ms
- 大文件（> 10MB）：< 1s

## 贡献指南

欢迎贡献新的设计工具适配器！详见 [贡献指南](../CONTRIBUTING.md)。

特别欢迎以下适配器：
- Adobe XD
- Framer
- ProtoPie
- Axure RP
- InVision
- MasterGo
- CoDesign
- 即时设计

## 许可证

所有适配器均采用 MIT 许可证。

## 相关资源

- [架构文档](../ARCHITECTURE.md)
- [开发路线图](../ROADMAP.md)
- [API 文档](https://docs.rs/origin-adapter-common)
