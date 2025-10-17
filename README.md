# Jupyter Notebook Extension for Zed

English | [简体中文](README.zh-CN.md)

This extension adds Jupyter Notebook (`.ipynb`) support to the [Zed editor](https://zed.dev/).

## Features

### Current (v0.1.0)
- ✅ **File Type Recognition**: Automatic detection of `.ipynb` files
- ✅ **Syntax Highlighting**: JSON-based highlighting with Jupyter-specific enhancements
- ✅ **Basic Editing**: Bracket matching, auto-indentation for notebook structure

### Planned Features
- 🔄 **Phase 2**: Enhanced syntax highlighting for code cells, markdown cells, and outputs
- 🔄 **Phase 3**: Jupyter Language Server integration
- 🔄 **Phase 3**: Kernel connection and code execution
- 🔄 **Phase 3**: Interactive debugging support

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

Simply open any `.ipynb` file in Zed, and the extension will automatically provide syntax highlighting and basic editing features.

## Development Roadmap

### Phase 1: Basic Support ✅ (Current)
- File type recognition
- JSON-based syntax highlighting
- Basic editor configuration

### Phase 2: Enhanced Experience (Planned)
- Custom syntax highlighting for different cell types
- Code folding support
- Notebook outline view

### Phase 3: Full Integration (Planned)
- Jupyter Language Server support
- Kernel connection and management
- Cell execution
- Interactive outputs
- Debugging capabilities

## Requirements

- Zed editor v0.100.0 or later
- (Phase 3) Python 3.8+ with Jupyter installed for kernel execution

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Acknowledgments

- Built with the [Zed Extension API](https://github.com/zed-industries/zed)
- Uses [Tree-sitter JSON](https://github.com/tree-sitter/tree-sitter-json) for parsing
