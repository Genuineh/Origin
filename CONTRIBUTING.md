# 贡献指南

感谢您对 Origin 项目的关注！我们欢迎各种形式的贡献。

## 🤝 如何贡献

### 报告 Bug

如果您发现了 bug，请在 [GitHub Issues](https://github.com/Genuineh/Origin/issues) 中创建一个新的 issue，并包含以下信息：

- Bug 的详细描述
- 复现步骤
- 预期行为
- 实际行为
- 环境信息（操作系统、Rust 版本等）
- 相关的日志或截图

### 提出功能建议

我们欢迎新的功能建议！请在 [GitHub Issues](https://github.com/Genuineh/Origin/issues) 中创建一个新的 issue，标记为 "enhancement"，并描述：

- 功能的详细说明
- 使用场景
- 为什么这个功能有用
- 可能的实现方案（可选）

### 提交代码

1. **Fork 项目**
   ```bash
   # 在 GitHub 上 fork 项目
   git clone https://github.com/your-username/Origin.git
   cd Origin
   ```

2. **创建分支**
   ```bash
   git checkout -b feature/your-feature-name
   # 或
   git checkout -b fix/your-bug-fix
   ```

3. **编写代码**
   - 遵循项目的代码风格
   - 添加必要的测试
   - 更新相关文档

4. **运行测试**
   ```bash
   cargo test
   cargo fmt --check
   cargo clippy -- -D warnings
   ```

5. **提交更改**
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   # 或
   git commit -m "fix: fix your bug description"
   ```

6. **推送到 GitHub**
   ```bash
   git push origin feature/your-feature-name
   ```

7. **创建 Pull Request**
   - 在 GitHub 上创建 Pull Request
   - 描述您的更改
   - 关联相关的 issue

### 添加新的设计工具适配器

如果您想为新的设计工具添加适配器：

1. 在 `adapters/` 目录下创建新的 crate
2. 实现 `origin-adapter-common` 中定义的 `DesignAdapter` trait
3. 添加必要的测试和文档
4. 更新主 README.md 中的支持列表

示例结构：
```
adapters/
  origin-your-tool/
    src/
      lib.rs
      parser.rs
      adapter.rs
    tests/
    README.md
    Cargo.toml
```

## 📝 代码风格

### Rust 代码风格

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 遵循 [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### 提交信息格式

使用 [Conventional Commits](https://www.conventionalcommits.org/) 格式：

- `feat:` - 新功能
- `fix:` - Bug 修复
- `docs:` - 文档更改
- `style:` - 代码格式（不影响代码运行）
- `refactor:` - 重构
- `perf:` - 性能优化
- `test:` - 添加测试
- `chore:` - 构建过程或辅助工具的变动

示例：
```
feat: add Pixso adapter support
fix: resolve SDF rendering artifacts
docs: update architecture documentation
```

## 🧪 测试

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定模块的测试
cargo test --package origin-gpu

# 运行性能测试
cargo bench
```

### 编写测试

- 每个新功能都应该有对应的测试
- 测试应该覆盖正常情况和边界情况
- 使用有意义的测试名称

## 📚 文档

### 代码文档

- 所有公共 API 必须有文档注释
- 使用 `///` 为函数和结构体添加文档
- 包含示例代码

示例：
```rust
/// Parses a Figma design file and returns a DesignDocument.
///
/// # Arguments
///
/// * `path` - Path to the Figma file
///
/// # Examples
///
/// ```
/// use origin_figma::FigmaAdapter;
/// 
/// let adapter = FigmaAdapter::new();
/// let document = adapter.parse("design.fig")?;
/// ```
pub fn parse(&self, path: &str) -> Result<DesignDocument> {
    // ...
}
```

### 更新文档

如果您的更改影响了以下文档，请同时更新：

- README.md
- ARCHITECTURE.md
- ROADMAP.md
- TODO.md
- 模块的 README.md

## 🔍 代码审查

所有的 Pull Request 都需要经过代码审查。审查者会关注：

- 代码质量和风格
- 测试覆盖率
- 文档完整性
- 性能影响
- 架构一致性

## 💬 交流

- GitHub Issues - 报告 bug 和功能请求
- GitHub Discussions - 一般讨论和问题
- Pull Requests - 代码审查和讨论

## 📋 开发环境设置

### 系统要求

- Rust 1.70+ (推荐使用 rustup)
- Git
- 支持的操作系统：Windows/macOS/Linux

### 依赖安装

```bash
# 安装 Rust (如果尚未安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装开发工具
cargo install cargo-watch
cargo install cargo-edit
```

### 构建项目

```bash
# 克隆项目
git clone https://github.com/Genuineh/Origin.git
cd Origin

# 构建
cargo build

# 运行测试
cargo test

# 开发模式（自动重新编译）
cargo watch -x check -x test
```

## 🎯 当前优先级

查看 [TODO.md](TODO.md) 了解当前的任务优先级和进度。

高优先级任务：
1. 完成 Layer 0-1 (Platform & GPU 抽象层)
2. 实现 SDF 渲染核心
3. 完成 OriginFigma 适配器基础功能

## ⚖️ 许可证

通过向本项目提交代码，您同意您的贡献将在 MIT 许可证下发布。

---

再次感谢您的贡献！如有任何问题，请随时在 Issues 或 Discussions 中提问。
