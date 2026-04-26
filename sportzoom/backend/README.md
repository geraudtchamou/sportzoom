# SportZoom Backend

High-performance Rust backend for the SportZoom social sports platform.

## Features

- 🔐 **Authentication**: JWT-based auth with refresh tokens
- 🎯 **Predictions**: AI-powered match predictions with gamification
- 📱 **Social Feed**: Personalized video/image feed with trending algorithm
- 🏆 **Gamification**: Points, badges, levels, and leaderboards
- 🎥 **Live Streaming**: Real-time streaming integration ready
- 📊 **Analytics**: Comprehensive user behavior tracking
- 🛡️ **Security**: Input validation, CORS, rate limiting

## Tech Stack

- **Framework**: Axum (Tokio-based)
- **Database**: PostgreSQL with SQLx
- **Cache**: Redis
- **Auth**: JWT with Argon2 password hashing
- **Validation**: Validator crate

## Quick Start

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies
apt-get install libssl-dev pkg-config libpq-dev
```

### Environment Setup

Create `.env` file:

```env
DATABASE_URL=postgresql://postgres:password@localhost:5432/sportzoom
REDIS_URL=redis://localhost:6379
JWT_SECRET=your-super-secret-key-change-in-production
PORT=8080
HOST=0.0.0.0
```

### Run with Docker Compose

```bash
cd ../
docker-compose up -d
```

### Build & Run

```bash
cd backend
cargo build --release
cargo run
```

## API Endpoints

### Authentication

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| POST | `/api/v1/auth/register` | Register new user | ❌ |
| POST | `/api/v1/auth/login` | Login user | ❌ |
| POST | `/api/v1/auth/refresh` | Refresh access token | ❌ |
| POST | `/api/v1/auth/logout` | Logout user | ✅ |

### Users

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| GET | `/api/v1/users/:id` | Get user profile | ❌ |
| GET | `/api/v1/users/profile` | Get current user | ✅ |
| PUT | `/api/v1/users/profile` | Update profile | ✅ |

### Predictions

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| GET | `/api/v1/predictions` | List all predictions | ❌ |
| POST | `/api/v1/predictions` | Create prediction | ✅ |
| GET | `/api/v1/predictions/:id` | Get prediction | ❌ |
| GET | `/api/v1/predictions/events/:event_id` | Event predictions | ❌ |
| GET | `/api/v1/predictions/events/:event_id/suggestions` | AI suggestions | ❌ |

### Events

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| GET | `/api/v1/events` | List events | ❌ |
| POST | `/api/v1/events` | Create event | ✅ |
| GET | `/api/v1/events/:id` | Get event | ❌ |
| POST | `/api/v1/events/:id/join` | Join event | ✅ |

### Feed & Posts

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| GET | `/api/v1/feed` | Personalized feed | Optional |
| GET | `/api/v1/feed/trending` | Trending posts | ❌ |
| GET | `/api/v1/feed/recommendations` | Recommended posts | ✅ |
| POST | `/api/v1/posts` | Create post | ✅ |
| POST | `/api/v1/posts/:id/like` | Like post | ✅ |
| POST | `/api/v1/posts/:id/comments` | Add comment | ✅ |

### Live Streaming

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| GET | `/api/v1/live` | List live streams | ❌ |
| POST | `/api/v1/live/start` | Start stream | ✅ |
| POST | `/api/v1/live/:id/end` | End stream | ✅ |

### Gamification

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| GET | `/api/v1/gamification/leaderboard` | Top users | ❌ |
| GET | `/api/v1/gamification/points` | User points | ✅ |
| GET | `/api/v1/gamification/badges` | All badges | ❌ |
| GET | `/api/v1/gamification/achievements` | Achievements | ❌ |

### Admin

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| GET | `/api/v1/admin/stats` | Dashboard stats | Admin |
| GET | `/api/v1/admin/users` | List all users | Admin |
| POST | `/api/v1/admin/users/:id/ban` | Ban user | Admin |

## Request Examples

### Register

```bash
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "email": "john@example.com",
    "password": "securepass123",
    "display_name": "John Doe"
  }'
```

### Create Prediction

```bash
curl -X POST http://localhost:8080/api/v1/predictions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -d '{
    "event_id": "uuid-here",
    "prediction_type": "match_winner",
    "predicted_value": "Team A"
  }'
```

## Security Features

- Password hashing with Argon2
- JWT tokens with short expiry (15min access, 7day refresh)
- Input validation on all endpoints
- CORS protection
- SQL injection prevention via parameterized queries
- Rate limiting ready

## Performance

- Connection pooling (PostgreSQL & Redis)
- Async I/O throughout
- Efficient indexing strategy
- Query optimization

## Monitoring

Logs are output in JSON format for easy integration with:
- Prometheus + Grafana
- ELK Stack
- Datadog

## License

MIT
