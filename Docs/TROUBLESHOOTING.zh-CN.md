# 🔧 故障排除指南

## ❌ 已解决的问题

### Grammar 编译错误

**错误信息：**
```
Error: Failed to install dev extension: failed to compile grammar 'json'
```

**原因：**
- grammar 版本号不正确或已过时
- 多个 grammar 同时加载导致冲突

**解决方案：**
1. 更新到最新的 grammar 版本
2. Phase 1 只使用 JSON grammar
3. Python 和 Markdown grammar 留到 Phase 2

**已修复：**
- ✅ 更新 JSON grammar 到 v0.24.8
- ✅ 移除暂时不需要的 Python 和 Markdown grammars
- ✅ 简化配置，专注于核心功能

## 🔍 常见问题

### 1. 扩展无法安装

**症状：**
- "Install Dev Extension" 失败
- 提示 grammar 编译错误

**检查步骤：**
```bash
# 1. 验证 extension.toml 格式
cat extension.toml

# 2. 检查 grammar 版本是否有效
gh api repos/tree-sitter/tree-sitter-json/tags --jq '.[0].name'

# 3. 验证文件结构
ls -R languages/
```

**解决方法：**
- 使用最新的稳定 grammar 版本
- 确保 extension.toml 格式正确
- 检查 languages/ 目录结构完整

### 2. 语法高亮不工作

**症状：**
- .ipynb 文件打开后没有高亮
- 显示为纯文本

**检查步骤：**
1. 确认文件扩展名是 `.ipynb`
2. 检查状态栏语言显示
3. 查看 Zed 日志

**解决方法：**
```bash
# 重新加载扩展
Ctrl+Shift+P → "zed: reload window"

# 查看日志
Ctrl+Shift+P → "zed: open log"
```

### 3. 编译失败

**症状：**
```
error: failed to compile
```

**解决方法：**
```bash
# 清理旧的编译产物
cargo clean

# 确保安装了正确的 target
rustup target add wasm32-wasip1

# 重新编译
cargo build --target wasm32-wasip1 --release
```

### 4. Git 提交问题

**症状：**
- 文件未被跟踪
- .gitignore 不生效

**解决方法：**
```bash
# 查看状态
git status

# 清理 Git 缓存
git rm -r --cached .
git add .

# 重新提交
git commit -m "Fix gitignore"
```

## 📝 测试清单

### 安装前检查
- [ ] Rust 已安装（`rustc --version`）
- [ ] wasm32-wasip1 target 已添加
- [ ] Zed 版本 >= 0.100.0

### 安装测试
- [ ] "Install Dev Extension" 成功
- [ ] 无错误提示
- [ ] 扩展出现在列表中

### 功能测试
- [ ] 打开 test.ipynb
- [ ] 状态栏显示 "Jupyter Notebook"
- [ ] JSON 结构有语法高亮
- [ ] 括号匹配工作
- [ ] 自动缩进正常

## 🛠️ 调试技巧

### 查看详细日志
```bash
# 前台模式运行 Zed（Windows）
zed.exe --foreground

# 查看实时日志
Ctrl+Shift+P → "zed: open log"
# 搜索 "jupyter" 或 "grammar"
```

### 验证 Grammar 配置
```bash
# 测试 grammar 是否可访问
curl -I https://github.com/tree-sitter/tree-sitter-json

# 检查版本标签
gh api repos/tree-sitter/tree-sitter-json/tags --jq '.[0]'
```

### 重置扩展
```bash
# 1. 卸载开发扩展
# 在 Zed Extensions 页面找到扩展，点击卸载

# 2. 清理编译产物
cd F:\jupyter-zed
cargo clean
rm extension.wasm

# 3. 重新编译
cargo build --target wasm32-wasip1 --release

# 4. 重新安装
# Ctrl+Shift+P → "Install Dev Extension"
```

## 📞 获取帮助

如果问题仍未解决：

1. **检查 Zed 日志**
   - `Ctrl+Shift+P` → "zed: open log"
   - 搜索错误关键词

2. **查看 GitHub Issues**
   - Zed: https://github.com/zed-industries/zed/issues
   - 本项目: https://github.com/KuaaMU/jupyter-zed/issues

3. **提交问题**
   - 包含完整错误信息
   - 附上 extension.toml 内容
   - 说明操作步骤

4. **联系作者**
   - 📧 XCM853629353@OUTLOOK.com

## 🔄 更新记录

| 日期 | 问题 | 解决方案 |
|------|------|---------|
| 2025-10-17 | Grammar 编译失败 | 更新到 v0.24.8，简化配置 |

---

**最后更新：** 2025-10-17
