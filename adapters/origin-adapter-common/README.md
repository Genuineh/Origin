# origin-adapter-common

通用设计适配器接口和工具库。

## 概述

`origin-adapter-common` 提供了所有设计工具适配器必须实现的统一接口，以及共享的工具函数和数据结构。

## 核心组件

### DesignAdapter Trait

所有设计工具适配器必须实现此 trait：

```rust
pub trait DesignAdapter {
    fn parse(&self, path: &Path) -> Result<DesignDocument>;
    fn get_metadata(&self) -> AdapterMetadata;
    fn supported_versions(&self) -> Vec<String>;
}
```

### DesignDocument

通用设计文档模型，独立于具体设计工具：

```rust
pub struct DesignDocument {
    pub metadata: DocumentMetadata,
    pub root: Box<dyn DesignNode>,
    pub styles: StyleLibrary,
    pub components: ComponentLibrary,
}
```

### DesignNode Trait

所有节点类型的统一抽象：

```rust
pub trait DesignNode {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn children(&self) -> &[Box<dyn DesignNode>];
    fn bounds(&self) -> Rect;
    fn transform(&self) -> Transform;
    fn styles(&self) -> &NodeStyles;
}
```

## 使用示例

### 实现自定义适配器

```rust
use origin_adapter_common::{DesignAdapter, DesignDocument, AdapterMetadata};

struct MyAdapter;

impl DesignAdapter for MyAdapter {
    fn parse(&self, path: &Path) -> Result<DesignDocument> {
        // 解析你的设计文件格式
        todo!()
    }
    
    fn get_metadata(&self) -> AdapterMetadata {
        AdapterMetadata {
            name: "MyTool",
            version: "1.0.0",
            author: "You",
        }
    }
    
    fn supported_versions(&self) -> Vec<String> {
        vec!["1.0".to_string()]
    }
}
```

## 功能

- ✅ 统一的适配器接口
- ✅ 通用文档模型
- ✅ 样式系统抽象
- ✅ 组件系统抽象
- ✅ 工具函数库
- ✅ 错误处理

## 状态

🚧 **开发中** - 计划在 Week 11-12 实现

## 相关模块

- [origin-figma](../origin-figma) - Figma 适配器实现
- [origin-pixso](../origin-pixso) - Pixso 适配器实现
- [origin-sketch](../origin-sketch) - Sketch 适配器实现
- [origin-penpot](../origin-penpot) - Penpot 适配器实现
