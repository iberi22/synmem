# SynMem Icon Assets

## Required Icons

Place the following icon files in this directory:

| File | Size | Format | Purpose |
|------|------|--------|---------|
| `icon-16.png` | 16×16 px | PNG | Toolbar, favicon |
| `icon-48.png` | 48×48 px | PNG | Extensions page |
| `icon-128.png` | 128×128 px | PNG | Store listing, management |

## Design Guidelines

### Concept
- **Theme**: Brain/memory with synthetic/digital aesthetic
- **Style**: Modern, clean, flat design
- **Colors**: Primary blue/purple gradient (#4A90D9 to #7C3AED)

### Technical Requirements
- **Format**: PNG with transparency
- **Color space**: sRGB
- **Compression**: Optimized for web
- **Background**: Transparent

### Design Tips
- Keep design simple - icons must be recognizable at 16px
- Avoid text - illegible at small sizes
- Use bold shapes with good contrast
- Test at all sizes before finalizing

## Optional Promotional Images

| File | Size | Purpose |
|------|------|---------|
| `promo-small.png` | 440×280 px | Store listing grid |
| `promo-large.png` | 920×680 px | Featured placement |
| `promo-marquee.png` | 1400×560 px | Featured collections |

## Color Palette

| Color | Hex | Use |
|-------|-----|-----|
| Primary Blue | #4A90D9 | Main accent |
| Primary Purple | #7C3AED | Gradient end |
| Dark | #1F2937 | Text, outlines |
| Light | #F3F4F6 | Background |
| Success | #10B981 | Active states |
| Warning | #F59E0B | Alerts |

## Source File

A source SVG (`icon.svg`) is provided. Convert to PNG at required sizes:

```bash
# Using Inkscape
inkscape -w 128 -h 128 icon.svg -o icon-128.png
inkscape -w 48 -h 48 icon.svg -o icon-48.png
inkscape -w 16 -h 16 icon.svg -o icon-16.png

# Using ImageMagick
convert -background none icon.svg -resize 128x128 icon-128.png
convert -background none icon.svg -resize 48x48 icon-48.png
convert -background none icon.svg -resize 16x16 icon-16.png

# Using rsvg-convert
rsvg-convert -w 128 -h 128 icon.svg > icon-128.png
rsvg-convert -w 48 -h 48 icon.svg > icon-48.png
rsvg-convert -w 16 -h 16 icon.svg > icon-16.png
```

---

*Generate icons using your preferred design tool (Figma, Illustrator, etc.) or convert the provided SVG.*
