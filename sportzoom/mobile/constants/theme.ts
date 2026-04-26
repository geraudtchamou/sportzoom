// Design System Constants for SportZoom

export const colors = {
  // Primary Colors
  primary: '#6C5CE7',
  secondary: '#00D1FF',
  accent: '#FF3B5C',
  
  // Dark Theme Backgrounds
  background: '#0F0F14',
  surface: '#1A1A22',
  card: '#22232B',
  border: '#2E2F38',
  
  // Text Colors
  textPrimary: '#FFFFFF',
  textSecondary: '#B0B3C7',
  textMuted: '#6E7191',
  
  // Status Colors
  success: '#00E676',
  warning: '#FFC107',
  error: '#FF5252',
  info: '#2196F3',
  
  // Overlay
  overlay: 'rgba(0, 0, 0, 0.6)',
  overlayLight: 'rgba(0, 0, 0, 0.3)',
};

export const typography = {
  headingXL: {
    fontSize: 32,
    fontWeight: 'bold' as const,
    lineHeight: 40,
  },
  headingL: {
    fontSize: 24,
    fontWeight: '600' as const,
    lineHeight: 32,
  },
  headingM: {
    fontSize: 20,
    fontWeight: '500' as const,
    lineHeight: 28,
  },
  body: {
    fontSize: 16,
    fontWeight: '400' as const,
    lineHeight: 24,
  },
  caption: {
    fontSize: 12,
    fontWeight: '400' as const,
    lineHeight: 16,
  },
  button: {
    fontSize: 16,
    fontWeight: '600' as const,
    lineHeight: 24,
  },
};

export const spacing = {
  xs: 4,
  s: 8,
  m: 16,
  l: 24,
  xl: 32,
  xxl: 48,
};

export const borderRadius = {
  small: 8,
  medium: 16,
  large: 24,
  full: 9999,
};

export const shadows = {
  small: {
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.25,
    shadowRadius: 3.84,
    elevation: 2,
  },
  medium: {
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 4 },
    shadowOpacity: 0.3,
    shadowRadius: 4.65,
    elevation: 4,
  },
  large: {
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 6 },
    shadowOpacity: 0.35,
    shadowRadius: 7.48,
    elevation: 8,
  },
};

export const buttonSizes = {
  small: {
    height: 40,
    paddingHorizontal: 16,
  },
  medium: {
    height: 48,
    paddingHorizontal: 24,
  },
  large: {
    height: 56,
    paddingHorizontal: 32,
  },
};

export const iconSizes = {
  small: 20,
  medium: 24,
  large: 32,
  xlarge: 48,
};

export const zIndex = {
  base: 1,
  overlay: 100,
  modal: 200,
  toast: 300,
};

export const animations = {
  duration: {
    fast: 150,
    normal: 300,
    slow: 500,
  },
  spring: {
    damping: 12,
    mass: 1,
    stiffness: 150,
  },
};
