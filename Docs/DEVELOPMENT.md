# Jupyter Zed Extension - Development Guide

## 项目结构

```
jupyter-zed/
├── extension.toml                    # 扩展主配置
├── Cargo.toml                        # Rust 项目配置
├── LICENSE                           # MIT 许可证
├── README.md                         # 项目文档
├── .gitignore                        # Git 忽略文件
├── src/
│   └── lib.rs                        # Rust 扩展代码
└── languages/
    └── jupyter/
        ├── config.toml               # Jupyter 语言配置
        ├── highlights.scm            # 语法高亮规则
        ├── brackets.scm              # 括号匹配
        └── indents.scm               # 缩进规则
```

## 测试扩展

### 1. 本地开发测试

1. 在 Zed 中打开扩展页面（`cmd+shift+p` → "zed: extensions"）
2. 点击 "Install Dev Extension"
3. 选择 `F:\jupyter-zed` 目录
4. 打开任意 `.ipynb` 文件测试

### 2. 编译检查

```bash
cd F:\jupyter-zed
cargo build --target wasm32-wasip1
```

### 3. 创建测试文件

创建一个简单的 Jupyter notebook 用于测试：

```json
{
  "cells": [
    {
      "cell_type": "code",
      "execution_count": 1,
      "metadata": {},
      "outputs": [],
      "source": ["print('Hello, Zed!')"]
    }
  ],
  "metadata": {},
  "nbformat": 4,
  "nbformat_minor": 5
}
```

## 开发阶段

### ✅ 阶段 1：基础支持（已完成）
- 文件类型识别（.ipynb）
- JSON 语法高亮
- 基本编辑器配置

### 🔄 阶段 2：增强体验（计划中）
需要添加的功能：
- 自定义 cell 类型的语法高亮
- 代码折叠支持
- Notebook 大纲视图

实现步骤：
1. 创建自定义 Tree-sitter grammar 或增强现有 JSON queries
2. 添加 `outline.scm` 用于结构视图
3. 改进 `highlights.scm` 以区分不同 cell 类型

### 🔄 阶段 3：完整集成（未来）
需要实现：
- Jupyter Language Server 集成
- 内核连接
- Cell 执行
- 调试支持

实现步骤：
1. 在 `extension.toml` 中配置 language server
2. 在 `src/lib.rs` 中实现 `language_server_command`
3. 添加内核管理功能
4. 实现 cell 执行逻辑

## 下一步开发建议

### 立即可做
1. **测试当前版本**
   - 在 Zed 中安装为 dev extension
   - 测试 .ipynb 文件的语法高亮
   - 检查基本编辑功能

2. **准备发布**
   - 更新 `extension.toml` 中的 repository URL
   - 完善 README
   - 创建示例 notebook 文件

### 阶段 2 开发
1. **增强语法高亮**
   - 修改 `highlights.scm` 以更好地突出 cell 类型
   - 为 code cells 添加 Python 语法注入
   - 为 markdown cells 添加 Markdown 语法注入

2. **添加 outline 支持**
   - 创建 `languages/jupyter/outline.scm`
   - 显示 cells 的结构化视图

### 阶段 3 开发
1. **集成 Jupyter LSP**
   ```toml
   [language_servers.jupyter-lsp]
   name = "Jupyter Language Server"
   languages = ["Jupyter Notebook"]
   ```

2. **实现内核通信**
   - 使用 Jupyter messaging protocol
   - 添加 WebSocket 或 ZMQ 通信
   - 实现 execute_request/execute_reply

## 有用的资源

- [Zed Extension API 文档](https://docs.rs/zed_extension_api)
- [Tree-sitter 文档](https://tree-sitter.github.io)
- [Jupyter Messaging Protocol](https://jupyter-client.readthedocs.io/en/stable/messaging.html)
- [Jupyter LSP](https://github.com/jupyter-lsp/jupyterlab-lsp)

## 问题排查

### 扩展无法加载
- 检查 `extension.toml` 格式是否正确
- 确认 grammar 的 repository 和 rev 是否有效
- 查看 Zed 日志：`zed: open log`

### 语法高亮不工作
- 检查 `.ipynb` 文件是否被正确识别为 Jupyter Notebook
- 验证 `highlights.scm` 语法是否正确
- 在前台模式运行 Zed：`zed --foreground`
