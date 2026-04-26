# SportZoom - Mobile App

The official React Native (Expo) mobile application for SportZoom - The Social Network for Live Experiences.

## 🎯 Features

- **Video-First Feed**: TikTok-style vertical scrolling video feed
- **Sports Predictions**: Predict match outcomes and compete with fans
- **Live Events**: Discover and join live sports, concerts, and festivals
- **Social Content**: Share photos and videos with emotions
- **Gamification**: Earn points, badges, and climb leaderboards
- **Real-Time Updates**: WebSocket-powered live chat and reactions
- **Dark Mode UI**: Modern, sleek dark theme design

## 🛠️ Tech Stack

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
| Storage | AsyncStorage |

## 🚀 Getting Started

### Prerequisites

- Node.js 18+ 
- npm or yarn
- Expo CLI
- iOS Simulator (Mac) or Android Emulator

### Installation

1. **Install dependencies**:
   ```bash
   npm install
   ```

2. **Configure environment**:
   Create a `.env` file in the root directory:
   ```env
   EXPO_PUBLIC_API_URL=http://localhost:8080/api/v1
   EXPO_PUBLIC_WS_URL=ws://localhost:8080/ws
   ```

3. **Start development server**:
   ```bash
   npm start
   ```

4. **Run on device**:
   - iOS: `npm run ios`
   - Android: `npm run android`
   - Web: `npm run web`

## 📁 Project Structure

```
mobile/
├── app/                    # Expo Router screens
│   ├── _layout.tsx        # Root layout
│   ├── index.tsx          # Landing page
│   └── (tabs)/            # Tab navigation
│       ├── _layout.tsx    # Tabs layout
│       ├── index.tsx      # Home screen
│       ├── feed.tsx       # Video feed
│       ├── create.tsx     # Create post
│       └── profile.tsx    # User profile
├── components/             # Reusable components
│   ├── ui/                # Base UI components
│   │   ├── Button.tsx
│   │   ├── Card.tsx
│   │   └── IconButton.tsx
│   └── feed/              # Feed components
│       └── VideoFeed.tsx
├── constants/              # App constants
│   ├── theme.ts           # Design system
│   └── config.ts          # Configuration
├── hooks/                  # Custom React hooks
├── services/               # API & WebSocket
│   ├── api.ts             # REST API client
│   └── websocket.ts       # WebSocket service
├── store/                  # Zustand stores
│   └── index.ts           # Auth, notifications, gamification
├── types/                  # TypeScript types
│   └── index.ts
└── utils/                  # Utility functions
```

## 🎨 Design System

### Colors
- **Primary**: `#6C5CE7` (Purple)
- **Secondary**: `#00D1FF` (Cyan)
- **Accent**: `#FF3B5C` (Red)
- **Background**: `#0F0F14`
- **Surface**: `#1A1A22`
- **Card**: `#22232B`

### Typography
- **Heading XL**: 32px bold
- **Heading L**: 24px semi-bold
- **Heading M**: 20px medium
- **Body**: 16px regular
- **Caption**: 12px

### Spacing
Based on 8pt grid: 4, 8, 16, 24, 32

## 📱 Screens

### 1. Home (`/(tabs)/index.tsx`)
- Trending events
- Live matches
- Quick actions
- Event discovery

### 2. Feed (`/(tabs)/feed.tsx`)
- Full-screen vertical video feed
- Auto-play videos
- Double-tap to like
- Like, comment, share actions
- Real-time view counts

### 3. Create (`/(tabs)/create.tsx`)
- Camera/Gallery media picker
- Caption input
- Emotion selector
- Post creation

### 4. Profile (`/(tabs)/profile.tsx`)
- User info & avatar
- Points & level progress
- Badges showcase
- Settings menu

## 🔌 API Integration

The app connects to the Rust backend via:

- **REST API**: Axios client with JWT authentication
- **WebSocket**: Socket.IO for real-time features
- **Auto token refresh**: Seamless session management

## 🎬 Animations

- Smooth page transitions
- Button press scale effects
- Double-tap heart animation
- Pull-to-refresh
- Skeleton loading states

## 📦 Available Scripts

```bash
npm start          # Start Expo dev server
npm run ios        # Run on iOS simulator
npm run android    # Run on Android emulator
npm run web        # Run in browser
npm run lint       # Run ESLint
npm run type-check # Run TypeScript check
```

## 🔐 Security

- JWT token storage in AsyncStorage
- Automatic token refresh
- Secure API communication
- Input validation
- Permission handling

## 🧪 Testing

```bash
# Run tests (when configured)
npm test
```

## 📤 Building for Production

### iOS
```bash
eas build --platform ios
```

### Android
```bash
eas build --platform android
```

## 🌐 Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `EXPO_PUBLIC_API_URL` | Backend API URL | `http://localhost:8080/api/v1` |
| `EXPO_PUBLIC_WS_URL` | WebSocket URL | `ws://localhost:8080/ws` |

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## 📄 License

MIT License

---

Built with ❤️ using React Native, Expo, and TypeScript
