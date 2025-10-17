# Jupyter Notebook Extension for Zed

English | [简体中文](README.zh-CN.md)

A **Jupyter Notebook JSON Editor** for the [Zed editor](https://zed.dev/). This extension provides enhanced editing capabilities for `.ipynb` files as structured JSON documents.

> **Note**: This extension is positioned as a JSON editor for notebook files. Full notebook features (cell rendering, code execution, markdown preview) require VS Code-style Notebook API support in Zed, which is tracked in [zed#17325](https://github.com/zed-industries/zed/issues/17325).

## Current Features (v0.1.1)

- ✅ **File Type Recognition**: Automatic detection of `.ipynb` files
- ✅ **Enhanced JSON Syntax Highlighting**:
  - Jupyter-specific keywords (`cell_type`, `execution_count`, `metadata`)
  - Cell type values (`code`, `markdown`, `raw`)
  - Output types (`stream`, `display_data`, `execute_result`)
- ✅ **Smart Editing**:
  - Bracket matching and auto-closing (`{}`, `[]`, `""`)
  - Context-aware indentation for nested structures
  - JSON structure validation
- ✅ **Notebook Structure Support**:
  - Parse and validate notebook format (nbformat v4)
  - Cell type recognition
  - Metadata extraction
- ✅ **Lightweight**: No external dependencies or downloads required

## What This Extension Does NOT Provide

Due to current limitations of Zed's extension API:

- ❌ **Cell-based UI** - Notebooks display as JSON, not visual cells
- ❌ **Code Execution** - Cannot run Python/R code or connect to kernels
- ❌ **Markdown Rendering** - Markdown cells shown as JSON strings
- ❌ **Output Display** - Cell outputs shown as JSON, not rendered
- ❌ **Interactive Features** - No variable viewer, debugging, or widgets

For these features, use [VS Code](https://code.visualstudio.com/) with the Jupyter extension or [JupyterLab](https://jupyter.org/).

## Installation

### From Zed Extensions (Coming Soon)
1. Open Zed
2. Press `cmd+shift+p` (Mac) or `ctrl+shift+p` (Windows/Linux)
3. Search for "zed: extensions"
4. Search for "Jupyter Notebook"
5. Click Install

### Development Installation
1. Clone this repository
2. In Zed, open the extensions view (`cmd+shift+p` → "zed: extensions")
3. Click "Install Dev Extension"
4. Select the `jupyter-zed` directory

## Usage

This extension is designed for **editing notebook structure and metadata**, not for interactive data science workflows.

### Good Use Cases

- ✅ Manually editing notebook metadata
- ✅ Reviewing notebook file structure
- ✅ Batch processing notebooks as JSON (with jq or similar tools)
- ✅ Version control operations on notebooks
- ✅ Debugging notebook format issues

### Not Suitable For

- ❌ Running code cells interactively
- ❌ Data exploration and visualization
- ❌ Reading markdown documentation in notebooks
- ❌ Viewing cell outputs

**Recommendation**: For interactive notebook work, use this extension alongside JupyterLab or VS Code. Use Zed when you need to directly manipulate the notebook's JSON structure.

## Future Roadmap

### Waiting on Zed Core Features

The following features **cannot be implemented** until Zed adds Notebook API support ([zed#17325](https://github.com/zed-industries/zed/issues/17325)):

- 🔮 **Cell-based Rendering**: Visual cell UI (like VS Code/JupyterLab)
- 🔮 **Code Execution**: Run code through Jupyter kernels
- 🔮 **Markdown Preview**: Render markdown cells
- 🔮 **Output Rendering**: Display plots, tables, images
- 🔮 **Interactive Widgets**: IPywidgets support

### Possible Near-term Enhancements

Features that could be added with current Zed API:

- 🔄 **Code Folding**: Collapse/expand cells in JSON view
- 🔄 **Outline View**: Navigate between cells
- 🔄 **Cell Commands**: Slash commands for common operations
- 🔄 **Format Validation**: Enhanced error checking

### Help Wanted

If you want full notebook support in Zed:
- 👍 Upvote [zed#17325](https://github.com/zed-industries/zed/issues/17325)
- 💬 Share your use cases in that issue
- 🤝 Contribute to Zed's core to help implement Notebook API

## Development Roadmap

### ✅ Phase 1: JSON Editor Foundation (Completed)
- File type recognition for `.ipynb` files
- Enhanced JSON syntax highlighting with Jupyter-specific keywords
- Bracket matching and auto-indentation
- Notebook structure parsing and validation

### 🔄 Phase 2: Editor Enhancements (Possible with Current API)
- Code folding for cells
- Outline/navigation view
- Custom slash commands
- Better error diagnostics

### 🔮 Phase 3: Full Notebook Experience (Blocked)
**Requires**: Zed Notebook API ([zed#17325](https://github.com/zed-industries/zed/issues/17325))

Once Zed implements a Notebook API similar to VS Code, we can add:
- Visual cell-based editor
- Code execution with kernel support
- Markdown rendering
- Rich output display
- Interactive debugging

## Technical Details

### Architecture

This extension uses:
- **Tree-sitter JSON** for syntax parsing
- **Zed Extension API 0.6.0** for language integration
- **Rust + WebAssembly** for extension runtime
- **Serde** for notebook structure validation

### File Format

Jupyter Notebooks (`.ipynb`) are JSON files following the [nbformat specification](https://nbformat.readthedocs.io/). This extension understands:
- nbformat 4.x (current standard)
- Cell types: code, markdown, raw
- Metadata structures
- Output formats

## Requirements

- Zed editor v0.160.0 or later
- No additional dependencies

## Known Limitations

1. **No Visual Cells**: Notebooks are displayed as formatted JSON
2. **No Code Execution**: Use JupyterLab, VS Code, or `jupyter notebook` for running code
3. **No Output Rendering**: Cell outputs appear as JSON data
4. **Large Files**: Very large notebooks (>10MB) may be slow to edit

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Acknowledgments

- Built with the [Zed Extension API](https://github.com/zed-industries/zed)
- Uses [Tree-sitter JSON](https://github.com/tree-sitter/tree-sitter-json) for parsing
