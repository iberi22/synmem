/**
 * SynMem Popup UI
 * 
 * Provides user interface for:
 * - Connection status display
 * - Manual scraping triggers
 * - Settings access
 */

// Make this file a module to avoid global scope conflicts
export {};

/** Maximum length for pathname display in UI */
const MAX_PATHNAME_DISPLAY_LENGTH = 30;

interface StatusState {
  connected: boolean;
  message: string;
}

/**
 * DOM Elements
 */
const elements = {
  statusIndicator: document.getElementById('status-indicator'),
  statusText: document.getElementById('status-text'),
  btnConnect: document.getElementById('btn-connect'),
  btnScrape: document.getElementById('btn-scrape'),
  currentPage: document.getElementById('current-page'),
};

/**
 * Update connection status UI
 */
function updateStatus(state: StatusState): void {
  if (elements.statusIndicator && elements.statusText) {
    elements.statusIndicator.className = `status-indicator ${state.connected ? 'connected' : 'disconnected'}`;
    elements.statusText.textContent = state.message;
  }
}

/**
 * Update current page info
 */
async function updateCurrentPage(): Promise<void> {
  try {
    const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
    if (tab?.url && elements.currentPage) {
      const url = new URL(tab.url);
      elements.currentPage.textContent = url.hostname + url.pathname.slice(0, MAX_PATHNAME_DISPLAY_LENGTH);
    }
  } catch (error) {
    console.error('[SynMem Popup] Failed to get current tab:', error);
  }
}

/**
 * Check service worker status
 */
async function checkStatus(): Promise<void> {
  try {
    const response = await chrome.runtime.sendMessage({ type: 'PING' });
    if (response?.type === 'PONG') {
      updateStatus({ connected: true, message: 'Extension active' });
    } else {
      updateStatus({ connected: false, message: 'Extension not responding' });
    }
  } catch (error) {
    updateStatus({ connected: false, message: 'Extension error' });
    console.error('[SynMem Popup] Status check failed:', error);
  }
}

/**
 * Connect to native host
 */
async function connectToHost(): Promise<void> {
  try {
    updateStatus({ connected: false, message: 'Connecting...' });
    const response = await chrome.runtime.sendMessage({ type: 'CONNECT_NATIVE' });
    
    if (response?.connected) {
      updateStatus({ connected: true, message: 'Connected to host' });
    } else {
      updateStatus({ connected: false, message: 'Connection failed' });
    }
  } catch (error) {
    updateStatus({ connected: false, message: 'Connection error' });
    console.error('[SynMem Popup] Connection failed:', error);
  }
}

/**
 * Trigger page scrape
 */
async function scrapePage(): Promise<void> {
  try {
    const response = await chrome.runtime.sendMessage({ type: 'GET_PAGE_CONTENT' });
    console.log('[SynMem Popup] Scrape result:', response);
    
    if (response?.payload) {
      // Send to native host
      await chrome.runtime.sendMessage({ 
        type: 'SEND_TO_NATIVE', 
        payload: { 
          action: 'scrape', 
          data: response.payload 
        } 
      });
      updateStatus({ connected: true, message: 'Page scraped!' });
    }
  } catch (error) {
    console.error('[SynMem Popup] Scrape failed:', error);
    updateStatus({ connected: false, message: 'Scrape failed' });
  }
}

/**
 * Initialize popup
 */
function init(): void {
  console.log('[SynMem Popup] Initializing...');
  
  // Set up event listeners
  elements.btnConnect?.addEventListener('click', connectToHost);
  elements.btnScrape?.addEventListener('click', scrapePage);
  
  // Initial state
  updateCurrentPage();
  checkStatus();
}

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', init);
