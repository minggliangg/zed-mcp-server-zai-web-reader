use schemars::JsonSchema;
use serde::Deserialize;

use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};
use zed::settings::ContextServerSettings;

const MCP_REMOTE_PACKAGE: &str = "mcp-remote";
const MCP_REMOTE_VERSION: &str = "latest";
const DEFAULT_MCP_URL: &str = "https://api.z.ai/api/mcp/web_reader/mcp";

struct ZaiWebReaderMcpExtension;

#[derive(Debug, Deserialize, JsonSchema)]
struct ZaiWebReaderContextServerSettings {
    #[serde(default)]
    zai_api_key: Option<String>,
}

impl zed::Extension for ZaiWebReaderMcpExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let version = zed::npm_package_installed_version(MCP_REMOTE_PACKAGE)?;
        if version.is_none() {
            zed::npm_install_package(MCP_REMOTE_PACKAGE, MCP_REMOTE_VERSION)?;
        }

        let settings = ContextServerSettings::for_project("mcp-server-zai-web-reader", project)?;
        let settings: ZaiWebReaderContextServerSettings =
            if let Some(settings_value) = settings.settings {
                serde_json::from_value(settings_value).map_err(|e| e.to_string())?
            } else {
                ZaiWebReaderContextServerSettings { zai_api_key: None }
            };

        let mut args = vec![DEFAULT_MCP_URL.to_string()];
        if let Some(api_key) = settings.zai_api_key {
            args.push("--header".to_string());
            args.push(format!("Authorization: Bearer {}", api_key));
        }

        let command = if cfg!(target_os = "windows") {
            "node_modules/.bin/mcp-remote.cmd".to_string()
        } else {
            let path = "node_modules/.bin/mcp-remote";
            zed::make_file_executable(path)?;
            path.to_string()
        };

        Ok(Command {
            command,
            args,
            env: Vec::new(),
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(ZaiWebReaderContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(ZaiWebReaderMcpExtension);
