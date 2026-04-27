import 'react-native-gesture-handler';
import React, { useEffect } from 'react';
import { Stack } from 'expo-router';
import { StatusBar } from 'expo-status-bar';
import { GestureHandlerRootView } from 'react-native-gesture-handler';
import { StyleSheet } from 'react-native';
import { useAuthStore } from '@/store';
import { colors } from '@/constants/theme';

export default function RootLayout() {
  const { checkAuth, isLoading } = useAuthStore();

  useEffect(() => {
    // Check authentication status on app load
    checkAuth();
  }, []);

  return (
    <GestureHandlerRootView style={styles.container}>
      <StatusBar style="light" />
      <Stack
        screenOptions={{
          headerStyle: {
            backgroundColor: colors.background,
          },
          headerTintColor: colors.textPrimary,
          headerTitleStyle: {
            fontWeight: 'bold',
          },
          contentStyle: {
            backgroundColor: colors.background,
          },
          animation: 'slide_from_right',
          animationDuration: 300,
        }}
      >
        <Stack.Screen
          name="(tabs)"
          options={{
            headerShown: false,
            title: 'SportZoom',
          }}
        />
        <Stack.Screen
          name="login"
          options={{
            title: 'Sign In',
            presentation: 'modal',
          }}
        />
        <Stack.Screen
          name="register"
          options={{
            title: 'Create Account',
            presentation: 'modal',
          }}
        />
        <Stack.Screen
          name="post/[id]"
          options={{
            title: 'Post Details',
            presentation: 'modal',
          }}
        />
        <Stack.Screen
          name="event/[id]"
          options={{
            title: 'Event Details',
          }}
        />
        <Stack.Screen
          name="predictions"
          options={{
            title: 'Predictions',
          }}
        />
        <Stack.Screen
          name="leaderboard"
          options={{
            title: 'Leaderboard',
          }}
        />
      </Stack>
    </GestureHandlerRootView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
  },
});
