use std::env;

use zed_extension_api::settings::LspSettings;
use zed_extension_api::{self as zed, Result};

struct VueTsgoExtension;

impl VueTsgoExtension {
    /// Resolve the vue-tsgo binary. Prefer an explicit override from settings
    /// (`lsp.vue-tsgo.binary.path`), otherwise use the binary bundled with this
    /// extension (`bin/tsgo[.exe]`, resolved relative to the extension's working
    /// directory).
    fn binary_path(&self, worktree: &zed::Worktree) -> Result<String> {
        if let Some(path) = LspSettings::for_worktree("vue-tsgo", worktree)
            .ok()
            .and_then(|settings| settings.binary)
            .and_then(|binary| binary.path)
        {
            return Ok(path);
        }

        let (os, _arch) = zed::current_platform();
        let exe = match os {
            zed::Os::Windows => "bin/tsgo.exe",
            zed::Os::Mac | zed::Os::Linux => "bin/tsgo",
        };
        let cwd = env::current_dir().map_err(|e| e.to_string())?;
        Ok(cwd.join(exe).to_string_lossy().to_string())
    }
}

impl zed::Extension for VueTsgoExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // vue-tsgo speaks LSP over stdio. `--lsp` alone errors with
        // "only stdio is supported"; the `--stdio` flag is required.
        let mut args = vec!["--lsp".to_string(), "--stdio".to_string()];
        if let Some(extra) = LspSettings::for_worktree("vue-tsgo", worktree)
            .ok()
            .and_then(|settings| settings.binary)
            .and_then(|binary| binary.arguments)
        {
            args.extend(extra);
        }

        Ok(zed::Command {
            command: self.binary_path(worktree)?,
            args,
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(LspSettings::for_worktree("vue-tsgo", worktree)
            .ok()
            .and_then(|settings| settings.initialization_options))
    }
}

zed::register_extension!(VueTsgoExtension);
