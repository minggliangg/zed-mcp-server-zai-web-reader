# Z.AI Web Reader MCP Server for Zed

A Zed extension that connects Zed's AI assistant to Z.AI's Web Reader MCP server. It fetches web pages and converts them into LLM-friendly content through Zed's context server integration.

## Features

- Fetches and reads web pages from URLs
- Converts web content into markdown or plain text
- Supports image and link summaries when provided by the remote server
- Passes your Z.AI API key as a bearer token
- Uses `mcp-remote` to bridge Zed's stdio MCP transport to the remote HTTP server

## Installation

1. Clone or download this repository.
2. In Zed, run **Install Development Extension** from the command palette.
3. Select this project directory.
4. Zed will build and install the extension.

## Configuration

This extension requires a Z.AI API key.

1. Get your API key from [Z.AI Console](https://z.ai/manage-apikey/apikey-list).
2. Add the API key to your Zed `settings.json`:

```json
{
  "context_servers": {
    "mcp-server-zai-web-reader": {
      "settings": {
        "zai_api_key": "your-api-key"
      }
    }
  }
}
```

## Available Tools

The available tools are provided by Z.AI's Web Reader MCP server and exposed to Zed through this extension.

## How It Works

This extension uses `mcp-remote` to bridge Zed's stdio-based MCP transport to Z.AI's remote HTTP endpoint. When configured, it sends the API key as an `Authorization: Bearer` header.

## Development

```bash
cargo check
cargo build --target wasm32-wasip1 --release
```

Install as a development extension in Zed with **Install Development Extension** and select the project root.

## License

MIT - See [LICENSE](LICENSE) for details.
