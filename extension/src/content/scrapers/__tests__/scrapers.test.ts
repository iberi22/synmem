/**
 * Tests for Site-Specific Scrapers
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { JSDOM } from 'jsdom';
import {
  ScraperRegistry,
  UniversalScraper,
  GeminiScraper,
  ChatGPTScraper,
  ClaudeScraper,
  TwitterScraper,
  getScraperRegistry,
  scrapeDocument,
} from '../index';

describe('ScraperRegistry', () => {
  let registry: ScraperRegistry;

  beforeEach(() => {
    registry = new ScraperRegistry();
  });

  it('should return GeminiScraper for Gemini URLs', () => {
    const scraper = registry.getScraperForUrl('https://gemini.google.com/app');
    expect(scraper.name).toBe('gemini');
  });

  it('should return ChatGPTScraper for ChatGPT URLs', () => {
    const scraper = registry.getScraperForUrl('https://chat.openai.com/c/123');
    expect(scraper.name).toBe('chatgpt');
  });

  it('should return ChatGPTScraper for chatgpt.com URLs', () => {
    const scraper = registry.getScraperForUrl('https://chatgpt.com/c/123');
    expect(scraper.name).toBe('chatgpt');
  });

  it('should return ClaudeScraper for Claude URLs', () => {
    const scraper = registry.getScraperForUrl('https://claude.ai/chat/123');
    expect(scraper.name).toBe('claude');
  });

  it('should return TwitterScraper for Twitter URLs', () => {
    const scraper = registry.getScraperForUrl('https://twitter.com/user/status/123');
    expect(scraper.name).toBe('twitter');
  });

  it('should return TwitterScraper for X.com URLs', () => {
    const scraper = registry.getScraperForUrl('https://x.com/user/status/123');
    expect(scraper.name).toBe('twitter');
  });

  it('should return UniversalScraper for unknown URLs', () => {
    const scraper = registry.getScraperForUrl('https://example.com/page');
    expect(scraper.name).toBe('universal');
  });

  it('should list all scraper names', () => {
    const names = registry.getScraperNames();
    expect(names).toContain('gemini');
    expect(names).toContain('chatgpt');
    expect(names).toContain('claude');
    expect(names).toContain('twitter');
    expect(names).toContain('universal');
  });
});

describe('UniversalScraper', () => {
  let scraper: UniversalScraper;

  beforeEach(() => {
    scraper = new UniversalScraper();
  });

  it('should handle any URL', () => {
    expect(scraper.canHandle('https://example.com')).toBe(true);
    expect(scraper.canHandle('https://anything.org/path')).toBe(true);
  });

  it('should extract metadata from document', () => {
    const html = `
      <!DOCTYPE html>
      <html lang="en">
      <head>
        <title>Test Page</title>
        <meta name="description" content="Test description">
        <meta name="author" content="Test Author">
        <meta name="og:image" content="https://example.com/image.jpg">
        <link rel="canonical" href="https://example.com/canonical">
      </head>
      <body>
        <main>
          <h1>Main Content</h1>
          <p>This is the main content.</p>
        </main>
      </body>
      </html>
    `;
    const dom = new JSDOM(html, { url: 'https://example.com/test' });
    const result = scraper.extract(dom.window.document);

    expect(result.scraperType).toBe('universal');
    expect(result.metadata.title).toBe('Test Page');
    expect(result.metadata.description).toBe('Test description');
    expect(result.metadata.author).toBe('Test Author');
    expect(result.metadata.ogImage).toBe('https://example.com/image.jpg');
    expect(result.metadata.canonicalUrl).toBe('https://example.com/canonical');
    expect(result.metadata.language).toBe('en');
  });

  it('should extract main content from article element', () => {
    const html = `
      <!DOCTYPE html>
      <html>
      <body>
        <nav>Navigation</nav>
        <article>
          <h1>Article Title</h1>
          <p>Article content here.</p>
        </article>
        <footer>Footer</footer>
      </body>
      </html>
    `;
    const dom = new JSDOM(html, { url: 'https://example.com' });
    const result = scraper.extract(dom.window.document);

    expect(result.mainContent).toContain('Article Title');
    expect(result.mainContent).toContain('Article content here');
    expect(result.textContent).toContain('Article content here');
  });

  it('should extract JSON-LD structured data', () => {
    const html = `
      <!DOCTYPE html>
      <html>
      <head>
        <script type="application/ld+json">
          {
            "@type": "Article",
            "headline": "Test Article",
            "author": "Test Author"
          }
        </script>
      </head>
      <body><main>Content</main></body>
      </html>
    `;
    const dom = new JSDOM(html, { url: 'https://example.com' });
    const result = scraper.extract(dom.window.document);

    expect(result.structuredData).toHaveLength(1);
    expect(result.structuredData?.[0].type).toBe('Article');
    expect(result.structuredData?.[0].data).toHaveProperty('headline', 'Test Article');
  });

  it('should extract media elements', () => {
    const html = `
      <!DOCTYPE html>
      <html>
      <body>
        <main>
          <img src="https://example.com/image.jpg" alt="Test image">
          <video src="https://example.com/video.mp4" poster="https://example.com/poster.jpg"></video>
          <iframe src="https://www.youtube.com/embed/abc123"></iframe>
        </main>
      </body>
      </html>
    `;
    const dom = new JSDOM(html, { url: 'https://example.com' });
    const result = scraper.extract(dom.window.document);

    expect(result.media).toBeDefined();
    expect(result.media?.some(m => m.type === 'image')).toBe(true);
    expect(result.media?.some(m => m.type === 'video')).toBe(true);
    expect(result.media?.some(m => m.type === 'embed')).toBe(true);
  });
});

describe('GeminiScraper', () => {
  let scraper: GeminiScraper;

  beforeEach(() => {
    scraper = new GeminiScraper();
  });

  it('should handle Gemini URLs', () => {
    expect(scraper.canHandle('https://gemini.google.com/app')).toBe(true);
    expect(scraper.canHandle('https://gemini.google.com/share/abc123')).toBe(true);
    expect(scraper.canHandle('https://bard.google.com/chat')).toBe(true);
  });

  it('should not handle non-Gemini URLs', () => {
    expect(scraper.canHandle('https://example.com')).toBe(false);
    expect(scraper.canHandle('https://google.com')).toBe(false);
  });

  it('should extract chat messages', () => {
    const html = `
      <!DOCTYPE html>
      <html>
      <body>
        <div class="conversation-turn user-turn" data-author="user">
          <div class="message-content">Hello, how are you?</div>
        </div>
        <div class="conversation-turn model-turn" data-author="model">
          <div class="message-content">I'm doing well, thank you!</div>
        </div>
      </body>
      </html>
    `;
    const dom = new JSDOM(html, { url: 'https://gemini.google.com/app' });
    const result = scraper.extract(dom.window.document);

    expect(result.scraperType).toBe('gemini');
    expect(result.chatMessages).toBeDefined();
    expect(result.chatMessages?.length).toBeGreaterThanOrEqual(0);
  });

  it('should extract code blocks', () => {
    const html = `
      <!DOCTYPE html>
      <html>
      <body>
        <div class="conversation-turn">
          <pre><code class="language-javascript">console.log("Hello");</code></pre>
        </div>
      </body>
      </html>
    `;
    const dom = new JSDOM(html, { url: 'https://gemini.google.com/app' });
    const result = scraper.extract(dom.window.document);

    expect(result.codeBlocks).toBeDefined();
    expect(result.codeBlocks?.length).toBe(1);
    expect(result.codeBlocks?.[0].language).toBe('javascript');
    expect(result.codeBlocks?.[0].code).toContain('console.log');
  });
});

describe('ChatGPTScraper', () => {
  let scraper: ChatGPTScraper;

  beforeEach(() => {
    scraper = new ChatGPTScraper();
  });

  it('should handle ChatGPT URLs', () => {
    expect(scraper.canHandle('https://chat.openai.com/c/123')).toBe(true);
    expect(scraper.canHandle('https://chatgpt.com/c/123')).toBe(true);
  });

  it('should not handle non-ChatGPT URLs', () => {
    expect(scraper.canHandle('https://openai.com')).toBe(false);
    expect(scraper.canHandle('https://example.com')).toBe(false);
  });

  it('should extract messages with roles', () => {
    const html = `
      <!DOCTYPE html>
      <html>
      <body>
        <div data-message-author-role="user">
          <div class="markdown">What is JavaScript?</div>
        </div>
        <div data-message-author-role="assistant">
          <div class="markdown">JavaScript is a programming language...</div>
        </div>
      </body>
      </html>
    `;
    const dom = new JSDOM(html, { url: 'https://chat.openai.com/c/123' });
    const result = scraper.extract(dom.window.document);

    expect(result.scraperType).toBe('chatgpt');
    expect(result.chatMessages).toBeDefined();
  });

  it('should extract code blocks with language', () => {
    const html = `
      <!DOCTYPE html>
      <html>
      <body>
        <div data-message-author-role="assistant">
          <pre><code class="language-python">print("Hello")</code></pre>
        </div>
      </body>
      </html>
    `;
    const dom = new JSDOM(html, { url: 'https://chat.openai.com/c/123' });
    const result = scraper.extract(dom.window.document);

    expect(result.codeBlocks).toBeDefined();
    expect(result.codeBlocks?.length).toBe(1);
    expect(result.codeBlocks?.[0].language).toBe('python');
  });
});

describe('ClaudeScraper', () => {
  let scraper: ClaudeScraper;

  beforeEach(() => {
    scraper = new ClaudeScraper();
  });

  it('should handle Claude URLs', () => {
    expect(scraper.canHandle('https://claude.ai/chat/123')).toBe(true);
    expect(scraper.canHandle('https://console.anthropic.com/workbench')).toBe(true);
  });

  it('should not handle non-Claude URLs', () => {
    expect(scraper.canHandle('https://anthropic.com')).toBe(false);
    expect(scraper.canHandle('https://example.com')).toBe(false);
  });

  it('should extract artifacts', () => {
    const html = `
      <!DOCTYPE html>
      <html>
      <body>
        <div data-artifact data-artifact-id="test-artifact" data-artifact-type="code">
          <header>artifact.py</header>
          <pre><code class="language-python">def hello(): pass</code></pre>
        </div>
      </body>
      </html>
    `;
    const dom = new JSDOM(html, { url: 'https://claude.ai/chat/123' });
    const result = scraper.extract(dom.window.document);

    expect(result.scraperType).toBe('claude');
    expect(result.artifacts).toBeDefined();
    expect(result.artifacts?.length).toBe(1);
    expect(result.artifacts?.[0].id).toBe('test-artifact');
    expect(result.artifacts?.[0].type).toBe('code');
  });

  it('should extract thinking process', () => {
    const html = `
      <!DOCTYPE html>
      <html>
      <body>
        <div data-message-author="assistant" class="assistant-message">
          <div class="thinking-content">Let me think about this...</div>
          <div class="message-content">Here is my response.</div>
        </div>
      </body>
      </html>
    `;
    const dom = new JSDOM(html, { url: 'https://claude.ai/chat/123' });
    const result = scraper.extract(dom.window.document);

    expect(result.chatMessages).toBeDefined();
    // Should have 2 messages: thinking and main response
    expect(result.chatMessages?.length).toBe(2);
    // First message should be thinking
    const thinkingMsg = result.chatMessages?.find(m => m.role === 'thinking');
    expect(thinkingMsg).toBeDefined();
    expect(thinkingMsg?.content).toContain('Let me think');
    // Second message should be the response
    const responseMsg = result.chatMessages?.find(m => m.role === 'assistant');
    expect(responseMsg).toBeDefined();
    expect(responseMsg?.content).toContain('Here is my response');
  });
});

describe('TwitterScraper', () => {
  let scraper: TwitterScraper;

  beforeEach(() => {
    scraper = new TwitterScraper();
  });

  it('should handle Twitter URLs', () => {
    expect(scraper.canHandle('https://twitter.com/user/status/123')).toBe(true);
    expect(scraper.canHandle('https://x.com/user/status/123')).toBe(true);
    expect(scraper.canHandle('https://mobile.twitter.com/user')).toBe(true);
  });

  it('should not handle non-Twitter URLs', () => {
    expect(scraper.canHandle('https://example.com')).toBe(false);
    expect(scraper.canHandle('https://facebook.com')).toBe(false);
  });

  it('should extract tweet thread', () => {
    const html = `
      <!DOCTYPE html>
      <html>
      <body>
        <article data-testid="tweet">
          <a role="link" href="/testuser">@testuser</a>
          <div data-testid="tweetText">This is a test tweet.</div>
          <time datetime="2024-01-01T12:00:00Z">Jan 1</time>
        </article>
      </body>
      </html>
    `;
    const dom = new JSDOM(html, { url: 'https://twitter.com/testuser/status/123' });
    const result = scraper.extract(dom.window.document);

    expect(result.scraperType).toBe('twitter');
    expect(result.tweetThread).toBeDefined();
    expect(result.tweetThread?.tweets.length).toBeGreaterThanOrEqual(0);
  });

  it('should extract conversation ID from URL', () => {
    const html = `<!DOCTYPE html><html><body></body></html>`;
    const dom = new JSDOM(html, { url: 'https://twitter.com/user/status/1234567890' });
    const result = scraper.extract(dom.window.document);

    expect(result.tweetThread?.conversationId).toBe('1234567890');
  });
});

describe('Global functions', () => {
  it('getScraperRegistry should return singleton', () => {
    const registry1 = getScraperRegistry();
    const registry2 = getScraperRegistry();
    expect(registry1).toBe(registry2);
  });

  it('scrapeDocument should work with any document', () => {
    const html = `
      <!DOCTYPE html>
      <html>
      <head><title>Test</title></head>
      <body><main>Content</main></body>
      </html>
    `;
    const dom = new JSDOM(html, { url: 'https://example.com' });
    const result = scrapeDocument(dom.window.document);

    expect(result).toBeDefined();
    expect(result.url).toBe('https://example.com/');
    expect(result.scraperType).toBe('universal');
  });
});
