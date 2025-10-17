# Zed 编辑器的 Jupyter Notebook 扩展

[English](README.md) | 简体中文

一个为 [Zed 编辑器](https://zed.dev/)设计的 **Jupyter Notebook JSON 编辑器**。此扩展为 `.ipynb` 文件提供了增强的 JSON 文档编辑功能。

> **说明**：此扩展定位为笔记本文件的 JSON 编辑器。完整的笔记本功能（单元格渲染、代码执行、Markdown 预览）需要 Zed 支持类似 VS Code 的 Notebook API，相关进展请关注 [zed#17325](https://github.com/zed-industries/zed/issues/17325)。

## 当前功能 (v0.1.1)

- ✅ **文件类型识别**：自动检测 `.ipynb` 文件
- ✅ **增强的 JSON 语法高亮**：
  - Jupyter 特定关键字（`cell_type`、`execution_count`、`metadata`）
  - 单元格类型值（`code`、`markdown`、`raw`）
  - 输出类型（`stream`、`display_data`、`execute_result`）
- ✅ **智能编辑**：
  - 括号匹配和自动补全（`{}`、`[]`、`""`）
  - 嵌套结构的智能缩进
  - JSON 结构验证
- ✅ **Notebook 结构支持**：
  - 解析和验证笔记本格式（nbformat v4）
  - 单元格类型识别
  - 元数据提取
- ✅ **轻量级**：无需额外依赖或下载

## 此扩展不提供的功能

由于 Zed 扩展 API 的当前限制：

- ❌ **单元格视图** - 笔记本以 JSON 格式显示，而非可视化单元格
- ❌ **代码执行** - 无法运行 Python/R 代码或连接到内核
- ❌ **Markdown 渲染** - Markdown 单元格以 JSON 字符串形式显示
- ❌ **输出显示** - 单元格输出以 JSON 形式显示，不渲染
- ❌ **交互功能** - 无变量查看器、调试或小部件

若需要这些功能，请使用 [VS Code](https://code.visualstudio.com/) 的 Jupyter 扩展或 [JupyterLab](https://jupyter.org/)。

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

## 🚀 使用说明

此扩展专为**编辑笔记本结构和元数据**设计，而非交互式数据科学工作流。

### 适用场景

- ✅ 手动编辑笔记本元数据
- ✅ 检查笔记本文件结构
- ✅ 使用 jq 等工具批量处理笔记本（作为 JSON）
- ✅ 笔记本的版本控制操作
- ✅ 调试笔记本格式问题

### 不适用场景

- ❌ 交互式运行代码单元格
- ❌ 数据探索和可视化
- ❌ 阅读笔记本中的 Markdown 文档
- ❌ 查看单元格输出

**建议**：对于交互式笔记本工作，请将此扩展与 JupyterLab 或 VS Code 配合使用。当需要直接操作笔记本的 JSON 结构时使用 Zed。

## 未来路线图

### 等待 Zed 核心功能

以下功能**无法实现**，直到 Zed 添加 Notebook API 支持（[zed#17325](https://github.com/zed-industries/zed/issues/17325)）：

- 🔮 **单元格渲染**：可视化单元格 UI（类似 VS Code/JupyterLab）
- 🔮 **代码执行**：通过 Jupyter 内核运行代码
- 🔮 **Markdown 预览**：渲染 Markdown 单元格
- 🔮 **输出渲染**：显示图表、表格、图像
- 🔮 **交互式小部件**：IPywidgets 支持

### 近期可能的增强功能

使用当前 Zed API 可以添加的功能：

- 🔄 **代码折叠**：在 JSON 视图中折叠/展开单元格
- 🔄 **大纲视图**：在单元格之间导航
- 🔄 **单元格命令**：用于常见操作的斜杠命令
- 🔄 **格式验证**：增强的错误检查

### 需要您的帮助

如果您希望 Zed 支持完整的笔记本功能：
- 👍 为 [zed#17325](https://github.com/zed-industries/zed/issues/17325) 点赞
- 💬 在该 issue 中分享您的使用场景
- 🤝 为 Zed 核心做贡献，帮助实现 Notebook API

## 📚 开发路线图

### ✅ 阶段 1：JSON 编辑器基础（已完成）
- `.ipynb` 文件类型识别
- 带有 Jupyter 特定关键字的增强 JSON 语法高亮
- 括号匹配和自动缩进
- 笔记本结构解析和验证

### 🔄 阶段 2：编辑器增强（当前 API 可实现）
- 单元格代码折叠
- 大纲/导航视图
- 自定义斜杠命令
- 更好的错误诊断

### 🔮 阶段 3：完整笔记本体验（受阻）
**需要**：Zed Notebook API（[zed#17325](https://github.com/zed-industries/zed/issues/17325)）

一旦 Zed 实现类似 VS Code 的 Notebook API，我们可以添加：
- 可视化的单元格编辑器
- 支持内核的代码执行
- Markdown 渲染
- 富媒体输出显示
- 交互式调试

## 🔧 技术细节

### 架构

此扩展使用：
- **Tree-sitter JSON** 用于语法解析
- **Zed Extension API 0.6.0** 用于语言集成
- **Rust + WebAssembly** 用于扩展运行时
- **Serde** 用于笔记本结构验证

### 文件格式

Jupyter Notebooks（`.ipynb`）是遵循 [nbformat 规范](https://nbformat.readthedocs.io/)的 JSON 文件。此扩展支持：
- nbformat 4.x（当前标准）
- 单元格类型：code、markdown、raw
- 元数据结构
- 输出格式

## 系统要求

- Zed 编辑器 v0.160.0 或更高版本
- 无需额外依赖

## 已知限制

1. **无可视化单元格**：笔记本以格式化的 JSON 显示
2. **无代码执行**：使用 JupyterLab、VS Code 或 `jupyter notebook` 来运行代码
3. **无输出渲染**：单元格输出以 JSON 数据形式显示
4. **大文件**：超大笔记本（>10MB）可能编辑较慢

## 🤝 贡献

欢迎贡献！请随时提交 issues 或 pull requests。

## 📄 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 🙏 致谢

- 基于 [Zed Extension API](https://github.com/zed-industries/zed) 构建
- 使用 [Tree-sitter JSON](https://github.com/tree-sitter/tree-sitter-json) 进行解析
