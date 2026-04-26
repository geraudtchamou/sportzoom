import React, { useState, useCallback } from 'react';
import { View, StyleSheet, TouchableOpacity } from 'react-native';
import Animated, {
  useSharedValue,
  useAnimatedStyle,
  withSpring,
  withSequence,
  withTiming,
  runOnJS,
} from 'react-native-reanimated';
import { colors, spacing, iconSizes } from '@/constants/theme';
import * as Haptics from 'expo-haptics';

interface IconButtonProps {
  icon: React.ReactNode;
  onPress?: () => void;
  onDoublePress?: () => void;
  isActive?: boolean;
  activeColor?: string;
  size?: 'small' | 'medium' | 'large' | 'xlarge';
  style?: any;
  disabled?: boolean;
}

export const IconButton: React.FC<IconButtonProps> = ({
  icon,
  onPress,
  onDoublePress,
  isActive = false,
  activeColor = colors.accent,
  size = 'large',
  style,
  disabled = false,
}) => {
  const scale = useSharedValue(1);
  const [isPressed, setIsPressed] = useState(false);
  const pressTimeout = useState<NodeJS.Timeout | null>(null)[0];

  const sizeValue = iconSizes[size];

  const animatedStyle = useAnimatedStyle(() => ({
    transform: [{ scale: scale.value }],
  }));

  const handlePressIn = useCallback(() => {
    if (disabled) return;
    
    scale.value = withSpring(0.95, {
      damping: 12,
      mass: 1,
      stiffness: 150,
    });
    setIsPressed(true);
  }, [disabled, scale]);

  const handlePressOut = useCallback(() => {
    if (disabled) return;
    
    scale.value = withSpring(1, {
      damping: 12,
      mass: 1,
      stiffness: 150,
    });
    setIsPressed(false);
  }, [disabled, scale]);

  const handlePress = useCallback(() => {
    if (disabled) return;

    // Trigger haptic feedback
    Haptics.selectionAsync();

    if (onDoublePress) {
      // Check for double press
      if (pressTimeout) {
        clearTimeout(pressTimeout);
      }
      
      const timeout = setTimeout(() => {
        if (onPress) {
          onPress();
        }
      }, 250);
      
      // Store timeout for cleanup
      return () => clearTimeout(timeout);
    }

    if (onPress) {
      onPress();
    }
  }, [disabled, onPress, onDoublePress, pressTimeout]);

  const handleDoubleTap = useCallback(() => {
    if (disabled || !onDoublePress) return;

    // Trigger heavy haptic feedback for double tap
    Haptics.notificationAsync(Haptics.NotificationFeedbackType.Success);

    // Animate heart burst effect
    scale.value = withSequence(
      withTiming(1.4, { duration: 100 }),
      withSpring(1, {
        damping: 10,
        mass: 1,
        stiffness: 200,
      })
    );

    onDoublePress();
  }, [disabled, onDoublePress, scale]);

  return (
    <TouchableOpacity
      onPress={handlePress}
      onLongPress={handleDoubleTap}
      delayLongPress={200}
      onPressIn={handlePressIn}
      onPressOut={handlePressOut}
      disabled={disabled}
      activeOpacity={0.7}
      style={[styles.container, style]}
    >
      <Animated.View
        style={[
          animatedStyle,
          styles.iconWrapper,
          { width: sizeValue, height: sizeValue },
        ]}
      >
        {React.cloneElement(icon as React.ReactElement, {
          fill: isActive ? activeColor : colors.textSecondary,
          stroke: isActive ? activeColor : colors.textSecondary,
        })}
      </Animated.View>
    </TouchableOpacity>
  );
};

const styles = StyleSheet.create({
  container: {
    padding: spacing.s,
    alignItems: 'center',
    justifyContent: 'center',
  },
  iconWrapper: {
    alignItems: 'center',
    justifyContent: 'center',
  },
});

export default IconButton;

// Like button with counter
interface LikeButtonProps {
  count: number;
  isLiked: boolean;
  onLike: () => void;
  size?: 'small' | 'medium' | 'large' | 'xlarge';
}

export const LikeButton: React.FC<LikeButtonProps> = ({
  count,
  isLiked,
  onLike,
  size = 'large',
}) => {
  const [localCount, setLocalCount] = useState(count);
  const [localIsLiked, setLocalIsLiked] = useState(isLiked);

  const handleLike = useCallback(() => {
    setLocalIsLiked((prev) => !prev);
    setLocalCount((prev) => prev + (localIsLiked ? -1 : 1));
    onLike();
  }, [localIsLiked, onLike]);

  // Heart icon SVG
  const HeartIcon = () => (
    <View style={styles.heartContainer}>
      <Animated.Text style={styles.heartEmoji}>
        {localIsLiked ? '❤️' : '🤍'}
      </Animated.Text>
    </View>
  );

  return (
    <View style={styles.likeButtonContainer}>
      <IconButton
        icon={<HeartIcon />}
        onPress={handleLike}
        isActive={localIsLiked}
        activeColor={colors.accent}
        size={size}
      />
      <Animated.Text style={styles.likeCount}>
        {localCount.toLocaleString()}
      </Animated.Text>
    </View>
  );
};

const likeStyles = StyleSheet.create({
  likeButtonContainer: {
    alignItems: 'center',
  },
  heartContainer: {
    width: 32,
    height: 32,
    alignItems: 'center',
    justifyContent: 'center',
  },
  heartEmoji: {
    fontSize: 28,
  },
  likeCount: {
    ...typography.caption,
    color: colors.textPrimary,
    marginTop: 2,
  },
});

Object.assign(styles, likeStyles);
