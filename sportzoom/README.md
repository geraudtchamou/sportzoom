# SportZoom

**The Social Network for Live Experiences**

A scalable, modern full-stack application combining **sports predictions, live events, short-form video content, live streaming, and AI-driven personalization**.

![Version](https://img.shields.io/badge/version-1.0.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Rust](https://img.shields.io/badge/rust-1.75+-orange)
![React Native](https://img.shields.io/badge/react_native-0.74-blue)

---

## 🎯 Core Vision

SportZoom is "The social network for live experiences" where users:
- ⚽ Engage with live sports and events
- 🎯 Predict outcomes and compete
- 📸 Share emotions through photos/videos
- 💬 Interact in real-time
- 🤖 Discover trending content via AI

---

## 🏗️ Architecture

```
sportzoom/
├── backend/              # Rust (Axum + PostgreSQL + Redis)
│   ├── src/
│   │   ├── handlers/     # HTTP request handlers
│   │   ├── services/     # Business logic
│   │   ├── db/           # Database layer (SQLx)
│   │   ├── websocket/    # Real-time features
│   │   └── middleware/   # Auth & security
│   └── migrations/       # SQL migrations
│
├── mobile/               # React Native (Expo)
│   ├── app/              # Expo Router screens
│   ├── components/       # Reusable UI components
│   ├── services/         # API & WebSocket clients
│   ├── store/            # Zustand state management
│   └── constants/        # Theme & config
│
└── infra/                # Docker & Kubernetes
    └── docker-compose.yml
```

---

## 🛠️ Tech Stack

### Backend (Rust)
| Component | Technology |
|-----------|------------|
| Language | Rust 1.75+ |
| Web Framework | Axum |
| Database | PostgreSQL 15 (SQLx) |
| Cache | Redis 7 |
| Real-time | WebSockets (Axum native) |
| Authentication | JWT + Argon2 |
| Logging | tracing + tracing-subscriber |

### Mobile (JavaScript/TypeScript)
| Component | Technology |
|-----------|------------|
| Framework | React Native (Expo SDK 51) |
| Navigation | Expo Router 3.5 |
| State Management | Zustand |
| Animations | React Native Reanimated 3 |
| Gestures | React Native Gesture Handler |
| Video | expo-av |
| Real-time | Socket.IO Client |
| HTTP | Axios |

---

## 🚀 Quick Start

### Prerequisites
- Rust 1.75+
- Node.js 18+
- Docker & Docker Compose

### 1. Clone the Repository
```bash
cd /workspace/sportzoom
```

### 2. Start Infrastructure
```bash
docker-compose -f infra/docker-compose.yml up -d
```

### 3. Setup Backend
```bash
cd backend
cp .env.example .env
sqlx migrate run
cargo run
```

Backend will be available at `http://localhost:8080`

### 4. Setup Mobile App
```bash
cd mobile
npm install
cat > .env << EOF
EXPO_PUBLIC_API_URL=http://localhost:8080/api/v1
EXPO_PUBLIC_WS_URL=ws://localhost:8080/ws
EOF
npm start
```

---

## 📡 API Endpoints

### Authentication
- `POST /api/v1/auth/register` - Register new user
- `POST /api/v1/auth/login` - Login user
- `POST /api/v1/auth/refresh` - Refresh access token
- `POST /api/v1/auth/logout` - Logout user

### Users
- `GET /api/v1/users/:id` - Get user profile
- `GET /api/v1/users/profile` - Get current user profile
- `PUT /api/v1/users/profile` - Update profile

### Predictions
- `GET /api/v1/predictions` - List predictions
- `POST /api/v1/predictions` - Create prediction
- `GET /api/v1/predictions/events/:event_id` - Get event predictions

### Events
- `GET /api/v1/events` - List events
- `POST /api/v1/events` - Create event
- `GET /api/v1/events/:id` - Get event details
- `POST /api/v1/events/:id/join` - Join event

### Feed & Posts
- `GET /api/v1/feed` - Get personalized feed
- `GET /api/v1/feed/trending` - Get trending posts
- `GET /api/v1/posts` - List posts
- `POST /api/v1/posts` - Create post
- `POST /api/v1/posts/:id/like` - Like post
- `POST /api/v1/posts/:id/comment` - Add comment

### Gamification
- `GET /api/v1/gamification/leaderboard` - Get leaderboard
- `GET /api/v1/gamification/points` - Get user points
- `GET /api/v1/gamification/badges` - Get user badges

### WebSocket
- `GET /ws` - Real-time connection for chat, reactions, updates

---

## 🎮 Key Features

### ⚽ Sports Prediction System
- Match winner predictions (Team A / Draw / Team B)
- Exact score predictions
- Player event predictions
- AI-based suggestions
- Points & bonuses

### 🏆 Gamification
- Points system
- Global/Friends/Country leaderboards
- Badges & achievements
- Streak rewards
- Level progression

### 🎥 Video-First Feed
- TikTok-style vertical scrolling
- Auto-play videos
- Double-tap to like
- Real-time view counts
- Emotion reactions

### 🔴 Live Streaming
- User-generated streams
- Live chat
- Viewer count
- Real-time reactions

### 💬 Real-Time Features
- WebSocket connections
- Live chat
- Presence system
- Instant notifications

---

## 📊 Database Schema

Key tables:
- `users` - User accounts with points & levels
- `events` - Sports matches, concerts, festivals
- `predictions` - User predictions with results
- `posts` - Social feed content
- `comments` - Post comments
- `live_streams` - Active/past streams
- `badges` & `user_badges` - Achievement system
- `notifications` - In-app notifications
- `likes` - Post/comment likes

---

## 🔐 Security Features

- JWT authentication with refresh tokens
- Argon2 password hashing
- Role-based access control
- Rate limiting
- XSS protection
- Input validation
- CORS configuration
- SQL injection prevention

---

## 📈 Scalability

- Connection pooling (20 max connections)
- Redis caching for hot data
- CDN delivery for media
- Adaptive streaming (HLS)
- Background jobs ready
- Horizontal scaling support

---

## 📱 Mobile Screens

1. **Home** - Trending events, live matches, quick actions
2. **Feed** - Full-screen video feed with interactions
3. **Create** - Media upload, caption, emotions
4. **Profile** - User info, stats, badges, settings

---

## 🎨 Design System

### Colors
- Primary: `#6C5CE7` (Purple)
- Secondary: `#00D1FF` (Cyan)
- Accent: `#FF3B5C` (Red)
- Background: `#0F0F14` (Dark)
- Surface: `#1A1A22`
- Card: `#22232B`

### Typography
- Heading XL: 32px bold
- Heading L: 24px semi-bold
- Body: 16px regular
- Caption: 12px

---

## 🧪 Testing

### Backend
```bash
cd backend
cargo test
```

### Mobile
```bash
cd mobile
npm test
```

---

## 📤 Deployment

### Backend (Production)
```bash
cargo build --release
./target/release/sportzoom-backend
```

### Mobile (Production)
```bash
eas build --platform ios
eas build --platform android
```

---

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📄 License

MIT License

---

## 🙏 Acknowledgments

Built with ❤️ using:
- [Rust](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum)
- [React Native](https://reactnative.dev/)
- [Expo](https://expo.dev/)

---

**SportZoom** - The Social Network for Live Experiences ⚡🎯
