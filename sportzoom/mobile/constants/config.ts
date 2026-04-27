import { Platform } from 'react-native';

const API_BASE_URL = process.env.EXPO_PUBLIC_API_URL || 'http://localhost:8080/api/v1';
const WS_URL = process.env.EXPO_PUBLIC_WS_URL || 'ws://localhost:8080/ws';

export const config = {
  apiBaseUrl: API_BASE_URL,
  wsUrl: WS_URL,
  timeout: 30000,
  retryAttempts: 3,
};

export const endpoints = {
  // Auth
  auth: {
    register: '/auth/register',
    login: '/auth/login',
    logout: '/auth/logout',
    refresh: '/auth/refresh',
  },
  
  // Users
  users: {
    profile: '/users/profile',
    byId: (id: string) => `/users/${id}`,
    update: '/users/profile',
  },
  
  // Posts & Feed
  posts: {
    list: '/posts',
    create: '/posts',
    like: (id: string) => `/posts/${id}/like`,
    comment: (id: string) => `/posts/${id}/comment`,
    share: (id: string) => `/posts/${id}/share`,
  },
  feed: {
    home: '/feed',
    trending: '/feed/trending',
    recommendations: '/feed/recommendations',
  },
  
  // Events
  events: {
    list: '/events',
    create: '/events',
    byId: (id: string) => `/events/${id}`,
    join: (id: string) => `/events/${id}/join`,
    room: (id: string) => `/events/${id}/room`,
  },
  
  // Predictions
  predictions: {
    list: '/predictions',
    create: '/predictions',
    byId: (id: string) => `/predictions/${id}`,
    byEvent: (eventId: string) => `/predictions/events/${eventId}`,
  },
  
  // Live Streaming
  live: {
    list: '/live',
    start: '/live/start',
    end: '/live/end',
  },
  
  // Gamification
  gamification: {
    leaderboard: '/gamification/leaderboard',
    points: '/gamification/points',
    badges: '/gamification/badges',
    achievements: '/gamification/achievements',
  },
  
  // Admin
  admin: {
    users: '/admin/users',
    content: '/admin/content',
    analytics: '/admin/analytics',
  },
};

export const storageKeys = {
  accessToken: '@sportzoom:access_token',
  refreshToken: '@sportzoom:refresh_token',
  user: '@sportzoom:user',
  theme: '@sportzoom:theme',
};

export const isIOS = Platform.OS === 'ios';
export const isAndroid = Platform.OS === 'android';
export const isWeb = Platform.OS === 'web';

export const videoConfig = {
  maxDuration: 60, // seconds
  maxFileSize: 50 * 1024 * 1024, // 50MB
  allowedFormats: ['mp4', 'mov', 'avi'],
};

export const imageConfig = {
  maxFileSize: 10 * 1024 * 1024, // 10MB
  allowedFormats: ['jpg', 'jpeg', 'png', 'webp'],
  maxWidth: 1920,
  maxHeight: 1920,
};

export const pagination = {
  defaultLimit: 20,
  maxLimit: 100,
};

export const emotions = [
  { type: 'fire' as const, emoji: '🔥', label: 'Fire' },
  { type: 'love' as const, emoji: '😍', label: 'Love' },
  { type: 'shock' as const, emoji: '😱', label: 'Shock' },
  { type: 'happy' as const, emoji: '🎉', label: 'Happy' },
  { type: 'sad' as const, emoji: '😢', label: 'Sad' },
];
