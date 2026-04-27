# Predicta - The Social Network for Live Experiences

A scalable, modern mobile application combining **sports predictions, live events, short-form video content, live streaming, and AI-driven personalization**.

## 🎯 Core Vision

Predicta is "The social network for live experiences" where users:
- Engage with live sports and events
- Predict outcomes and compete
- Share emotions through photos/videos
- Interact in real-time
- Discover trending content via AI

---

## 📁 Project Structure

```
predicta/
├── backend/                 # Rust backend (Axum + PostgreSQL + Redis)
│   ├── src/
│   │   ├── main.rs         # Application entry point
│   │   ├── config.rs       # Configuration management
│   │   ├── db/             # Database layer (SQLx)
│   │   │   ├── mod.rs      # DB connection & migrations
│   │   │   ├── models.rs   # Data models
│   │   │   └── repositories.rs
│   │   ├── handlers/       # HTTP request handlers
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs     # Authentication endpoints
│   │   │   ├── predictions.rs
│   │   │   ├── events.rs
│   │   │   ├── feed.rs
│   │   │   ├── posts.rs
│   │   │   ├── live.rs
│   │   │   ├── users.rs
│   │   │   ├── gamification.rs
│   │   │   └── admin.rs
│   │   ├── services/       # Business logic
│   │   │   ├── mod.rs
│   │   │   ├── ai.rs       # AI recommendations
│   │   │   ├── gamification.rs
│   │   │   └── notifications.rs
│   │   ├── websocket/      # Real-time features
│   │   │   └── mod.rs
│   │   ├── middleware/     # Custom middleware
│   │   │   ├── mod.rs
│   │   │   └── auth.rs
│   │   └── utils/          # Utilities
│   │       ├── mod.rs
│   │       └── errors.rs
│   ├── Cargo.toml
│   └── README.md
│
├── mobile/                  # React Native mobile app (Expo)
│   ├── app/                # Expo Router screens
│   │   ├── _layout.tsx
│   │   ├── index.tsx
│   │   ├── predictions/
│   │   ├── events/
│   │   ├── feed/
│   │   ├── live/
│   │   ├── profile/
│   │   └── create/
│   ├── components/         # Reusable components
│   ├── hooks/              # Custom React hooks
│   ├── services/           # API services
│   ├── store/              # Zustand state management
│   ├── types/              # TypeScript types
│   ├── utils/              # Utility functions
│   ├── package.json
│   ├── tsconfig.json
│   ├── app.json
│   └── README.md
│
└── infra/                  # Infrastructure configs
    ├── docker-compose.yml  # Local development
    └── k8s/                # Kubernetes manifests
```

---

## 🛠️ Tech Stack

### Backend (Rust)
| Component | Technology |
|-----------|------------|
| Language | Rust |
| Web Framework | Axum |
| Database | PostgreSQL (with SQLx) |
| Cache | Redis |
| Real-time | WebSockets (Axum) |
| Authentication | JWT + Argon2 |
| Logging | tracing + tracing-subscriber |

### Mobile (JavaScript/TypeScript)
| Component | Technology |
|-----------|------------|
| Framework | React Native (Expo) |
| Navigation | Expo Router |
| State Management | Zustand |
| Animations | React Native Reanimated 3 |
| Gestures | React Native Gesture Handler |
| Video | expo-av |
| Real-time | Socket.IO Client |
| HTTP | Axios |

---

## 🚀 Getting Started

### Backend Setup

1. **Install Rust** (1.75+):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Set up PostgreSQL**:
   ```bash
   docker run -d --name predicta-db \
     -e POSTGRES_USER=predicta \
     -e POSTGRES_PASSWORD=password \
     -e POSTGRES_DB=predicta \
     -p 5432:5432 postgres:15
   ```

3. **Set up Redis**:
   ```bash
   docker run -d --name predicta-redis \
     -p 6379:6379 redis:7
   ```

4. **Configure environment**:
   ```bash
   cd backend
   cat > .env << EOF
   DATABASE_URL=postgresql://predicta:password@localhost:5432/predicta
   REDIS_URL=redis://localhost:6379
   JWT_SECRET=your-super-secret-key-change-in-production
   PORT=8080
   EOF
   ```

5. **Run the backend**:
   ```bash
   cargo run
   ```

### Mobile Setup

1. **Install Node.js** (18+):
   ```bash
   nvm install 18
   ```

2. **Install dependencies**:
   ```bash
   cd mobile
   npm install
   ```

3. **Start development server**:
   ```bash
   npm start
   ```

4. **Run on device**:
   - iOS: `npm run ios`
   - Android: `npm run android`

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
- `GET /api/v1/predictions/:id` - Get prediction
- `GET /api/v1/predictions/events/:event_id` - Get event predictions

### Events
- `GET /api/v1/events` - List events
- `POST /api/v1/events` - Create event
- `GET /api/v1/events/:id` - Get event details
- `POST /api/v1/events/:id/join` - Join event
- `GET /api/v1/events/:id/room` - Get event room

### Feed & Posts
- `GET /api/v1/feed` - Get personalized feed
- `GET /api/v1/feed/trending` - Get trending posts
- `GET /api/v1/feed/recommendations` - Get AI recommendations
- `GET /api/v1/posts` - List posts
- `POST /api/v1/posts` - Create post
- `POST /api/v1/posts/:id/like` - Like post
- `POST /api/v1/posts/:id/comment` - Add comment

### Live Streaming
- `GET /api/v1/live` - List active streams
- `POST /api/v1/live/start` - Start stream
- `POST /api/v1/live/end` - End stream

### Gamification
- `GET /api/v1/gamification/leaderboard` - Get leaderboard
- `GET /api/v1/gamification/points` - Get user points
- `GET /api/v1/gamification/badges` - Get user badges
- `GET /api/v1/gamification/achievements` - Get all achievements

### Admin
- `GET /api/v1/admin/users` - List all users
- `GET /api/v1/admin/content` - Moderate content
- `GET /api/v1/admin/analytics` - Get analytics

### WebSocket
- `GET /ws` - Real-time connection for chat, reactions, updates

---

## 🎮 Key Features

### ⚽ Sports Prediction System
- Match winner predictions (Team A / Draw / Team B)
- Exact score predictions
- Player event predictions (first goal, assists, etc.)
- AI-based prediction suggestions
- Points awarded based on accuracy
- Bonus for exact predictions

### 🏆 Gamification System
- Points system (predictions, engagement, referrals)
- Leaderboards (Global, Friends, Country)
- Badges (Top Predictor, Event King, Content Creator)
- Streak rewards (daily predictions)
- Levels & achievements

### 🎵 Events & Social Experience
- Event discovery by location & category
- "Attending", "Interested", "Check-in" statuses
- Real-time chat during events
- Emoji reactions (🔥😱🎉)
- Photo/video uploads with captions
- 24-hour disappearing stories
- Nearby fans check-ins

### 🎥 Video-First Social Feed
- TikTok-style vertical scrolling
- Fullscreen videos with auto-play + loop
- Double-tap like animation
- Gesture navigation (swipe)
- Real-time feed updates

### 🤖 AI Recommendation Engine
- Personalized feed based on watch time, likes, comments
- Trending posts detection (last 24h)
- Content ranking algorithm:
  ```
  Score = (likes × 3) + (comments × 5) + (watch_time × 2)
  ```
- Future-ready: Collaborative filtering, behavior-based learning

### 🔴 Live Streaming
- User-generated live streams
- Real-time video (WebRTC/Agora/LiveKit integration ready)
- Live chat during streams
- Viewer count display
- Real-time reactions

### 💬 Real-Time Features
- WebSocket connections via Socket.IO
- Live chat (matches, events, streams)
- Real-time feed updates
- Live notifications
- Presence system (online users)

---

## 📊 Database Schema

Key tables:
- `users` - User accounts with points & levels
- `events` - Sports matches, concerts, festivals
- `predictions` - User predictions with results
- `posts` - Social feed content (videos, images)
- `comments` - Post comments
- `live_streams` - Active/past live streams
- `badges` & `user_badges` - Achievement system
- `notifications` - In-app notifications
- `likes` - Post/comment likes
- `event_attendees` - Event participation
- `analytics` - Metrics tracking

---

## 🔐 Security Features

- JWT authentication with refresh tokens
- Argon2 password hashing
- Role-based access control (user/admin)
- Rate limiting
- XSS protection
- Input validation
- CORS configuration

---

## 📈 Scalability & Performance

- **Connection pooling** for database (20 max connections)
- **Redis caching** for frequently accessed data
- **CDN delivery** for media content
- **Adaptive streaming** (HLS) for videos
- **Preloading & lazy loading** for smooth UX
- **Background jobs** via BullMQ (future)

---

## 💰 Monetization (Future)

- Native feed ads (every 5 posts)
- Sponsored posts
- Event promotions
- Premium subscription (advanced stats, ad-free)
- Revenue tracking (impressions, clicks, conversions)

---

## 🎁 Growth Hacking

- Viral loop (share predictions → invite friends → rewards)
- Referral system (earn points/premium)
- Push notifications for retention
- FOMO triggers ("2,000 users watching now")
- Social sharing (WhatsApp, Instagram)

---

## 📄 License

MIT License

---

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

---

Built with ❤️ using Rust and React Native
