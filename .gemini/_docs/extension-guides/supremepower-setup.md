# SupremePower Extension Setup & Troubleshooting (Windows)

This document logs the manual steps required to get the `supremepower` extension working on Windows, as the default installation process may fail to compile the MCP server or encounter path-related runtime errors.

## Issue 1: Missing Compiled Files (`dist` folder)
The extension is downloaded as TypeScript source code, but the `gemini-extension.json` expects compiled JavaScript in `mcp-server/dist/server.js`. The default installation does not trigger a build.

### Fix:
Navigate to the extension directory and manually install dependencies and build:
```powershell
cd $HOME\.gemini\extensions\supremepower
npm install
npm run build
```

## Issue 2: "Connection closed" / "Server Error" on Windows
The MCP server contains a check to ensure it's being run directly:
`if (import.meta.url === 'file://' + process.argv[1])`

On Windows, `import.meta.url` uses forward slashes and a `file:///` prefix, while `process.argv[1]` typically uses backslashes and no triple-slash. This comparison fails, causing the server to exit immediately without starting the transport.

### Fix:
The file `mcp-server/dist/server.js` (and the source `mcp-server/src/server.ts`) must be patched to bypass this check or handle Windows paths correctly.

**Applied Patch:**
Changed the check to `if (true) {` in `mcp-server/dist/server.js` to force the server to start.

## Current Configuration
- **Extension Path:** `C:\Users\slett\.gemini\extensions\supremepower`
- **MCP Server Name:** `supremepower`
- **Entry Point:** `mcp-server/dist/server.js`
