// API types

export interface User {
  id: string;
  username: string;
  email: string;
  display_name?: string;
  avatar_url?: string;
  bio?: string;
  country?: string;
  points: number;
  level: number;
  is_verified: boolean;
}

export interface Event {
  id: string;
  title: string;
  description?: string;
  event_type: string;
  category: string;
  location?: string;
  latitude?: number;
  longitude?: number;
  start_time: string;
  end_time?: string;
  organizer_id: string;
  attendee_count: number;
  is_live: boolean;
  status: 'upcoming' | 'live' | 'completed' | 'cancelled';
  created_at: string;
}

export interface Prediction {
  id: string;
  user_id: string;
  event_id: string;
  prediction_type: string;
  predicted_outcome: string;
  points_awarded: number;
  is_correct?: boolean;
  created_at: string;
}

export interface Post {
  id: string;
  user_id: string;
  username: string;
  user_avatar?: string;
  event_id?: string;
  content_type: 'video' | 'image' | 'text';
  media_url: string;
  thumbnail_url?: string;
  caption?: string;
  duration_seconds?: number;
  like_count: number;
  comment_count: number;
  share_count: number;
  view_count: number;
  is_liked: boolean;
  created_at: string;
}

export interface LiveStream {
  id: string;
  user_id: string;
  title: string;
  description?: string;
  stream_url: string;
  viewer_count: number;
  like_count: number;
  is_active: boolean;
  started_at: string;
}

export interface LeaderboardEntry {
  rank: number;
  user_id: string;
  username: string;
  avatar_url?: string;
  points: number;
  level: number;
  country?: string;
}

export interface Badge {
  id: string;
  name: string;
  description: string;
  icon_url: string;
  criteria: string;
}

export interface Notification {
  id: string;
  user_id: string;
  notification_type: string;
  title: string;
  message: string;
  data?: Record<string, any>;
  is_read: boolean;
  created_at: string;
}

export interface AuthResponse {
  user: User;
  access_token: string;
  refresh_token: string;
}
