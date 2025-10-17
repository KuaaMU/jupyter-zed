use zed_extension_api::{self as zed, Result, LanguageServerId, Worktree};
use serde::{Deserialize, Serialize};

/// Jupyter Notebook Extension for Zed
///
/// This extension provides support for Jupyter Notebook (.ipynb) files.
///
/// Current Features (Phase 2):
/// - Enhanced syntax highlighting for notebook structure
/// - Cell type recognition (code, markdown, raw)
/// - Output formatting
/// - Bracket matching and auto-indentation
///
/// Planned Features (Phase 3):
/// - Jupyter Language Server integration
/// - Kernel connection and management
/// - Cell execution
/// - Interactive outputs
struct JupyterExtension;

/// Jupyter Notebook cell structure
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
struct NotebookCell {
    cell_type: String,
    source: Vec<String>,
    metadata: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outputs: Option<Vec<serde_json::Value>>,
}

/// Jupyter Notebook structure
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
struct Notebook {
    cells: Vec<NotebookCell>,
    metadata: serde_json::Value,
    nbformat: i32,
    nbformat_minor: i32,
}

impl JupyterExtension {
    /// Parse a notebook file and validate its structure
    #[allow(dead_code)]
    fn parse_notebook(&self, content: &str) -> Result<Notebook> {
        serde_json::from_str(content)
            .map_err(|e| format!("Failed to parse notebook: {}", e).into())
    }

    /// Get the language ID for a code cell based on kernel metadata
    #[allow(dead_code)]
    fn get_cell_language(&self, notebook: &Notebook) -> String {
        // Extract language from metadata
        if let Some(lang_info) = notebook.metadata.get("language_info") {
            if let Some(name) = lang_info.get("name") {
                if let Some(lang) = name.as_str() {
                    return lang.to_string();
                }
            }
        }

        // Default to Python if not specified
        "python".to_string()
    }
}

impl zed::Extension for JupyterExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<zed::Command> {
        // Phase 3: Jupyter Language Server integration
        // This will enable features like:
        // - Code completion from kernel
        // - Hover information
        // - Go to definition
        // - Find references

        Err("Jupyter Language Server support is planned for Phase 3. \
             Currently providing enhanced syntax highlighting and editing support for .ipynb files.".into())
    }

    // Future methods for Phase 3:
    //
    // fn language_server_initialization_options(
    //     &mut self,
    //     language_server_id: &LanguageServerId,
    //     worktree: &Worktree,
    // ) -> Result<Option<serde_json::Value>>
    //
    // fn language_server_workspace_configuration(
    //     &mut self,
    //     language_server_id: &LanguageServerId,
    //     worktree: &Worktree,
    // ) -> Result<Option<serde_json::Value>>
}

zed::register_extension!(JupyterExtension);
