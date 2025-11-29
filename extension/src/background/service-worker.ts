/**
 * Background Service Worker
 * 
 * Manages native host connection and routes messages between content scripts
 * and the native host.
 */

import { getNativeHostBridge, NativeHostBridge } from '../native-host';
import type { Message, ConnectionState, ErrorCode } from '../types';

// Native host bridge instance
let bridge: NativeHostBridge;

// Track active content script tabs
const activeTabs = new Map<number, { url: string; lastActivity: number }>();

/**
 * Initialize background service worker
 */
function initialize(): void {
  bridge = getNativeHostBridge();
  
  // Set up message handlers
  setupNativeHostHandlers();
  setupContentScriptHandlers();
  setupContextMenus();
  
  // Connect to native host
  connectToNativeHost();
  
  console.log('[SynMem Background] Initialized');
}

/**
 * Connect to native host with error handling
 */
async function connectToNativeHost(): Promise<void> {
  try {
    await bridge.connect();
    console.log('[SynMem Background] Connected to native host');
    
    // Notify all active tabs
    broadcastToContentScripts({
      type: 'CONNECTED',
      payload: { version: '1.0.0' },
    });
  } catch (error) {
    console.error('[SynMem Background] Failed to connect to native host:', error);
    
    // Notify popup about connection failure
    broadcastToContentScripts({
      type: 'ERROR',
      payload: {
        code: 'CONNECTION_FAILED' as ErrorCode,
        message: 'Failed to connect to native host',
        timestamp: Date.now(),
      },
    });
  }
}

/**
 * Set up native host message handlers
 */
function setupNativeHostHandlers(): void {
  // Handle incoming messages from native host
  bridge.onMessage((message: Message) => {
    console.log('[SynMem Background] Message from native host:', message.type);
    
    switch (message.type) {
      case 'COMMAND':
        // Route command to appropriate content script
        routeCommandToTab(message);
        break;
        
      default:
        // Broadcast other messages to all content scripts
        broadcastToContentScripts(message);
    }
  });
  
  // Handle connection errors
  bridge.onError(({ code, message }) => {
    console.error(`[SynMem Background] Native host error [${code}]:`, message);
    
    broadcastToContentScripts({
      type: 'ERROR',
      payload: {
        code,
        message,
        timestamp: Date.now(),
      },
    });
  });
  
  // Handle state changes
  bridge.onStateChange((state: ConnectionState) => {
    console.log('[SynMem Background] Connection state:', state);
    
    // Update extension badge based on connection state
    updateBadge(state);
    
    if (state === 'disconnected') {
      broadcastToContentScripts({
        type: 'DISCONNECTED',
        payload: { reason: 'Connection lost' },
      });
    }
  });
}

/**
 * Set up content script message handlers
 */
function setupContentScriptHandlers(): void {
  chrome.runtime.onMessage.addListener((message: Message, sender, sendResponse) => {
    const tabId = sender.tab?.id;
    const url = sender.tab?.url;
    
    if (tabId && url) {
      // Track active tab
      activeTabs.set(tabId, { url, lastActivity: Date.now() });
    }
    
    console.log('[SynMem Background] Message from content script:', message.type);
    
    // Forward message to native host
    const sent = bridge.sendMessage(message);
    sendResponse({ sent, queued: !sent });
    
    return false; // Synchronous response
  });
  
  // Clean up when tab is closed
  chrome.tabs.onRemoved.addListener((tabId) => {
    activeTabs.delete(tabId);
  });
}

/**
 * Route command to specific tab
 */
function routeCommandToTab(message: Message): void {
  // If command has a target tab, send only to that tab
  // Otherwise, send to the currently active tab
  chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
    const tab = tabs[0];
    if (tab?.id) {
      chrome.tabs.sendMessage(tab.id, message).catch((error) => {
        console.error('[SynMem Background] Failed to route command:', error);
      });
    }
  });
}

/**
 * Broadcast message to all content scripts
 */
function broadcastToContentScripts(message: Message): void {
  activeTabs.forEach((_, tabId) => {
    chrome.tabs.sendMessage(tabId, message).catch(() => {
      // Tab might be closed or not ready
      activeTabs.delete(tabId);
    });
  });
}

/**
 * Update extension badge based on connection state
 */
function updateBadge(state: ConnectionState): void {
  const colors: Record<ConnectionState, string> = {
    connected: '#4CAF50',
    connecting: '#FFC107',
    reconnecting: '#FFC107',
    disconnected: '#F44336',
  };
  
  const texts: Record<ConnectionState, string> = {
    connected: '',
    connecting: '...',
    reconnecting: '...',
    disconnected: '!',
  };
  
  chrome.action.setBadgeBackgroundColor({ color: colors[state] });
  chrome.action.setBadgeText({ text: texts[state] });
}

/**
 * Set up context menus
 */
function setupContextMenus(): void {
  chrome.contextMenus.create({
    id: 'synmem-scrape-page',
    title: 'Save this page to SynMem',
    contexts: ['page'],
  });
  
  chrome.contextMenus.create({
    id: 'synmem-scrape-selection',
    title: 'Save selection to SynMem',
    contexts: ['selection'],
  });
  
  chrome.contextMenus.onClicked.addListener((info, tab) => {
    if (!tab?.id) return;
    
    switch (info.menuItemId) {
      case 'synmem-scrape-page':
        chrome.tabs.sendMessage(tab.id, {
          type: 'COMMAND',
          payload: { action: 'SCRAPE', payload: {} },
        });
        break;
        
      case 'synmem-scrape-selection':
        if (info.selectionText) {
          bridge.sendMessage({
            type: 'PAGE_SCRAPED',
            payload: {
              url: tab.url || '',
              title: `Selection from: ${tab.title || 'Unknown'}`,
              content: info.selectionText,
              metadata: {},
              timestamp: Date.now(),
            },
          });
        }
        break;
    }
  });
}

// Initialize on install/update
chrome.runtime.onInstalled.addListener(() => {
  initialize();
});

// Initialize on startup
chrome.runtime.onStartup.addListener(() => {
  initialize();
});

// Export for testing
export { bridge, activeTabs, connectToNativeHost };
