# lumina-tasks

A production-grade REST API for task management, built with **Rust**, **Axum**, **PostgreSQL**, and clean architecture principles.

## Overview

lumina-tasks is a full-featured task management backend that demonstrates professional Rust development practices including:
- Type-safe SQL queries with `sqlx`
- Clean modular architecture (models, handlers, database, error handling)
- Async/await patterns with `tokio`
- Proper error handling with custom error types
- CORS-enabled HTTP API

## Tech Stack

- **Language**: Rust 1.91+
- **Web Framework**: Axum 0.7
- **Database**: PostgreSQL 16
- **Async Runtime**: Tokio
- **SQL**: sqlx (type-safe)
- **OS Support**: Linux, macOS, Windows (via WSL)

## Getting Started

### Prerequisites

- Rust 1.70+ (or use `nix shell nixpkgs#rust`)
- PostgreSQL 16
- Cargo

### Installation

1. Clone the repository:
```bash
git clone https://github.com/NyxLucy/lumina-tasks.git
cd lumina-tasks
```

2. Set up environment variables:
```bash
cp .env.example .env
# Edit .env with your database credentials
```

3. Create the database:
```bash
createdb lumina_tasks
```

4. Create the tasks table:
```bash
psql -U postgres -d lumina_tasks -f schema.sql
```

Or run manually:
```sql
CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    categorie VARCHAR(100),
    description TEXT,
    progress SMALLINT DEFAULT 0,
    achieved BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
```

5. Build and run:
```bash
cargo build
cargo run
```

Server starts at `http://127.0.0.1:3001`

## API Endpoints

### Tasks

#### List all tasks
```
GET /tasks
Response: [{ id, title, categorie, description, progress, achieved, created_at, updated_at }, ...]
```

#### Get a specific task
```
GET /tasks/:id
Response: { id, title, categorie, description, progress, achieved, created_at, updated_at }
```

#### Create a new task
```
POST /tasks
Content-Type: application/json

{
  "title": "Learn Rust",
  "categorie": "education",
  "description": "Master Rust and systems programming"
}

Response: { id, title, categorie, description, progress, achieved, created_at, updated_at }
```

#### Update a task
```
PUT /tasks/:id
Content-Type: application/json

{
  "title": "Master Rust",
  "progress": 50,
  "achieved": false
}

Response: { id, title, categorie, description, progress, achieved, created_at, updated_at }
```

#### Delete a task
```
DELETE /tasks/:id
Response: 204 No Content
```

## Project Structure

```
src/
├── main.rs          # Server setup, routing, CORS configuration
├── lib.rs           # Module declarations
├── error.rs         # Custom error types and HTTP response conversion
├── models/
│   └── mod.rs       # Task, CreateTaskRequest, UpdateTaskRequest structs
├── db/
│   └── mod.rs       # Database functions (CRUD operations)
└── handlers/
    └── mod.rs       # HTTP endpoint handlers
```

## Architecture Highlights

### Error Handling
Custom `AppError` enum that automatically converts to HTTP responses:
- `TaskNotFound` → 404
- `DatabaseError` → 500
- `ValidationError` → 400

### Database Layer
Type-safe SQL with `sqlx`:
- `query_as::<_, Task>()` for automatic deserialization
- `.bind()` for safe parameterized queries (prevents SQL injection)
- Connection pooling with `PgPool`

### Handlers
Clean separation between HTTP layer and business logic:
- Handlers extract request data
- Call database functions
- Return typed responses with proper status codes

## Development

### Run tests
```bash
cargo test
```

### Check code
```bash
cargo clippy
```

### Format code
```bash
cargo fmt
```

## Deployment

### Railway
Deploy backend + database in one command:
1. Push to GitHub
2. Connect repo to Railway
3. Add PostgreSQL service
4. Set `DATABASE_URL` environment variable
5. Deploy

### DigitalOcean
Self-hosted option:
1. Create Ubuntu droplet
2. Install Rust and PostgreSQL
3. Clone repo, `cargo build --release`
4. Use systemd to run as service
5. Use nginx as reverse proxy

## Learning Resources

This project demonstrates:
- **Rust fundamentals**: ownership, borrowing, pattern matching, traits
- **Async Rust**: tokio, async/await, concurrent task handling
- **Web development**: REST API design, HTTP status codes, JSON serialization
- **Database**: SQL, connection pooling, type-safe queries
- **Software architecture**: modular design, separation of concerns, error handling

## Future Improvements

- [ ] User authentication (JWT)
- [ ] Task filtering and sorting
- [ ] Priority levels
- [ ] Due dates
- [ ] Task subtasks
- [ ] Batch operations
- [ ] WebSocket support for real-time updates

## Contributing

This is a personal learning project, but feedback is welcome!

## License

MIT

## Author

**Nyx Lucianis** 2026
