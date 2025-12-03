# Task Manager

A minimal yet elegant task management web application built with **Rust** and **Axum**. Features a beautiful dark-themed UI with server-side rendering and persistent SQLite storage.

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-0.8-orange)
![SQLite](https://img.shields.io/badge/SQLite-003B57?style=flat&logo=sqlite&logoColor=white)

## ✨ Features

- **View Tasks** — Display all tasks in a clean, modern list
- **Add Tasks** — Create new tasks with a simple input form
- **Complete Tasks** — Toggle task completion status with visual feedback
- **Persistent Storage** — Tasks survive server restarts via SQLite
- **Real-time UI** — Instant updates without page reloads (AJAX)
- **Beautiful Design** — Dark theme with gradient accents and smooth animations

## 🛠️ Tech Stack

| Component | Choice | Rationale |
|-----------|--------|-----------|
| **Web Framework** | [Axum](https://docs.rs/axum) | Modern, tokio-native, ergonomic API |
| **Database** | [SQLite](https://www.sqlite.org/) via [sqlx](https://docs.rs/sqlx) | Async, compile-time query checking |
| **Runtime** | [Tokio](https://tokio.rs/) | Required by Axum |
| **Templating** | [Askama](https://docs.rs/askama) | Type-safe, compile-time templates |
| **Serialization** | [Serde](https://serde.rs/) | Industry standard for Rust JSON |

## 📁 Project Structure

```
task-manager/
├── Cargo.toml              # Dependencies and package config
├── src/
│   ├── main.rs             # Entry point, router setup, server config
│   ├── handlers.rs         # Request handlers (HTTP endpoints)
│   ├── models.rs           # Task struct, database operations
│   └── db.rs               # Database pool initialization
├── templates/
│   └── index.html          # Askama template for the UI
└── tasks.db                # SQLite database (auto-generated)
```

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70+)
- Cargo (included with Rust)

### Installation

1. **Clone the repository**

   ```bash
   git clone <repository-url>
   cd 001-task-manager/task-manager
   ```

2. **Build the project**

   ```bash
   cargo build
   ```

3. **Run the server**

   ```bash
   cargo run
   ```

4. **Open in browser**

   Navigate to [http://localhost:8080](http://localhost:8080)

## 📡 API Reference

### Endpoints

| Method | Endpoint | Description | Request Body | Response |
|--------|----------|-------------|--------------|----------|
| `GET` | `/` | Render main page with tasks | — | HTML |
| `GET` | `/tasks` | Retrieve all tasks | — | `200 OK` + `Task[]` |
| `POST` | `/tasks` | Create a new task | `{ "title": "..." }` | `201 Created` + `Task` |
| `PUT` | `/tasks/{id}` | Update a task's title | `{ "title": "..." }` | `200 OK` + `Task` |
| `POST` | `/tasks/{id}/complete` | Mark task as completed | — | `200 OK` + `Task` |
| `POST` | `/tasks/{id}/toggle` | Toggle completion status | — | `200 OK` + `Task` |

### Data Model

```json
{
  "id": 1,
  "title": "Buy groceries",
  "completed": false,
  "created_at": "2025-12-02 10:30:00"
}
```

### Example Requests

**Create a task:**

```bash
curl -X POST http://localhost:8080/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "Learn Rust"}'
```

**List all tasks:**

```bash
curl http://localhost:8080/tasks
```

**Toggle task completion:**

```bash
curl -X POST http://localhost:8080/tasks/1/toggle
```

## 🎨 UI Preview

The application features a sleek dark theme inspired by modern development tools:

- **Color Palette:** Deep blacks (`#0d1117`), subtle grays, purple-to-teal gradients
- **Typography:** Outfit for headings, JetBrains Mono for code/metadata
- **Interactions:** Smooth hover transitions, slide-in animations for new tasks
- **Design Language:** GitHub-inspired dark mode with custom accents

## 🔧 Configuration

The server runs on port `8080` by default. To change this, modify `main.rs`:

```rust
let addr = SocketAddr::from(([0, 0, 0, 0], 8080)); // Change port here
```

## 📚 Resources

- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [SQLx Documentation](https://docs.rs/sqlx/latest/sqlx/)
- [Axum Examples](https://github.com/tokio-rs/axum/tree/main/examples)
- [Askama Templates](https://docs.rs/askama/latest/askama/)

## 🏗️ Built For

**BB Web Dev Lab** — Language Roulette exercise where developers build the same app spec in randomly assigned languages/frameworks.

---

*Built with Rust 🦀 and a cup of coffee ☕*

