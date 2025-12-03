/**
 * Real-time Sync Service
 * 
 * Implements:
 * - MutationObserver for DOM changes
 * - Debounced sync to native host
 * - Session state management
 * 
 * Flow:
 * [Page DOM Change] → MutationObserver → Debounce (500ms) → Content Script
 * → Native Messaging → Rust Host → SQLite
 */

import type {
  Message,
  ScrapedPage,
  PageMetadata,
  MutationRecord as SynmemMutationRecord,
  SessionState,
} from '../types';

// Default debounce delay in milliseconds
const DEFAULT_DEBOUNCE_DELAY = 500;

type SendMessageFn = (message: Message) => void;

/**
 * Debounce utility function
 */
function debounce<T extends (...args: unknown[]) => void>(
  fn: T,
  delay: number
): (...args: Parameters<T>) => void {
  let timeoutId: ReturnType<typeof setTimeout> | null = null;
  
  return (...args: Parameters<T>) => {
    if (timeoutId) {
      clearTimeout(timeoutId);
    }
    timeoutId = setTimeout(() => {
      fn(...args);
      timeoutId = null;
    }, delay);
  };
}

/**
 * Generate a CSS selector path for an element
 */
function getElementPath(element: Element): string {
  const parts: string[] = [];
  let current: Element | null = element;
  
  while (current && current !== document.body) {
    let selector = current.tagName.toLowerCase();
    
    if (current.id) {
      selector += `#${current.id}`;
      parts.unshift(selector);
      break;
    }
    
    if (current.className && typeof current.className === 'string') {
      const classes = current.className.trim().split(/\s+/).slice(0, 2);
      if (classes.length > 0) {
        selector += `.${classes.join('.')}`;
      }
    }
    
    // Add index if there are siblings with same tag
    const parent = current.parentElement;
    if (parent) {
      const siblings = Array.from(parent.children).filter(
        child => child.tagName === current!.tagName
      );
      if (siblings.length > 1) {
        const index = siblings.indexOf(current) + 1;
        selector += `:nth-of-type(${index})`;
      }
    }
    
    parts.unshift(selector);
    current = current.parentElement;
  }
  
  return parts.join(' > ');
}

/**
 * Convert native MutationRecord to our serializable format
 */
function convertMutationRecord(mutation: MutationRecord): SynmemMutationRecord {
  const record: SynmemMutationRecord = {
    type: mutation.type,
    target: mutation.target instanceof Element 
      ? getElementPath(mutation.target) 
      : 'text-node',
  };

  if (mutation.type === 'childList') {
    record.addedNodes = Array.from(mutation.addedNodes)
      .filter((node): node is Element => node instanceof Element)
      .map(getElementPath)
      .slice(0, 10); // Limit to prevent huge payloads
    
    record.removedNodes = Array.from(mutation.removedNodes)
      .filter((node): node is Element => node instanceof Element)
      .map(getElementPath)
      .slice(0, 10);
  }

  if (mutation.type === 'attributes') {
    record.attributeName = mutation.attributeName ?? undefined;
    record.oldValue = mutation.oldValue ?? undefined;
    
    if (mutation.target instanceof Element && mutation.attributeName) {
      record.newValue = mutation.target.getAttribute(mutation.attributeName) ?? undefined;
    }
  }

  if (mutation.type === 'characterData') {
    record.oldValue = mutation.oldValue ?? undefined;
    record.newValue = mutation.target.textContent ?? undefined;
  }

  return record;
}

/**
 * Extract page metadata from document
 */
function extractPageMetadata(): PageMetadata {
  const getMeta = (name: string): string | undefined => {
    const el = document.querySelector(`meta[name="${name}"], meta[property="${name}"]`);
    return el?.getAttribute('content') ?? undefined;
  };

  return {
    description: getMeta('description') || getMeta('og:description'),
    keywords: getMeta('keywords')?.split(',').map(k => k.trim()),
    author: getMeta('author'),
    canonicalUrl: document.querySelector<HTMLLinkElement>('link[rel="canonical"]')?.href,
    ogImage: getMeta('og:image'),
    favicon: document.querySelector<HTMLLinkElement>('link[rel="icon"]')?.href ??
             document.querySelector<HTMLLinkElement>('link[rel="shortcut icon"]')?.href,
  };
}

/**
 * Extract main content from page
 */
function extractPageContent(): string {
  // Try to find main content area
  const mainSelectors = [
    'main',
    '[role="main"]',
    '#content',
    '#main-content',
    '.main-content',
    'article',
  ];

  for (const selector of mainSelectors) {
    const el = document.querySelector(selector);
    if (el) {
      return el.textContent?.trim().slice(0, 50000) ?? '';
    }
  }

  // Fallback to body content
  return document.body.textContent?.trim().slice(0, 50000) ?? '';
}

/**
 * Create scraped page object from current document
 */
function createScrapedPage(): ScrapedPage {
  return {
    url: window.location.href,
    title: document.title,
    content: extractPageContent(),
    metadata: extractPageMetadata(),
    timestamp: Date.now(),
  };
}

/**
 * RealTimeSyncService manages DOM observation and syncing
 */
export class RealTimeSyncService {
  private observer: MutationObserver | null = null;
  private sendMessage: SendMessageFn;
  private debounceDelay: number;
  private isObserving = false;
  private pendingMutations: SynmemMutationRecord[] = [];
  private sessionId: string;
  
  private debouncedSync: () => void;

  constructor(sendMessage: SendMessageFn, debounceDelay = DEFAULT_DEBOUNCE_DELAY) {
    this.sendMessage = sendMessage;
    this.debounceDelay = debounceDelay;
    this.sessionId = this.generateSessionId();
    
    this.debouncedSync = debounce(() => {
      this.syncToNativeHost();
    }, this.debounceDelay);
  }

  private generateSessionId(): string {
    return `session-${Date.now()}-${Math.random().toString(36).slice(2, 11)}`;
  }

  /**
   * Start observing DOM changes
   */
  start(): void {
    if (this.isObserving) {
      return;
    }

    // Send initial page scrape
    this.sendPageScraped();

    this.observer = new MutationObserver((mutations) => {
      this.handleMutations(mutations);
    });

    this.observer.observe(document.body, {
      childList: true,
      subtree: true,
      attributes: true,
      attributeOldValue: true,
      characterData: true,
      characterDataOldValue: true,
    });

    this.isObserving = true;
    console.log('[SynMem] Real-time sync started');
  }

  /**
   * Stop observing DOM changes
   */
  stop(): void {
    if (this.observer) {
      this.observer.disconnect();
      this.observer = null;
    }
    
    this.isObserving = false;
    this.pendingMutations = [];
    console.log('[SynMem] Real-time sync stopped');
  }

  /**
   * Handle mutation events
   */
  private handleMutations(mutations: MutationRecord[]): void {
    // Filter out mutations that are not significant
    const significantMutations = mutations.filter(mutation => {
      // Skip script and style changes
      if (mutation.target instanceof Element) {
        const tagName = mutation.target.tagName.toLowerCase();
        if (tagName === 'script' || tagName === 'style' || tagName === 'noscript') {
          return false;
        }
      }
      
      // Skip empty childList mutations
      if (mutation.type === 'childList' && 
          mutation.addedNodes.length === 0 && 
          mutation.removedNodes.length === 0) {
        return false;
      }
      
      return true;
    });

    if (significantMutations.length === 0) {
      return;
    }

    // Convert and queue mutations
    const records = significantMutations.map(convertMutationRecord);
    this.pendingMutations.push(...records);

    // Limit pending mutations to prevent memory issues
    if (this.pendingMutations.length > 100) {
      this.pendingMutations = this.pendingMutations.slice(-100);
    }

    // Trigger debounced sync
    this.debouncedSync();
  }

  /**
   * Sync pending changes to native host
   */
  private syncToNativeHost(): void {
    if (this.pendingMutations.length === 0) {
      return;
    }

    // Send DOM changes
    this.sendMessage({
      type: 'DOM_CHANGED',
      payload: {
        url: window.location.href,
        changes: [...this.pendingMutations],
        timestamp: Date.now(),
      },
    });

    // Clear pending mutations
    this.pendingMutations = [];

    // Also send updated page content
    this.sendPageScraped();
  }

  /**
   * Send scraped page content
   */
  private sendPageScraped(): void {
    this.sendMessage({
      type: 'PAGE_SCRAPED',
      payload: createScrapedPage(),
    });
  }

  /**
   * Save and sync session state
   */
  saveSession(): void {
    const sessionState: SessionState = {
      sessionId: this.sessionId,
      url: window.location.href,
      startTime: Date.now(),
      lastActivity: Date.now(),
      pages: [createScrapedPage()],
      chatMessages: [],
    };

    this.sendMessage({
      type: 'SESSION_SAVED',
      payload: sessionState,
    });
  }

  /**
   * Get current session ID
   */
  getSessionId(): string {
    return this.sessionId;
  }
}

// Export factory function
export function createRealTimeSyncService(
  sendMessage: SendMessageFn,
  debounceDelay?: number
): RealTimeSyncService {
  return new RealTimeSyncService(sendMessage, debounceDelay);
}
