# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A task manager web application built with Rust and Axum. The application provides a REST API backed by SQLite and serves a single-page HTML interface with vanilla JavaScript for interactive task management.

## Technology Stack

- **Backend**: Axum 0.8 web framework
- **Database**: SQLite with SQLx (async, runtime-tokio)
- **Templating**: Askama for server-side HTML rendering
- **Frontend**: Vanilla JavaScript with modern CSS
- **Runtime**: Tokio async runtime

## Development Commands

### Building and Running
```bash
# Run the application (starts server on http://localhost:8080)
cargo run

# Build for development
cargo build

# Build optimized release binary
cargo build --release

# Check code compiles without building
cargo check
```

### Code Quality
```bash
# Format code
cargo fmt

# Run linter
cargo clippy
```

### Custom Slash Commands
- `/start-app` - Starts the application in background mode and opens it in Chrome
- `/kill-app` - Stops all running instances of the application

## Architecture

### Module Structure

The application follows a 4-module architecture:

- **src/main.rs**: Application entry point, routing configuration, CORS setup, and server initialization
- **src/db.rs**: Database pool initialization and schema creation
- **src/models.rs**: Task model definition and database operations layer
- **src/handlers.rs**: HTTP request handlers and response formatting
- **templates/index.html**: Askama template with embedded JavaScript for SPA-like interactions

### Database

SQLite database (`tasks.db`) is created automatically on first run. The schema includes:

```sql
CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    completed BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
)
```

Database operations are implemented as methods on the `Task` model in src/models.rs:
- `Task::get_all()` - Fetch all tasks ordered by creation date
- `Task::create()` - Create a new task
- `Task::complete()` - Mark a task as completed
- `Task::toggle()` - Toggle completion state
- `Task::update()` - Update task title

### State Management

The application uses Axum's state management pattern to share a `SqlitePool` across all handlers. The pool is initialized in src/main.rs:13-21 and attached to the router with `.with_state(pool)`.

### Error Handling

Custom error handling is implemented via the `AppError` wrapper type (src/handlers.rs:20-39) which uses `anyhow::Error` internally. This provides consistent error responses across all endpoints and simplifies error propagation with the `?` operator.

## API Endpoints

| Method | Path | Description | Response |
|--------|------|-------------|----------|
| GET | `/` | Render HTML page with all tasks | HTML |
| GET | `/tasks` | Get all tasks | JSON array |
| POST | `/tasks` | Create a new task | JSON (201) |
| PUT | `/tasks/:id` | Update task title | JSON or 404 |
| POST | `/tasks/:id/complete` | Mark task as completed | JSON or 404 |
| POST | `/tasks/:id/toggle` | Toggle completion state | JSON or 404 |

## Key Implementation Patterns

### Database Layer Pattern
All database operations are encapsulated as async methods on the `Task` struct. This keeps business logic close to the data model and provides a clean interface for handlers.

### Axum Handler Pattern
Handlers use Axum extractors (`State`, `Path`, `Json`) to access request data and return `Result<T, AppError>` for automatic error handling.

### Template Rendering
The HTML template (templates/index.html) uses Askama syntax for server-side rendering while including JavaScript for client-side interactivity, providing both initial page load performance and dynamic updates.

### CORS Configuration
CORS is configured in src/main.rs:24-27 to allow requests from any origin, which is useful for development but should be restricted in production.

## Server Configuration

The server binds to `0.0.0.0:8080` (all interfaces, port 8080), making it accessible at http://localhost:8080 locally.
