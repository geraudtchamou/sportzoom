import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
import { Link } from 'expo-router';
import { SafeAreaView } from 'react-native-safe-area-context';
import { colors, typography, spacing, borderRadius } from '@/constants/theme';
import Button from '@/components/ui/Button';
import Card from '@/components/ui/Card';

export default function Index() {
  return (
    <SafeAreaView style={styles.container} edges={['top', 'bottom']}>
      <View style={styles.content}>
        {/* Logo */}
        <View style={styles.logoContainer}>
          <View style={styles.logo}>
            <Text style={styles.logoIcon}>🎯</Text>
          </View>
          <Text style={styles.appName}>SportZoom</Text>
          <Text style={styles.tagline}>The Social Network for Live Experiences</Text>
        </View>

        {/* Features */}
        <View style={styles.features}>
          <Card style={styles.featureCard}>
            <Text style={styles.featureIcon}>⚽</Text>
            <Text style={styles.featureTitle}>Sports Predictions</Text>
            <Text style={styles.featureDescription}>
              Predict match outcomes and compete with fans worldwide
            </Text>
          </Card>

          <Card style={styles.featureCard}>
            <Text style={styles.featureIcon}>🎥</Text>
            <Text style={styles.featureTitle}>Video Feed</Text>
            <Text style={styles.featureDescription}>
              Share and discover amazing moments from live events
            </Text>
          </Card>

          <Card style={styles.featureCard}>
            <Text style={styles.featureIcon}>🏆</Text>
            <Text style={styles.featureTitle}>Gamification</Text>
            <Text style={styles.featureDescription}>
              Earn points, badges, and climb the leaderboard
            </Text>
          </Card>

          <Card style={styles.featureCard}>
            <Text style={styles.featureIcon}>🔴</Text>
            <Text style={styles.featureTitle}>Live Streaming</Text>
            <Text style={styles.featureDescription}>
              Watch and stream live events in real-time
            </Text>
          </Card>
        </View>

        {/* CTA Buttons */}
        <View style={styles.ctaContainer}>
          <Link href="/(tabs)" asChild>
            <Button
              title="Get Started 🚀"
              onPress={() => {}}
              variant="primary"
              size="large"
              style={styles.ctaButton}
            />
          </Link>

          <Link href="/login" asChild>
            <Button
              title="Sign In"
              onPress={() => {}}
              variant="outline"
              size="large"
              style={styles.ctaButton}
            />
          </Link>
        </View>

        {/* Footer */}
        <View style={styles.footer}>
          <Text style={styles.footerText}>
            Join thousands of fans experiencing sports like never before
          </Text>
          <Text style={styles.footerLinks}>
            <Text style={styles.footerLink}>Terms</Text>
            {' • '}
            <Text style={styles.footerLink}>Privacy</Text>
            {' • '}
            <Text style={styles.footerLink}>Contact</Text>
          </Text>
        </View>
      </View>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
  },
  content: {
    flex: 1,
    paddingHorizontal: spacing.l,
    justifyContent: 'center',
  },
  logoContainer: {
    alignItems: 'center',
    marginBottom: spacing.xxl,
  },
  logo: {
    width: 100,
    height: 100,
    borderRadius: 50,
    backgroundColor: colors.primary,
    alignItems: 'center',
    justifyContent: 'center',
    marginBottom: spacing.m,
  },
  logoIcon: {
    fontSize: 48,
  },
  appName: {
    ...typography.headingXL,
    color: colors.textPrimary,
    fontWeight: 'bold',
    marginBottom: spacing.s,
  },
  tagline: {
    ...typography.body,
    color: colors.textSecondary,
    textAlign: 'center',
  },
  features: {
    marginBottom: spacing.xxl,
  },
  featureCard: {
    marginBottom: spacing.m,
    padding: spacing.m,
    flexDirection: 'row',
    alignItems: 'center',
  },
  featureIcon: {
    fontSize: 32,
    marginRight: spacing.m,
  },
  featureTitle: {
    ...typography.headingM,
    color: colors.textPrimary,
    marginBottom: spacing.xs,
  },
  featureDescription: {
    ...typography.caption,
    color: colors.textMuted,
    flex: 1,
  },
  ctaContainer: {
    gap: spacing.m,
    marginBottom: spacing.xl,
  },
  ctaButton: {
    width: '100%',
  },
  footer: {
    alignItems: 'center',
  },
  footerText: {
    ...typography.caption,
    color: colors.textMuted,
    textAlign: 'center',
    marginBottom: spacing.s,
  },
  footerLinks: {
    ...typography.caption,
    color: colors.textSecondary,
  },
  footerLink: {
    color: colors.primary,
  },
});
