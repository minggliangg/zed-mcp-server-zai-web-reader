# Installation Instructions

## 1. Get Your Z.AI API Key

1. Visit [https://z.ai/manage-apikey/apikey-list](https://z.ai/manage-apikey/apikey-list)
2. Sign in to your Z.AI account or create a new one
3. Click "Create API Key" or "New API Key"
4. Give your API key a descriptive name (e.g., "Zed MCP Server")
5. Copy the generated API key - you'll need it for the next step

## 2. Subscribe to GLM Coding Plan

The Web Reader MCP server requires a GLM Coding Plan subscription:

1. Navigate to [https://z.ai/manage-apikey/apikey-list](https://z.ai/manage-apikey/apikey-list)
2. Select the GLM Coding Plan that fits your usage needs
3. Complete the subscription process

## 3. Configure Zed

Once you have your API key, add it to your Zed settings:

1. Open Zed
2. Press `Cmd + ,` (macOS) or `Ctrl + ,` (Linux/Windows) to open settings
3. Click the "Open Settings" button to edit your `settings.json`
4. Add the following configuration:

```json
{
  "context_servers": {
    "mcp-server-zai-web-reader": {
      "settings": {
        "zai_api_key": "your-actual-api-key-here"
      }
    }
  }
}
```

Replace `your-actual-api-key-here` with the API key you copied from step 1.

5. Save the file and reload Zed

The Web Reader MCP server is now ready to use!
