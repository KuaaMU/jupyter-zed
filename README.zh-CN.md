# Jupyter Notebook 扩展 - Zed 编辑器

[English](README.md) | 简体中文

这是一个为 [Zed 编辑器](https://zed.dev/) 提供 Jupyter Notebook (`.ipynb`) 支持的扩展。

## ✨ 功能特性

### 当前版本 (v0.1.0)
- ✅ **文件类型识别**: 自动识别 `.ipynb` 文件
- ✅ **语法高亮**: 基于 JSON 的语法高亮，包含 Jupyter 特定增强
- ✅ **基础编辑**: 括号匹配、自动缩进等基础编辑功能

### 计划中的功能
- 🔄 **阶段 2**: 增强的语法高亮（代码单元格、Markdown 单元格、输出内容）
- 🔄 **阶段 3**: Jupyter Language Server 集成
- 🔄 **阶段 3**: 内核连接和代码执行
- 🔄 **阶段 3**: 交互式调试支持

## 📦 安装

### 从 Zed 扩展市场安装（即将推出）
1. 打开 Zed 编辑器
2. 按 `Ctrl+Shift+P`（Windows/Linux）或 `Cmd+Shift+P`（Mac）
3. 搜索 "zed: extensions"
4. 搜索 "Jupyter Notebook"
5. 点击安装

### 开发版本安装
1. 克隆此仓库
2. 在 Zed 中打开扩展视图（`Ctrl+Shift+P` → "zed: extensions"）
3. 点击 "Install Dev Extension"
4. 选择 `jupyter-zed` 目录

## 🚀 使用方法

只需在 Zed 中打开任意 `.ipynb` 文件，扩展会自动提供语法高亮和基础编辑功能。

## 📚 开发路线图

### 阶段 1: 基础支持 ✅（当前版本）
- 文件类型识别
- 基于 JSON 的语法高亮
- 基础编辑器配置

### 阶段 2: 增强体验（计划中）
- 针对不同单元格类型的自定义语法高亮
- 代码折叠支持
- Notebook 大纲视图
- 更好的 cell 结构展示

### 阶段 3: 完整集成（未来）
- Jupyter Language Server 支持
- 内核连接和管理
- 单元格执行
- 交互式输出
- 调试功能

## 🔧 系统要求

- Zed 编辑器 v0.100.0 或更高版本
- （阶段 3）Python 3.8+ 和 Jupyter（用于内核执行）

## 🛠️ 技术实现

### 当前实现
- 使用 Tree-sitter JSON grammar 解析 `.ipynb` 文件
- 自定义语法高亮规则识别 Jupyter 特定字段
- Rust + WebAssembly 扩展架构

### 未来计划
```toml
# 阶段 3：Language Server 配置示例
[language_servers.jupyter-lsp]
name = "Jupyter Language Server"
languages = ["Jupyter Notebook"]
```

## 📖 快速开始

### 测试扩展

1. 安装扩展后，创建或打开一个 `.ipynb` 文件
2. 享受语法高亮和编辑功能
3. 查看 `test.ipynb` 查看示例

### 本地开发

```bash
# 克隆仓库
git clone https://github.com/KuaaMU/jupyter-zed.git
cd jupyter-zed

# 编译扩展
rustup target add wasm32-wasip1
cargo build --target wasm32-wasip1 --release

# 在 Zed 中安装为开发扩展
# Ctrl+Shift+P → "zed: extensions" → "Install Dev Extension"
```

## 🤝 贡献

欢迎贡献！请随时提交 issues 或 pull requests。

### 开发指南
- 查看 [DEVELOPMENT.md](DEVELOPMENT.md) 了解详细的开发指南
- 查看 [QUICKSTART.md](QUICKSTART.md) 快速开始开发

### 贡献流程
1. Fork 本仓库
2. 创建你的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交你的更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启一个 Pull Request

## 📝 常见问题

### Q: 为什么我的 .ipynb 文件没有语法高亮？
A: 请确保：
- 扩展已正确安装
- 文件扩展名是 `.ipynb`
- 重启 Zed 或重新加载窗口（`Ctrl+Shift+P` → "reload window"）

### Q: 支持哪些 Jupyter notebook 格式？
A: 目前支持 nbformat 4.x 格式（标准 Jupyter notebook 格式）

### Q: 什么时候会支持代码执行？
A: 代码执行功能计划在阶段 3 实现，需要集成 Jupyter Language Server 和内核管理

### Q: 可以在 Zed 中创建新的 notebook 吗？
A: 目前可以编辑现有的 `.ipynb` 文件。创建新 notebook 的模板功能正在考虑中。

## 📄 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 🙏 致谢

- 基于 [Zed Extension API](https://github.com/zed-industries/zed) 构建
- 使用 [Tree-sitter JSON](https://github.com/tree-sitter/tree-sitter-json) 进行解析
- 感谢 Zed 和 Jupyter 社区

## 🔗 相关链接

- [Zed 编辑器](https://zed.dev/)
- [Jupyter Project](https://jupyter.org/)
- [Tree-sitter](https://tree-sitter.github.io/)
- [扩展开发文档](https://zed.dev/docs/extensions)

## 📊 项目状态

| 功能 | 状态 |
|------|------|
| 文件类型识别 | ✅ 已完成 |
| 语法高亮 | ✅ 已完成 |
| 括号匹配 | ✅ 已完成 |
| 自动缩进 | ✅ 已完成 |
| Cell 类型区分 | 🔄 计划中 |
| 代码折叠 | 🔄 计划中 |
| LSP 集成 | 🔄 计划中 |
| 内核执行 | 🔄 计划中 |
| 调试支持 | 🔄 计划中 |

## 💬 反馈

如果您有任何问题、建议或反馈，请：
- 提交 [Issue](https://github.com/KuaaMU/jupyter-zed/issues)
- 发起 [Discussion](https://github.com/KuaaMU/jupyter-zed/discussions)
- 发送邮件至：XCM853629353@OUTLOOK.com

---

**Made with ❤️ for the Zed and Jupyter communities**
