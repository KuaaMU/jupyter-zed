# 🔧 Grammar 编译失败解决方案

## 问题诊断

您遇到的错误：
```
Error: Failed to install dev extension: failed to compile grammar 'json'
```

**根本原因：** Zed 无法从 GitHub 下载 Tree-sitter grammar（网络连接问题）

## 解决方案

### 方案 1：使用代理（推荐）

如果您使用代理访问 GitHub：

```bash
# 设置 Git 代理（临时）
set HTTP_PROXY=http://127.0.0.1:7890
set HTTPS_PROXY=http://127.0.0.1:7890

# 或者配置 Git 全局代理
git config --global http.proxy http://127.0.0.1:7890
git config --global https.proxy http://127.0.0.1:7890

# 然后重新在 Zed 中安装扩展
```

### 方案 2：使用内置 JSON 支持（临时方案）

Zed 可能已经内置了 JSON 支持。我们可以移除 grammar 配置，直接使用内置支持：

让我为您创建一个简化版本：
