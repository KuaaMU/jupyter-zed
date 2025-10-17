# 📚 文档索引

欢迎来到 Jupyter Zed 扩展项目！这里是所有文档的导航页面。

## 🌍 语言版本

- **中文文档优先** - 所有主要文档都有中文版本
- **English Available** - English versions available for all docs

## 📖 核心文档

### 新手入门

| 文档 | 中文版 | 英文版 | 说明 |
|------|--------|--------|------|
| **项目说明** | [README.zh-CN.md](README.zh-CN.md) | [README.md](README.md) | 项目介绍、功能特性、安装说明 |
| **快速开始** | [QUICKSTART.zh-CN.md](QUICKSTART.zh-CN.md) | [QUICKSTART.md](QUICKSTART.md) | 立即测试、编译、发布流程 |
| **项目总览** | [PROJECT.zh-CN.md](PROJECT.zh-CN.md) | - | 完整的项目结构和技术细节 |

### 开发文档

| 文档 | 语言 | 说明 |
|------|------|------|
| [DEVELOPMENT.md](DEVELOPMENT.md) | 英文 | 开发指南、调试技巧、问题排查 |

## 🎯 按需求查找

### 我想...

#### 快速了解项目
👉 阅读 [README.zh-CN.md](README.zh-CN.md)
- 功能特性
- 安装方法
- 开发路线图

#### 立即开始测试
👉 阅读 [QUICKSTART.zh-CN.md](QUICKSTART.zh-CN.md)
- 在 Zed 中安装
- 测试扩展
- 编译和发布

#### 深入了解实现
👉 阅读 [PROJECT.zh-CN.md](PROJECT.zh-CN.md)
- 完整文件清单
- 技术实现细节
- 代码统计
- 开发路线图

#### 贡献代码
👉 阅读 [DEVELOPMENT.md](DEVELOPMENT.md)
- 项目结构
- 开发环境设置
- 调试方法
- PR 流程

## 📁 项目结构

```
F:\jupyter-zed/
├── 📄 文档 (Docs)
│   ├── README.md                 # 英文项目说明
│   ├── README.zh-CN.md          # 中文项目说明 ⭐
│   ├── QUICKSTART.md            # 英文快速开始
│   ├── QUICKSTART.zh-CN.md      # 中文快速开始 ⭐
│   ├── DEVELOPMENT.md           # 开发指南
│   ├── PROJECT.zh-CN.md         # 项目总览 ⭐
│   └── DOCS.md                  # 本文件
│
├── ⚙️ 配置 (Config)
│   ├── extension.toml           # 扩展配置
│   ├── Cargo.toml              # Rust 配置
│   └── LICENSE                 # MIT 许可证
│
├── 💻 代码 (Code)
│   ├── src/
│   │   └── lib.rs              # Rust 扩展实现
│   └── languages/
│       └── jupyter/
│           ├── config.toml      # 语言配置
│           ├── highlights.scm   # 语法高亮
│           ├── brackets.scm     # 括号匹配
│           └── indents.scm      # 缩进规则
│
└── 🧪 测试 (Test)
    └── test.ipynb               # 测试 notebook
```

## 🚀 快速链接

### 开始使用
- [安装扩展](QUICKSTART.zh-CN.md#-立即测试)
- [测试功能](QUICKSTART.zh-CN.md#步骤-2-测试扩展)
- [查看示例](test.ipynb)

### 开发相关
- [编译扩展](QUICKSTART.zh-CN.md#-编译扩展)
- [提交代码](QUICKSTART.zh-CN.md#-提交到-git)
- [发布流程](QUICKSTART.zh-CN.md#-发布到-zed-扩展市场)

### 技术细节
- [文件类型识别](PROJECT.zh-CN.md#1-文件类型识别)
- [语法高亮实现](PROJECT.zh-CN.md#2-语法高亮)
- [Rust 框架](PROJECT.zh-CN.md#3-rust-扩展框架)

## 📊 文档统计

| 文档类型 | 文件数 | 总行数 | 总大小 |
|---------|--------|--------|--------|
| 项目说明 | 2 | ~500 | ~8 KB |
| 快速开始 | 2 | ~800 | ~16 KB |
| 开发指南 | 2 | ~600 | ~12 KB |
| **总计** | **6** | **~1900** | **~36 KB** |

## 🎓 学习路径

### 初级（0-1 小时）
1. 阅读 [README.zh-CN.md](README.zh-CN.md)
2. 按照 [QUICKSTART.zh-CN.md](QUICKSTART.zh-CN.md) 测试扩展
3. 打开 [test.ipynb](test.ipynb) 查看效果

### 中级（1-3 小时）
1. 阅读 [PROJECT.zh-CN.md](PROJECT.zh-CN.md) 了解实现细节
2. 查看 [src/lib.rs](src/lib.rs) 理解代码结构
3. 研究 [languages/jupyter/*.scm](languages/jupyter/) 语法规则

### 高级（3+ 小时）
1. 阅读 [DEVELOPMENT.md](DEVELOPMENT.md) 掌握开发技巧
2. 尝试修改语法高亮规则
3. 规划 Phase 2 功能实现

## ❓ 常见问题

### 找不到某个文档？
使用上面的表格或文件树快速定位。

### 想贡献文档？
1. Fork 项目
2. 编辑或添加文档
3. 提交 PR

### 文档有错误？
请在 [GitHub Issues](https://github.com/KuaaMU/jupyter-zed/issues) 报告。

## 🔄 文档更新

| 版本 | 日期 | 更新内容 |
|------|------|---------|
| v1.0 | 2025-10-17 | 初始版本，包含所有核心文档 |

## 📧 获取帮助

- 📖 查看文档
- 🐛 提交 [Issue](https://github.com/KuaaMU/jupyter-zed/issues)
- 💬 开始 [Discussion](https://github.com/KuaaMU/jupyter-zed/discussions)
- 📧 发送邮件：XCM853629353@OUTLOOK.com

---

**提示：** 如果您是第一次使用，建议从 [README.zh-CN.md](README.zh-CN.md) 开始！
