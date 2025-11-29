/**
 * SynMem Content Script
 * 
 * Injected into pages matching host_permissions.
 * Handles:
 * - Page content extraction
 * - DOM observation for dynamic content
 * - Communication with service worker
 */

// Make this file a module to avoid global scope conflicts
export {};

/**
 * Message types for communication
 */
interface Message {
  type: string;
  payload?: unknown;
}

interface ExtractedContent {
  url: string;
  title: string;
  content: string;
  timestamp: number;
  metadata: {
    domain: string;
    pathname: string;
  };
}

/**
 * Extract page content for scraping
 */
function extractPageContent(): ExtractedContent {
  const url = window.location.href;
  const title = document.title;
  
  // Get main content - attempt to find article/main content first
  const mainSelectors = ['main', 'article', '[role="main"]', '.content', '#content'];
  let contentElement: Element | null = null;
  
  for (const selector of mainSelectors) {
    contentElement = document.querySelector(selector);
    if (contentElement) break;
  }
  
  // Fall back to body if no main content found
  const content = contentElement?.textContent || document.body?.textContent || '';
  
  return {
    url,
    title,
    content: content.trim().slice(0, 50000), // Limit content size
    timestamp: Date.now(),
    metadata: {
      domain: window.location.hostname,
      pathname: window.location.pathname,
    },
  };
}

/**
 * Check if current page is a supported AI chat site
 */
function isSupportedChatSite(): boolean {
  const hostname = window.location.hostname;
  const supportedSites = [
    'gemini.google.com',
    'chat.openai.com',
    'claude.ai',
    'x.com',
    'twitter.com',
  ];
  return supportedSites.some(site => hostname.includes(site));
}

/**
 * Get site-specific identifier
 */
function getSiteType(): string {
  const hostname = window.location.hostname;
  
  if (hostname.includes('gemini.google.com')) return 'gemini';
  if (hostname.includes('chat.openai.com')) return 'chatgpt';
  if (hostname.includes('claude.ai')) return 'claude';
  if (hostname.includes('x.com') || hostname.includes('twitter.com')) return 'twitter';
  
  return 'unknown';
}

/**
 * Listen for messages from service worker
 */
chrome.runtime.onMessage.addListener((message: Message, _sender, sendResponse) => {
  console.log('[SynMem Content] Received message:', message);
  
  switch (message.type) {
    case 'EXTRACT_CONTENT':
      const content = extractPageContent();
      sendResponse({ 
        type: 'CONTENT_EXTRACTED', 
        payload: content,
        siteType: getSiteType()
      });
      break;
    
    case 'SCRAPE_RESULT':
      console.log('[SynMem Content] Scrape result:', message.payload);
      break;
    
    case 'PING':
      sendResponse({ type: 'PONG', status: 'content_script_ready' });
      break;
    
    default:
      console.log('[SynMem Content] Unhandled message type:', message.type);
  }
  
  return false;
});

/**
 * Initialize content script
 */
function init(): void {
  console.log('[SynMem Content] Initialized on:', window.location.href);
  console.log('[SynMem Content] Site type:', getSiteType());
  console.log('[SynMem Content] Is supported chat site:', isSupportedChatSite());
  
  // Notify service worker that content script is ready
  chrome.runtime.sendMessage({ 
    type: 'CONTENT_SCRIPT_READY', 
    payload: { 
      url: window.location.href,
      siteType: getSiteType()
    } 
  }).catch(() => {
    // Service worker might not be ready yet, that's okay
  });
}

// Run initialization when DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}
