# Complete Rust Extension Template

Copy this template and replace the `CHANGEME` markers with your specific values.

```rust
use schemars::JsonSchema;
use serde::Deserialize;

use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};
use zed::settings::ContextServerSettings;

// CHANGEME: Update the package name and version for your npm package
const MCP_REMOTE_PACKAGE: &str = "mcp-remote";
const MCP_REMOTE_VERSION: &str = "0.1.29";
// CHANGEME: Set your remote MCP server URL
const DEFAULT_MCP_URL: &str = "https://example.com/mcp/endpoint";

struct MyMcpExtension;

// CHANGEME: Rename the settings struct and add your configuration fields
#[derive(Debug, Deserialize, JsonSchema)]
struct MyMcpSettings {
    #[serde(default)]
    api_key: Option<String>,
}

impl zed::Extension for MyMcpExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        // Step 1: Ensure mcp-remote is installed
        let version = zed::npm_package_installed_version(MCP_REMOTE_PACKAGE)?;
        if version.is_none() {
            zed::npm_install_package(MCP_REMOTE_PACKAGE, MCP_REMOTE_VERSION)?;
        }

        // Step 2: Read settings from the project
        // CHANGEME: Use your extension ID (must match extension.toml context_servers key)
        let settings = ContextServerSettings::for_project("my-extension-id", project)?;
        let settings: MyMcpSettings = if let Some(settings_value) = settings.settings {
            serde_json::from_value(settings_value).map_err(|e| e.to_string())?
        } else {
            MyMcpSettings { api_key: None }
        };

        // Step 3: Build args — URL first, then any auth headers
        let mut args = vec![DEFAULT_MCP_URL.to_string()];
        // CHANGEME: Adjust how you pass the API key. Options:
        //   - As a header: args.push("--header"); args.push(format!("Authorization: Bearer {}", key));
        //   - As a query param: append ?key=value to the URL
        if let Some(api_key) = settings.api_key {
            args.push("--header".to_string());
            args.push(format!("Authorization: Bearer {}", api_key));
        }

        // Step 4: Build the command path
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
        project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();

        // Load default settings template and preserve user's current values
        let mut default_settings =
            include_str!("../configuration/default_settings.jsonc").to_string();

        // CHANGEME: Use your extension ID here too
        let settings = ContextServerSettings::for_project("my-extension-id", project);
        if let Ok(user_settings) = settings {
            if let Some(settings_value) = user_settings.settings {
                // CHANGEME: Use your settings struct
                if let Ok(my_settings) =
                    serde_json::from_value::<MyMcpSettings>(settings_value)
                {
                    match my_settings.api_key {
                        Some(api_key) => {
                            // CHANGEME: Use your placeholder string from default_settings.jsonc
                            let serialized_api_key =
                                serde_json::to_string(&api_key).map_err(|e| e.to_string())?;
                            default_settings = default_settings.replace(
                                "\"YOUR_API_KEY\"",
                                &serialized_api_key,
                            );
                        }
                        None => {
                            default_settings =
                                default_settings.replace("\"YOUR_API_KEY\"", "\"\"");
                        }
                    }
                }
            }
        }

        // CHANGEME: Use your settings struct for schema generation
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(MyMcpSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(MyMcpExtension);
```

## CHANGEME Checklist

When adapting this template, make sure to update all these markers:

1. `MCP_REMOTE_PACKAGE` / `MCP_REMOTE_VERSION` — only change if not using mcp-remote
2. `DEFAULT_MCP_URL` — your remote MCP server HTTP endpoint
3. Settings struct name and fields (`MyMcpSettings` → your name)
4. Extension struct name (`MyMcpExtension` → your name)
5. Extension ID string in `for_project()` calls — must match `extension.toml`
6. Placeholder string in `default_settings.replace()` — must match your `default_settings.jsonc`
7. How you pass auth (header vs query param vs env var)
8. `register_extension!` macro argument
