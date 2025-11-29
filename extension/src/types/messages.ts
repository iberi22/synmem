/**
 * Message types for real-time sync between extension and native host.
 * Implements bi-directional communication for synmem browser automation.
 */

// ============================================
// Scraped Page Types
// ============================================

export interface ScrapedPage {
  url: string;
  title: string;
  content: string;
  metadata: PageMetadata;
  timestamp: number;
}

export interface PageMetadata {
  description?: string;
  keywords?: string[];
  author?: string;
  canonicalUrl?: string;
  ogImage?: string;
  favicon?: string;
}

// ============================================
// Chat Message Types (for AI chat scrapers)
// ============================================

export interface ChatMessage {
  id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: number;
  metadata?: ChatMessageMetadata;
}

export interface ChatMessageMetadata {
  model?: string;
  tokens?: number;
  isStreaming?: boolean;
}

// ============================================
// Session State Types
// ============================================

export interface SessionState {
  sessionId: string;
  url: string;
  startTime: number;
  lastActivity: number;
  pages: ScrapedPage[];
  chatMessages: ChatMessage[];
  cookies?: SessionCookie[];
  localStorage?: Record<string, string>;
}

export interface SessionCookie {
  name: string;
  value: string;
  domain: string;
  path: string;
  secure: boolean;
  httpOnly: boolean;
  expirationDate?: number;
}

// ============================================
// Command Types
// ============================================

export type Command =
  | { action: 'NAVIGATE'; payload: { url: string } }
  | { action: 'CLICK'; payload: { selector: string } }
  | { action: 'TYPE'; payload: { selector: string; text: string } }
  | { action: 'SCROLL'; payload: { x: number; y: number } }
  | { action: 'SCREENSHOT'; payload: { fullPage?: boolean } }
  | { action: 'WAIT'; payload: { selector?: string; timeout?: number } }
  | { action: 'SCRAPE'; payload: { selectors?: string[] } }
  | { action: 'GET_SESSION'; payload: Record<string, never> }
  | { action: 'PING'; payload: Record<string, never> };

// ============================================
// Message Types (as specified in issue)
// ============================================

export type Message =
  | { type: 'PAGE_SCRAPED'; payload: ScrapedPage }
  | { type: 'CHAT_UPDATED'; payload: ChatMessage[] }
  | { type: 'SESSION_SAVED'; payload: SessionState }
  | { type: 'COMMAND'; payload: Command }
  | { type: 'DOM_CHANGED'; payload: DOMChange }
  | { type: 'ERROR'; payload: ErrorPayload }
  | { type: 'PONG'; payload: { timestamp: number } }
  | { type: 'CONNECTED'; payload: { version: string } }
  | { type: 'DISCONNECTED'; payload: { reason: string } };

// ============================================
// DOM Change Types (for MutationObserver)
// ============================================

export interface DOMChange {
  url: string;
  changes: MutationRecord[];
  timestamp: number;
}

export interface MutationRecord {
  type: 'childList' | 'attributes' | 'characterData';
  target: string; // CSS selector path
  addedNodes?: string[];
  removedNodes?: string[];
  attributeName?: string;
  oldValue?: string;
  newValue?: string;
}

// ============================================
// Error Types
// ============================================

export interface ErrorPayload {
  code: ErrorCode;
  message: string;
  details?: unknown;
  timestamp: number;
}

export type ErrorCode =
  | 'CONNECTION_FAILED'
  | 'NATIVE_HOST_NOT_FOUND'
  | 'INVALID_MESSAGE'
  | 'TIMEOUT'
  | 'PERMISSION_DENIED'
  | 'UNKNOWN';

// ============================================
// Connection State Types
// ============================================

export type ConnectionState = 'disconnected' | 'connecting' | 'connected' | 'reconnecting';

export interface ConnectionConfig {
  reconnectAttempts: number;
  reconnectDelay: number;
  maxReconnectDelay: number;
  pingInterval: number;
  debounceDelay: number;
}

export const DEFAULT_CONNECTION_CONFIG: ConnectionConfig = {
  reconnectAttempts: 5,
  reconnectDelay: 1000,
  maxReconnectDelay: 30000,
  pingInterval: 30000,
  debounceDelay: 500,
};
