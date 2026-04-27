import React, { useEffect } from 'react';
import { View, Text, StyleSheet, ScrollView, RefreshControl } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { colors, typography, spacing, borderRadius } from '@/constants/theme';
import Card from '@/components/ui/Card';
import Button from '@/components/ui/Button';
import type { Event } from '@/types';

// Mock data for demonstration
const mockEvents: Event[] = [
  {
    id: '1',
    title: 'Lakers vs Warriors',
    description: 'NBA Regular Season',
    type: 'sports',
    startDate: new Date().toISOString(),
    endDate: new Date(Date.now() + 7200000).toISOString(),
    location: { lat: 34.0522, lng: -118.2437, address: 'Crypto.com Arena', city: 'Los Angeles', country: 'USA' },
    attendeesCount: 15420,
    isLive: true,
    category: 'Basketball',
  },
  {
    id: '2',
    title: 'Taylor Swift Concert',
    description: 'Eras Tour Live',
    type: 'concert',
    startDate: new Date(Date.now() + 86400000).toISOString(),
    endDate: new Date(Date.now() + 93600000).toISOString(),
    location: { lat: 40.7128, lng: -74.0060, address: 'MetLife Stadium', city: 'New York', country: 'USA' },
    attendeesCount: 82000,
    isLive: false,
    category: 'Music',
  },
];

export default function HomeScreen() {
  const [refreshing, setRefreshing] = React.useState(false);
  const [events, setEvents] = React.useState<Event[]>(mockEvents);

  const onRefresh = React.useCallback(() => {
    setRefreshing(true);
    // Simulate API call
    setTimeout(() => {
      setRefreshing(false);
    }, 2000);
  }, []);

  return (
    <SafeAreaView style={styles.container} edges={['top']}>
      <ScrollView
        style={styles.scrollView}
        refreshControl={
          <RefreshControl refreshing={refreshing} onRefresh={onRefresh} tintColor={colors.primary} />
        }
      >
        {/* Header */}
        <View style={styles.header}>
          <Text style={styles.greeting}>Good Evening 👋</Text>
          <Text style={styles.title}>SportZoom</Text>
        </View>

        {/* Live Now Section */}
        <View style={styles.section}>
          <View style={styles.sectionHeader}>
            <Text style={styles.sectionTitle}>🔴 Live Now</Text>
            <Button title="See All" variant="ghost" size="small" onPress={() => {}} />
          </View>
          
          <ScrollView horizontal showsHorizontalScrollIndicator={false}>
            {events.filter(e => e.isLive).map((event) => (
              <Card key={event.id} style={styles.liveCard} onPress={() => {}}>
                <View style={styles.liveBadge}>
                  <Text style={styles.liveText}>LIVE</Text>
                </View>
                <Text style={styles.eventTitle}>{event.title}</Text>
                <Text style={styles.eventCategory}>{event.category}</Text>
                <Text style={styles.attendees}>{event.attendeesCount.toLocaleString()} watching</Text>
              </Card>
            ))}
          </ScrollView>
        </View>

        {/* Trending Events */}
        <View style={styles.section}>
          <View style={styles.sectionHeader}>
            <Text style={styles.sectionTitle}>📈 Trending Events</Text>
            <Button title="See All" variant="ghost" size="small" onPress={() => {}} />
          </View>
          
          {events.map((event) => (
            <Card key={event.id} style={styles.eventCard} onPress={() => {}}>
              <View style={styles.eventCardContent}>
                <View>
                  <Text style={styles.eventTitle}>{event.title}</Text>
                  <Text style={styles.eventDescription}>{event.description}</Text>
                  <Text style={styles.eventLocation}>📍 {event.location.city}</Text>
                </View>
                <View style={styles.eventMeta}>
                  <Text style={styles.eventDate}>
                    {new Date(event.startDate).toLocaleDateString()}
                  </Text>
                  <Text style={styles.attendees}>{event.attendeesCount.toLocaleString()} attending</Text>
                </View>
              </View>
            </Card>
          ))}
        </View>

        {/* Quick Actions */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>⚡ Quick Actions</Text>
          <View style={styles.quickActionsGrid}>
            <Card style={styles.quickActionCard} onPress={() => {}}>
              <Text style={styles.quickActionIcon}>⚽</Text>
              <Text style={styles.quickActionLabel}>Predictions</Text>
            </Card>
            <Card style={styles.quickActionCard} onPress={() => {}}>
              <Text style={styles.quickActionIcon}>🏆</Text>
              <Text style={styles.quickActionLabel}>Leaderboard</Text>
            </Card>
            <Card style={styles.quickActionCard} onPress={() => {}}>
              <Text style={styles.quickActionIcon}>🎥</Text>
              <Text style={styles.quickActionLabel}>Create</Text>
            </Card>
            <Card style={styles.quickActionCard} onPress={() => {}}>
              <Text style={styles.quickActionIcon}>👥</Text>
              <Text style={styles.quickActionLabel}>Friends</Text>
            </Card>
          </View>
        </View>

        <View style={{ height: 100 }} />
      </ScrollView>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
  },
  scrollView: {
    flex: 1,
  },
  header: {
    paddingHorizontal: spacing.m,
    paddingTop: spacing.m,
    paddingBottom: spacing.l,
  },
  greeting: {
    ...typography.body,
    color: colors.textSecondary,
  },
  title: {
    ...typography.headingXL,
    color: colors.textPrimary,
    marginTop: spacing.xs,
  },
  section: {
    marginBottom: spacing.l,
  },
  sectionHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingHorizontal: spacing.m,
    marginBottom: spacing.m,
  },
  sectionTitle: {
    ...typography.headingM,
    color: colors.textPrimary,
  },
  liveCard: {
    width: 200,
    marginLeft: spacing.m,
    marginRight: spacing.s,
  },
  liveBadge: {
    backgroundColor: colors.accent,
    paddingHorizontal: spacing.s,
    paddingVertical: 4,
    borderRadius: borderRadius.small,
    alignSelf: 'flex-start',
    marginBottom: spacing.s,
  },
  liveText: {
    ...typography.caption,
    color: colors.textPrimary,
    fontWeight: 'bold',
  },
  eventCard: {
    marginHorizontal: spacing.m,
    marginBottom: spacing.m,
  },
  eventCardContent: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  eventTitle: {
    ...typography.headingM,
    color: colors.textPrimary,
    marginBottom: spacing.xs,
  },
  eventDescription: {
    ...typography.body,
    color: colors.textSecondary,
    marginBottom: spacing.xs,
  },
  eventLocation: {
    ...typography.caption,
    color: colors.textMuted,
  },
  eventMeta: {
    alignItems: 'flex-end',
  },
  eventDate: {
    ...typography.caption,
    color: colors.textSecondary,
    marginBottom: spacing.xs,
  },
  attendees: {
    ...typography.caption,
    color: colors.primary,
    fontWeight: '600',
  },
  eventCategory: {
    ...typography.caption,
    color: colors.textSecondary,
    marginTop: spacing.xs,
  },
  quickActionsGrid: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    paddingHorizontal: spacing.m,
    gap: spacing.m,
  },
  quickActionCard: {
    width: '47%',
    alignItems: 'center',
    padding: spacing.l,
  },
  quickActionIcon: {
    fontSize: 32,
    marginBottom: spacing.s,
  },
  quickActionLabel: {
    ...typography.body,
    color: colors.textPrimary,
    textAlign: 'center',
  },
});
