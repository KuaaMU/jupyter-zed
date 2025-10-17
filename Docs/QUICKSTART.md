# Quick Start Guide - Jupyter Zed Extension

English | [简体中文](QUICKSTART.zh-CN.md)

## 🎉 扩展已创建完成！

您的 Jupyter 扩展现在已经准备好进行测试了。

## 📁 项目结构

```
F:\jupyter-zed/
├── extension.toml          # ✅ 扩展配置
├── Cargo.toml             # ✅ Rust 配置
├── src/lib.rs             # ✅ 扩展代码
├── languages/jupyter/     # ✅ 语言定义
│   ├── config.toml
│   ├── highlights.scm
│   ├── brackets.scm
│   └── indents.scm
├── README.md              # ✅ 项目文档
├── DEVELOPMENT.md         # ✅ 开发指南
├── LICENSE                # ✅ MIT 许可证
└── test.ipynb             # ✅ 测试文件
```

## 🚀 立即测试

### 步骤 1: 在 Zed 中安装开发扩展

1. 打开 Zed 编辑器
2. 按 `Ctrl+Shift+P`（Windows）打开命令面板
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

```bash
# 在前台模式运行 Zed 查看详细日志
zed --foreground
```

或在 Zed 中：
- `Ctrl+Shift+P` → 输入 "log" → 选择 **"zed: open log"**

## 🔧 编译扩展

```bash
cd F:\jupyter-zed

# 安装 Rust（如果还没安装）
# 访问 https://rustup.rs

# 添加 wasm32-wasip1 target
rustup target add wasm32-wasip1

# 编译扩展
cargo build --target wasm32-wasip1 --release
```

## 📝 提交到 Git

```bash
cd F:\jupyter-zed

# 初始化提交
git add .
git commit -m "Initial commit: Jupyter Notebook extension for Zed

- Basic file type support for .ipynb
- JSON-based syntax highlighting
- Bracket matching and indentation
- Ready for Phase 1 release"

# 推送到 GitHub（记得先创建远程仓库）
git remote add origin https://github.com/KuaaMU/jupyter-zed.git
git branch -M main
git push -u origin main
```

## 🌟 发布到 Zed 扩展市场

### 步骤 1: Fork Zed Extensions 仓库

```bash
# Fork https://github.com/zed-industries/extensions
# 然后克隆你的 fork
cd ..
git clone https://github.com/KuaaMU/extensions.git zed-extensions
cd zed-extensions
```

### 步骤 2: 添加你的扩展作为 submodule

```bash
git submodule add https://github.com/KuaaMU/jupyter-zed.git extensions/jupyter
```

### 步骤 3: 更新 extensions.toml

在 `extensions.toml` 文件顶部添加：

```toml
[jupyter]
submodule = "extensions/jupyter"
version = "0.1.0"
```

### 步骤 4: 排序并提交

```bash
# 如果有 pnpm（需要 Node.js）
pnpm sort-extensions

# 提交更改
git add .
git commit -m "Add Jupyter Notebook extension"
git push origin main
```

### 步骤 5: 创建 Pull Request

1. 访问 https://github.com/zed-industries/extensions
2. 点击 "Pull requests" → "New pull request"
3. 选择你的 fork
4. 提交 PR，等待审核

## 📚 下一步开发

查看 `DEVELOPMENT.md` 了解：
- ✅ 阶段 1: 基础支持（已完成）
- 🔄 阶段 2: 增强体验（计划中）
- 🔄 阶段 3: 完整集成（未来）

## ❓ 常见问题

### Q: 扩展无法加载
A: 检查 Zed 日志，确认 `extension.toml` 格式正确

### Q: 语法高亮不工作
A: 确认 `.ipynb` 文件被正确识别，检查 `highlights.scm`

### Q: 如何更新扩展
A: 修改代码后，在 Zed 中重新加载窗口（`Ctrl+Shift+P` → "reload window"）

## 🎯 当前功能清单

- [x] 文件类型识别（.ipynb）
- [x] JSON 语法高亮
- [x] Jupyter 特定关键字高亮
- [x] 括号匹配
- [x] 自动缩进
- [x] 基础编辑器配置
- [ ] Language Server 集成（Phase 3）
- [ ] 内核执行（Phase 3）
- [ ] 调试支持（Phase 3）

---

**恭喜！** 🎊 您的 Jupyter 扩展已经可以使用了！
