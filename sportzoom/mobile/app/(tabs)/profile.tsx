import React from 'react';
import { View, Text, StyleSheet, ScrollView, Image, TouchableOpacity } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { colors, typography, spacing, borderRadius } from '@/constants/theme';
import Card from '@/components/ui/Card';
import Button from '@/components/ui/Button';
import { useAuthStore, useGamificationStore } from '@/store';

// Mock user data
const mockUser = {
  id: '1',
  username: 'sportsfan_mike',
  email: 'mike@example.com',
  avatar: null,
  points: 2500,
  level: 3,
  badges: [
    { id: '1', name: 'Top Predictor', description: 'Won 10 predictions', icon: '🏆', earnedAt: new Date().toISOString() },
    { id: '2', name: 'Event King', description: 'Attended 50 events', icon: '🎉', earnedAt: new Date().toISOString() },
    { id: '3', name: 'Content Creator', description: 'Posted 100 videos', icon: '🎥', earnedAt: new Date().toISOString() },
  ],
  createdAt: new Date().toISOString(),
};

export default function ProfileScreen() {
  const { user, isAuthenticated, logout } = useAuthStore();
  const { points, level, badges, leaderboard } = useGamificationStore();
  
  const currentUser = user || mockUser;
  const currentPoints = points || currentUser.points;
  const currentLevel = level || currentUser.level;
  const currentBadges = badges.length > 0 ? badges : currentUser.badges;

  const stats = [
    { label: 'Points', value: currentPoints.toLocaleString() },
    { label: 'Level', value: currentLevel.toString() },
    { label: 'Following', value: '234' },
    { label: 'Followers', value: '1.2K' },
  ];

  return (
    <SafeAreaView style={styles.container} edges={['top', 'bottom']}>
      <ScrollView style={styles.scrollView} showsVerticalScrollIndicator={false}>
        {/* Header */}
        <View style={styles.header}>
          <Text style={styles.title}>Profile</Text>
        </View>

        {/* User Info Card */}
        <Card style={styles.profileCard}>
          <View style={styles.avatarContainer}>
            {currentUser.avatar ? (
              <Image source={{ uri: currentUser.avatar }} style={styles.avatar} />
            ) : (
              <View style={[styles.avatar, styles.avatarPlaceholder]}>
                <Text style={styles.avatarPlaceholderText}>
                  {currentUser.username.charAt(0).toUpperCase()}
                </Text>
              </View>
            )}
            <View style={styles.levelBadge}>
              <Text style={styles.levelBadgeText}>Lvl {currentLevel}</Text>
            </View>
          </View>
          
          <Text style={styles.username}>@{currentUser.username}</Text>
          <Text style={styles.email}>{currentUser.email}</Text>
          
          {/* Stats Grid */}
          <View style={styles.statsGrid}>
            {stats.map((stat, index) => (
              <View key={index} style={styles.statItem}>
                <Text style={styles.statValue}>{stat.value}</Text>
                <Text style={styles.statLabel}>{stat.label}</Text>
              </View>
            ))}
          </View>

          {/* Points Progress */}
          <View style={styles.progressContainer}>
            <View style={styles.progressHeader}>
              <Text style={styles.progressLabel}>Points to next level</Text>
              <Text style={styles.progressValue}>
                {currentPoints % 1000}/1000
              </Text>
            </View>
            <View style={styles.progressBar}>
              <View 
                style={[
                  styles.progressFill,
                  { width: `${(currentPoints % 1000) / 10}%` },
                ]}
              />
            </View>
          </View>

          {!isAuthenticated && (
            <Button
              title="Sign In"
              onPress={() => {}}
              variant="primary"
              size="medium"
              style={styles.signInButton}
            />
          )}
        </Card>

        {/* Badges Section */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>🏅 Badges</Text>
          <ScrollView horizontal showsHorizontalScrollIndicator={false}>
            {currentBadges.map((badge) => (
              <Card key={badge.id} style={styles.badgeCard}>
                <Text style={styles.badgeIcon}>{badge.icon}</Text>
                <Text style={styles.badgeName}>{badge.name}</Text>
                <Text style={styles.badgeDescription} numberOfLines={2}>
                  {badge.description}
                </Text>
              </Card>
            ))}
            {currentBadges.length === 0 && (
              <Card style={styles.badgeCard}>
                <Text style={styles.badgeIcon}>🔒</Text>
                <Text style={styles.badgeName}>No badges yet</Text>
                <Text style={styles.badgeDescription}>
                  Start predicting and creating!
                </Text>
              </Card>
            )}
          </ScrollView>
        </View>

        {/* Quick Actions */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>⚙️ Settings</Text>
          <Card style={styles.menuCard} onPress={() => {}}>
            <View style={styles.menuItem}>
              <Text style={styles.menuIcon}>👤</Text>
              <Text style={styles.menuLabel}>Edit Profile</Text>
              <Text style={styles.menuArrow}>›</Text>
            </View>
          </Card>
          <Card style={styles.menuCard} onPress={() => {}}>
            <View style={styles.menuItem}>
              <Text style={styles.menuIcon}>🔔</Text>
              <Text style={styles.menuLabel}>Notifications</Text>
              <Text style={styles.menuArrow}>›</Text>
            </View>
          </Card>
          <Card style={styles.menuCard} onPress={() => {}}>
            <View style={styles.menuItem}>
              <Text style={styles.menuIcon}>🔒</Text>
              <Text style={styles.menuLabel}>Privacy</Text>
              <Text style={styles.menuArrow}>›</Text>
            </View>
          </Card>
          <Card style={styles.menuCard} onPress={() => {}}>
            <View style={styles.menuItem}>
              <Text style={styles.menuIcon}>❓</Text>
              <Text style={styles.menuLabel}>Help & Support</Text>
              <Text style={styles.menuArrow}>›</Text>
            </View>
          </Card>
          
          {isAuthenticated && (
            <Button
              title="Sign Out"
              onPress={logout}
              variant="outline"
              size="medium"
              style={styles.logoutButton}
            />
          )}
        </View>

        {/* App Version */}
        <View style={styles.versionContainer}>
          <Text style={styles.versionText}>SportZoom v1.0.0</Text>
          <Text style={styles.copyrightText}>© 2024 SportZoom. All rights reserved.</Text>
        </View>

        <View style={{ height: spacing.xxl }} />
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
    paddingHorizontal: spacing.m,
  },
  header: {
    paddingTop: spacing.l,
    paddingBottom: spacing.m,
  },
  title: {
    ...typography.headingXL,
    color: colors.textPrimary,
  },
  profileCard: {
    marginBottom: spacing.l,
    alignItems: 'center',
    padding: spacing.l,
  },
  avatarContainer: {
    position: 'relative',
    marginBottom: spacing.m,
  },
  avatar: {
    width: 100,
    height: 100,
    borderRadius: 50,
    backgroundColor: colors.primary,
    alignItems: 'center',
    justifyContent: 'center',
  },
  avatarPlaceholder: {
    alignItems: 'center',
    justifyContent: 'center',
  },
  avatarPlaceholderText: {
    ...typography.headingXL,
    color: colors.textPrimary,
  },
  levelBadge: {
    position: 'absolute',
    bottom: 0,
    right: 0,
    backgroundColor: colors.primary,
    paddingHorizontal: spacing.s,
    paddingVertical: 4,
    borderRadius: borderRadius.small,
  },
  levelBadgeText: {
    ...typography.caption,
    color: colors.textPrimary,
    fontWeight: 'bold',
  },
  username: {
    ...typography.headingM,
    color: colors.textPrimary,
    fontWeight: '600',
    marginBottom: spacing.xs,
  },
  email: {
    ...typography.body,
    color: colors.textSecondary,
    marginBottom: spacing.l,
  },
  statsGrid: {
    flexDirection: 'row',
    justifyContent: 'space-around',
    width: '100%',
    marginBottom: spacing.l,
  },
  statItem: {
    alignItems: 'center',
  },
  statValue: {
    ...typography.headingM,
    color: colors.textPrimary,
    fontWeight: '600',
  },
  statLabel: {
    ...typography.caption,
    color: colors.textMuted,
    marginTop: spacing.xs,
  },
  progressContainer: {
    width: '100%',
    marginBottom: spacing.m,
  },
  progressHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: spacing.s,
  },
  progressLabel: {
    ...typography.caption,
    color: colors.textSecondary,
  },
  progressValue: {
    ...typography.caption,
    color: colors.primary,
    fontWeight: '600',
  },
  progressBar: {
    width: '100%',
    height: 8,
    backgroundColor: colors.surface,
    borderRadius: borderRadius.small,
    overflow: 'hidden',
  },
  progressFill: {
    height: '100%',
    backgroundColor: colors.primary,
    borderRadius: borderRadius.small,
  },
  signInButton: {
    width: '100%',
  },
  section: {
    marginBottom: spacing.l,
  },
  sectionTitle: {
    ...typography.headingM,
    color: colors.textPrimary,
    marginBottom: spacing.m,
  },
  badgeCard: {
    width: 120,
    marginRight: spacing.m,
    alignItems: 'center',
    padding: spacing.m,
  },
  badgeIcon: {
    fontSize: 40,
    marginBottom: spacing.s,
  },
  badgeName: {
    ...typography.caption,
    color: colors.textPrimary,
    fontWeight: '600',
    textAlign: 'center',
    marginBottom: spacing.xs,
  },
  badgeDescription: {
    ...typography.caption,
    color: colors.textMuted,
    textAlign: 'center',
    fontSize: 10,
  },
  menuCard: {
    marginBottom: spacing.s,
  },
  menuItem: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingVertical: spacing.s,
  },
  menuIcon: {
    fontSize: 24,
    marginRight: spacing.m,
  },
  menuLabel: {
    ...typography.body,
    color: colors.textPrimary,
    flex: 1,
  },
  menuArrow: {
    ...typography.headingM,
    color: colors.textMuted,
  },
  logoutButton: {
    marginTop: spacing.m,
    borderColor: colors.error,
  },
  versionContainer: {
    alignItems: 'center',
    marginTop: spacing.xl,
  },
  versionText: {
    ...typography.caption,
    color: colors.textMuted,
    marginBottom: spacing.xs,
  },
  copyrightText: {
    ...typography.caption,
    color: colors.textMuted,
  },
});
