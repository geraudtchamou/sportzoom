import { io, Socket } from 'socket.io-client';
import { config } from '@/constants/config';
import type { Post, Comment, Notification, LiveStream } from '@/types';

type WebSocketEventMap = {
  // Connection events
  connect: () => void;
  disconnect: (reason: string) => void;
  
  // Chat events
  'chat:message': (data: { roomId: string; message: string; userId: string; timestamp: string }) => void;
  'chat:user_joined': (data: { roomId: string; userId: string; username: string }) => void;
  'chat:user_left': (data: { roomId: string; userId: string }) => void;
  
  // Post events
  'post:like': (data: { postId: string; userId: string; count: number }) => void;
  'post:comment': (data: { postId: string; comment: Comment }) => void;
  'post:new': (data: { post: Post }) => void;
  
  // Live stream events
  'live:started': (data: { stream: LiveStream }) => void;
  'live:ended': (data: { streamId: string }) => void;
  'live:viewers_update': (data: { streamId: string; viewersCount: number }) => void;
  'live:reaction': (data: { streamId: string; emoji: string; userId: string }) => void;
  
  // Event events
  'event:start': (data: { eventId: string }) => void;
  'event:end': (data: { eventId: string }) => void;
  'event:prediction_result': (data: { eventId: string; predictions: any[] }) => void;
  
  // Notification events
  'notification:new': (data: { notification: Notification }) => void;
  
  // Presence events
  'presence:online': (data: { userId: string }) => void;
  'presence:offline': (data: { userId: string }) => void;
};

class WebSocketService {
  private socket: Socket | null = null;
  private static instance: WebSocketService;
  private eventListeners: Map<string, Set<Function>> = new Map();

  private constructor() {}

  public static getInstance(): WebSocketService {
    if (!WebSocketService.instance) {
      WebSocketService.instance = new WebSocketService();
    }
    return WebSocketService.instance;
  }

  public connect(token: string): Promise<void> {
    return new Promise((resolve, reject) => {
      try {
        this.socket = io(config.wsUrl, {
          auth: { token },
          transports: ['websocket', 'polling'],
          reconnection: true,
          reconnectionAttempts: 5,
          reconnectionDelay: 1000,
          timeout: 20000,
        });

        this.socket.on('connect', () => {
          console.log('WebSocket connected');
          this.emit('connect');
          resolve();
        });

        this.socket.on('disconnect', (reason: string) => {
          console.log('WebSocket disconnected:', reason);
          this.emit('disconnect', reason);
        });

        this.socket.on('connect_error', (error) => {
          console.error('WebSocket connection error:', error);
          reject(error);
        });

        // Register default event listeners
        this.registerDefaultListeners();
      } catch (error) {
        reject(error);
      }
    });
  }

  public disconnect(): void {
    if (this.socket) {
      this.socket.disconnect();
      this.socket = null;
      this.eventListeners.clear();
    }
  }

  public isConnected(): boolean {
    return this.socket?.connected ?? false;
  }

  private registerDefaultListeners() {
    if (!this.socket) return;

    const events: (keyof WebSocketEventMap)[] = [
      'chat:message',
      'chat:user_joined',
      'chat:user_left',
      'post:like',
      'post:comment',
      'post:new',
      'live:started',
      'live:ended',
      'live:viewers_update',
      'live:reaction',
      'event:start',
      'event:end',
      'event:prediction_result',
      'notification:new',
      'presence:online',
      'presence:offline',
    ];

    events.forEach((event) => {
      this.socket!.on(event, (data: any) => {
        this.emit(event, data);
      });
    });
  }

  public on<K extends keyof WebSocketEventMap>(event: K, callback: WebSocketEventMap[K]): void {
    if (!this.eventListeners.has(event)) {
      this.eventListeners.set(event, new Set());
    }
    this.eventListeners.get(event)!.add(callback as Function);
  }

  public off<K extends keyof WebSocketEventMap>(event: K, callback: WebSocketEventMap[K]): void {
    const listeners = this.eventListeners.get(event);
    if (listeners) {
      listeners.delete(callback as Function);
    }
  }

  private emit<K extends keyof WebSocketEventMap>(event: K, ...args: Parameters<WebSocketEventMap[K]>): void {
    const listeners = this.eventListeners.get(event);
    if (listeners) {
      listeners.forEach((callback) => {
        try {
          callback(...args);
        } catch (error) {
          console.error(`Error in WebSocket event listener for ${event}:`, error);
        }
      });
    }
  }

  // Chat methods
  public joinRoom(roomId: string): void {
    this.socket?.emit('chat:join', { roomId });
  }

  public leaveRoom(roomId: string): void {
    this.socket?.emit('chat:leave', { roomId });
  }

  public sendMessage(roomId: string, message: string): void {
    this.socket?.emit('chat:message', { roomId, message });
  }

  public sendReaction(roomId: string, emoji: string): void {
    this.socket?.emit('chat:reaction', { roomId, emoji });
  }

  // Post methods
  public likePost(postId: string): void {
    this.socket?.emit('post:like', { postId });
  }

  // Live stream methods
  public watchStream(streamId: string): void {
    this.socket?.emit('live:watch', { streamId });
  }

  public stopWatching(streamId: string): void {
    this.socket?.emit('live:stop_watching', { streamId });
  }

  public sendLiveReaction(streamId: string, emoji: string): void {
    this.socket?.emit('live:reaction', { streamId, emoji });
  }

  // Event methods
  public joinEvent(eventId: string): void {
    this.socket?.emit('event:join', { eventId });
  }

  public leaveEvent(eventId: string): void {
    this.socket?.emit('event:leave', { eventId });
  }

  // Presence
  public updatePresence(status: 'online' | 'away' | 'busy'): void {
    this.socket?.emit('presence:update', { status });
  }
}

export const webSocketService = WebSocketService.getInstance();
export default webSocketService;
