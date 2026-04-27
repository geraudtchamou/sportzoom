import { Stack } from 'expo-router';

export default function RootLayout() {
  return (
    <Stack screenOptions={{ headerShown: false }}>
      <Stack.Screen name="index" />
      <Stack.Screen name="predictions" />
      <Stack.Screen name="events" />
      <Stack.Screen name="feed" />
      <Stack.Screen name="live" />
      <Stack.Screen name="profile" />
      <Stack.Screen name="create" />
    </Stack>
  );
}
