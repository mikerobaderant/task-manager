# Kill Application

Stop the running Rust task manager application.

## Steps:
1. First, try to kill any running `cargo run` or `task-manager` processes
2. Use `pkill -f "cargo run"` to kill cargo processes
3. Also kill any `task-manager` binary processes with `pkill task-manager`
4. Confirm the processes were terminated successfully
5. Inform the user that the application has been stopped

## Notes:
- This will stop all running instances of the task manager application
- If the application was started with the `/start-app` command, this will terminate the background bash process
- The command will attempt both killing cargo and the binary to ensure clean shutdown
