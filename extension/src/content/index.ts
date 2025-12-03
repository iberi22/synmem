/**
 * Content Script - Main entry point
 * 
 * Integrates real-time sync with background service worker communication.
 */

import { createRealTimeSyncService, RealTimeSyncService } from './sync';
import type { Message, Command } from '../types';

// Service instance
let syncService: RealTimeSyncService | null = null;

/**
 * Send message to background service worker
 */
function sendToBackground(message: Message): void {
  try {
    chrome.runtime.sendMessage(message);
  } catch (error) {
    console.error('[SynMem Content] Failed to send message:', error);
  }
}

/**
 * Handle commands from background service worker
 */
function handleCommand(command: Command): void {
  switch (command.action) {
    case 'SCRAPE':
      // Trigger immediate scrape
      if (syncService) {
        syncService.saveSession();
      }
      break;
      
    case 'GET_SESSION':
      // Send current session state
      if (syncService) {
        syncService.saveSession();
      }
      break;
      
    default:
      console.log('[SynMem Content] Unknown command:', command.action);
  }
}

/**
 * Initialize content script
 */
function initialize(): void {
  // Create sync service
  syncService = createRealTimeSyncService(sendToBackground);
  
  // Listen for messages from background
  chrome.runtime.onMessage.addListener((message: Message, _sender, sendResponse) => {
    if (message.type === 'COMMAND') {
      handleCommand(message.payload);
      sendResponse({ received: true });
    }
    return false; // Synchronous response
  });
  
  // Start observing DOM changes
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
      syncService?.start();
    });
  } else {
    syncService.start();
  }
  
  // Save session before page unload
  window.addEventListener('beforeunload', () => {
    syncService?.saveSession();
  });
  
  // Handle visibility changes (for tab switching)
  document.addEventListener('visibilitychange', () => {
    if (document.hidden) {
      syncService?.saveSession();
    }
  });
  
  console.log('[SynMem Content] Initialized');
}

// Initialize when script loads
initialize();

// Export for testing
export { syncService, handleCommand };
