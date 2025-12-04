# Start Application

Start the Rust task manager application in a background bash process and open it in Chrome using the chrome devtools MCP server.

## Steps

1. Start the application using `cargo run` in background mode with the Bash tool (set run_in_background to true)
2. Use BashOutput to monitor the server output and wait until you see "Server running at <http://localhost:8080>"
3. Once the server is ready, use the chrome devtools MCP server to navigate to <http://localhost:8080>
4. Take a snapshot of the page to show the user what's displayed
5. Inform the user that the application is running and provide the background bash ID so they can monitor it

## IMPORTANT

- Use background bash so the server can continue running and be monitored
- Wait for the server to be fully ready before navigating Chrome
- The server typically runs on port 8080
