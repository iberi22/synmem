# 🔌 SynMem Extension API Reference

This document describes the Chrome Extension API for SynMem, including the content scripts, background service worker, and native messaging integration.

## Overview

The SynMem Chrome Extension provides:
- **Content Scripts**: Site-specific scrapers and DOM manipulation
- **Background Worker**: Coordination and native messaging
- **Popup UI**: Quick access to common actions
- **Native Host Bridge**: Communication with the Rust backend

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      Chrome Extension                            │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │   Popup UI   │  │   Content    │  │  Background  │          │
│  │  (React/TS)  │  │   Scripts    │  │   Worker     │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│         │                 │                  │                  │
│         └─────────────────┼──────────────────┘                  │
│                           │                                      │
│                    Message Passing                               │
│                           │                                      │
└───────────────────────────┼─────────────────────────────────────┘
                            │
                    Native Messaging
                            │
                            ▼
               ┌────────────────────────┐
               │   SynMem Native Host   │
               │       (Rust)           │
               └────────────────────────┘
```

---

## Content Scripts API

### Universal Scraper

The universal scraper works on any webpage and extracts common content patterns.

```typescript
// src/content/scrapers/universal.ts

interface ScrapedContent {
  url: string;
  title: string;
  content: string;
  metadata: {
    description?: string;
    keywords?: string[];
    author?: string;
    publishedDate?: string;
  };
  links: Array<{
    text: string;
    href: string;
  }>;
  images: Array<{
    src: string;
    alt?: string;
  }>;
}

// Exported function for content extraction
export async function scrapeCurrentPage(): Promise<ScrapedContent>;
```

### Site-Specific Scrapers

#### ChatGPT Scraper

```typescript
// src/content/scrapers/chatgpt.ts

interface ChatGPTConversation {
  id: string;
  title: string;
  model: string;
  messages: Array<{
    role: 'user' | 'assistant' | 'system';
    content: string;
    timestamp?: Date;
  }>;
}

export async function scrapeConversation(): Promise<ChatGPTConversation>;
export async function getConversationList(): Promise<Array<{ id: string; title: string }>>;
```

#### Claude Scraper

```typescript
// src/content/scrapers/claude.ts

interface ClaudeConversation {
  id: string;
  title: string;
  messages: Array<{
    role: 'human' | 'assistant';
    content: string;
    timestamp?: Date;
  }>;
}

export async function scrapeConversation(): Promise<ClaudeConversation>;
```

#### Gemini Scraper

```typescript
// src/content/scrapers/gemini.ts

interface GeminiConversation {
  id: string;
  title: string;
  messages: Array<{
    role: 'user' | 'model';
    content: string;
  }>;
}

export async function scrapeConversation(): Promise<GeminiConversation>;
```

#### Twitter Scraper

```typescript
// src/content/scrapers/twitter.ts

interface Tweet {
  id: string;
  author: {
    handle: string;
    displayName: string;
    verified: boolean;
  };
  content: string;
  media?: Array<{
    type: 'image' | 'video' | 'gif';
    url: string;
  }>;
  metrics: {
    likes: number;
    retweets: number;
    replies: number;
    views?: number;
  };
  timestamp: Date;
}

interface TwitterThread {
  tweets: Tweet[];
  replies?: Tweet[];
}

export async function scrapeTweet(): Promise<Tweet>;
export async function scrapeThread(): Promise<TwitterThread>;
export async function scrapeTimeline(count?: number): Promise<Tweet[]>;
```

---

## Background Service Worker API

### Message Types

The background worker handles messages from content scripts and the popup:

```typescript
// src/background/types.ts

type MessageType =
  | 'SCRAPE_PAGE'
  | 'SCRAPE_CHAT'
  | 'NAVIGATE'
  | 'CLICK'
  | 'TYPE'
  | 'SCREENSHOT'
  | 'SAVE_SESSION'
  | 'GET_SESSION'
  | 'SEARCH_MEMORY'
  | 'NATIVE_COMMAND';

interface ExtensionMessage {
  type: MessageType;
  payload: unknown;
  tabId?: number;
}

interface ExtensionResponse {
  success: boolean;
  data?: unknown;
  error?: {
    code: string;
    message: string;
  };
}
```

### Message Handling

```typescript
// src/background/service-worker.ts

chrome.runtime.onMessage.addListener(
  (message: ExtensionMessage, sender, sendResponse) => {
    handleMessage(message, sender)
      .then(sendResponse)
      .catch(error => sendResponse({ success: false, error }));
    return true; // Keep channel open for async response
  }
);

async function handleMessage(
  message: ExtensionMessage,
  sender: chrome.runtime.MessageSender
): Promise<ExtensionResponse> {
  switch (message.type) {
    case 'SCRAPE_PAGE':
      return handleScrapePage(message.payload, sender.tab?.id);
    case 'NAVIGATE':
      return handleNavigate(message.payload, sender.tab?.id);
    // ... other handlers
  }
}
```

### Content Script Injection

```typescript
// Inject content script into a tab
async function injectContentScript(tabId: number): Promise<void> {
  await chrome.scripting.executeScript({
    target: { tabId },
    files: ['content/index.js']
  });
}

// Send message to content script
async function sendToContentScript<T>(
  tabId: number,
  message: ExtensionMessage
): Promise<T> {
  return chrome.tabs.sendMessage(tabId, message);
}
```

---

## Native Messaging API

### Native Host Protocol

Messages are exchanged as JSON over stdio with length-prefixed framing:

```
[4 bytes: message length (little-endian)][JSON message]
```

### Message Format

```typescript
// Messages from Extension to Native Host
interface NativeRequest {
  id: string;        // Unique request ID for correlation
  method: string;    // Method name (e.g., "navigate", "scrape")
  params: unknown;   // Method-specific parameters
}

// Messages from Native Host to Extension
interface NativeResponse {
  id: string;        // Matching request ID
  result?: unknown;  // Success result
  error?: {
    code: number;
    message: string;
  };
}
```

### Native Host Bridge

```typescript
// src/native-host/bridge.ts

class NativeHostBridge {
  private port: chrome.runtime.Port | null = null;
  private pendingRequests: Map<string, {
    resolve: (value: unknown) => void;
    reject: (error: Error) => void;
  }> = new Map();

  connect(): void {
    this.port = chrome.runtime.connectNative('com.synmem.host');
    this.port.onMessage.addListener(this.handleMessage.bind(this));
    this.port.onDisconnect.addListener(this.handleDisconnect.bind(this));
  }

  async send<T>(method: string, params: unknown): Promise<T> {
    if (!this.port) {
      throw new Error('Native host not connected');
    }

    const id = crypto.randomUUID();
    const request: NativeRequest = { id, method, params };

    return new Promise((resolve, reject) => {
      this.pendingRequests.set(id, { resolve, reject });
      this.port!.postMessage(request);

      // Timeout after 30 seconds
      setTimeout(() => {
        if (this.pendingRequests.has(id)) {
          this.pendingRequests.delete(id);
          reject(new Error('Request timeout'));
        }
      }, 30000);
    });
  }

  private handleMessage(response: NativeResponse): void {
    const pending = this.pendingRequests.get(response.id);
    if (!pending) return;

    this.pendingRequests.delete(response.id);

    if (response.error) {
      pending.reject(new Error(response.error.message));
    } else {
      pending.resolve(response.result);
    }
  }

  private handleDisconnect(): void {
    const error = chrome.runtime.lastError;
    console.error('Native host disconnected:', error?.message);
    
    // Reject all pending requests
    for (const [id, pending] of this.pendingRequests) {
      pending.reject(new Error('Native host disconnected'));
      this.pendingRequests.delete(id);
    }
    
    this.port = null;
  }

  disconnect(): void {
    if (this.port) {
      this.port.disconnect();
      this.port = null;
    }
  }
}

export const nativeHost = new NativeHostBridge();
```

---

## Popup UI API

### State Management

```typescript
// src/popup/state.ts

interface PopupState {
  isConnected: boolean;
  currentTab: {
    url: string;
    title: string;
    canScrape: boolean;
  } | null;
  recentCaptures: Array<{
    id: string;
    type: 'page' | 'chat';
    title: string;
    timestamp: Date;
  }>;
  settings: {
    autoCapture: boolean;
    captureChats: boolean;
    capturePages: boolean;
  };
}
```

### Actions

```typescript
// src/popup/actions.ts

// Capture current page
async function captureCurrentPage(): Promise<void> {
  const tab = await getCurrentTab();
  await chrome.runtime.sendMessage({
    type: 'SCRAPE_PAGE',
    tabId: tab.id
  });
}

// Toggle auto-capture
async function toggleAutoCapture(enabled: boolean): Promise<void> {
  await chrome.storage.sync.set({ autoCapture: enabled });
}

// Open memory search
async function openMemorySearch(): Promise<void> {
  await chrome.tabs.create({
    url: chrome.runtime.getURL('search.html')
  });
}
```

---

## Events

### Extension Events

```typescript
// Tab update events (for auto-capture)
chrome.tabs.onUpdated.addListener((tabId, changeInfo, tab) => {
  if (changeInfo.status === 'complete' && tab.url) {
    handlePageLoad(tabId, tab.url);
  }
});

// Window focus events
chrome.windows.onFocusChanged.addListener((windowId) => {
  if (windowId !== chrome.windows.WINDOW_ID_NONE) {
    handleWindowFocus(windowId);
  }
});

// Storage change events
chrome.storage.onChanged.addListener((changes, areaName) => {
  if (areaName === 'sync') {
    handleSettingsChange(changes);
  }
});
```

### Custom Events

```typescript
// Dispatch custom event from content script
function dispatchSynMemEvent(type: string, detail: unknown): void {
  window.dispatchEvent(
    new CustomEvent('synmem', { detail: { type, ...detail } })
  );
}

// Listen for custom events in content script
window.addEventListener('synmem', (event: CustomEvent) => {
  handleSynMemEvent(event.detail);
});
```

---

## Storage

### Storage Schema

```typescript
// Local storage (device-specific)
interface LocalStorage {
  sessions: Record<string, Session>;
  cache: Record<string, CachedPage>;
  nativeHostPath: string;
}

// Sync storage (synced across devices)
interface SyncStorage {
  settings: Settings;
  preferences: Preferences;
  macros: Macro[];
}
```

### Storage Usage

```typescript
// Get settings
const { settings } = await chrome.storage.sync.get('settings');

// Save settings
await chrome.storage.sync.set({ settings: newSettings });

// Get session from local storage
const { sessions } = await chrome.storage.local.get('sessions');

// Clear cache
await chrome.storage.local.remove('cache');
```

---

## Permissions

### Required Permissions

```json
{
  "permissions": [
    "activeTab",
    "storage",
    "scripting",
    "nativeMessaging"
  ],
  "host_permissions": [
    "https://*.twitter.com/*",
    "https://*.x.com/*",
    "https://chat.openai.com/*",
    "https://claude.ai/*",
    "https://gemini.google.com/*"
  ],
  "optional_host_permissions": [
    "<all_urls>"
  ]
}
```

### Permission Requests

```typescript
// Request additional permissions
async function requestPermission(origins: string[]): Promise<boolean> {
  return chrome.permissions.request({
    origins
  });
}

// Check if permission is granted
async function hasPermission(origin: string): Promise<boolean> {
  return chrome.permissions.contains({
    origins: [origin]
  });
}
```

---

## Error Handling

### Error Types

```typescript
enum ExtensionErrorCode {
  NATIVE_HOST_UNAVAILABLE = 'NATIVE_HOST_UNAVAILABLE',
  PERMISSION_DENIED = 'PERMISSION_DENIED',
  SCRAPING_FAILED = 'SCRAPING_FAILED',
  TIMEOUT = 'TIMEOUT',
  INVALID_TAB = 'INVALID_TAB',
  STORAGE_ERROR = 'STORAGE_ERROR'
}

class ExtensionError extends Error {
  constructor(
    public code: ExtensionErrorCode,
    message: string,
    public details?: unknown
  ) {
    super(message);
    this.name = 'ExtensionError';
  }
}
```

### Error Handling Pattern

```typescript
async function safeOperation<T>(
  operation: () => Promise<T>,
  fallback?: T
): Promise<T> {
  try {
    return await operation();
  } catch (error) {
    console.error('Operation failed:', error);
    
    if (fallback !== undefined) {
      return fallback;
    }
    
    throw error;
  }
}
```

---

## Development

### Building the Extension

```bash
# Install dependencies
cd extension
npm install

# Development build with watch
npm run dev

# Production build
npm run build
```

### Loading in Chrome

1. Open `chrome://extensions`
2. Enable "Developer mode"
3. Click "Load unpacked"
4. Select the `extension/dist` folder

### Debugging

- **Background Worker**: `chrome://extensions` → SynMem → "Inspect views: service worker"
- **Content Scripts**: Open DevTools on any page → Console
- **Popup**: Right-click extension icon → "Inspect popup"
