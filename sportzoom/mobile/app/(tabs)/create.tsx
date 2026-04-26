import React, { useState } from 'react';
import { View, Text, StyleSheet, ScrollView, Image } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import * as ImagePicker from 'expo-image-picker';
import { colors, typography, spacing, borderRadius } from '@/constants/theme';
import Button from '@/components/ui/Button';
import Card from '@/components/ui/Card';
import { emotions } from '@/constants/config';

export default function CreateScreen() {
  const [caption, setCaption] = useState('');
  const [selectedMedia, setSelectedMedia] = useState<string | null>(null);
  const [selectedEmotion, setSelectedEmotion] = useState<typeof emotions[0]['type'] | null>(null);
  const [mediaType, setMediaType] = useState<'image' | 'video' | null>(null);
  const [uploading, setUploading] = useState(false);

  const pickMedia = async () => {
    const { status } = await ImagePicker.requestMediaLibraryPermissionsAsync();
    
    if (status !== 'granted') {
      alert('Sorry, we need camera roll permissions to make this work!');
      return;
    }

    const result = await ImagePicker.launchImageLibraryAsync({
      mediaTypes: ['images', 'videos'],
      allowsEditing: true,
      aspect: [9, 16],
      quality: 0.8,
    });

    if (!result.canceled && result.assets[0]) {
      setSelectedMedia(result.assets[0].uri);
      setMediaType(result.assets[0].type === 'video' ? 'video' : 'image');
    }
  };

  const takePhotoOrVideo = async () => {
    const { status } = await ImagePicker.requestCameraPermissionsAsync();
    
    if (status !== 'granted') {
      alert('Sorry, we need camera permissions to make this work!');
      return;
    }

    const result = await ImagePicker.launchCameraAsync({
      allowsEditing: true,
      aspect: [9, 16],
      quality: 0.8,
      videoMaxDuration: 60,
    });

    if (!result.canceled && result.assets[0]) {
      setSelectedMedia(result.assets[0].uri);
      setMediaType(result.assets[0].type === 'video' ? 'video' : 'image');
    }
  };

  const handlePost = async () => {
    if (!selectedMedia) {
      alert('Please select a photo or video first!');
      return;
    }

    if (!caption.trim()) {
      alert('Please add a caption!');
      return;
    }

    setUploading(true);
    
    try {
      // TODO: Upload media and create post via API
      console.log('Posting:', {
        media: selectedMedia,
        type: mediaType,
        caption,
        emotion: selectedEmotion,
      });
      
      // Simulate upload
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      // Reset form
      setSelectedMedia(null);
      setCaption('');
      setSelectedEmotion(null);
      setMediaType(null);
      
      alert('Post created successfully! 🎉');
    } catch (error) {
      console.error('Failed to create post:', error);
      alert('Failed to create post. Please try again.');
    } finally {
      setUploading(false);
    }
  };

  return (
    <SafeAreaView style={styles.container} edges={['top', 'bottom']}>
      <ScrollView style={styles.scrollView} showsVerticalScrollIndicator={false}>
        {/* Header */}
        <View style={styles.header}>
          <Text style={styles.title}>Create Post</Text>
          <Text style={styles.subtitle}>Share your moment with the world</Text>
        </View>

        {/* Media Selection */}
        <Card style={styles.mediaCard}>
          {!selectedMedia ? (
            <View style={styles.mediaPlaceholder}>
              <Text style={styles.mediaPlaceholderIcon}>📷</Text>
              <Text style={styles.mediaPlaceholderText}>No media selected</Text>
              <View style={styles.mediaButtons}>
                <Button
                  title="📷 Camera"
                  onPress={takePhotoOrVideo}
                  variant="primary"
                  size="medium"
                  style={styles.mediaButton}
                />
                <Button
                  title="🖼️ Gallery"
                  onPress={pickMedia}
                  variant="outline"
                  size="medium"
                  style={styles.mediaButton}
                />
              </View>
            </View>
          ) : (
            <View style={styles.mediaPreview}>
              <Image source={{ uri: selectedMedia }} style={styles.previewImage} />
              <View style={styles.mediaTypeBadge}>
                <Text style={styles.mediaTypeText}>{mediaType === 'video' ? '🎥 Video' : '📷 Photo'}</Text>
              </View>
              <Button
                title="Change Media"
                onPress={pickMedia}
                variant="outline"
                size="small"
                style={styles.changeMediaButton}
              />
            </View>
          )}
        </Card>

        {/* Caption Input */}
        <Card style={styles.inputCard}>
          <Text style={styles.label}>Caption</Text>
          <View style={styles.captionInput}>
            <Text
              style={styles.captionText}
              suppressHighlighting
              onPress={() => {
                // In a real app, this would be a TextInput
                alert('In production, this would be an editable text input');
              }}
            >
              {caption || 'Tap to add a caption...'}
            </Text>
          </View>
          <Text style={styles.charCount}>{caption.length}/150</Text>
        </Card>

        {/* Emotion Selector */}
        <Card style={styles.emotionCard}>
          <Text style={styles.label}>How are you feeling?</Text>
          <View style={styles.emotionsContainer}>
            {emotions.map((emotion) => (
              <View
                key={emotion.type}
                style={[
                  styles.emotionOption,
                  selectedEmotion === emotion.type && styles.emotionSelected,
                ]}
              >
                <Button
                  title={emotion.emoji}
                  onPress={() => setSelectedEmotion(emotion.type)}
                  variant={selectedEmotion === emotion.type ? 'primary' : 'ghost'}
                  size="large"
                  style={styles.emotionButton}
                />
                <Text style={styles.emotionLabel}>{emotion.label}</Text>
              </View>
            ))}
          </View>
        </Card>

        {/* Tips */}
        <Card style={styles.tipsCard} variant="outlined">
          <Text style={styles.tipsTitle}>💡 Tips for great content</Text>
          <Text style={styles.tip}>• Use good lighting for better quality</Text>
          <Text style={styles.tip}>• Keep videos under 60 seconds</Text>
          <Text style={styles.tip}>• Add relevant hashtags</Text>
          <Text style={styles.tip}>• Tag events when applicable</Text>
        </Card>

        {/* Post Button */}
        <Button
          title={uploading ? 'Posting...' : 'Post Now 🚀'}
          onPress={handlePost}
          variant="primary"
          size="large"
          disabled={!selectedMedia || !caption.trim() || uploading}
          loading={uploading}
          style={styles.postButton}
        />

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
    paddingBottom: spacing.l,
  },
  title: {
    ...typography.headingXL,
    color: colors.textPrimary,
  },
  subtitle: {
    ...typography.body,
    color: colors.textSecondary,
    marginTop: spacing.xs,
  },
  mediaCard: {
    marginBottom: spacing.m,
    minHeight: 300,
    alignItems: 'center',
    justifyContent: 'center',
  },
  mediaPlaceholder: {
    alignItems: 'center',
    padding: spacing.xl,
  },
  mediaPlaceholderIcon: {
    fontSize: 64,
    marginBottom: spacing.m,
  },
  mediaPlaceholderText: {
    ...typography.body,
    color: colors.textMuted,
    marginBottom: spacing.l,
  },
  mediaButtons: {
    flexDirection: 'row',
    gap: spacing.m,
  },
  mediaButton: {
    minWidth: 120,
  },
  mediaPreview: {
    width: '100%',
    alignItems: 'center',
  },
  previewImage: {
    width: '100%',
    height: 400,
    borderRadius: borderRadius.medium,
  },
  mediaTypeBadge: {
    backgroundColor: colors.primary,
    paddingHorizontal: spacing.m,
    paddingVertical: spacing.s,
    borderRadius: borderRadius.small,
    marginTop: spacing.m,
  },
  mediaTypeText: {
    ...typography.caption,
    color: colors.textPrimary,
    fontWeight: '600',
  },
  changeMediaButton: {
    marginTop: spacing.m,
  },
  inputCard: {
    marginBottom: spacing.m,
  },
  label: {
    ...typography.body,
    color: colors.textPrimary,
    fontWeight: '600',
    marginBottom: spacing.s,
  },
  captionInput: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.small,
    padding: spacing.m,
    minHeight: 80,
  },
  captionText: {
    ...typography.body,
    color: colors.textPrimary,
  },
  charCount: {
    ...typography.caption,
    color: colors.textMuted,
    textAlign: 'right',
    marginTop: spacing.xs,
  },
  emotionCard: {
    marginBottom: spacing.m,
  },
  emotionsContainer: {
    flexDirection: 'row',
    justifyContent: 'space-around',
    flexWrap: 'wrap',
    marginTop: spacing.s,
  },
  emotionOption: {
    alignItems: 'center',
    margin: spacing.xs,
  },
  emotionSelected: {
    transform: [{ scale: 1.1 }],
  },
  emotionButton: {
    width: 60,
    height: 60,
    borderRadius: 30,
  },
  emotionLabel: {
    ...typography.caption,
    color: colors.textSecondary,
    marginTop: spacing.xs,
  },
  tipsCard: {
    marginBottom: spacing.m,
    padding: spacing.m,
  },
  tipsTitle: {
    ...typography.headingM,
    color: colors.textPrimary,
    marginBottom: spacing.s,
  },
  tip: {
    ...typography.body,
    color: colors.textSecondary,
    marginBottom: spacing.xs,
  },
  postButton: {
    marginTop: spacing.m,
  },
});
