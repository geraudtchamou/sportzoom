import React, { useState, useCallback } from 'react';
import { View, Text, StyleSheet, Dimensions } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { colors, typography, spacing } from '@/constants/theme';
import { Feed } from '@/components/feed/VideoFeed';
import type { Post } from '@/types';

const SCREEN_HEIGHT = Dimensions.get('window').height;

// Mock posts for demonstration
const mockPosts: Post[] = [
  {
    id: '1',
    userId: 'user1',
    user: {
      id: 'user1',
      username: 'sportsfan_mike',
      email: 'mike@example.com',
      points: 2500,
      level: 3,
      badges: [],
      createdAt: new Date().toISOString(),
    },
    type: 'video',
    mediaUrl: 'https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ForBiggerBlazes.mp4',
    thumbnailUrl: '',
    caption: 'Amazing goal! 🔥 What a match! #football #sports',
    emotions: [{ type: 'fire', count: 234 }, { type: 'love', count: 89 }],
    likesCount: 1240,
    commentsCount: 87,
    sharesCount: 45,
    viewsCount: 15420,
    isLiked: false,
    createdAt: new Date().toISOString(),
    duration: 15,
  },
  {
    id: '2',
    userId: 'user2',
    user: {
      id: 'user2',
      username: 'basketball_queen',
      email: 'queen@example.com',
      points: 3200,
      level: 4,
      badges: [],
      createdAt: new Date().toISOString(),
    },
    type: 'video',
    mediaUrl: 'https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ForBiggerEscapes.mp4',
    thumbnailUrl: '',
    caption: 'Last second buzzer beater! 🏀 Unbelievable! #basketball #nba',
    emotions: [{ type: 'shock', count: 567 }, { type: 'fire', count: 432 }],
    likesCount: 2890,
    commentsCount: 156,
    sharesCount: 234,
    viewsCount: 45230,
    isLiked: false,
    createdAt: new Date().toISOString(),
    duration: 20,
  },
  {
    id: '3',
    userId: 'user3',
    user: {
      id: 'user3',
      username: 'tennis_pro',
      email: 'pro@example.com',
      points: 1800,
      level: 2,
      badges: [],
      createdAt: new Date().toISOString(),
    },
    type: 'video',
    mediaUrl: 'https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ForBiggerFun.mp4',
    thumbnailUrl: '',
    caption: 'Epic rally at Wimbledon! 🎾 #tennis #wimbledon',
    emotions: [{ type: 'happy', count: 345 }],
    likesCount: 987,
    commentsCount: 54,
    sharesCount: 32,
    viewsCount: 8760,
    isLiked: false,
    createdAt: new Date().toISOString(),
    duration: 25,
  },
];

export default function FeedScreen() {
  const [posts, setPosts] = useState<Post[]>(mockPosts);
  const [loading, setLoading] = useState(false);

  const handleLike = useCallback((postId: string) => {
    console.log('Liked post:', postId);
    // Update local state
    setPosts(currentPosts =>
      currentPosts.map(post =>
        post.id === postId
          ? {
              ...post,
              isLiked: !post.isLiked,
              likesCount: post.isLiked ? post.likesCount - 1 : post.likesCount + 1,
            }
          : post
      )
    );
    
    // TODO: Call API to like post
  }, []);

  const handleComment = useCallback((postId: string) => {
    console.log('Comment on post:', postId);
    // TODO: Navigate to comments screen or open modal
  }, []);

  const handleShare = useCallback((postId: string) => {
    console.log('Share post:', postId);
    // TODO: Open share sheet
  }, []);

  const handleDoubleTap = useCallback((postId: string) => {
    console.log('Double tap on post:', postId);
    // Auto-like on double tap
    handleLike(postId);
  }, [handleLike]);

  const handleEndReached = useCallback(() => {
    if (loading) return;
    
    setLoading(true);
    // Simulate loading more posts
    setTimeout(() => {
      setLoading(false);
    }, 1000);
  }, [loading]);

  return (
    <SafeAreaView style={styles.container} edges={['top', 'bottom']}>
      <View style={styles.header}>
        <Text style={styles.title}>For You</Text>
        <View style={styles.tabs}>
          <Text style={[styles.tab, styles.activeTab]}>For You</Text>
          <Text style={styles.tab}>Following</Text>
          <Text style={styles.tab}>Nearby</Text>
        </View>
      </View>
      
      <Feed
        posts={posts}
        onLike={handleLike}
        onComment={handleComment}
        onShare={handleShare}
        onDoubleTap={handleDoubleTap}
        onEndReached={handleEndReached}
      />
      
      {loading && (
        <View style={styles.loadingContainer}>
          <Text style={styles.loadingText}>Loading more...</Text>
        </View>
      )}
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
  },
  header: {
    position: 'absolute',
    top: 0,
    left: 0,
    right: 0,
    zIndex: 100,
    paddingTop: spacing.m,
    paddingHorizontal: spacing.m,
  },
  title: {
    ...typography.headingM,
    color: colors.textPrimary,
    marginBottom: spacing.s,
  },
  tabs: {
    flexDirection: 'row',
    gap: spacing.l,
  },
  tab: {
    ...typography.body,
    color: colors.textSecondary,
    paddingBottom: spacing.s,
    borderBottomWidth: 2,
    borderBottomColor: 'transparent',
  },
  activeTab: {
    color: colors.textPrimary,
    borderBottomColor: colors.primary,
    fontWeight: '600',
  },
  loadingContainer: {
    position: 'absolute',
    bottom: spacing.xl,
    left: 0,
    right: 0,
    alignItems: 'center',
  },
  loadingText: {
    ...typography.caption,
    color: colors.textSecondary,
  },
});
