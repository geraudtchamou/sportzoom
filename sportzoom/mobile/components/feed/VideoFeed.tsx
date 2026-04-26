import React, { useRef, useState, useCallback, useEffect } from 'react';
import { View, Text, StyleSheet, Dimensions, TouchableOpacity, FlatList, ViewToken } from 'react-native';
import Animated, {
  useSharedValue,
  useAnimatedStyle,
  withSpring,
  withSequence,
  withTiming,
  FadeIn,
  FadeOut,
} from 'react-native-reanimated';
import { Video, ResizeMode } from 'expo-av';
import { colors, typography, spacing, borderRadius } from '@/constants/theme';
import type { Post } from '@/types';
import { LikeButton } from './IconButton';
import Card from './Card';

const { width: SCREEN_WIDTH, height: SCREEN_HEIGHT } = Dimensions.get('window');

interface VideoPostProps {
  post: Post;
  isActive: boolean;
  onLike: (postId: string) => void;
  onComment: (postId: string) => void;
  onShare: (postId: string) => void;
  onDoubleTap: (postId: string) => void;
}

export const VideoPost: React.FC<VideoPostProps> = ({
  post,
  isActive,
  onLike,
  onComment,
  onShare,
  onDoubleTap,
}) => {
  const videoRef = useRef<Video>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [showHeart, setShowHeart] = useState(false);
  const heartScale = useSharedValue(0);

  useEffect(() => {
    if (isActive) {
      videoRef.current?.playAsync();
      setIsPlaying(true);
    } else {
      videoRef.current?.pauseAsync();
      setIsPlaying(false);
      videoRef.current?.setPositionAsync(0);
    }
  }, [isActive]);

  const handleDoubleTap = useCallback(() => {
    setShowHeart(true);
    heartScale.value = withSequence(
      withSpring(1, { damping: 10, mass: 1, stiffness: 200 }),
      withTiming(0, { duration: 800 })
    );

    setTimeout(() => setShowHeart(false), 1000);
    onDoubleTap(post.id);
  }, [post.id, onDoubleTap, heartScale]);

  const togglePlayPause = useCallback(async () => {
    if (isPlaying) {
      await videoRef.current?.pauseAsync();
    } else {
      await videoRef.current?.playAsync();
    }
    setIsPlaying(!isPlaying);
  }, [isPlaying]);

  return (
    <View style={styles.container}>
      {/* Video */}
      <TouchableOpacity 
        style={styles.videoContainer} 
        onPress={togglePlayPause}
        onLongPress={handleDoubleTap}
        delayLongPress={200}
        activeOpacity={1}
      >
        <Video
          ref={videoRef}
          source={{ uri: post.mediaUrl }}
          style={styles.video}
          resizeMode={ResizeMode.COVER}
          shouldPlay={isActive}
          isLooping
          isMuted={false}
        />

        {/* Play/Pause Indicator */}
        {!isPlaying && (
          <View style={styles.playIndicator}>
            <Text style={styles.playIcon}>▶️</Text>
          </View>
        )}

        {/* Double Tap Heart Animation */}
        {showHeart && (
          <Animated.View
            entering={FadeIn.duration(100)}
            exiting={FadeOut.duration(800)}
            style={[styles.heartOverlay, { transform: [{ scale: heartScale }] }]}
          >
            <Text style={styles.heartEmoji}>❤️</Text>
          </Animated.View>
        )}
      </TouchableOpacity>

      {/* Overlay Content */}
      <View style={styles.overlay}>
        {/* Right Side Actions */}
        <View style={styles.actionsContainer}>
          <LikeButton
            count={post.likesCount}
            isLiked={post.isLiked}
            onLike={() => onLike(post.id)}
            size="xlarge"
          />
          
          <TouchableOpacity 
            style={styles.actionButton}
            onPress={() => onComment(post.id)}
          >
            <Text style={styles.actionIcon}>💬</Text>
            <Text style={styles.actionCount}>{post.commentsCount}</Text>
          </TouchableOpacity>
          
          <TouchableOpacity 
            style={styles.actionButton}
            onPress={() => onShare(post.id)}
          >
            <Text style={styles.actionIcon}>🔗</Text>
            <Text style={styles.actionCount}>{post.sharesCount}</Text>
          </TouchableOpacity>
        </View>

        {/* Bottom Info */}
        <View style={styles.infoContainer}>
          <View style={styles.userInfo}>
            <View style={styles.avatar}>
              <Text style={styles.avatarText}>
                {post.user.username.charAt(0).toUpperCase()}
              </Text>
            </View>
            <View style={styles.userInfoText}>
              <Text style={styles.username}>@{post.user.username}</Text>
              {post.event && (
                <Text style={styles.eventTag}>📍 {post.event.title}</Text>
              )}
            </View>
          </View>
          
          <Text style={styles.caption} numberOfLines={2}>
            {post.caption}
          </Text>
          
          {/* Emotions */}
          {post.emotions.length > 0 && (
            <View style={styles.emotionsContainer}>
              {post.emotions.slice(0, 3).map((emotion, index) => (
                <Text key={index} style={styles.emotion}>
                  {emotion.type === 'fire' ? '🔥' : 
                   emotion.type === 'love' ? '😍' :
                   emotion.type === 'shock' ? '😱' :
                   emotion.type === 'happy' ? '🎉' : '😢'} {emotion.count}
                </Text>
              ))}
            </View>
          )}
          
          <Text style={styles.views}>{post.viewsCount.toLocaleString()} views</Text>
        </View>
      </View>
    </View>
  );
};

interface FeedProps {
  posts: Post[];
  onLike: (postId: string) => void;
  onComment: (postId: string) => void;
  onShare: (postId: string) => void;
  onDoubleTap: (postId: string) => void;
  onEndReached?: () => void;
}

export const Feed: React.FC<FeedProps> = ({
  posts,
  onLike,
  onComment,
  onShare,
  onDoubleTap,
  onEndReached,
}) => {
  const [activePostId, setActivePostId] = useState<string | null>(posts[0]?.id || null);
  const flatListRef = useRef<FlatList>(null);

  const onViewableItemsChanged = useCallback(({ viewableItems }: { viewableItems: ViewToken[] }) => {
    if (viewableItems.length > 0) {
      const firstVisibleItem = viewableItems[0];
      if (firstVisibleItem.item && firstVisibleItem.isViewable) {
        setActivePostId(firstVisibleItem.item.id);
      }
    }
  }, []);

  const viewabilityConfig = {
    itemVisiblePercentThreshold: 50,
  };

  const renderItem = ({ item, index }: { item: Post; index: number }) => (
    <VideoPost
      post={item}
      isActive={activePostId === item.id}
      onLike={onLike}
      onComment={onComment}
      onShare={onShare}
      onDoubleTap={onDoubleTap}
    />
  );

  return (
    <FlatList
      ref={flatListRef}
      data={posts}
      renderItem={renderItem}
      keyExtractor={(item) => item.id}
      pagingEnabled
      showsVerticalScrollIndicator={false}
      snapToInterval={SCREEN_HEIGHT}
      snapToAlignment="start"
      decelerationRate="fast"
      viewabilityConfig={viewabilityConfig}
      onViewableItemsChanged={onViewableItemsChanged}
      onEndReached={onEndReached}
      onEndReachedThreshold={0.5}
      removeClippedSubviews={true}
      maxToRenderPerBatch={3}
      windowSize={5}
      initialNumToRender={2}
    />
  );
};

const styles = StyleSheet.create({
  container: {
    width: SCREEN_WIDTH,
    height: SCREEN_HEIGHT,
    backgroundColor: colors.background,
  },
  videoContainer: {
    width: SCREEN_WIDTH,
    height: SCREEN_HEIGHT,
    position: 'relative',
  },
  video: {
    width: SCREEN_WIDTH,
    height: SCREEN_HEIGHT,
  },
  playIndicator: {
    position: 'absolute',
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: 'rgba(0,0,0,0.3)',
  },
  playIcon: {
    fontSize: 64,
    opacity: 0.8,
  },
  heartOverlay: {
    position: 'absolute',
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
    alignItems: 'center',
    justifyContent: 'center',
    pointerEvents: 'none',
  },
  heartEmoji: {
    fontSize: 120,
  },
  overlay: {
    position: 'absolute',
    bottom: 0,
    left: 0,
    right: 0,
    padding: spacing.m,
    paddingBottom: spacing.xl + 20,
  },
  actionsContainer: {
    position: 'absolute',
    right: spacing.m,
    bottom: spacing.xl + 20,
    alignItems: 'center',
  },
  actionButton: {
    alignItems: 'center',
    marginTop: spacing.m,
  },
  actionIcon: {
    fontSize: 32,
  },
  actionCount: {
    ...typography.caption,
    color: colors.textPrimary,
    marginTop: 2,
  },
  infoContainer: {
    flex: 1,
    justifyContent: 'flex-end',
  },
  userInfo: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: spacing.s,
  },
  avatar: {
    width: 40,
    height: 40,
    borderRadius: 20,
    backgroundColor: colors.primary,
    alignItems: 'center',
    justifyContent: 'center',
    marginRight: spacing.s,
  },
  avatarText: {
    ...typography.headingM,
    color: colors.textPrimary,
  },
  userInfoText: {
    flex: 1,
  },
  username: {
    ...typography.body,
    fontWeight: '600',
    color: colors.textPrimary,
  },
  eventTag: {
    ...typography.caption,
    color: colors.textSecondary,
    marginTop: 2,
  },
  caption: {
    ...typography.body,
    color: colors.textPrimary,
    marginBottom: spacing.s,
  },
  emotionsContainer: {
    flexDirection: 'row',
    marginBottom: spacing.s,
  },
  emotion: {
    ...typography.caption,
    color: colors.textSecondary,
    marginRight: spacing.s,
  },
  views: {
    ...typography.caption,
    color: colors.textMuted,
  },
});

export default Feed;
