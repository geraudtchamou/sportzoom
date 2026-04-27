# Predicta Backend - Rust

High-performance backend for Predicta, a real-time fan engagement platform.

## Tech Stack

- **Language**: Rust
- **Web Framework**: Axum
- **Database**: PostgreSQL (with SQLx)
- **Cache**: Redis
- **Real-time**: WebSockets (via Axum)
- **Authentication**: JWT + Argon2

## Project Structure

```
src/
├── main.rs              # Application entry point
├── config.rs            # Configuration management
├── db/                  # Database layer
│   ├── mod.rs
│   ├── models.rs        # Data models
│   └── repositories.rs  # Data access
├── handlers/            # HTTP request handlers
│   ├── mod.rs
│   ├── auth.rs          # Authentication endpoints
│   ├── predictions.rs   # Prediction system
│   ├── events.rs        # Event management
│   ├── feed.rs          # Social feed
│   ├── live.rs          # Live streaming
│   └── users.rs         # User management
├── services/            # Business logic
│   ├── mod.rs
│   ├── ai.rs            # AI recommendations
│   ├── gamification.rs  # Points, badges, leaderboards
│   └── notifications.rs # Push notifications
├── websocket/           # Real-time features
│   ├── mod.rs
│   └── manager.rs       # Connection management
├── middleware/          # Custom middleware
│   ├── mod.rs
│   ├── auth.rs          # JWT validation
│   └── rate_limit.rs    # Rate limiting
└── utils/               # Utilities
    ├── mod.rs
    └── errors.rs        # Error handling
```

## Getting Started

### Prerequisites

- Rust 1.75+
- PostgreSQL 15+
- Redis 7+

### Environment Variables

Create a `.env` file:

```env
DATABASE_URL=postgresql://user:password@localhost:5432/predicta
REDIS_URL=redis://localhost:6379
JWT_SECRET=your-super-secret-key
PORT=8080
```

### Run

```bash
cargo run
```

### Build Release

```bash
cargo build --release
```

## API Endpoints

See `docs/API.md` for complete API documentation.

## License

MIT
