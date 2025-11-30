/**
 * ChatGPT Scraper - Extracts conversation content from OpenAI ChatGPT
 * 
 * Features:
 * - Conversation extraction
 * - Model info detection (GPT-4, GPT-3.5, etc.)
 * - Code block handling
 */

import {
  Scraper,
  ScrapedContent,
  ChatMessage,
  ChatRole,
  CodeBlock,
  PageMetadata,
} from './types';

export class ChatGPTScraper implements Scraper {
  readonly name = 'chatgpt';

  /** ChatGPT URL patterns */
  private readonly urlPatterns = [
    /^https?:\/\/chat\.openai\.com/,
    /^https?:\/\/chatgpt\.com/,
  ];

  canHandle(url: string): boolean {
    return this.urlPatterns.some(pattern => pattern.test(url));
  }

  extract(document: Document): ScrapedContent {
    const url = document.location?.href || '';
    const timestamp = new Date().toISOString();
    const modelInfo = this.detectModelInfo(document);

    return {
      url,
      timestamp,
      scraperType: this.name,
      metadata: this.extractMetadata(document, modelInfo),
      chatMessages: this.extractChatMessages(document, modelInfo),
      codeBlocks: this.extractAllCodeBlocks(document),
    };
  }

  /** Extract metadata including model information */
  private extractMetadata(document: Document, modelInfo?: string): PageMetadata {
    return {
      title: document.title || 'ChatGPT Conversation',
      siteName: 'ChatGPT',
      description: modelInfo ? `Conversation with ${modelInfo}` : undefined,
    };
  }

  /** Detect which model is being used */
  private detectModelInfo(document: Document): string | undefined {
    // Look for model selector/indicator
    const modelSelectors = [
      '[data-testid="model-selector"]',
      '[class*="model-selector"]',
      '[class*="model-name"]',
      '.text-token-text-secondary', // Sometimes shows model name
    ];

    for (const selector of modelSelectors) {
      const element = document.querySelector(selector);
      if (element) {
        const text = element.textContent?.toLowerCase() || '';
        if (text.includes('gpt-4o')) return 'GPT-4o';
        if (text.includes('gpt-4')) return 'GPT-4';
        if (text.includes('gpt-3.5')) return 'GPT-3.5';
        if (text.includes('o1')) return 'o1';
      }
    }

    // Check the page title for model hints
    const title = document.title.toLowerCase();
    if (title.includes('gpt-4o')) return 'GPT-4o';
    if (title.includes('gpt-4')) return 'GPT-4';
    if (title.includes('gpt-3.5')) return 'GPT-3.5';

    return undefined;
  }

  /** Extract all chat messages from the conversation */
  private extractChatMessages(document: Document, modelInfo?: string): ChatMessage[] {
    const messages: ChatMessage[] = [];

    // ChatGPT conversation structure
    const messageSelectors = [
      '[data-message-author-role]',
      '[class*="agent-turn"]',
      '[class*="human-turn"]',
      '[data-testid="conversation-turn"]',
      '.group[class*="dark:bg"]', // Message groups
    ];

    let messageElements: NodeListOf<Element> | null = null;
    for (const selector of messageSelectors) {
      messageElements = document.querySelectorAll(selector);
      if (messageElements.length > 0) break;
    }

    if (messageElements && messageElements.length > 0) {
      messageElements.forEach(element => {
        const message = this.extractMessageFromElement(element, modelInfo);
        if (message) {
          messages.push(message);
        }
      });
    } else {
      // Fallback: try alternative extraction
      messages.push(...this.extractMessagesAlternative(document, modelInfo));
    }

    return messages;
  }

  /** Extract a single message from an element */
  private extractMessageFromElement(element: Element, modelInfo?: string): ChatMessage | null {
    const role = this.detectRole(element);
    const content = this.extractContent(element);
    
    if (!content) return null;

    const codeBlocks = this.extractCodeBlocksFromElement(element);

    return {
      role,
      content,
      codeBlocks: codeBlocks.length > 0 ? codeBlocks : undefined,
      modelInfo: role === 'assistant' ? modelInfo : undefined,
    };
  }

  /** Detect the role from element attributes */
  private detectRole(element: Element): ChatRole {
    // Check data attributes first (most reliable)
    const roleAttr = element.getAttribute('data-message-author-role');
    if (roleAttr === 'user') return 'user';
    if (roleAttr === 'assistant') return 'assistant';
    if (roleAttr === 'system') return 'system';

    // Check class names
    const classList = element.className.toLowerCase();
    
    if (classList.includes('user') || classList.includes('human')) {
      return 'user';
    }

    if (classList.includes('assistant') || classList.includes('agent') || classList.includes('gpt')) {
      return 'assistant';
    }

    // Check for user avatar indicators
    const hasUserAvatar = element.querySelector('[class*="user-avatar"], img[alt*="User"]');
    if (hasUserAvatar) {
      return 'user';
    }

    // Check for assistant avatar (OpenAI logo)
    const hasAssistantAvatar = element.querySelector('[class*="gpt-avatar"], svg[class*="icon"]');
    if (hasAssistantAvatar) {
      return 'assistant';
    }

    return 'assistant';
  }

  /** Extract text content from an element */
  private extractContent(element: Element): string {
    // Look for the markdown content container
    const contentSelectors = [
      '[class*="markdown"]',
      '[class*="prose"]',
      '.text-base',
      '[class*="message-content"]',
    ];

    for (const selector of contentSelectors) {
      const contentEl = element.querySelector(selector);
      if (contentEl) {
        // Clone to avoid modifying the DOM
        const clone = contentEl.cloneNode(true) as Element;
        // Remove code blocks from text extraction (they're handled separately)
        clone.querySelectorAll('pre').forEach(pre => pre.remove());
        return this.cleanText(clone.textContent || '');
      }
    }

    return this.cleanText(element.textContent || '');
  }

  /** Extract code blocks from an element */
  private extractCodeBlocksFromElement(element: Element): CodeBlock[] {
    const codeBlocks: CodeBlock[] = [];

    // ChatGPT wraps code in pre > code with a language class
    const codeElements = element.querySelectorAll('pre');
    
    codeElements.forEach(pre => {
      const codeEl = pre.querySelector('code');
      if (codeEl) {
        const code = codeEl.textContent || '';
        if (code.trim()) {
          const language = this.detectLanguage(codeEl);
          const filename = this.detectFilename(pre);
          codeBlocks.push({
            code: code.trim(),
            language,
            filename,
          });
        }
      }
    });

    return codeBlocks;
  }

  /** Extract all code blocks from the document */
  private extractAllCodeBlocks(document: Document): CodeBlock[] {
    const codeBlocks: CodeBlock[] = [];
    
    const preElements = document.querySelectorAll('pre');
    
    preElements.forEach(pre => {
      const codeEl = pre.querySelector('code');
      if (codeEl) {
        const code = codeEl.textContent || '';
        if (code.trim()) {
          const language = this.detectLanguage(codeEl);
          const filename = this.detectFilename(pre);
          codeBlocks.push({
            code: code.trim(),
            language,
            filename,
          });
        }
      }
    });

    return codeBlocks;
  }

  /** Detect programming language from element */
  private detectLanguage(element: Element): string | undefined {
    const classList = element.className;
    
    // ChatGPT uses language-xxx classes
    const langMatch = classList.match(/language-(\w+)/);
    if (langMatch && langMatch[1] !== 'plaintext') {
      return langMatch[1];
    }

    // Check for hljs-xxx classes (highlight.js)
    const hljsMatch = classList.match(/hljs-(\w+)/);
    if (hljsMatch) {
      return hljsMatch[1];
    }

    return undefined;
  }

  /** Detect filename from code block header */
  private detectFilename(pre: Element): string | undefined {
    // Look for a filename indicator before the code block
    const prevSibling = pre.previousElementSibling;
    if (prevSibling) {
      const text = prevSibling.textContent || '';
      // Look for common filename patterns
      const filenameMatch = text.match(/(\w+\.\w+)$/);
      if (filenameMatch) {
        return filenameMatch[1];
      }
    }

    // Check for header within the pre element
    const header = pre.querySelector('[class*="header"], [class*="filename"]');
    if (header) {
      const text = header.textContent?.trim();
      if (text && text.includes('.')) {
        return text;
      }
    }

    return undefined;
  }

  /** Alternative message extraction for different DOM structures */
  private extractMessagesAlternative(document: Document, modelInfo?: string): ChatMessage[] {
    const messages: ChatMessage[] = [];

    // Try to find by role-based containers
    const roleContainers = document.querySelectorAll('[class*="group"]');
    
    roleContainers.forEach(container => {
      const text = container.textContent || '';
      if (text.trim().length < 10) return; // Skip near-empty containers

      const role = this.detectRole(container);
      const content = this.extractContent(container);
      
      if (content) {
        messages.push({
          role,
          content,
          codeBlocks: this.extractCodeBlocksFromElement(container),
          modelInfo: role === 'assistant' ? modelInfo : undefined,
        });
      }
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
