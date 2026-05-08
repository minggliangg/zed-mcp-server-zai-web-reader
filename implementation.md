# Plan: Build zed-mcp-server-zai-web-reader

## Context

Build a Zed editor extension that integrates the [Z.AI Web Reader MCP Server](https://docs.z.ai/devpack/mcp/reader-mcp-server) into Zed's Assistant panel. The Z.AI Web Reader is a **remote HTTP MCP server** at `https://api.z.ai/api/mcp/web_reader/mcp` exposing a `webReader` tool. Since Zed only supports **stdio-based** MCP servers (via `context_server_command` returning a `Command`), we use `mcp-remote` (npm package) as a stdio-to-HTTP bridge — the same pattern used by the [Exa MCP extension](https://github.com/exa-labs/zed-exa-mcp-extension).

## Architecture

```
Zed Editor
  └── loads extension.wasm (Rust, ~80 lines)
        ├── context_server_command() → installs mcp-remote via npm, returns Command
        └── context_server_configuration() → returns settings UI
  └── spawns: node_modules/.bin/mcp-remote https://api.z.ai/api/mcp/web_reader/mcp --header "Authorization: Bearer KEY"
        └── bridges stdio ↔ HTTP (MCP protocol)
              └── Z.AI Web Reader API responds with webpage content
```

The Rust extension is a thin bridge. `mcp-remote` handles all MCP protocol translation. No custom Node.js code needed.

## Files to Create (9 files)

### 1. `extension.toml` — Extension manifest

Registers the extension and context server with Zed.

```toml
id = "mcp-server-zai-web-reader"
name = "Z.AI Web Reader MCP Server"
description = "Model Context Protocol Server for Z.AI Web Reader"
version = "0.1.0"
schema_version = 1
authors = ["minggliangg"]
repository = "https://github.com/minggliangg/zed-mcp-server-zai-web-reader"

[context_servers.mcp-server-zai-web-reader]
name = "Z.AI Web Reader MCP Server"
```

### 2. `Cargo.toml` — Rust project config

```toml
[package]
name = "mcp_server_zai_web_reader"
version = "0.1.0"
edition = "2021"
publish = false
license = "Apache-2.0"

[lib]
path = "src/mcp_server_zai_web_reader.rs"
crate-type = ["cdylib"]

[dependencies]
zed_extension_api = "0.7.0"
serde = "1.0"
schemars = "0.8"
```

### 3. `src/mcp_server_zai_web_reader.rs` — Core extension (~80 lines)

Implements `zed::Extension` trait:
- `context_server_command()`: Installs `mcp-remote` via npm, reads API key from settings, returns `Command` launching `mcp-remote` with the Z.AI endpoint URL and `--header "Authorization: Bearer KEY"`
- `context_server_configuration()`: Returns settings UI (installation instructions, default settings, JSON schema)

Settings struct has one field: `zai_api_key: Option<String>`.

Key constants:
- `MCP_REMOTE_PACKAGE = "mcp-remote"` with version `"0.1.29"`
- `DEFAULT_MCP_URL = "https://api.z.ai/api/mcp/web_reader/mcp"`

### 4. `configuration/default_settings.jsonc` — Default settings template

```jsonc
{
  // Your Z.AI API key from https://z.ai/manage-apikey/apikey-list
  "zai_api_key": "YOUR_ZAI_API_KEY"
}
```

### 5. `configuration/installation_instructions.md` — Setup instructions

Instructions for getting a Z.AI API key and subscribing to GLM Coding Plan.

### 6. `.gitignore` — Ignore build artifacts and node_modules

### 7. `README.md` — Project documentation

Features, installation, configuration, available tools (`webReader`), how it works.

### 8. `LICENSE` — Apache-2.0

### 9. `.github/workflows/release.yml` — CI/CD

Uses `huacnlee/zed-extension-action@v2.0.0` to publish on tag push.

## MCP Tool (exposed by remote server, passed through by mcp-remote)

- **Tool name**: `webReader`
- **Required param**: `url` (string) — URL to fetch
- **Optional params**: `timeout` (int, default 20), `return_format` ("markdown"|"text"), `retain_images` (bool), `no_cache` (bool), `no_gfm` (bool), `keep_img_data_url` (bool), `with_images_summary` (bool), `with_links_summary` (bool)

## Implementation Steps

1. Create directory structure (`src/`, `configuration/`, `.github/workflows/`)
2. Write `extension.toml` and `Cargo.toml`
3. Write `src/mcp_server_zai_web_reader.rs`
4. Write `configuration/default_settings.jsonc` and `configuration/installation_instructions.md`
5. Write `.gitignore`, `README.md`, `LICENSE`
6. Write `.github/workflows/release.yml`
7. Initialize git repo, make initial commit

## Verification

1. `cargo check` — verify Rust code compiles
2. `cargo build --target wasm32-wasip1` — verify WASM output (requires `rustup target add wasm32-wasip1`)
3. Install as dev extension in Zed, configure API key, verify `webReader` tool appears in Assistant
