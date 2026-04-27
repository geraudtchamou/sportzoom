# Predicta - Mobile App (React Native + Expo)

A modern, scalable mobile application for real-time fan engagement and social experiences.

## Tech Stack

- **Framework**: React Native with Expo
- **Navigation**: React Navigation 6
- **State Management**: Zustand
- **Animations**: React Native Reanimated 3
- **Gestures**: React Native Gesture Handler
- **Video**: expo-av
- **Real-time**: Socket.IO Client
- **HTTP**: Axios
- **Storage**: AsyncStorage + expo-secure-store

## Project Structure

```
mobile/
├── app/
│   ├── _layout.tsx          # Root layout with navigation
│   ├── index.tsx            # Home screen (video feed)
│   ├── predictions/         # Prediction screens
│   ├── events/              # Event screens
│   ├── feed/                # Social feed
│   ├── live/                # Live streaming
│   ├── profile/             # User profile
│   └── create/              # Content creation
├── components/
│   ├── ui/                  # Base UI components
│   ├── video/               # Video player components
│   ├── prediction/          # Prediction components
│   ├── event/               # Event components
│   └── feed/                # Feed components
├── hooks/                   # Custom React hooks
├── services/                # API services
├── store/                   # State management (Zustand)
├── utils/                   # Utility functions
├── types/                   # TypeScript types
└── assets/                  # Images, fonts, etc.
```

## Getting Started

### Prerequisites

- Node.js 18+
- npm or yarn
- Expo CLI
- iOS Simulator or Android Emulator

### Installation

```bash
cd mobile
npm install
```

### Run Development Server

```bash
npm start
```

### Run on iOS

```bash
npm run ios
```

### Run on Android

```bash
npm run android
```

## Features

### 🎥 Video-First Feed
- TikTok-style vertical scrolling
- Auto-play videos
- Double-tap to like
- Gesture-based navigation

### ⚽ Sports Predictions
- Match winner predictions
- Exact score predictions
- Player event predictions
- Real-time results

### 🎉 Events & Social
- Event discovery
- Live chat rooms
- Check-ins
- Stories (24h content)

### 🔴 Live Streaming
- Go live during events
- Real-time viewer count
- Live reactions
- Chat integration

### 🏆 Gamification
- Points system
- Leaderboards
- Badges & achievements
- Daily streaks

## Environment Variables

Create a `.env` file:

```env
API_URL=http://localhost:8080/api/v1
WS_URL=ws://localhost:8080/ws
FIREBASE_PROJECT_ID=your-project-id
```

## Build Production

```bash
# Build for iOS
npm run build:ios

# Build for Android
npm run build:android
```

## License

MIT
