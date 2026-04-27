import React from 'react';
import { TouchableOpacity, Text, StyleSheet, ActivityIndicator } from 'react-native';
import { colors, typography, spacing, borderRadius, buttonSizes } from '@/constants/theme';
import { LinearGradient } from 'expo-linear-gradient';

interface ButtonProps {
  title: string;
  onPress: () => void;
  variant?: 'primary' | 'secondary' | 'outline' | 'ghost';
  size?: 'small' | 'medium' | 'large';
  disabled?: boolean;
  loading?: boolean;
  leftIcon?: React.ReactNode;
  rightIcon?: React.ReactNode;
  style?: any;
}

export const Button: React.FC<ButtonProps> = ({
  title,
  onPress,
  variant = 'primary',
  size = 'medium',
  disabled = false,
  loading = false,
  leftIcon,
  rightIcon,
  style,
}) => {
  const isPrimary = variant === 'primary';
  const isSecondary = variant === 'secondary';
  const isOutline = variant === 'outline';
  
  const buttonStyle = [
    styles.button,
    styles[size],
    isPrimary && styles.primary,
    isSecondary && styles.secondary,
    isOutline && styles.outline,
    variant === 'ghost' && styles.ghost,
    disabled && styles.disabled,
    style,
  ];

  const renderContent = () => {
    if (loading) {
      return <ActivityIndicator color={isOutline || variant === 'ghost' ? colors.textPrimary : colors.textPrimary} />;
    }

    return (
      <>
        {leftIcon && <>{leftIcon}</>}
        <Text style={[
          styles.text,
          styles[`${size}Text`],
          (isOutline || variant === 'ghost') && styles.textOutline,
        ]}>
          {title}
        </Text>
        {rightIcon && <>{rightIcon}</>}
      </>
    );
  };

  if (isPrimary || isSecondary) {
    return (
      <TouchableOpacity
        onPress={onPress}
        disabled={disabled || loading}
        activeOpacity={0.8}
        style={buttonStyle}
      >
        {isPrimary ? (
          <LinearGradient
            colors={[colors.primary, '#5a4bd6']}
            start={{ x: 0, y: 0 }}
            end={{ x: 1, y: 1 }}
            style={styles.gradient}
          >
            {renderContent()}
          </LinearGradient>
        ) : (
          <LinearGradient
            colors={[colors.secondary, '#00b8e6']}
            start={{ x: 0, y: 0 }}
            end={{ x: 1, y: 1 }}
            style={styles.gradient}
          >
            {renderContent()}
          </LinearGradient>
        )}
      </TouchableOpacity>
    );
  }

  return (
    <TouchableOpacity
      onPress={onPress}
      disabled={disabled || loading}
      activeOpacity={0.7}
      style={buttonStyle}
    >
      {renderContent()}
    </TouchableOpacity>
  );
};

const styles = StyleSheet.create({
  button: {
    borderRadius: borderRadius.medium,
    alignItems: 'center',
    justifyContent: 'center',
    flexDirection: 'row',
  },
  small: {
    height: buttonSizes.small.height,
    paddingHorizontal: buttonSizes.small.paddingHorizontal,
  },
  medium: {
    height: buttonSizes.medium.height,
    paddingHorizontal: buttonSizes.medium.paddingHorizontal,
  },
  large: {
    height: buttonSizes.large.height,
    paddingHorizontal: buttonSizes.large.paddingHorizontal,
  },
  primary: {
    overflow: 'hidden',
  },
  secondary: {
    overflow: 'hidden',
  },
  gradient: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    width: '100%',
    height: '100%',
  },
  outline: {
    backgroundColor: 'transparent',
    borderWidth: 2,
    borderColor: colors.primary,
  },
  ghost: {
    backgroundColor: 'transparent',
  },
  disabled: {
    opacity: 0.5,
  },
  text: {
    color: colors.textPrimary,
    textAlign: 'center',
  },
  textOutline: {
    color: colors.primary,
  },
  smallText: {
    ...typography.body,
    fontSize: 14,
  },
  mediumText: {
    ...typography.button,
  },
  largeText: {
    ...typography.headingM,
  },
});

export default Button;
