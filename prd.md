# Product Requirements Document: Task Manager API

## BB Web Dev Lab — Rust + Axum Implementation

---

## 1. Overview

**Project:** Language Roulette Task Manager  
**Developer:** Mike  
**Stack:** Rust (Axum framework)  
**Presentation:** 2 minutes at biweekly meeting

### Purpose

Build a minimal task management web application as part of the BB Web Dev Lab exercise. This PRD serves as the implementation roadmap for the Rust/Axum version, following the Explore-Plan-Execute methodology.

---

## 2. Functional Requirements

### 2.1 Web UI

| Feature | Description | Priority |
|---------|-------------|----------|
| View Tasks | Display all tasks in a list format | Must Have |
| Add Task | Form/input to create a new task | Must Have |
| Complete Task | Button/action to mark a task as done | Must Have |

### 2.2 API Endpoints

| Method | Endpoint | Description | Request Body | Response |
|--------|----------|-------------|--------------|----------|
| `GET` | `/tasks` | Retrieve all tasks | — | `200 OK` + Task[] |
| `POST` | `/tasks` | Create a new task | `{ "title": "..." }` | `201 Created` + Task |
| `POST` | `/tasks/{id}/complete` | Mark task as completed | — | `200 OK` + Task |

### 2.3 Data Model

```sql
CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    completed BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

**Rust struct equivalent:**

```rust
struct Task {
    id: i64,
    title: String,
    completed: bool,
    created_at: String,
}
```

---

## 3. Technical Requirements

### 3.1 Stack Decisions

| Component | Choice | Rationale |
|-----------|--------|-----------|
| **Web Framework** | Axum | Modern, tokio-native, ergonomic API |
| **Database** | SQLite via `sqlx` | Async, compile-time query checking |
| **Runtime** | Tokio | Required by Axum |
| **Serialization** | Serde | Industry standard for Rust JSON |
| **HTML Templating** | Inline HTML or `askama` | Keep it simple |

### 3.2 Project Setup

Using `cargo new` + `cargo add` for maximum control and latest dependencies:

```bash
cargo new task-manager
cd task-manager
cargo add axum
cargo add tokio -F full
cargo add sqlx -F "sqlite runtime-tokio tls-native-tls"
cargo add serde -F derive
cargo add serde_json
cargo add tower-http -F "fs cors"
```

**Why this approach:**
- No extra tooling to install (no `cargo-generate`, no `axum_cli`)
- Full understanding of what's in the project
- Pulls latest dependency versions automatically
- Better demo story: "Built from scratch with AI assistance"

**Resulting Cargo.toml:**

```toml
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio", "tls-native-tls"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower-http = { version = "0.6", features = ["fs", "cors"] }
```

### 3.3 Project Structure

```
task-manager/
├── Cargo.toml
├── src/
│   ├── main.rs          # Entry point, router setup
│   ├── handlers.rs      # Request handlers
│   ├── models.rs        # Task struct, DB operations
│   └── db.rs            # Database initialization
├── static/
│   └── index.html       # Simple UI
└── tasks.db             # SQLite database (generated)
```

---

## 4. Implementation Plan

Following the **Explore → Plan → Execute** methodology:

### Phase 1: Explore

- [x] Confirm Axum 0.7 patterns (routing, state management)
- [x] Verify sqlx SQLite async patterns
- [x] Check current best practices for error handling

### Phase 2: Plan

**Build order:**

1. Project scaffold with dependencies
2. Database connection pool setup
3. SQLite table creation on startup
4. `GET /tasks` endpoint
5. `POST /tasks` endpoint
6. `POST /tasks/{id}/complete` endpoint
7. Static file serving for UI
8. Minimal HTML frontend

### Phase 3: Execute

| Step | Task |
|------|------|
| 1 | Run setup commands (see Section 3.2), verify `cargo build` succeeds |
| 2 | Database pool + table init |
| 3 | GET /tasks handler |
| 4 | POST /tasks handler |
| 5 | POST /tasks/{id}/complete handler |
| 6 | Static file serving + basic HTML |
| 7 | Testing & bug fixes |

---

## 5. Acceptance Criteria

### Minimum Viable Demo (MVP)

- [x] Application starts without errors
- [x] `GET /tasks` returns JSON array
- [x] `POST /tasks` creates a task in SQLite
- [x] Tasks persist after restart
- [x] Basic HTML form submits tasks

### Full Success

- [x] All three API endpoints functional
- [x] UI displays task list
- [x] UI allows adding tasks
- [x] UI allows completing tasks
- [x] Clean error handling (no panics)

---

## 6. Risk Mitigation

| Risk | Mitigation |
|------|------------|
| sqlx compile-time checks slow builds | Use `sqlx::query!` sparingly, fallback to runtime queries |
| Async lifetime issues | Keep handlers simple, clone where needed |
| Time overrun on UI | Serve raw HTML string, skip templating |
| Database connection issues | Use connection pool, handle errors gracefully |

---

## 7. Presentation Prep

**Demo script (45 sec):**

1. Show app running in browser
2. Add a task "Buy groceries"
3. Add a task "Walk the dog"
4. Mark "Buy groceries" complete
5. Refresh page to prove persistence

**Experience talking points (60 sec):**

- Rust's type system + sqlx = confidence in queries
- Axum's ergonomics surprisingly good
- Borrow checker challenges with shared state
- AI assistance effectiveness with Rust (note any hallucinations)

**Hot take (15 sec):**

- Would I use Axum again? (Prepare honest answer)

---

## 8. Resources

- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [SQLx Documentation](https://docs.rs/sqlx/latest/sqlx/)
- [Axum Examples](https://github.com/tokio-rs/axum/tree/main/examples)

---

*PRD Version 1.0 — Ready for build session*