/**
 * Claude Scraper - Extracts conversation content from Anthropic Claude
 * 
 * Features:
 * - Chat message extraction
 * - Artifact handling (code, documents, etc.)
 * - Thinking process extraction (extended thinking)
 */

import {
  Scraper,
  ScrapedContent,
  ChatMessage,
  ChatRole,
  CodeBlock,
  PageMetadata,
  Artifact,
} from './types';

export class ClaudeScraper implements Scraper {
  readonly name = 'claude';

  /** Claude URL patterns */
  private readonly urlPatterns = [
    /^https?:\/\/claude\.ai/,
    /^https?:\/\/console\.anthropic\.com/,
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
      artifacts: this.extractArtifacts(document),
      codeBlocks: this.extractAllCodeBlocks(document),
    };
  }

  /** Extract basic metadata */
  private extractMetadata(document: Document): PageMetadata {
    // Try to get conversation title
    const titleSelectors = [
      '[class*="conversation-title"]',
      '[class*="chat-title"]',
      'h1',
    ];

    let title = 'Claude Conversation';
    for (const selector of titleSelectors) {
      const element = document.querySelector(selector);
      if (element?.textContent?.trim()) {
        title = element.textContent.trim();
        break;
      }
    }

    return {
      title,
      siteName: 'Claude',
    };
  }

  /** Extract all chat messages from the conversation */
  private extractChatMessages(document: Document): ChatMessage[] {
    const messages: ChatMessage[] = [];

    // Claude conversation structure
    const messageSelectors = [
      '[data-message-author]',
      '[class*="message-row"]',
      '[class*="human-message"]',
      '[class*="assistant-message"]',
      '[data-testid="user-message"]',
      '[data-testid="assistant-message"]',
    ];

    let messageElements: NodeListOf<Element> | null = null;
    for (const selector of messageSelectors) {
      messageElements = document.querySelectorAll(selector);
      if (messageElements.length > 0) break;
    }

    if (messageElements && messageElements.length > 0) {
      messageElements.forEach(element => {
        const extractedMessages = this.extractMessagesFromElement(element);
        messages.push(...extractedMessages);
      });
    } else {
      // Fallback: try container-based extraction
      messages.push(...this.extractMessagesFromContainers(document));
    }

    return messages;
  }

  /** Extract messages from an element (may return multiple for thinking + response) */
  private extractMessagesFromElement(element: Element): ChatMessage[] {
    const messages: ChatMessage[] = [];
    const role = this.detectRole(element);
    
    // Check for thinking process first
    const thinking = this.extractThinking(element);
    
    // If this is thinking content from assistant, add as separate message
    if (thinking && role === 'assistant') {
      messages.push({
        role: 'thinking',
        content: thinking,
      });
    }
    
    const content = this.extractContent(element);
    
    if (content) {
      const codeBlocks = this.extractCodeBlocksFromElement(element);
      messages.push({
        role,
        content,
        codeBlocks: codeBlocks.length > 0 ? codeBlocks : undefined,
      });
    }
    
    return messages;
  }

  /** Detect the role from element attributes */
  private detectRole(element: Element): ChatRole {
    // Check data attributes
    const authorAttr = element.getAttribute('data-message-author');
    if (authorAttr === 'human' || authorAttr === 'user') return 'user';
    if (authorAttr === 'assistant' || authorAttr === 'claude') return 'assistant';

    // Check class names
    const classList = element.className.toLowerCase();
    
    if (classList.includes('human') || classList.includes('user')) {
      return 'user';
    }

    if (classList.includes('assistant') || classList.includes('claude')) {
      return 'assistant';
    }

    // Check testid
    const testId = element.getAttribute('data-testid') || '';
    if (testId.includes('user')) return 'user';
    if (testId.includes('assistant')) return 'assistant';

    // Default to assistant
    return 'assistant';
  }

  /** Extract thinking process content (extended thinking) */
  private extractThinking(element: Element): string | null {
    const thinkingSelectors = [
      '[class*="thinking"]',
      '[data-type="thinking"]',
      '[class*="extended-thinking"]',
      '.thinking-content',
    ];

    for (const selector of thinkingSelectors) {
      const thinkingEl = element.querySelector(selector);
      if (thinkingEl) {
        return this.cleanText(thinkingEl.textContent || '');
      }
    }

    return null;
  }

  /** Extract text content from an element */
  private extractContent(element: Element): string {
    // Skip thinking sections
    const clone = element.cloneNode(true) as Element;
    clone.querySelectorAll('[class*="thinking"], [data-type="thinking"]').forEach(el => el.remove());
    
    // Look for the main content container
    const contentSelectors = [
      '[class*="prose"]',
      '[class*="markdown"]',
      '[class*="message-content"]',
      '[class*="text-content"]',
    ];

    for (const selector of contentSelectors) {
      const contentEl = clone.querySelector(selector);
      if (contentEl) {
        // Remove code blocks from text (handled separately)
        const textClone = contentEl.cloneNode(true) as Element;
        textClone.querySelectorAll('pre').forEach(pre => pre.remove());
        return this.cleanText(textClone.textContent || '');
      }
    }

    return this.cleanText(clone.textContent || '');
  }

  /** Extract all artifacts from the document */
  private extractArtifacts(document: Document): Artifact[] {
    const artifacts: Artifact[] = [];

    // Claude artifacts are typically in special containers
    const artifactSelectors = [
      '[data-artifact]',
      '[class*="artifact"]',
      '[data-type="artifact"]',
      '[class*="code-artifact"]',
      '[class*="document-artifact"]',
    ];

    for (const selector of artifactSelectors) {
      const artifactElements = document.querySelectorAll(selector);
      artifactElements.forEach((element, index) => {
        const artifact = this.extractArtifactFromElement(element, index);
        if (artifact) {
          artifacts.push(artifact);
        }
      });
    }

    return artifacts;
  }

  /** Extract a single artifact from an element */
  private extractArtifactFromElement(element: Element, index: number): Artifact | null {
    const id = element.getAttribute('data-artifact-id') || 
               element.getAttribute('id') || 
               `artifact-${index}`;

    // Determine artifact type
    const type = this.detectArtifactType(element);
    
    // Get title
    const titleEl = element.querySelector('[class*="title"], header, h1, h2');
    const title = titleEl?.textContent?.trim();

    // Get content
    const content = this.extractArtifactContent(element);
    if (!content) return null;

    // Get language for code artifacts
    const language = type === 'code' ? this.detectLanguageFromArtifact(element) : undefined;

    return {
      id,
      type,
      title,
      content,
      language,
    };
  }

  /** Detect artifact type */
  private detectArtifactType(element: Element): string {
    const typeAttr = element.getAttribute('data-artifact-type') || 
                     element.getAttribute('data-type');
    if (typeAttr) return typeAttr;

    const classList = element.className.toLowerCase();
    
    if (classList.includes('code')) return 'code';
    if (classList.includes('document')) return 'document';
    if (classList.includes('image')) return 'image';
    if (classList.includes('mermaid')) return 'mermaid';
    if (classList.includes('svg')) return 'svg';
    if (classList.includes('html')) return 'html';
    if (classList.includes('react')) return 'react';

    // Check for code element inside
    if (element.querySelector('pre code')) return 'code';

    return 'unknown';
  }

  /** Extract content from artifact */
  private extractArtifactContent(element: Element): string {
    // For code artifacts
    const codeEl = element.querySelector('pre code, code');
    if (codeEl) {
      return codeEl.textContent?.trim() || '';
    }

    // For other artifacts
    const contentEl = element.querySelector('[class*="content"], .body, main');
    if (contentEl) {
      return contentEl.textContent?.trim() || '';
    }

    return element.textContent?.trim() || '';
  }

  /** Detect language from artifact */
  private detectLanguageFromArtifact(element: Element): string | undefined {
    const langAttr = element.getAttribute('data-language') || 
                     element.getAttribute('data-lang');
    if (langAttr) return langAttr;

    const codeEl = element.querySelector('code');
    if (codeEl) {
      const langMatch = codeEl.className.match(/language-(\w+)/);
      if (langMatch) return langMatch[1];
    }

    return undefined;
  }

  /** Extract code blocks from an element */
  private extractCodeBlocksFromElement(element: Element): CodeBlock[] {
    const codeBlocks: CodeBlock[] = [];

    const codeElements = element.querySelectorAll('pre code, code[class*="language-"]');
    
    codeElements.forEach(codeEl => {
      const code = codeEl.textContent || '';
      if (code.trim()) {
        codeBlocks.push({
          code: code.trim(),
          language: this.detectLanguage(codeEl),
        });
      }
    });

    return codeBlocks;
  }

  /** Extract all code blocks from the document */
  private extractAllCodeBlocks(document: Document): CodeBlock[] {
    const codeBlocks: CodeBlock[] = [];
    
    const codeElements = document.querySelectorAll('pre code');
    
    codeElements.forEach(codeEl => {
      const code = codeEl.textContent || '';
      if (code.trim()) {
        codeBlocks.push({
          code: code.trim(),
          language: this.detectLanguage(codeEl),
        });
      }
    });

    return codeBlocks;
  }

  /** Detect programming language from element */
  private detectLanguage(element: Element): string | undefined {
    const classList = element.className;
    const langMatch = classList.match(/language-(\w+)/);
    if (langMatch) return langMatch[1];

    const parent = element.closest('pre');
    if (parent) {
      const parentLangMatch = parent.className.match(/language-(\w+)/);
      if (parentLangMatch) return parentLangMatch[1];
    }

    return undefined;
  }

  /** Container-based message extraction fallback */
  private extractMessagesFromContainers(document: Document): ChatMessage[] {
    const messages: ChatMessage[] = [];

    // Try to find message containers
    const containers = document.querySelectorAll('[class*="message"], [class*="turn"]');
    
    containers.forEach(container => {
      const text = container.textContent || '';
      if (text.trim().length < 5) return;

      const role = this.detectRole(container);
      const thinking = this.extractThinking(container);
      const content = this.extractContent(container);
      
      if (thinking) {
        messages.push({
          role: 'thinking',
          content: thinking,
        });
      }

      if (content) {
        messages.push({
          role,
          content,
          codeBlocks: this.extractCodeBlocksFromElement(container),
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
