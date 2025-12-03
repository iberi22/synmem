/**
 * Popup Component
 * 
 * Shows connection status and basic controls for the extension.
 */

import { getNativeHostBridge } from '../native-host';
import type { ConnectionState } from '../types';

// DOM Elements
const statusIndicator = document.getElementById('status-indicator') as HTMLDivElement;
const statusText = document.getElementById('status-text') as HTMLSpanElement;
const syncButton = document.getElementById('sync-button') as HTMLButtonElement;
const settingsButton = document.getElementById('settings-button') as HTMLButtonElement;

/**
 * Initialize popup
 */
function initialize(): void {
  const bridge = getNativeHostBridge();
  
  // Update UI with current state
  updateConnectionStatus(bridge.getState());
  
  // Subscribe to state changes
  bridge.onStateChange((state) => {
    updateConnectionStatus(state);
  });
  
  // Set up button handlers
  setupButtonHandlers();
}

/**
 * Update connection status UI
 */
function updateConnectionStatus(state: ConnectionState): void {
  const statusClasses: Record<ConnectionState, string> = {
    connected: 'status-connected',
    connecting: 'status-connecting',
    reconnecting: 'status-connecting',
    disconnected: 'status-disconnected',
  };
  
  const statusMessages: Record<ConnectionState, string> = {
    connected: 'Connected',
    connecting: 'Connecting...',
    reconnecting: 'Reconnecting...',
    disconnected: 'Disconnected',
  };
  
  // Update indicator class
  statusIndicator.className = `status-indicator ${statusClasses[state]}`;
  
  // Update status text
  statusText.textContent = statusMessages[state];
  
  // Enable/disable sync button
  syncButton.disabled = state !== 'connected';
}

/**
 * Set up button handlers
 */
function setupButtonHandlers(): void {
  // Sync button - trigger immediate sync
  syncButton.addEventListener('click', () => {
    chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
      const tab = tabs[0];
      if (tab?.id) {
        chrome.tabs.sendMessage(tab.id, {
          type: 'COMMAND',
          payload: { action: 'SCRAPE', payload: {} },
        });
        
        // Visual feedback
        syncButton.textContent = 'Syncing...';
        setTimeout(() => {
          syncButton.textContent = 'Sync Now';
        }, 1000);
      }
    });
  });
  
  // Settings button
  settingsButton.addEventListener('click', () => {
    chrome.runtime.openOptionsPage();
  });
}

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', initialize);
