use zed_extension_api::{self as zed, Result};

struct JupyterExtension;

impl zed::Extension for JupyterExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // TODO: Phase 3 - Implement Jupyter Language Server integration
        // For now, return an error indicating LSP is not yet implemented
        Err("Jupyter Language Server support is planned for a future release. \
             Currently providing basic syntax highlighting for .ipynb files.".into())
    }

    // Future methods for Jupyter kernel integration:
    // - fn connect_to_kernel() - Connect to existing Jupyter kernel
    // - fn start_kernel() - Start a new Jupyter kernel
    // - fn execute_cell() - Execute a notebook cell
    // - fn get_completions() - Get code completions from kernel
}

zed::register_extension!(JupyterExtension);
