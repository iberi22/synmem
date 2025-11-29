/**
 * Universal Scraper - Extracts content from any webpage
 * 
 * Features:
 * - Main content detection using readability heuristics
 * - Metadata extraction (Open Graph, Twitter Cards, standard meta)
 * - Structured data extraction (JSON-LD, microdata)
 */

import {
  Scraper,
  ScrapedContent,
  PageMetadata,
  StructuredData,
  MediaContent,
} from './types';

export class UniversalScraper implements Scraper {
  readonly name = 'universal';

  canHandle(_url: string): boolean {
    // Universal scraper handles all URLs as a fallback
    return true;
  }

  extract(document: Document): ScrapedContent {
    const url = document.location?.href || '';
    const timestamp = new Date().toISOString();

    return {
      url,
      timestamp,
      scraperType: this.name,
      metadata: this.extractMetadata(document),
      structuredData: this.extractStructuredData(document),
      mainContent: this.extractMainContent(document),
      textContent: this.extractTextContent(document),
      media: this.extractMedia(document),
    };
  }

  /** Extract page metadata from various sources */
  private extractMetadata(document: Document): PageMetadata {
    const getMeta = (name: string): string | undefined => {
      const element = document.querySelector(
        `meta[name="${name}"], meta[property="${name}"], meta[itemprop="${name}"]`
      );
      return element?.getAttribute('content') || undefined;
    };

    return {
      title: document.title || getMeta('og:title') || getMeta('twitter:title'),
      description: getMeta('description') || getMeta('og:description') || getMeta('twitter:description'),
      author: getMeta('author') || getMeta('article:author'),
      publishedDate: getMeta('article:published_time') || getMeta('datePublished'),
      modifiedDate: getMeta('article:modified_time') || getMeta('dateModified'),
      canonicalUrl: document.querySelector('link[rel="canonical"]')?.getAttribute('href') || undefined,
      language: document.documentElement.lang || getMeta('language'),
      keywords: getMeta('keywords')?.split(',').map(k => k.trim()) || undefined,
      ogImage: getMeta('og:image') || getMeta('twitter:image'),
      siteName: getMeta('og:site_name'),
    };
  }

  /** Extract JSON-LD and other structured data */
  private extractStructuredData(document: Document): StructuredData[] {
    const structuredData: StructuredData[] = [];

    // Extract JSON-LD
    const jsonLdScripts = document.querySelectorAll('script[type="application/ld+json"]');
    jsonLdScripts.forEach(script => {
      try {
        const data = JSON.parse(script.textContent || '');
        if (Array.isArray(data)) {
          data.forEach(item => {
            if (item['@type']) {
              structuredData.push({
                type: item['@type'],
                data: item,
              });
            }
          });
        } else if (data['@type']) {
          structuredData.push({
            type: data['@type'],
            data,
          });
        }
      } catch {
        // Ignore invalid JSON-LD
      }
    });

    return structuredData;
  }

  /** Extract main content using readability heuristics */
  private extractMainContent(document: Document): string {
    // Priority order for finding main content
    const selectors = [
      'main',
      'article',
      '[role="main"]',
      '#content',
      '#main-content',
      '.content',
      '.post-content',
      '.article-content',
      '.entry-content',
    ];

    for (const selector of selectors) {
      const element = document.querySelector(selector);
      if (element) {
        return this.cleanHtml(element.innerHTML);
      }
    }

    // Fallback: use body but remove noise
    const body = document.body?.cloneNode(true) as HTMLElement;
    if (!body) return '';

    // Remove common noise elements
    const noiseSelectors = [
      'script',
      'style',
      'nav',
      'header',
      'footer',
      'aside',
      '.sidebar',
      '.advertisement',
      '.ad',
      '.social-share',
      '.comments',
      '#comments',
    ];

    noiseSelectors.forEach(selector => {
      body.querySelectorAll(selector).forEach(el => el.remove());
    });

    return this.cleanHtml(body.innerHTML);
  }

  /** Extract plain text content */
  private extractTextContent(document: Document): string {
    const mainContent = this.extractMainContent(document);
    // Create a temporary element to parse HTML and get text
    const temp = document.createElement('div');
    temp.innerHTML = mainContent;
    return temp.textContent?.trim() || '';
  }

  /** Extract media elements */
  private extractMedia(document: Document): MediaContent[] {
    const media: MediaContent[] = [];

    // Extract images
    document.querySelectorAll('img').forEach(img => {
      const src = img.src || img.getAttribute('src') || img.getAttribute('data-src');
      if (src && !this.isTrackingPixel(img)) {
        media.push({
          type: 'image',
          url: src,
          alt: img.alt || undefined,
        });
      }
    });

    // Extract videos
    document.querySelectorAll('video').forEach(video => {
      const src = video.src || video.getAttribute('src') || video.querySelector('source')?.getAttribute('src');
      if (src) {
        media.push({
          type: 'video',
          url: src,
          thumbnailUrl: video.poster || video.getAttribute('poster') || undefined,
        });
      }
    });

    // Extract iframes (embeds)
    document.querySelectorAll('iframe').forEach(iframe => {
      const src = iframe.src || iframe.getAttribute('src');
      if (src && this.isValidEmbed(src)) {
        media.push({
          type: 'embed',
          url: src,
        });
      }
    });

    return media;
  }

  /** Check if an image is likely a tracking pixel */
  private isTrackingPixel(img: HTMLImageElement): boolean {
    // Check explicit dimensions in HTML attributes
    const widthAttr = img.getAttribute('width');
    const heightAttr = img.getAttribute('height');
    
    // If explicit dimensions are set and both are <= 1, it's likely a tracking pixel
    if (widthAttr && heightAttr) {
      const width = parseInt(widthAttr, 10);
      const height = parseInt(heightAttr, 10);
      if (!isNaN(width) && !isNaN(height) && width <= 1 && height <= 1) {
        return true;
      }
    }
    
    // Check computed dimensions (for images that have loaded)
    const width = img.width || img.naturalWidth;
    const height = img.height || img.naturalHeight;
    if (width > 0 && height > 0 && width <= 1 && height <= 1) {
      return true;
    }
    
    // Check URL for common tracking pixel indicators
    const src = img.src || img.getAttribute('src') || '';
    return src.includes('pixel') || src.includes('tracking') || src.includes('beacon');
  }

  /** Check if an iframe URL is a valid embed */
  private isValidEmbed(url: string): boolean {
    const validDomains = [
      'youtube.com',
      'youtu.be',
      'vimeo.com',
      'twitter.com',
      'x.com',
      'codepen.io',
      'jsfiddle.net',
    ];
    return validDomains.some(domain => url.includes(domain));
  }

  /** Clean HTML content by removing extra whitespace */
  private cleanHtml(html: string): string {
    return html
      .replace(/\s+/g, ' ')
      .replace(/>\s+</g, '><')
      .trim();
  }
}
