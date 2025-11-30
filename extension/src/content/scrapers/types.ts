/**
 * Base types and interfaces for site-specific scrapers
 */

/** Metadata extracted from a page */
export interface PageMetadata {
  title?: string;
  description?: string;
  author?: string;
  publishedDate?: string;
  modifiedDate?: string;
  canonicalUrl?: string;
  language?: string;
  keywords?: string[];
  ogImage?: string;
  siteName?: string;
}

/** Structured data (JSON-LD, microdata, etc.) */
export interface StructuredData {
  type: string;
  data: Record<string, unknown>;
}

/** Code block extracted from content */
export interface CodeBlock {
  language?: string;
  code: string;
  filename?: string;
}

/** Media content (images, videos, etc.) */
export interface MediaContent {
  type: 'image' | 'video' | 'audio' | 'embed';
  url: string;
  alt?: string;
  caption?: string;
  thumbnailUrl?: string;
}

/** Chat message role */
export type ChatRole = 'user' | 'assistant' | 'system' | 'thinking';

/** A single chat message */
export interface ChatMessage {
  role: ChatRole;
  content: string;
  codeBlocks?: CodeBlock[];
  timestamp?: string;
  modelInfo?: string;
}

/** Artifact from Claude */
export interface Artifact {
  id: string;
  type: string;
  title?: string;
  content: string;
  language?: string;
}

/** Tweet/post content */
export interface TweetContent {
  id: string;
  author: {
    username: string;
    displayName: string;
    avatarUrl?: string;
    verified?: boolean;
  };
  content: string;
  timestamp?: string;
  media?: MediaContent[];
  metrics?: {
    likes?: number;
    retweets?: number;
    replies?: number;
    views?: number;
  };
  isRetweet?: boolean;
  isQuote?: boolean;
  quotedTweet?: TweetContent;
  replyToId?: string;
}

/** Thread of tweets */
export interface TweetThread {
  tweets: TweetContent[];
  conversationId?: string;
}

/** Universal scraped content */
export interface ScrapedContent {
  url: string;
  timestamp: string;
  scraperType: string;
  metadata: PageMetadata;
  structuredData?: StructuredData[];
  mainContent?: string;
  textContent?: string;
  chatMessages?: ChatMessage[];
  artifacts?: Artifact[];
  tweetThread?: TweetThread;
  codeBlocks?: CodeBlock[];
  media?: MediaContent[];
  raw?: string;
}

/** Base Scraper interface */
export interface Scraper {
  /** Unique identifier for this scraper */
  readonly name: string;

  /** Check if this scraper can handle the given URL */
  canHandle(url: string): boolean;

  /** Extract content from the document */
  extract(document: Document): ScrapedContent;
}
