/**
 * Site-Specific Scrapers Index
 * 
 * Exports all scrapers and provides a registry for automatic scraper selection.
 */

export * from './types';
export { UniversalScraper } from './universal';
export { GeminiScraper } from './gemini';
export { ChatGPTScraper } from './chatgpt';
export { ClaudeScraper } from './claude';
export { TwitterScraper } from './twitter';

import { Scraper, ScrapedContent } from './types';
import { UniversalScraper } from './universal';
import { GeminiScraper } from './gemini';
import { ChatGPTScraper } from './chatgpt';
import { ClaudeScraper } from './claude';
import { TwitterScraper } from './twitter';

/**
 * ScraperRegistry manages all available scrapers and selects the best one for a URL.
 */
export class ScraperRegistry {
  private scrapers: Scraper[];
  private universalScraper: UniversalScraper;

  constructor() {
    // Site-specific scrapers (order matters - checked first to last)
    this.scrapers = [
      new GeminiScraper(),
      new ChatGPTScraper(),
      new ClaudeScraper(),
      new TwitterScraper(),
    ];

    // Universal scraper as fallback
    this.universalScraper = new UniversalScraper();
  }

  /**
   * Get the best scraper for the given URL.
   * Returns a site-specific scraper if available, otherwise the universal scraper.
   */
  getScraperForUrl(url: string): Scraper {
    for (const scraper of this.scrapers) {
      if (scraper.canHandle(url)) {
        return scraper;
      }
    }
    return this.universalScraper;
  }

  /**
   * Extract content from the current document.
   * Automatically selects the best scraper based on the URL.
   */
  extract(document: Document): ScrapedContent {
    const url = document.location?.href || '';
    const scraper = this.getScraperForUrl(url);
    return scraper.extract(document);
  }

  /**
   * Get all registered scrapers (excluding universal).
   */
  getRegisteredScrapers(): Scraper[] {
    return [...this.scrapers];
  }

  /**
   * Get the universal scraper.
   */
  getUniversalScraper(): UniversalScraper {
    return this.universalScraper;
  }

  /**
   * Register a custom scraper.
   * Custom scrapers are checked before built-in scrapers.
   */
  registerScraper(scraper: Scraper): void {
    this.scrapers.unshift(scraper);
  }

  /**
   * Get all scraper names.
   */
  getScraperNames(): string[] {
    return [...this.scrapers.map(s => s.name), this.universalScraper.name];
  }
}

// Singleton instance for convenience
let registryInstance: ScraperRegistry | null = null;

/**
 * Get the global ScraperRegistry instance.
 */
export function getScraperRegistry(): ScraperRegistry {
  if (!registryInstance) {
    registryInstance = new ScraperRegistry();
  }
  return registryInstance;
}

/**
 * Convenience function to scrape the current document.
 * Uses the global registry to select the best scraper.
 */
export function scrapeDocument(document: Document): ScrapedContent {
  return getScraperRegistry().extract(document);
}
