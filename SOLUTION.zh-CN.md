# 🎯 解决方案：移除 Grammar 配置

## 问题分析

您遇到的 `failed to compile grammar 'json'` 错误是因为：
1. ❌ Zed 无法从 GitHub 下载 Tree-sitter grammar
2. ❌ 网络连接问题（curl 测试显示无法连接 github.com）

## 已实施的解决方案

### ✅ 使用 Zed 内置 JSON 支持

**修改内容：**
- 移除了 `extension.toml` 中的 `[grammars.json]` 配置
- 保留 `languages/jupyter/config.toml` 中的 `grammar = "json"`
- Zed 会自动使用其内置的 JSON grammar

**优势：**
- ✅ 无需下载外部 grammar
- ✅ 不依赖网络连接
- ✅ 使用 Zed 原生支持
- ✅ 安装更快、更稳定

## 现在请重试

### 步骤：
1. **在 Zed 中重新安装扩展**：
   ```
   Ctrl+Shift+P → "zed: extensions"
   → "Install Dev Extension"
   → 选择 F:\jupyter-zed
   ```

2. **应该成功了！** 然后测试：
   ```
   - 打开 F:\jupyter-zed\test.ipynb
   - 检查状态栏显示 "Jupyter Notebook"
   - 验证 JSON 语法高亮工作
   ```

## 备用方案（如果仍然失败）

### 方案 A：配置代理

如果您的环境需要代理访问 GitHub：

```bash
# Windows CMD
set HTTP_PROXY=http://127.0.0.1:7890
set HTTPS_PROXY=http://127.0.0.1:7890

# 或 PowerShell
$env:HTTP_PROXY="http://127.0.0.1:7890"
$env:HTTPS_PROXY="http://127.0.0.1:7890"

# 然后启动 Zed
zed.exe
```

### 方案 B：完全移除 grammar 引用

如果还有问题，让我知道，我可以进一步简化配置。

## 技术说明

**为什么这样做有效：**
- Zed 内置支持多种常见语言，包括 JSON
- 当 `grammar = "json"` 但没有在 `[grammars]` 中定义时
- Zed 会查找并使用内置的 JSON grammar
- `.ipynb` 文件本质上就是 JSON，所以内置支持足够

**Phase 2 规划：**
- 将来如果需要自定义 grammar（如特殊的 Jupyter 高亮）
- 可以考虑：
  - 手动下载 grammar 并放入项目
  - 或提供离线 grammar 包
  - 或使用 Zed 的 grammar 注入功能

---

**现在试试看能否成功安装！** 🤞
