---
name: zed-mcp-extension
description: Build Zed editor extensions that bridge to remote HTTP MCP servers using mcp-remote as a stdio-to-HTTP bridge. Use this skill whenever the user wants to create a Zed extension for an MCP server, integrate an MCP tool into Zed, or build a context server extension for the Zed editor — even if they don't explicitly mention "mcp-remote" or "stdio bridge". Also trigger when the user asks about Zed extension development involving npm packages, WASM compilation, or context servers.
---

# Building Zed MCP Server Extensions

You are helping build a Zed editor extension that integrates a remote HTTP MCP server. Zed only supports stdio-based MCP servers, so the extension uses `mcp-remote` (npm package) as a bridge between stdio and HTTP SSE transport.

## Architecture

```
Zed Editor
  └── loads extension.wasm (Rust, compiled to WASM)
        └── context_server_command() → installs mcp-remote, returns Command
  └── spawns: node_modules/.bin/mcp-remote <URL> --header "Authorization: Bearer KEY"
        └── bridges stdio ↔ HTTP SSE (MCP protocol)
              └── Remote MCP server responds
```

The Rust extension is a thin bridge. `mcp-remote` handles all MCP protocol translation between Zed's stdio expectations and the remote HTTP endpoint. No custom Node.js code needed.

## File Structure

```
project-root/
├── extension.toml              # Extension manifest
├── Cargo.toml                  # Rust project config
├── src/
│   └── <snake_case_name>.rs    # Core extension (~80-100 lines)
├── configuration/
│   ├── default_settings.jsonc  # Settings template with placeholders
│   └── installation_instructions.md  # Minimal setup guide (1-2 lines)
├── .github/workflows/
│   └── release.yml             # CI/CD for publishing
├── .gitignore
├── README.md
└── LICENSE
```

## Step 1: Create extension.toml

```toml
id = "<unique-extension-id>"
name = "<Display Name>"
description = "<One-line description>"
version = "0.1.0"
schema_version = 1
authors = ["<author>"]
repository = "<github-repo-url>"

[context_servers.<unique-extension-id>]
name = "<Display Name>"
```

The `id` and `[context_servers.<id>]` key must match. This is the identifier used in Zed's settings under `"context_servers"`.

## Step 2: Create Cargo.toml

```toml
[package]
name = "<snake_case_name>"
version = "0.1.0"
edition = "2021"
publish = false
license = "Apache-2.0"

[lib]
path = "src/<snake_case_name>.rs"
crate-type = ["cdylib"]

[dependencies]
zed_extension_api = "0.7.0"
serde = "1.0"
schemars = "0.8"
```

The crate type must be `cdylib` (compiled to WASM). The lib path must point to the single Rust source file.

## Step 3: Write the Rust Extension

Read `references/rust_template.md` for a complete, copy-paste-ready implementation with inline comments explaining each section.

### Key API facts about zed_extension_api 0.7.0

The npm-related functions are **top-level** in the `zed` module — there is no `zed::npm` namespace:

```rust
zed::npm_package_installed_version(package_name)  // → Result<Option<String>>
zed::npm_install_package(package_name, version)    // → Result<(), String>
zed::npm_package_latest_version(package_name)      // → Result<String, String>
zed::node_binary_path()                            // → Result<String, String>
zed::make_file_executable(path)                    // → Result<(), String>
```

**Critical:** There is NO `npm_package_installed_path` function. Installed package binaries are always at `node_modules/.bin/<name>` relative to the extension's working directory.

### Extension trait signatures

```rust
fn context_server_command(&mut self, &ContextServerId, &Project) -> Result<Command>
fn context_server_configuration(&mut self, &ContextServerId, &Project) -> Result<Option<ContextServerConfiguration>>
```

### Key types

```rust
Command {
    command: String,
    args: Vec<String>,
    env: Vec<(String, String)>,  // Vec of tuples, NOT HashMap
}

ContextServerConfiguration {
    installation_instructions: String,  // Markdown string
    default_settings: String,           // JSONC string
    settings_schema: String,            // JSON Schema string
}

// Read user settings:
ContextServerSettings::for_project("server-id", project)  // → Result<ContextServerSettings>
```

### mcp-remote bridge pattern

The check-then-install pattern ensures mcp-remote is available:

```rust
let version = zed::npm_package_installed_version("mcp-remote")?;
if version.is_none() {
    zed::npm_install_package("mcp-remote", "0.1.29")?;
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
    args: vec![remote_url, "--header".into(), format!("Authorization: Bearer {}", api_key)],
    env: Vec::new(),
})
```

Always call `make_file_executable` on non-Windows — npm bin scripts may not have execute permissions.

### Settings preservation pattern

When Zed reconfigures a context server, it calls `context_server_configuration` and uses the `default_settings` to populate the UI. If you return a hardcoded default, it **overwrites** the user's saved API key every time.

The fix: read the user's current settings and inject them into `default_settings` before returning. This is the Context7 pattern:

```rust
let mut default_settings = include_str!("../configuration/default_settings.jsonc").to_string();

let settings = ContextServerSettings::for_project("server-id", project);
if let Ok(user_settings) = settings {
    if let Some(settings_value) = user_settings.settings {
        if let Ok(parsed) = serde_json::from_value::<SettingsStruct>(settings_value) {
            match parsed.api_key {
                Some(key) => {
                    let serialized_key =
                        serde_json::to_string(&key).map_err(|e| e.to_string())?;
                    default_settings = default_settings.replace(
                        "\"PLACEHOLDER_KEY\"",
                        &serialized_key,
                    );
                }
                None => {
                    default_settings = default_settings.replace("\"PLACEHOLDER_KEY\"", "\"\"");
                }
            }
        }
    }
}
```

The `default_settings.jsonc` file uses a placeholder that gets replaced at runtime:
```jsonc
{
  "api_key": "PLACEHOLDER_KEY"
}
```

Reference: `akbxr/zed-mcp-server-context7` on GitHub for the canonical implementation.

## Step 4: Configuration Files

### default_settings.jsonc

Use a placeholder value that gets replaced at runtime by the settings preservation logic:

```jsonc
{
  "api_key": "YOUR_API_KEY"
}
```

### installation_instructions.md

Keep this to 1-2 lines. Zed displays this in the settings UI and verbose instructions are overwhelming. Just point to where the user gets their API key:

```markdown
Get your API key from [provider dashboard](https://example.com/keys) and set it below.
```

## Step 5: CI/CD (release.yml)

```yaml
name: Release
on:
  push:
    tags:
      - "v*"
permissions:
  contents: read
jobs:
  release:
    runs-on: ubuntu-latest
    permissions:
      contents: write
    steps:
      - uses: actions/checkout@v4
      - uses: huacnlee/zed-extension-action@v2.0.0
        with:
          extension-name: your-extension-name
          push-to: your-name/extensions
        env:
          COMMITTER_TOKEN: ${{ secrets.COMMITTER_TOKEN }}
```

## Step 6: Build and Verify

```bash
cargo check                                    # Verify Rust compiles
rustup target add wasm32-wasip1                # One-time: add WASM target
cargo build --target wasm32-wasip1 --release   # Build WASM (~370K output)
```

WASM output lands at `target/wasm32-wasip1/release/<crate_name>.wasm`.

## Step 7: Install as Dev Extension in Zed

1. Open Zed
2. `Cmd+Shift+P` → type **"install dev extension"**
3. Select the **project root directory**
4. Zed compiles the WASM and loads the extension
5. Configure in Zed's `settings.json`:

```json
{
  "context_servers": {
    "<extension-id>": {
      "settings": {
        "api_key": "actual-key-here"
      }
    }
  }
}
```

**Important:** Use the `"context_servers"` key — NOT `"lsp"`. The server ID must match the `[context_servers.<id>]` key in `extension.toml`.

## Reference Implementations

When in doubt, refer to these existing extensions that follow this exact pattern:
- **Exa MCP**: `exa-labs/zed-exa-mcp-extension` — the canonical mcp-remote bridge example
- **Context7**: `akbxr/zed-mcp-server-context7` — settings preservation pattern
- **This project**: `minggliangg/zed-mcp-server-zai-web-reader` — Z.AI Web Reader with auth header
