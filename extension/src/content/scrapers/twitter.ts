/**
 * Twitter/X Scraper - Extracts tweets and threads from Twitter/X
 * 
 * Features:
 * - Tweet extraction with author info
 * - Thread navigation and extraction
 * - Media handling (images, videos)
 * - Engagement metrics
 */

import {
  Scraper,
  ScrapedContent,
  PageMetadata,
  TweetContent,
  TweetThread,
  MediaContent,
} from './types';

export class TwitterScraper implements Scraper {
  readonly name = 'twitter';

  /** Twitter/X URL patterns */
  private readonly urlPatterns = [
    /^https?:\/\/(www\.)?(twitter\.com|x\.com)/,
    /^https?:\/\/mobile\.(twitter\.com|x\.com)/,
  ];

  canHandle(url: string): boolean {
    return this.urlPatterns.some(pattern => pattern.test(url));
  }

  extract(document: Document): ScrapedContent {
    const url = document.location?.href || '';
    const timestamp = new Date().toISOString();

    const tweetThread = this.extractTweetThread(document);

    return {
      url,
      timestamp,
      scraperType: this.name,
      metadata: this.extractMetadata(document, tweetThread),
      tweetThread,
      media: this.extractAllMedia(document),
    };
  }

  /** Extract page metadata */
  private extractMetadata(document: Document, thread?: TweetThread): PageMetadata {
    const mainTweet = thread?.tweets[0];
    
    return {
      title: document.title || 'Twitter/X',
      author: mainTweet?.author.displayName,
      siteName: 'X (Twitter)',
      ogImage: this.getMetaContent(document, 'og:image'),
      description: mainTweet?.content?.substring(0, 200),
    };
  }

  /** Get meta tag content */
  private getMetaContent(document: Document, name: string): string | undefined {
    const meta = document.querySelector(`meta[property="${name}"], meta[name="${name}"]`);
    return meta?.getAttribute('content') || undefined;
  }

  /** Extract tweet thread from the page */
  private extractTweetThread(document: Document): TweetThread {
    const tweets: TweetContent[] = [];

    // Tweet selectors (X/Twitter DOM structure)
    const tweetSelectors = [
      'article[data-testid="tweet"]',
      '[data-testid="tweet"]',
      'article[role="article"]',
      '[data-testid="tweetText"]',
    ];

    let tweetElements: NodeListOf<Element> | null = null;
    for (const selector of tweetSelectors) {
      tweetElements = document.querySelectorAll(selector);
      if (tweetElements.length > 0) break;
    }

    if (tweetElements) {
      tweetElements.forEach(element => {
        const tweet = this.extractTweetFromElement(element);
        if (tweet) {
          tweets.push(tweet);
        }
      });
    }

    // Try to get conversation ID from URL
    const conversationId = this.extractConversationId(document.location?.href || '');

    return {
      tweets,
      conversationId,
    };
  }

  /** Extract a single tweet from an element */
  private extractTweetFromElement(element: Element): TweetContent | null {
    // Extract author information
    const author = this.extractAuthor(element);
    if (!author) return null;

    // Extract tweet content
    const content = this.extractTweetContent(element);
    if (!content) return null;

    // Extract tweet ID
    const id = this.extractTweetId(element);

    // Extract timestamp
    const timestamp = this.extractTimestamp(element);

    // Extract media
    const media = this.extractMedia(element);

    // Extract metrics
    const metrics = this.extractMetrics(element);

    // Check if retweet or quote
    const isRetweet = this.isRetweet(element);
    const isQuote = this.isQuoteTweet(element);

    // Extract quoted tweet if present
    const quotedTweet = isQuote ? this.extractQuotedTweet(element) : undefined;

    // Extract reply info
    const replyToId = this.extractReplyToId(element);

    return {
      id: id || `tweet-${Date.now()}`,
      author,
      content,
      timestamp,
      media: media.length > 0 ? media : undefined,
      metrics,
      isRetweet,
      isQuote,
      quotedTweet,
      replyToId,
    };
  }

  /** Extract author information from tweet */
  private extractAuthor(element: Element): TweetContent['author'] | null {
    // Look for username/handle
    const usernameSelectors = [
      '[data-testid="User-Name"] a[href*="/"]',
      'a[href*="/"] span[class*="css"]',
      '[class*="username"]',
    ];

    let username = '';
    let displayName = '';

    // Find the user link
    const userLinks = element.querySelectorAll('a[role="link"]');
    for (const link of userLinks) {
      const href = link.getAttribute('href') || '';
      const match = href.match(/^\/(\w+)$/);
      if (match && !['home', 'explore', 'notifications', 'messages', 'settings'].includes(match[1])) {
        username = match[1];
        // Display name is usually near the username
        const parent = link.closest('[data-testid="User-Name"]');
        if (parent) {
          const spans = parent.querySelectorAll('span');
          for (const span of spans) {
            const text = span.textContent?.trim() || '';
            if (text && !text.startsWith('@') && text.length < 50) {
              displayName = text;
              break;
            }
          }
        }
        break;
      }
    }

    // Fallback: try to find in other places
    if (!username) {
      for (const selector of usernameSelectors) {
        const el = element.querySelector(selector);
        if (el) {
          const href = el.getAttribute('href') || '';
          const match = href.match(/\/(\w+)$/);
          if (match) {
            username = match[1];
            break;
          }
        }
      }
    }

    if (!username) return null;

    // Get avatar URL
    const avatarEl = element.querySelector('img[src*="profile_images"]');
    const avatarUrl = avatarEl?.getAttribute('src') || undefined;

    // Check for verification badge
    const verified = element.querySelector('[data-testid="icon-verified"], svg[aria-label*="Verified"]') !== null;

    return {
      username,
      displayName: displayName || username,
      avatarUrl,
      verified,
    };
  }

  /** Extract tweet text content */
  private extractTweetContent(element: Element): string {
    const contentSelectors = [
      '[data-testid="tweetText"]',
      '[lang]',
      '.tweet-text',
    ];

    for (const selector of contentSelectors) {
      const contentEl = element.querySelector(selector);
      if (contentEl) {
        return this.cleanText(contentEl.textContent || '');
      }
    }

    return '';
  }

  /** Extract tweet ID from element or link */
  private extractTweetId(element: Element): string | undefined {
    // Look for status links
    const links = element.querySelectorAll('a[href*="/status/"]');
    for (const link of links) {
      const href = link.getAttribute('href') || '';
      const match = href.match(/\/status\/(\d+)/);
      if (match) {
        return match[1];
      }
    }

    // Check time element link
    const timeLink = element.querySelector('time')?.closest('a');
    if (timeLink) {
      const href = timeLink.getAttribute('href') || '';
      const match = href.match(/\/status\/(\d+)/);
      if (match) {
        return match[1];
      }
    }

    return undefined;
  }

  /** Extract timestamp from tweet */
  private extractTimestamp(element: Element): string | undefined {
    const timeEl = element.querySelector('time');
    if (timeEl) {
      return timeEl.getAttribute('datetime') || timeEl.textContent || undefined;
    }
    return undefined;
  }

  /** Extract media from tweet */
  private extractMedia(element: Element): MediaContent[] {
    const media: MediaContent[] = [];

    // Images
    const images = element.querySelectorAll('[data-testid="tweetPhoto"] img, img[src*="pbs.twimg.com/media"]');
    images.forEach(img => {
      const src = img.getAttribute('src');
      if (src && !src.includes('profile_images')) {
        media.push({
          type: 'image',
          url: src,
          alt: img.getAttribute('alt') || undefined,
        });
      }
    });

    // Videos
    const videos = element.querySelectorAll('video');
    videos.forEach(video => {
      const src = video.getAttribute('src') || video.querySelector('source')?.getAttribute('src');
      if (src) {
        media.push({
          type: 'video',
          url: src,
          thumbnailUrl: video.getAttribute('poster') || undefined,
        });
      }
    });

    // Video thumbnails (when video hasn't loaded)
    const videoThumbs = element.querySelectorAll('[data-testid="videoPlayer"] img');
    videoThumbs.forEach(thumb => {
      const src = thumb.getAttribute('src');
      if (src) {
        media.push({
          type: 'video',
          url: '', // Video URL not available without interaction
          thumbnailUrl: src,
        });
      }
    });

    return media;
  }

  /** Extract engagement metrics */
  private extractMetrics(element: Element): TweetContent['metrics'] {
    const metrics: TweetContent['metrics'] = {};

    // Reply count
    const replyButton = element.querySelector('[data-testid="reply"]');
    if (replyButton) {
      const count = this.parseMetricCount(replyButton.textContent);
      if (count !== undefined) metrics.replies = count;
    }

    // Retweet count
    const retweetButton = element.querySelector('[data-testid="retweet"], [data-testid="unretweet"]');
    if (retweetButton) {
      const count = this.parseMetricCount(retweetButton.textContent);
      if (count !== undefined) metrics.retweets = count;
    }

    // Like count
    const likeButton = element.querySelector('[data-testid="like"], [data-testid="unlike"]');
    if (likeButton) {
      const count = this.parseMetricCount(likeButton.textContent);
      if (count !== undefined) metrics.likes = count;
    }

    // View count
    const viewElement = element.querySelector('[href*="/analytics"], a[href$="/analytics"]');
    if (viewElement) {
      const count = this.parseMetricCount(viewElement.textContent);
      if (count !== undefined) metrics.views = count;
    }

    return Object.keys(metrics).length > 0 ? metrics : undefined;
  }

  /** Parse metric count from text (handles K, M suffixes) */
  private parseMetricCount(text: string | null): number | undefined {
    if (!text) return undefined;
    
    const match = text.match(/([\d.]+)([KMB])?/i);
    if (!match) return undefined;

    let count = parseFloat(match[1]);
    const suffix = match[2]?.toUpperCase();

    if (suffix === 'K') count *= 1000;
    else if (suffix === 'M') count *= 1000000;
    else if (suffix === 'B') count *= 1000000000;

    return Math.round(count);
  }

  /** Check if tweet is a retweet */
  private isRetweet(element: Element): boolean {
    const retweetIndicators = [
      '[data-testid="socialContext"]',
      '[class*="retweet"]',
    ];

    for (const selector of retweetIndicators) {
      const el = element.querySelector(selector);
      if (el?.textContent?.toLowerCase().includes('retweet')) {
        return true;
      }
    }

    return false;
  }

  /** Check if tweet is a quote tweet */
  private isQuoteTweet(element: Element): boolean {
    // Quote tweets have an embedded tweet
    return element.querySelector('[data-testid="quoteTweet"]') !== null ||
           element.querySelectorAll('article').length > 1;
  }

  /** Extract quoted tweet from quote tweet */
  private extractQuotedTweet(element: Element): TweetContent | undefined {
    const quotedEl = element.querySelector('[data-testid="quoteTweet"], article article');
    if (quotedEl) {
      // Avoid infinite recursion by not processing nested quotes
      const nestedQuoted = this.extractTweetFromElement(quotedEl);
      if (nestedQuoted) {
        nestedQuoted.quotedTweet = undefined; // Remove any nested quotes
        return nestedQuoted;
      }
    }
    return undefined;
  }

  /** Extract reply-to tweet ID */
  private extractReplyToId(element: Element): string | undefined {
    // Look for "Replying to" indicator
    const replyIndicator = element.querySelector('[class*="reply"], [data-testid="reply"]');
    if (replyIndicator) {
      const links = replyIndicator.querySelectorAll('a[href*="/status/"]');
      for (const link of links) {
        const href = link.getAttribute('href') || '';
        const match = href.match(/\/status\/(\d+)/);
        if (match) {
          return match[1];
        }
      }
    }
    return undefined;
  }

  /** Extract conversation ID from URL */
  private extractConversationId(url: string): string | undefined {
    const match = url.match(/\/status\/(\d+)/);
    return match ? match[1] : undefined;
  }

  /** Extract all media from the page */
  private extractAllMedia(document: Document): MediaContent[] {
    const media: MediaContent[] = [];
    const seen = new Set<string>();

    // All images
    document.querySelectorAll('img[src*="pbs.twimg.com"]').forEach(img => {
      const src = img.getAttribute('src');
      if (src && !seen.has(src) && !src.includes('profile_images')) {
        seen.add(src);
        media.push({
          type: 'image',
          url: src,
          alt: img.getAttribute('alt') || undefined,
        });
      }
    });

    // All videos
    document.querySelectorAll('video').forEach(video => {
      const src = video.getAttribute('src');
      if (src && !seen.has(src)) {
        seen.add(src);
        media.push({
          type: 'video',
          url: src,
          thumbnailUrl: video.getAttribute('poster') || undefined,
        });
      }
    });

    return media;
  }

  /** Clean text by removing extra whitespace */
  private cleanText(text: string): string {
    return text
      .replace(/\s+/g, ' ')
      .trim();
  }
}
