/**
 * SynMem Service Worker (Background Script)
 * 
 * Handles:
 * - Native messaging communication with SynMem host
 * - Message routing between content scripts and native host
 * - Extension lifecycle events
 */

const NATIVE_HOST_NAME = 'com.synmem.host';

/**
 * Message types for communication between extension components
 */
interface Message {
  type: string;
  payload?: unknown;
}

interface NativeMessage {
  action: string;
  data?: unknown;
}

/**
 * Native messaging port for communication with Rust host
 */
let nativePort: chrome.runtime.Port | null = null;

/**
 * Connect to the native messaging host
 */
function connectToNativeHost(): chrome.runtime.Port | null {
  try {
    nativePort = chrome.runtime.connectNative(NATIVE_HOST_NAME);
    
    nativePort.onMessage.addListener((message: NativeMessage) => {
      console.log('[SynMem] Received from native host:', message);
      handleNativeMessage(message);
    });
    
    nativePort.onDisconnect.addListener(() => {
      console.log('[SynMem] Native host disconnected');
      if (chrome.runtime.lastError) {
        console.error('[SynMem] Disconnect error:', chrome.runtime.lastError.message);
      }
      nativePort = null;
    });
    
    console.log('[SynMem] Connected to native host');
    return nativePort;
  } catch (error) {
    console.error('[SynMem] Failed to connect to native host:', error);
    return null;
  }
}

/**
 * Send message to native host
 */
function sendToNativeHost(message: NativeMessage): boolean {
  if (!nativePort) {
    nativePort = connectToNativeHost();
  }
  
  if (nativePort) {
    try {
      nativePort.postMessage(message);
      return true;
    } catch (error) {
      console.error('[SynMem] Failed to send message to native host:', error);
      return false;
    }
  }
  
  return false;
}

/**
 * Handle messages from native host
 */
function handleNativeMessage(message: NativeMessage): void {
  // Forward relevant messages to content scripts or popup
  switch (message.action) {
    case 'scrape_result':
      // Broadcast to interested content scripts
      broadcastToContentScripts({ type: 'SCRAPE_RESULT', payload: message.data });
      break;
    case 'navigate':
      handleNavigate(message.data as { url: string });
      break;
    default:
      console.log('[SynMem] Unhandled native message:', message.action);
  }
}

/**
 * Handle navigation request from native host
 */
async function handleNavigate(data: { url: string }): Promise<void> {
  if (data?.url) {
    try {
      await chrome.tabs.create({ url: data.url });
    } catch (error) {
      console.error('[SynMem] Failed to navigate:', error);
    }
  }
}

/**
 * Broadcast message to all content scripts
 */
async function broadcastToContentScripts(message: Message): Promise<void> {
  const tabs = await chrome.tabs.query({});
  for (const tab of tabs) {
    if (tab.id) {
      try {
        await chrome.tabs.sendMessage(tab.id, message);
      } catch {
        // Tab might not have content script, ignore
      }
    }
  }
}

/**
 * Listen for messages from content scripts and popup
 */
chrome.runtime.onMessage.addListener((message: Message, sender, sendResponse) => {
  console.log('[SynMem] Received message:', message, 'from:', sender);
  
  switch (message.type) {
    case 'PING':
      sendResponse({ type: 'PONG', status: 'ok' });
      break;
    
    case 'CONNECT_NATIVE':
      const port = connectToNativeHost();
      sendResponse({ type: 'CONNECT_RESULT', connected: port !== null });
      break;
    
    case 'SEND_TO_NATIVE':
      const success = sendToNativeHost(message.payload as NativeMessage);
      sendResponse({ type: 'SEND_RESULT', success });
      break;
    
    case 'GET_PAGE_CONTENT':
      // Forward to content script in active tab
      chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
        if (tabs[0]?.id) {
          chrome.tabs.sendMessage(tabs[0].id, { type: 'EXTRACT_CONTENT' }, sendResponse);
        }
      });
      return true; // Keep message channel open for async response
    
    default:
      console.log('[SynMem] Unhandled message type:', message.type);
      sendResponse({ type: 'ERROR', error: 'Unknown message type' });
  }
  
  return false;
});

/**
 * Extension installation event
 */
chrome.runtime.onInstalled.addListener((details) => {
  console.log('[SynMem] Extension installed/updated:', details.reason);
  
  if (details.reason === 'install') {
    // First time installation
    chrome.storage.local.set({ 
      synmem_installed: Date.now(),
      synmem_settings: {
        autoConnect: false,
        enabledSites: ['gemini.google.com', 'chat.openai.com', 'claude.ai', 'x.com', 'twitter.com']
      }
    });
  }
});

/**
 * Extension startup event
 */
chrome.runtime.onStartup.addListener(() => {
  console.log('[SynMem] Extension started');
});

console.log('[SynMem] Service worker loaded');
