# Chrome Web Store Submission Checklist

## Overview

This document tracks the complete checklist for submitting SynMem to the Chrome Web Store.

## Pre-Submission Requirements

### Documentation
- [x] Privacy policy page created
- [x] Store description (< 132 chars) written
- [x] Detailed description written
- [ ] Privacy policy hosted at public URL

### Visual Assets
- [ ] Icon 128×128 px
- [ ] Icon 48×48 px
- [ ] Icon 16×16 px
- [ ] Screenshots (1280×800 px) - minimum 1
- [ ] Small promo tile (440×280 px) - optional
- [ ] Large promo tile (920×680 px) - optional
- [ ] Marquee (1400×560 px) - optional

### Extension Code
- [ ] manifest.json properly configured
- [ ] All permissions justified
- [ ] No policy violations
- [ ] Extension builds without errors
- [ ] Extension passes linting
- [ ] Source code available (no obfuscation)

### Developer Account
- [ ] Google Developer account created
- [ ] $5 registration fee paid
- [ ] Account verification completed

## Submission Process

### Step 1: Prepare Package
```bash
# Build extension
cd extension
npm run build

# Create zip for submission
zip -r synmem-extension.zip dist/
```

### Step 2: Upload to Dashboard
1. Go to Chrome Web Store Developer Dashboard
2. Click "Add new item"
3. Upload .zip package
4. Fill in store listing details
5. Upload visual assets
6. Set visibility (public/unlisted)

### Step 3: Submit for Review
1. Review all information
2. Accept developer agreement
3. Click "Submit for review"

### Step 4: Monitor Status
- Review typically takes 1-3 business days
- Check email for updates
- Address any rejection feedback

## Current Status

| Requirement | Status | Notes |
|-------------|--------|-------|
| Privacy Policy | ✅ Done | docs/store/PRIVACY_POLICY.md |
| Store Listing | ✅ Done | docs/store/STORE_LISTING.md |
| Icons | ⏳ Pending | Design phase |
| Screenshots | ⏳ Pending | Waiting for UI |
| Extension Code | ⏳ Pending | In development |
| Developer Account | ⏳ Pending | Setup required |

## Quick Reference

### Short Description (96 of 132 chars)
```
Give your AI agents memory. Capture conversations, automate browsing, search everything via MCP.
```

### Category
Productivity

### Pricing
Free

### Regions
All regions

---

*Use this checklist to track submission progress.*
