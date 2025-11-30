/**
 * Gemini Scraper - Extracts chat content from Google Gemini
 * 
 * Features:
 * - Chat message extraction
 * - User/Assistant role detection
 * - Code block handling with language detection
 */

import {
  Scraper,
  ScrapedContent,
  ChatMessage,
  ChatRole,
  CodeBlock,
  PageMetadata,
} from './types';

export class GeminiScraper implements Scraper {
  readonly name = 'gemini';

  /** Gemini URL patterns */
  private readonly urlPatterns = [
    /^https?:\/\/(gemini\.google\.com|bard\.google\.com)/,
    /^https?:\/\/g\.co\/gemini/,
  ];

  canHandle(url: string): boolean {
    return this.urlPatterns.some(pattern => pattern.test(url));
  }

  extract(document: Document): ScrapedContent {
    const url = document.location?.href || '';
    const timestamp = new Date().toISOString();

    return {
      url,
      timestamp,
      scraperType: this.name,
      metadata: this.extractMetadata(document),
      chatMessages: this.extractChatMessages(document),
      codeBlocks: this.extractAllCodeBlocks(document),
    };
  }

  /** Extract basic metadata */
  private extractMetadata(document: Document): PageMetadata {
    return {
      title: document.title || 'Gemini Conversation',
      siteName: 'Google Gemini',
    };
  }

  /** Extract all chat messages from the conversation */
  private extractChatMessages(document: Document): ChatMessage[] {
    const messages: ChatMessage[] = [];

    // Gemini uses a turn-based conversation structure
    // Look for conversation turns container
    const conversationSelectors = [
      '[data-test-id="conversation-turn"]',
      '.conversation-turn',
      '[class*="conversation-turn"]',
      '[class*="chat-turn"]',
      '[role="listitem"]',
    ];

    let turns: NodeListOf<Element> | null = null;
    for (const selector of conversationSelectors) {
      turns = document.querySelectorAll(selector);
      if (turns.length > 0) break;
    }

    if (turns && turns.length > 0) {
      turns.forEach(turn => {
        const message = this.extractMessageFromTurn(turn);
        if (message) {
          messages.push(message);
        }
      });
    } else {
      // Fallback: try to find message pairs
      messages.push(...this.extractMessagesFromDOM(document));
    }

    return messages;
  }

  /** Extract a single message from a conversation turn */
  private extractMessageFromTurn(turn: Element): ChatMessage | null {
    const role = this.detectRole(turn);
    const content = this.extractContent(turn);
    
    if (!content) return null;

    const codeBlocks = this.extractCodeBlocksFromElement(turn);

    return {
      role,
      content,
      codeBlocks: codeBlocks.length > 0 ? codeBlocks : undefined,
    };
  }

  /** Detect the role (user or assistant) from element attributes/classes */
  private detectRole(element: Element): ChatRole {
    const html = element.outerHTML.toLowerCase();
    const classList = element.className.toLowerCase();
    
    // Check for user indicators
    if (
      html.includes('user') ||
      classList.includes('user') ||
      classList.includes('human') ||
      element.getAttribute('data-author') === 'user'
    ) {
      return 'user';
    }

    // Check for model/assistant indicators
    if (
      html.includes('model') ||
      html.includes('gemini') ||
      classList.includes('model') ||
      classList.includes('assistant') ||
      classList.includes('response') ||
      element.getAttribute('data-author') === 'model'
    ) {
      return 'assistant';
    }

    // Default to assistant for ambiguous cases
    return 'assistant';
  }

  /** Extract text content from an element */
  private extractContent(element: Element): string {
    // Try to find the main message text container
    const contentSelectors = [
      '[class*="message-content"]',
      '[class*="text-content"]',
      '.response-text',
      '.user-text',
      'p',
    ];

    for (const selector of contentSelectors) {
      const contentEl = element.querySelector(selector);
      if (contentEl) {
        return this.cleanText(contentEl.textContent || '');
      }
    }

    return this.cleanText(element.textContent || '');
  }

  /** Extract code blocks from an element */
  private extractCodeBlocksFromElement(element: Element): CodeBlock[] {
    const codeBlocks: CodeBlock[] = [];

    // Look for code elements
    const codeElements = element.querySelectorAll('pre code, code[class*="language-"], .code-block');
    
    codeElements.forEach(codeEl => {
      const code = codeEl.textContent || '';
      if (code.trim()) {
        const language = this.detectLanguage(codeEl);
        codeBlocks.push({
          code: code.trim(),
          language,
        });
      }
    });

    return codeBlocks;
  }

  /** Extract all code blocks from the document */
  private extractAllCodeBlocks(document: Document): CodeBlock[] {
    const codeBlocks: CodeBlock[] = [];
    
    const codeElements = document.querySelectorAll('pre code, code[class*="language-"], .code-block');
    
    codeElements.forEach(codeEl => {
      const code = codeEl.textContent || '';
      if (code.trim()) {
        const language = this.detectLanguage(codeEl);
        codeBlocks.push({
          code: code.trim(),
          language,
        });
      }
    });

    return codeBlocks;
  }

  /** Detect programming language from element classes */
  private detectLanguage(element: Element): string | undefined {
    const classList = element.className;
    
    // Check for language- prefix (common Prism/Highlight.js pattern)
    const langMatch = classList.match(/language-(\w+)/);
    if (langMatch) {
      return langMatch[1];
    }

    // Check parent pre element
    const pre = element.closest('pre');
    if (pre) {
      const preLangMatch = pre.className.match(/language-(\w+)/);
      if (preLangMatch) {
        return preLangMatch[1];
      }
    }

    // Check data attributes
    const dataLang = element.getAttribute('data-language') || 
                     element.getAttribute('data-lang');
    if (dataLang) {
      return dataLang;
    }

    return undefined;
  }

  /** Fallback method to extract messages from DOM structure */
  private extractMessagesFromDOM(document: Document): ChatMessage[] {
    const messages: ChatMessage[] = [];

    // Try to find user queries and model responses
    const userSelectors = ['[class*="user-query"]', '[class*="user-message"]', '[class*="human"]'];
    const modelSelectors = ['[class*="model-response"]', '[class*="model-message"]', '[class*="gemini"]'];

    userSelectors.forEach(selector => {
      document.querySelectorAll(selector).forEach(el => {
        const content = this.cleanText(el.textContent || '');
        if (content) {
          messages.push({
            role: 'user',
            content,
            codeBlocks: this.extractCodeBlocksFromElement(el),
          });
        }
      });
    });

    modelSelectors.forEach(selector => {
      document.querySelectorAll(selector).forEach(el => {
        const content = this.cleanText(el.textContent || '');
        if (content) {
          messages.push({
            role: 'assistant',
            content,
            codeBlocks: this.extractCodeBlocksFromElement(el),
          });
        }
      });
    });

    return messages;
  }

  /** Clean text by removing extra whitespace */
  private cleanText(text: string): string {
    return text
      .replace(/\s+/g, ' ')
      .trim();
  }
}
