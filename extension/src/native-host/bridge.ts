/**
 * Native Host Bridge - Bi-directional communication with Rust native host
 * 
 * Flow:
 * [Page DOM Change] → MutationObserver → Debounce (500ms) → Content Script
 * → Native Messaging → Rust Host → SQLite
 */

import {
  Message,
  ConnectionState,
  ConnectionConfig,
  DEFAULT_CONNECTION_CONFIG,
  ErrorCode,
} from '../types';

// Native host application name (must match manifest)
const NATIVE_HOST_NAME = 'com.synmem.native_host';

type MessageHandler = (message: Message) => void;
type ErrorHandler = (error: { code: ErrorCode; message: string }) => void;
type StateChangeHandler = (state: ConnectionState) => void;

/**
 * NativeHostBridge manages bi-directional communication with the Rust native host.
 * Implements reconnection logic and message queuing.
 */
export class NativeHostBridge {
  private port: chrome.runtime.Port | null = null;
  private state: ConnectionState = 'disconnected';
  private config: ConnectionConfig;
  private reconnectAttempts = 0;
  private reconnectTimeoutId: ReturnType<typeof setTimeout> | null = null;
  private pingIntervalId: ReturnType<typeof setInterval> | null = null;
  private messageQueue: Message[] = [];
  
  private messageHandlers: Set<MessageHandler> = new Set();
  private errorHandlers: Set<ErrorHandler> = new Set();
  private stateChangeHandlers: Set<StateChangeHandler> = new Set();

  constructor(config: Partial<ConnectionConfig> = {}) {
    this.config = { ...DEFAULT_CONNECTION_CONFIG, ...config };
  }

  /**
   * Get current connection state
   */
  getState(): ConnectionState {
    return this.state;
  }

  /**
   * Subscribe to incoming messages
   */
  onMessage(handler: MessageHandler): () => void {
    this.messageHandlers.add(handler);
    return () => this.messageHandlers.delete(handler);
  }

  /**
   * Subscribe to errors
   */
  onError(handler: ErrorHandler): () => void {
    this.errorHandlers.add(handler);
    return () => this.errorHandlers.delete(handler);
  }

  /**
   * Subscribe to state changes
   */
  onStateChange(handler: StateChangeHandler): () => void {
    this.stateChangeHandlers.add(handler);
    return () => this.stateChangeHandlers.delete(handler);
  }

  /**
   * Connect to native host
   */
  connect(): Promise<void> {
    return new Promise((resolve, reject) => {
      if (this.state === 'connected') {
        resolve();
        return;
      }

      this.setState('connecting');
      
      try {
        this.port = chrome.runtime.connectNative(NATIVE_HOST_NAME);
        
        this.port.onMessage.addListener((message: unknown) => {
          this.handleIncomingMessage(message);
        });
        
        this.port.onDisconnect.addListener(() => {
          this.handleDisconnect();
        });

        // Wait for connection confirmation or timeout
        const timeout = setTimeout(() => {
          if (this.state === 'connecting') {
            this.setState('disconnected');
            reject(new Error('Connection timeout'));
          }
        }, 5000);

        // Setup ping to verify connection
        this.sendMessage({ type: 'COMMAND', payload: { action: 'PING', payload: {} } });
        
        // Consider connected after sending first message without error
        setTimeout(() => {
          if (this.port && this.state === 'connecting') {
            clearTimeout(timeout);
            this.setState('connected');
            this.reconnectAttempts = 0;
            this.startPingInterval();
            this.flushMessageQueue();
            resolve();
          }
        }, 100);
        
      } catch (error) {
        this.setState('disconnected');
        reject(error);
      }
    });
  }

  /**
   * Disconnect from native host
   */
  disconnect(): void {
    this.stopPingInterval();
    this.clearReconnectTimeout();
    
    if (this.port) {
      this.port.disconnect();
      this.port = null;
    }
    
    this.setState('disconnected');
  }

  /**
   * Send message to native host
   * If not connected, queues message for later delivery
   */
  sendMessage(message: Message): boolean {
    if (this.state !== 'connected' || !this.port) {
      // Queue message for when connection is established
      this.messageQueue.push(message);
      return false;
    }

    try {
      this.port.postMessage(message);
      return true;
    } catch (error) {
      console.error('[NativeHostBridge] Send error:', error);
      this.messageQueue.push(message);
      return false;
    }
  }

  private setState(newState: ConnectionState): void {
    if (this.state !== newState) {
      this.state = newState;
      this.stateChangeHandlers.forEach(handler => handler(newState));
    }
  }

  private handleIncomingMessage(rawMessage: unknown): void {
    try {
      const message = rawMessage as Message;
      
      // Handle PONG for connection health check
      if (message.type === 'PONG') {
        // Connection is healthy
        return;
      }
      
      this.messageHandlers.forEach(handler => handler(message));
    } catch (error) {
      console.error('[NativeHostBridge] Invalid message:', error);
      this.emitError('INVALID_MESSAGE', 'Received invalid message from native host');
    }
  }

  private handleDisconnect(): void {
    const lastError = chrome.runtime.lastError;
    console.warn('[NativeHostBridge] Disconnected:', lastError?.message || 'Unknown reason');
    
    this.port = null;
    this.stopPingInterval();
    
    if (this.state === 'connected') {
      this.emitError(
        'CONNECTION_FAILED',
        lastError?.message || 'Connection to native host lost'
      );
    }
    
    this.setState('disconnected');
    this.attemptReconnect();
  }

  private attemptReconnect(): void {
    if (this.reconnectAttempts >= this.config.reconnectAttempts) {
      this.emitError('CONNECTION_FAILED', 'Max reconnection attempts reached');
      return;
    }

    this.setState('reconnecting');
    this.reconnectAttempts++;
    
    const delay = Math.min(
      this.config.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1),
      this.config.maxReconnectDelay
    );

    console.log(`[NativeHostBridge] Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts}/${this.config.reconnectAttempts})`);
    
    this.reconnectTimeoutId = setTimeout(() => {
      this.connect().catch(error => {
        console.error('[NativeHostBridge] Reconnect failed:', error);
      });
    }, delay);
  }

  private clearReconnectTimeout(): void {
    if (this.reconnectTimeoutId) {
      clearTimeout(this.reconnectTimeoutId);
      this.reconnectTimeoutId = null;
    }
  }

  private startPingInterval(): void {
    this.stopPingInterval();
    this.pingIntervalId = setInterval(() => {
      this.sendMessage({ type: 'COMMAND', payload: { action: 'PING', payload: {} } });
    }, this.config.pingInterval);
  }

  private stopPingInterval(): void {
    if (this.pingIntervalId) {
      clearInterval(this.pingIntervalId);
      this.pingIntervalId = null;
    }
  }

  private flushMessageQueue(): void {
    while (this.messageQueue.length > 0 && this.state === 'connected') {
      const message = this.messageQueue.shift();
      if (message) {
        this.sendMessage(message);
      }
    }
  }

  private emitError(code: ErrorCode, message: string): void {
    this.errorHandlers.forEach(handler => handler({ code, message }));
  }
}

// Singleton instance for extension-wide use
let bridgeInstance: NativeHostBridge | null = null;

export function getNativeHostBridge(config?: Partial<ConnectionConfig>): NativeHostBridge {
  if (!bridgeInstance) {
    bridgeInstance = new NativeHostBridge(config);
  }
  return bridgeInstance;
}
