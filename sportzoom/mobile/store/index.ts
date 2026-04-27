import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';
import AsyncStorage from '@react-native-async-storage/async-storage';
import type { User, AuthTokens, Notification, Badge } from '@/types';
import { apiClient } from '@/services/api';
import { webSocketService } from '@/services/websocket';

interface AuthState {
  user: User | null;
  tokens: AuthTokens | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  
  // Actions
  login: (email: string, password: string) => Promise<void>;
  register: (username: string, email: string, password: string) => Promise<void>;
  logout: () => Promise<void>;
  updateUser: (user: Partial<User>) => void;
  checkAuth: () => Promise<void>;
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set, get) => ({
      user: null,
      tokens: null,
      isAuthenticated: false,
      isLoading: true,

      login: async (email: string, password: string) => {
        try {
          set({ isLoading: true });
          const tokens = await apiClient.login(email, password);
          
          // Fetch user profile
          const client = apiClient.getClient();
          const response = await client.get('/users/profile');
          const user = response.data.data;
          
          set({ 
            tokens, 
            user, 
            isAuthenticated: true,
            isLoading: false 
          });
          
          // Connect WebSocket
          await webSocketService.connect(tokens.accessToken);
        } catch (error) {
          set({ isLoading: false });
          throw error;
        }
      },

      register: async (username: string, email: string, password: string) => {
        try {
          set({ isLoading: true });
          const tokens = await apiClient.register(username, email, password);
          
          // Fetch user profile
          const client = apiClient.getClient();
          const response = await client.get('/users/profile');
          const user = response.data.data;
          
          set({ 
            tokens, 
            user, 
            isAuthenticated: true,
            isLoading: false 
          });
          
          // Connect WebSocket
          await webSocketService.connect(tokens.accessToken);
        } catch (error) {
          set({ isLoading: false });
          throw error;
        }
      },

      logout: async () => {
        try {
          await apiClient.logout();
          webSocketService.disconnect();
        } catch {
          // Ignore errors
        } finally {
          set({ 
            user: null, 
            tokens: null, 
            isAuthenticated: false,
            isLoading: false 
          });
        }
      },

      updateUser: (userPartial: Partial<User>) => {
        const currentUser = get().user;
        if (currentUser) {
          set({ user: { ...currentUser, ...userPartial } });
        }
      },

      checkAuth: async () => {
        const state = get();
        if (state.tokens?.accessToken) {
          try {
            const client = apiClient.getClient();
            const response = await client.get('/users/profile');
            const user = response.data.data;
            
            set({ 
              user, 
              isAuthenticated: true,
              isLoading: false 
            });
            
            // Reconnect WebSocket
            await webSocketService.connect(state.tokens.accessToken);
          } catch {
            set({ 
              user: null, 
              tokens: null, 
              isAuthenticated: false,
              isLoading: false 
            });
          }
        } else {
          set({ isLoading: false });
        }
      },
    }),
    {
      name: 'auth-storage',
      storage: createJSONStorage(() => AsyncStorage),
      partialize: (state) => ({ 
        tokens: state.tokens,
        user: state.user,
        isAuthenticated: state.isAuthenticated 
      }),
    }
  )
);

interface NotificationState {
  notifications: Notification[];
  unreadCount: number;
  
  // Actions
  addNotification: (notification: Notification) => void;
  markAsRead: (id: string) => void;
  markAllAsRead: () => void;
  clearNotifications: () => void;
}

export const useNotificationStore = create<NotificationState>((set, get) => ({
  notifications: [],
  unreadCount: 0,

  addNotification: (notification: Notification) => {
    set((state) => ({
      notifications: [notification, ...state.notifications].slice(0, 50),
      unreadCount: state.unreadCount + 1,
    }));
  },

  markAsRead: (id: string) => {
    set((state) => ({
      notifications: state.notifications.map((n) =>
        n.id === id ? { ...n, isRead: true } : n
      ),
      unreadCount: Math.max(0, state.unreadCount - 1),
    }));
  },

  markAllAsRead: () => {
    set((state) => ({
      notifications: state.notifications.map((n) => ({ ...n, isRead: true })),
      unreadCount: 0,
    }));
  },

  clearNotifications: () => {
    set({ notifications: [], unreadCount: 0 });
  },
}));

interface GamificationState {
  points: number;
  level: number;
  streak: number;
  badges: Badge[];
  leaderboard: Array<{ rank: number; user: User; points: number }>;
  
  // Actions
  updatePoints: (points: number) => void;
  addBadge: (badge: Badge) => void;
  fetchLeaderboard: (type?: 'global' | 'friends' | 'country') => Promise<void>;
}

export const useGamificationStore = create<GamificationState>((set, get) => ({
  points: 0,
  level: 1,
  streak: 0,
  badges: [],
  leaderboard: [],

  updatePoints: (points: number) => {
    set((state) => ({
      points: state.points + points,
      level: Math.floor((state.points + points) / 1000) + 1,
    }));
  },

  addBadge: (badge: Badge) => {
    set((state) => ({
      badges: [...state.badges, badge],
    }));
  },

  fetchLeaderboard: async (type = 'global') => {
    try {
      const client = apiClient.getClient();
      const response = await client.get('/gamification/leaderboard', {
        params: { type },
      });
      set({ leaderboard: response.data.data });
    } catch (error) {
      console.error('Failed to fetch leaderboard:', error);
    }
  },
}));
