# 快速开始指南 - Jupyter Zed 扩展

[English](QUICKSTART.md) | 简体中文

## 🎉 扩展已创建完成！

您的 Jupyter 扩展现在已经准备好进行测试了。

## 📁 项目结构

```
F:\jupyter-zed/
├── extension.toml          # ✅ 扩展配置
├── Cargo.toml             # ✅ Rust 配置
├── src/lib.rs             # ✅ 扩展代码
├── languages/jupyter/     # ✅ 语言定义
│   ├── config.toml       # Jupyter 语言配置
│   ├── highlights.scm    # 语法高亮规则
│   ├── brackets.scm      # 括号匹配
│   └── indents.scm       # 缩进规则
├── README.md              # ✅ 英文文档
├── README.zh-CN.md        # ✅ 中文文档
├── DEVELOPMENT.md         # ✅ 开发指南
├── LICENSE                # ✅ MIT 许可证
└── test.ipynb             # ✅ 测试文件
```

## 🚀 立即测试

### 步骤 1: 在 Zed 中安装开发扩展

1. 打开 Zed 编辑器
2. 按 `Ctrl+Shift+P`（Windows/Linux）或 `Cmd+Shift+P`（Mac）打开命令面板
3. 输入 "extensions" 并选择 **"zed: extensions"**
4. 点击 **"Install Dev Extension"** 按钮
5. 选择目录：`F:\jupyter-zed`

### 步骤 2: 测试扩展

1. 在 Zed 中打开 `F:\jupyter-zed\test.ipynb`
2. 检查语法高亮是否工作
3. 测试以下功能：
   - ✅ 文件被识别为 Jupyter Notebook
   - ✅ JSON 结构的语法高亮
   - ✅ 括号匹配
   - ✅ 自动缩进

### 步骤 3: 查看日志（如有问题）

**方法 1：前台模式运行**
```bash
zed --foreground
```

**方法 2：在 Zed 中查看日志**
- `Ctrl+Shift+P` → 输入 "log" → 选择 **"zed: open log"**

## 🔧 编译扩展

```bash
cd F:\jupyter-zed

# 1. 安装 Rust（如果还没安装）
# 访问 https://rustup.rs 下载安装

# 2. 添加 WebAssembly 编译目标
rustup target add wasm32-wasip1

# 3. 编译扩展
cargo build --target wasm32-wasip1 --release

# 编译成功后，产物位于：
# target/wasm32-wasip1/release/jupyter_zed.wasm
```

## 📝 提交到 Git

```bash
cd F:\jupyter-zed

# 查看文件状态
git status

# 添加所有文件
git add .

# 创建初始提交
git commit -m "Initial commit: Jupyter Notebook extension for Zed

功能特性:
- 支持 .ipynb 文件类型识别
- 基于 JSON 的语法高亮
- Jupyter 特定字段增强高亮
- 括号匹配和自动缩进
- Phase 1 版本，可以发布"

# 推送到 GitHub（需要先在 GitHub 创建远程仓库）
git remote add origin https://github.com/KuaaMU/jupyter-zed.git
git branch -M main
git push -u origin main
```

## 🌟 发布到 Zed 扩展市场

### 前置条件
- ✅ GitHub 账号
- ✅ 扩展仓库已推送到 GitHub
- ✅ 许可证为 MIT 或 Apache 2.0

### 步骤 1: Fork Zed Extensions 仓库

```bash
# 1. 访问并 Fork：https://github.com/zed-industries/extensions
# 2. 克隆你 fork 的仓库
cd ..
git clone https://github.com/KuaaMU/extensions.git zed-extensions
cd zed-extensions
```

### 步骤 2: 添加你的扩展作为 submodule

```bash
# 添加 submodule
git submodule add https://github.com/KuaaMU/jupyter-zed.git extensions/jupyter

# 更新 submodule
git add extensions/jupyter
```

### 步骤 3: 更新 extensions.toml

在 `extensions.toml` 文件**顶部**添加：

```toml
[jupyter]
submodule = "extensions/jupyter"
version = "0.1.0"
```

### 步骤 4: 排序并提交

```bash
# 安装 Node.js 和 pnpm（如果还没安装）
# 访问 https://nodejs.org/
# 然后：npm install -g pnpm

# 排序扩展列表（确保字母顺序）
pnpm sort-extensions

# 提交更改
git add .
git commit -m "Add Jupyter Notebook extension

添加 Jupyter Notebook (.ipynb) 文件支持
- 文件类型识别
- 语法高亮
- 基础编辑功能"

# 推送到你的 fork
git push origin main
```

### 步骤 5: 创建 Pull Request

1. 访问 https://github.com/zed-industries/extensions
2. 点击 **"Pull requests"** → **"New pull request"**
3. 点击 **"compare across forks"**
4. 选择你的 fork：`KuaaMU/extensions`
5. 填写 PR 标题和描述：
   ```
   标题：Add Jupyter Notebook extension

   描述：
   This PR adds support for Jupyter Notebook (.ipynb) files in Zed.

   Features:
   - File type recognition for .ipynb files
   - JSON-based syntax highlighting with Jupyter-specific enhancements
   - Bracket matching and auto-indentation
   - MIT licensed

   Repository: https://github.com/KuaaMU/jupyter-zed
   ```
6. 提交 PR，等待 Zed 团队审核

### 注意事项

⚠️ **PR 检查项**：
- ✅ 许可证文件存在（LICENSE）
- ✅ 使用 HTTPS URL（不是 SSH）
- ✅ extensions.toml 已排序
- ✅ .gitmodules 已排序
- ✅ 版本号匹配

## 📚 下一步开发

### 阶段 1: 基础支持 ✅（已完成）
- [x] 文件类型识别
- [x] JSON 语法高亮
- [x] 基础编辑器配置

### 阶段 2: 增强体验（开发中）

**目标**：更好的可视化和导航体验

1. **增强语法高亮**
   ```bash
   # 编辑 languages/jupyter/highlights.scm
   # 添加更多 Jupyter 特定的高亮规则
   ```

2. **添加 Outline 支持**
   ```bash
   # 创建 languages/jupyter/outline.scm
   # 显示 notebook cells 的结构
   ```

3. **代码注入（Injections）**
   ```bash
   # 创建 languages/jupyter/injections.scm
   # 在 code cells 中注入 Python 语法
   # 在 markdown cells 中注入 Markdown 语法
   ```

### 阶段 3: 完整集成（计划中）

**目标**：完整的 Jupyter 开发体验

1. **集成 Jupyter LSP**
   - 修改 `extension.toml` 添加 language server 配置
   - 实现 `language_server_command` 方法

2. **内核管理**
   - 连接现有内核
   - 启动新内核
   - 管理多个内核

3. **代码执行**
   - 执行单个 cell
   - 执行多个 cells
   - 显示执行结果

## 🔍 调试技巧

### 查看扩展是否加载

```bash
# 在 Zed 中查看日志
Ctrl+Shift+P → "zed: open log"

# 搜索 "jupyter" 查看相关日志
```

### 重新加载扩展

```bash
# 修改代码后，在 Zed 中重新加载
Ctrl+Shift+P → "zed: reload window"
```

### 检查语法高亮

```bash
# 打开一个 .ipynb 文件
# 检查状态栏是否显示 "Jupyter Notebook"
# 如果显示 "JSON"，说明文件类型识别有问题
```

## ❓ 常见问题

### Q: 扩展无法加载
**A:**
1. 检查 Zed 日志中的错误信息
2. 确认 `extension.toml` 格式正确
3. 确认 grammar 的 repository 和 rev 有效
4. 尝试重新编译：`cargo clean && cargo build --target wasm32-wasip1`

### Q: 语法高亮不工作
**A:**
1. 确认文件扩展名是 `.ipynb`
2. 检查 `languages/jupyter/config.toml` 中的 `path_suffixes`
3. 验证 `highlights.scm` 语法正确
4. 尝试重新加载窗口

### Q: 如何更新扩展版本
**A:**
1. 修改 `extension.toml` 中的 `version`
2. 更新代码
3. 提交到 GitHub
4. 更新 zed-extensions 仓库中的 submodule
5. 提交新的 PR

### Q: 编译失败怎么办
**A:**
```bash
# 清理旧的编译产物
cargo clean

# 更新依赖
cargo update

# 检查 Rust 版本
rustc --version

# 确保安装了 wasm32-wasip1 target
rustup target add wasm32-wasip1

# 重新编译
cargo build --target wasm32-wasip1 --release
```

## 🎯 功能清单

### 已完成 ✅
- [x] 文件类型识别（.ipynb）
- [x] JSON 语法高亮
- [x] Jupyter 特定关键字高亮（cell_type, execution_count 等）
- [x] 括号匹配
- [x] 自动缩进
- [x] 基础编辑器配置
- [x] MIT 许可证
- [x] 英文和中文文档

### 计划中 🔄
- [ ] Cell 类型区分高亮（code/markdown/raw）
- [ ] 代码折叠支持
- [ ] Outline 视图
- [ ] Python 代码注入（code cells）
- [ ] Markdown 语法注入（markdown cells）
- [ ] Language Server 集成
- [ ] 内核连接
- [ ] Cell 执行
- [ ] 调试支持

## 📖 相关资源

- [Zed 扩展开发文档](https://zed.dev/docs/extensions/developing-extensions)
- [Tree-sitter 查询语法](https://tree-sitter.github.io/tree-sitter/using-parsers#query-syntax)
- [Jupyter Messaging Protocol](https://jupyter-client.readthedocs.io/en/stable/messaging.html)
- [Language Server Protocol](https://microsoft.github.io/language-server-protocol/)

## 🎊 总结

恭喜！您已经成功创建了一个 Jupyter Notebook 扩展！

**现在您可以**：
1. ✅ 在 Zed 中测试扩展
2. ✅ 提交代码到 GitHub
3. ✅ 发布到 Zed 扩展市场
4. ✅ 开始阶段 2 的开发

**需要帮助？**
- 📧 发送邮件：XCM853629353@OUTLOOK.com
- 🐛 提交 Issue：https://github.com/KuaaMU/jupyter-zed/issues
- 💬 开始讨论：https://github.com/KuaaMU/jupyter-zed/discussions

---

**祝您开发愉快！** 🚀
