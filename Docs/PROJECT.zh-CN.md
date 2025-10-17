# 项目总览 - Jupyter Zed 扩展

## 📂 文件清单

### 核心配置文件
- ✅ `extension.toml` - Zed 扩展主配置
- ✅ `Cargo.toml` - Rust 项目配置
- ✅ `LICENSE` - MIT 许可证

### 源代码
- ✅ `src/lib.rs` - Rust 扩展实现（包含未来 Phase 3 接口）

### 语言定义
- ✅ `languages/jupyter/config.toml` - Jupyter Notebook 语言配置
- ✅ `languages/jupyter/highlights.scm` - 语法高亮规则
- ✅ `languages/jupyter/brackets.scm` - 括号匹配规则
- ✅ `languages/jupyter/indents.scm` - 缩进规则

### 文档
- ✅ `README.md` - 英文项目说明
- ✅ `README.zh-CN.md` - 中文项目说明
- ✅ `QUICKSTART.md` - 英文快速开始
- ✅ `QUICKSTART.zh-CN.md` - 中文快速开始
- ✅ `DEVELOPMENT.md` - 开发指南
- ✅ `PROJECT.zh-CN.md` - 本文件（项目总览）

### 测试文件
- ✅ `test.ipynb` - Jupyter Notebook 测试文件

## 🎯 当前状态

### Phase 1: 基础支持 ✅ **已完成**

| 功能 | 状态 | 文件位置 |
|------|------|---------|
| 文件类型识别 | ✅ | `languages/jupyter/config.toml` |
| JSON 语法高亮 | ✅ | `languages/jupyter/highlights.scm` |
| Jupyter 特定高亮 | ✅ | `languages/jupyter/highlights.scm` |
| 括号匹配 | ✅ | `languages/jupyter/brackets.scm` |
| 自动缩进 | ✅ | `languages/jupyter/indents.scm` |
| Rust 框架代码 | ✅ | `src/lib.rs` |

## 🔄 开发路线图

### Phase 2: 增强体验（下一步）

**预计时间：** 1-2 周

**需要添加的文件：**
- `languages/jupyter/outline.scm` - Cell 结构大纲
- `languages/jupyter/injections.scm` - 语法注入（Python/Markdown）

**需要修改的文件：**
- `languages/jupyter/highlights.scm` - 增强 cell 类型高亮
- `languages/jupyter/config.toml` - 添加更多配置

**功能列表：**
- [ ] 区分不同 cell 类型的高亮（code/markdown/raw）
- [ ] Python 语法注入到 code cells
- [ ] Markdown 语法注入到 markdown cells
- [ ] Cell 结构的 outline 视图
- [ ] 代码折叠支持

### Phase 3: 完整集成（未来）

**预计时间：** 4-8 周

**需要添加的依赖：**
```toml
# 在 Cargo.toml 中添加
tokio = { version = "1.0", features = ["full"] }
jupyter-protocol = "0.1"
# 或其他 Jupyter 通信库
```

**需要修改的文件：**
- `extension.toml` - 添加 language server 配置
- `src/lib.rs` - 实现完整的扩展逻辑

**功能列表：**
- [ ] Jupyter Language Server 集成
- [ ] 内核连接和管理
- [ ] Cell 执行功能
- [ ] 结果显示
- [ ] 调试支持

## 🛠️ 技术栈

### 当前使用
- **语言：** Rust
- **编译目标：** wasm32-wasip1
- **解析器：** Tree-sitter (JSON grammar)
- **框架：** Zed Extension API v0.2.0

### 未来计划
- **LSP：** Jupyter Language Server
- **协议：** Jupyter Messaging Protocol
- **通信：** WebSocket / ZMQ
- **异步：** Tokio

## 📊 代码统计

```bash
# 当前代码行数（不含文档和测试）
extension.toml:       30 行
Cargo.toml:           12 行
src/lib.rs:           24 行
languages/jupyter/*:  ~50 行
---
总计:                ~116 行
```

## 🔍 关键实现细节

### 1. 文件类型识别
**位置：** `languages/jupyter/config.toml:3`
```toml
path_suffixes = ["ipynb"]
```
所有 `.ipynb` 文件自动识别为 Jupyter Notebook。

### 2. 语法高亮
**位置：** `languages/jupyter/highlights.scm`

**实现方式：**
- 基于 JSON Tree-sitter grammar
- 自定义 Jupyter 特定字段的高亮
- 使用 `#match?` 和 `#eq?` 进行模式匹配

**关键模式：**
```scheme
; 高亮 cell_type
(pair
  key: (string (string_content) @keyword)
  (#match? @keyword "^cell_type$"))

; 高亮 execution_count
(pair
  key: (string (string_content) @keyword)
  (#eq? @keyword "execution_count"))
```

### 3. Rust 扩展框架
**位置：** `src/lib.rs:1-24`

**关键点：**
- 实现 `zed::Extension` trait
- 预留 `language_server_command` 方法
- 使用 `register_extension!` 宏注册

**未来扩展点：**
```rust
// Phase 3 需要实现的方法
- connect_to_kernel()
- start_kernel()
- execute_cell()
- get_completions()
```

## 🧪 测试

### 本地测试
```bash
# 1. 安装为开发扩展
Ctrl+Shift+P → "zed: extensions" → "Install Dev Extension"

# 2. 打开测试文件
F:\jupyter-zed\test.ipynb

# 3. 验证功能
- 文件类型显示为 "Jupyter Notebook"
- JSON 结构有语法高亮
- cell_type, execution_count 等字段特殊高亮
```

### 编译测试
```bash
cd F:\jupyter-zed
cargo build --target wasm32-wasip1 --release
```

### 发布前检查
- [ ] 所有文件已提交到 Git
- [ ] LICENSE 文件存在且为 MIT
- [ ] README 文档完整
- [ ] extension.toml 版本号正确
- [ ] 编译成功无错误
- [ ] 在 Zed 中测试通过

## 📝 Git 工作流

### 初始化
```bash
cd F:\jupyter-zed
git init
git add .
git commit -m "Initial commit: Jupyter Notebook extension"
```

### 推送到 GitHub
```bash
git remote add origin https://github.com/KuaaMU/jupyter-zed.git
git branch -M main
git push -u origin main
```

### 日常开发
```bash
# 创建功能分支
git checkout -b feature/phase2-enhancements

# 开发和提交
git add .
git commit -m "Add cell type highlighting"

# 推送分支
git push origin feature/phase2-enhancements

# 合并到 main
git checkout main
git merge feature/phase2-enhancements
git push origin main
```

## 🚀 发布流程

### 1. 准备发布
- [ ] 更新版本号（extension.toml）
- [ ] 更新 CHANGELOG
- [ ] 测试所有功能
- [ ] 更新文档

### 2. 提交到 Zed Extensions
```bash
# Fork https://github.com/zed-industries/extensions
# 添加 submodule
git submodule add https://github.com/KuaaMU/jupyter-zed.git extensions/jupyter

# 更新 extensions.toml
# 提交 PR
```

### 3. 版本发布
```bash
# 打标签
git tag -a v0.1.0 -m "Release version 0.1.0"
git push origin v0.1.0

# 创建 GitHub Release
# 上传编译好的 .wasm 文件
```

## 🔗 相关资源

### 文档
- [Zed Extension API](https://docs.rs/zed_extension_api)
- [Tree-sitter](https://tree-sitter.github.io)
- [Jupyter Protocol](https://jupyter-client.readthedocs.io)

### 示例扩展
- [Python Extension](https://github.com/zed-extensions/python)
- [Rust Extension](https://github.com/zed-extensions/rust)

### 社区
- [Zed Discord](https://discord.gg/zed)
- [Zed GitHub Discussions](https://github.com/zed-industries/zed/discussions)

## 📧 联系方式

- **作者：** KuaaMU
- **邮箱：** XCM853629353@OUTLOOK.com
- **仓库：** https://github.com/KuaaMU/jupyter-zed

## 🎓 学习资源

如果您想深入了解扩展开发：

1. **Tree-sitter 查询语法**
   - 阅读官方文档
   - 查看现有语言的 `.scm` 文件
   - 使用 Tree-sitter playground 测试

2. **Zed Extension API**
   - 阅读 API 文档
   - 查看官方示例扩展
   - 参考其他社区扩展

3. **Jupyter 协议**
   - 了解 Jupyter Messaging Protocol
   - 研究 Jupyter LSP
   - 查看 jupyter-client 源码

## 🏁 下一步行动

### 立即可做
1. ✅ 在 Zed 中测试扩展
2. ✅ 提交代码到 GitHub
3. ✅ 准备发布到 Zed 扩展市场

### Phase 2 准备
1. 研究 Tree-sitter injections
2. 学习 outline queries
3. 设计 cell 类型区分方案

### Phase 3 规划
1. 调研 Jupyter LSP 方案
2. 设计内核管理架构
3. 规划 UI 交互方式

---

**项目状态：** 🟢 Phase 1 完成，可以发布
**最后更新：** 2025-10-17
