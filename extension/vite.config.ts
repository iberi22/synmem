import { defineConfig } from 'vite';
import { resolve } from 'path';
import { copyFileSync, mkdirSync, existsSync, rmSync, readFileSync, writeFileSync } from 'fs';

// Plugin to copy static files to dist and fix paths
function copyStaticFiles() {
  return {
    name: 'copy-static-files',
    closeBundle() {
      const distDir = resolve(__dirname, 'dist');
      
      // Ensure dist directory exists
      if (!existsSync(distDir)) {
        mkdirSync(distDir, { recursive: true });
      }
      
      // Copy manifest.json
      copyFileSync(
        resolve(__dirname, 'manifest.json'),
        resolve(distDir, 'manifest.json')
      );
      
      // Fix popup HTML location: move from dist/src/popup to dist/popup
      const srcPopupDir = resolve(distDir, 'src', 'popup');
      const destPopupDir = resolve(distDir, 'popup');
      
      if (existsSync(srcPopupDir)) {
        const htmlSrc = resolve(srcPopupDir, 'index.html');
        const htmlDest = resolve(destPopupDir, 'index.html');
        
        if (existsSync(htmlSrc)) {
          // Read HTML and fix paths
          let html = readFileSync(htmlSrc, 'utf-8');
          // Fix paths: ../../popup/ -> ./ (since we're moving to popup directory)
          html = html.replace(/\.\.\/\.\.\/popup\//g, './');
          writeFileSync(htmlDest, html);
        }
        
        // Clean up src directory
        rmSync(resolve(distDir, 'src'), { recursive: true, force: true });
      }
    }
  };
}

export default defineConfig({
  base: './',
  build: {
    outDir: 'dist',
    emptyOutDir: true,
    rollupOptions: {
      input: {
        'background/service-worker': resolve(__dirname, 'src/background/service-worker.ts'),
        'content/index': resolve(__dirname, 'src/content/index.ts'),
        'popup/index': resolve(__dirname, 'src/popup/index.html'),
      },
      output: {
        entryFileNames: '[name].js',
        chunkFileNames: 'chunks/[name]-[hash].js',
        assetFileNames: (assetInfo) => {
          // Keep CSS files in popup directory
          if (assetInfo.name?.endsWith('.css')) {
            return 'popup/[name][extname]';
          }
          return 'assets/[name]-[hash][extname]';
        },
      },
    },
    sourcemap: process.env.NODE_ENV === 'development',
  },
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src'),
    },
  },
  plugins: [copyStaticFiles()],
});
