// TypeScript Types for SportZoom

export interface User {
  id: string;
  username: string;
  email: string;
  avatar?: string;
  points: number;
  level: number;
  badges: Badge[];
  createdAt: string;
}

export interface Badge {
  id: string;
  name: string;
  description: string;
  icon: string;
  earnedAt: string;
}

export interface Event {
  id: string;
  title: string;
  description: string;
  type: 'sports' | 'concert' | 'festival' | 'local';
  startDate: string;
  endDate: string;
  location: Location;
  imageUrl?: string;
  attendeesCount: number;
  isLive: boolean;
  category: string;
}

export interface Location {
  lat: number;
  lng: number;
  address: string;
  city: string;
  country: string;
}

export interface Prediction {
  id: string;
  eventId: string;
  userId: string;
  type: 'winner' | 'score' | 'player_event';
  prediction: PredictionData;
  points: number;
  isCorrect?: boolean;
  createdAt: string;
}

export interface PredictionData {
  teamA?: string;
  teamB?: string;
  draw?: boolean;
  scoreA?: number;
  scoreB?: number;
  playerId?: string;
  eventType?: 'goal' | 'assist' | 'card';
}

export interface Post {
  id: string;
  userId: string;
  user: User;
  type: 'video' | 'image';
  mediaUrl: string;
  thumbnailUrl?: string;
  caption: string;
  emotions: Emotion[];
  likesCount: number;
  commentsCount: number;
  sharesCount: number;
  viewsCount: number;
  isLiked: boolean;
  eventId?: string;
  event?: Event;
  createdAt: string;
  duration?: number;
}

export interface Emotion {
  type: 'fire' | 'love' | 'shock' | 'happy' | 'sad';
  count: number;
}

export interface Comment {
  id: string;
  postId: string;
  userId: string;
  user: User;
  content: string;
  likesCount: number;
  createdAt: string;
}

export interface LiveStream {
  id: string;
  userId: string;
  user: User;
  title: string;
  viewersCount: number;
  isLive: boolean;
  thumbnailUrl?: string;
  startedAt: string;
  eventId?: string;
}

export interface LeaderboardEntry {
  rank: number;
  user: User;
  points: number;
  streak: number;
}

export interface Notification {
  id: string;
  userId: string;
  type: 'like' | 'comment' | 'follow' | 'prediction' | 'event';
  title: string;
  message: string;
  data?: any;
  isRead: boolean;
  createdAt: string;
}

export interface FeedConfig {
  page: number;
  limit: number;
  filter?: 'trending' | 'following' | 'nearby';
}

export interface ApiResponse<T> {
  data: T;
  message?: string;
  status: 'success' | 'error';
}

export interface AuthTokens {
  accessToken: string;
  refreshToken: string;
  expiresIn: number;
}

export interface LoginCredentials {
  email: string;
  password: string;
}

export interface RegisterData {
  username: string;
  email: string;
  password: string;
}
