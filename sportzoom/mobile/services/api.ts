import AsyncStorage from '@react-native-async-storage/async-storage';
import axios, { AxiosInstance, AxiosError, InternalAxiosRequestConfig } from 'axios';
import { config, endpoints, storageKeys } from '@/constants/config';
import type { ApiResponse, AuthTokens } from '@/types';

class ApiClient {
  private client: AxiosInstance;
  private static instance: ApiClient;

  private constructor() {
    this.client = axios.create({
      baseURL: config.apiBaseUrl,
      timeout: config.timeout,
      headers: {
        'Content-Type': 'application/json',
      },
    });

    this.setupInterceptors();
  }

  public static getInstance(): ApiClient {
    if (!ApiClient.instance) {
      ApiClient.instance = new ApiClient();
    }
    return ApiClient.instance;
  }

  private setupInterceptors() {
    // Request interceptor - add auth token
    this.client.interceptors.request.use(
      async (request: InternalAxiosRequestConfig) => {
        const token = await this.getAccessToken();
        if (token) {
          request.headers.Authorization = `Bearer ${token}`;
        }
        return request;
      },
      (error) => Promise.reject(error)
    );

    // Response interceptor - handle token refresh
    this.client.interceptors.response.use(
      (response) => response,
      async (error: AxiosError) => {
        const originalRequest = error.config as InternalAxiosRequestConfig & { _retry?: boolean };

        if (error.response?.status === 401 && !originalRequest._retry) {
          originalRequest._retry = true;

          try {
            const newTokens = await this.refreshToken();
            if (newTokens) {
              originalRequest.headers.Authorization = `Bearer ${newTokens.accessToken}`;
              return this.client(originalRequest);
            }
          } catch (refreshError) {
            // Refresh failed, logout user
            await this.logout();
            return Promise.reject(refreshError);
          }
        }

        return Promise.reject(error);
      }
    );
  }

  private async getAccessToken(): Promise<string | null> {
    try {
      return await AsyncStorage.getItem(storageKeys.accessToken);
    } catch {
      return null;
    }
  }

  private async refreshToken(): Promise<AuthTokens | null> {
    try {
      const refreshToken = await AsyncStorage.getItem(storageKeys.refreshToken);
      if (!refreshToken) {
        return null;
      }

      const response = await axios.post<ApiResponse<AuthTokens>>(
        `${config.apiBaseUrl}${endpoints.auth.refresh}`,
        { refreshToken }
      );

      if (response.data.status === 'success') {
        await this.saveTokens(response.data.data);
        return response.data.data;
      }

      return null;
    } catch {
      return null;
    }
  }

  private async saveTokens(tokens: AuthTokens): Promise<void> {
    try {
      await AsyncStorage.multiSet([
        [storageKeys.accessToken, tokens.accessToken],
        [storageKeys.refreshToken, tokens.refreshToken],
      ]);
    } catch (error) {
      console.error('Failed to save tokens:', error);
    }
  }

  public async login(email: string, password: string): Promise<AuthTokens> {
    try {
      const response = await this.client.post<ApiResponse<AuthTokens>>(
        endpoints.auth.login,
        { email, password }
      );

      if (response.data.status === 'success' && response.data.data) {
        await this.saveTokens(response.data.data);
        return response.data.data;
      }

      throw new Error('Login failed');
    } catch (error) {
      this.handleError(error);
      throw error;
    }
  }

  public async register(username: string, email: string, password: string): Promise<AuthTokens> {
    try {
      const response = await this.client.post<ApiResponse<AuthTokens>>(
        endpoints.auth.register,
        { username, email, password }
      );

      if (response.data.status === 'success' && response.data.data) {
        await this.saveTokens(response.data.data);
        return response.data.data;
      }

      throw new Error('Registration failed');
    } catch (error) {
      this.handleError(error);
      throw error;
    }
  }

  public async logout(): Promise<void> {
    try {
      await this.client.post(endpoints.auth.logout);
    } catch {
      // Ignore errors on logout
    } finally {
      await this.clearTokens();
    }
  }

  private async clearTokens(): Promise<void> {
    try {
      await AsyncStorage.multiRemove([
        storageKeys.accessToken,
        storageKeys.refreshToken,
        storageKeys.user,
      ]);
    } catch (error) {
      console.error('Failed to clear tokens:', error);
    }
  }

  public getClient(): AxiosInstance {
    return this.client;
  }

  private handleError(error: unknown): void {
    if (axios.isAxiosError(error)) {
      const message = error.response?.data 
        ? (error.response.data as any).message || 'An error occurred'
        : error.message || 'Network error';
      console.error('API Error:', message);
    } else {
      console.error('Unknown error:', error);
    }
  }
}

export const apiClient = ApiClient.getInstance();
export default apiClient;
