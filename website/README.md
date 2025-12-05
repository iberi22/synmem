# SynMem Website

Landing page and documentation for SynMem - Give your AI agent a memory.

## Tech Stack

- **Astro** - Static site generator
- **Tailwind CSS** - Utility-first CSS framework
- **TypeScript** - Type-safe JavaScript

## Development

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

## Deployment

This site is configured for deployment on Vercel or Cloudflare Pages.

### Vercel

1. Connect your GitHub repository to Vercel
2. Vercel will automatically detect the Astro framework
3. The site will be deployed on push to main

### Cloudflare Pages

1. Connect your GitHub repository to Cloudflare Pages
2. Build command: `npm run build`
3. Output directory: `dist`

## Project Structure

```
website/
├── public/           # Static assets
│   └── favicon.svg
├── src/
│   ├── components/   # Reusable UI components
│   ├── layouts/      # Page layouts
│   ├── pages/        # Route pages
│   └── styles/       # Global styles
├── astro.config.mjs  # Astro configuration
├── tailwind.config.mjs # Tailwind configuration
└── package.json
```

## Pages

- `/` - Landing page with hero, features, demo, pricing, installation
- `/docs` - Documentation hub
- `/blog` - Blog with articles

## License

MIT
