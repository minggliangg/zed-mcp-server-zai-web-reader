# Z.AI Web Reader MCP Server for Zed

A Model Context Protocol (MCP) server for the Zed editor that fetches and converts web pages into LLM-friendly input. This extension enables Zed's AI assistant to read and understand web content by converting URLs into structured, markdown-formatted text.

## Features

- Fetches web pages from any URL
- Converts HTML to clean, LLM-friendly markdown or plain text
- Configurable image retention or removal
- Optional caching for improved performance
- Image and link summary generation
- Handles complex page structures with intelligent content extraction

## Installation

1. Clone or download this repository
2. In Zed, go to **Zed > Install Development Extension** (or press `Cmd + Shift + P` and type "install development extension")
3. Navigate to this project directory and select it
4. Zed will build and install the extension

## Configuration

This extension requires a Z.AI API key:

1. Get your API key from [https://z.ai/manage-apikey/apikey-list](https://z.ai/manage-apikey/apikey-list)
2. Subscribe to the GLM Coding Plan
3. Add the API key to your Zed settings:

```json
{
  "context_servers": {
    "mcp-server-zai-web-reader": {
      "settings": {
        "zai_api_key": "your-api-key-here"
      }
    }
  }
}
```

## Available Tools

### webReader

Fetches and converts a URL into LLM-friendly input.

**Required Parameters:**
- `url` (string) - The URL of the website to fetch and read

**Optional Parameters:**
- `timeout` (integer, default: 20) - Request timeout in seconds
- `return_format` (string, default: "markdown") - Response content type: "markdown" or "text"
- `retain_images` (boolean, default: true) - Whether to retain images in the output
- `no_cache` (boolean, default: false) - Disable caching for this request
- `no_gfm` (boolean, default: false) - Disable GitHub Flavored Markdown
- `keep_img_data_url` (boolean, default: false) - Keep image data URLs
- `with_images_summary` (boolean, default: false) - Include an images summary
- `with_links_summary` (boolean, default: false) - Include a links summary

**Example usage in Zed's AI chat:**
```
Read the content from https://example.com and summarize it
```

## How It Works

This extension uses the MCP-Remote bridge architecture:

1. Zed's MCP client communicates with the local MCP server
2. The server makes requests to the Z.AI Web Reader API
3. The API fetches the web page and converts it to optimized markdown
4. The formatted content is returned to Zed's AI assistant for processing

This architecture allows seamless integration with Zed's built-in AI capabilities while offloading the heavy lifting of web scraping and content extraction to the Z.AI service.

## License

Apache-2.0 - See [LICENSE](LICENSE) for details.
