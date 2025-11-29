# Privacy Policy for SynMem - AI Browser Memory

**Last Updated:** November 2024

## Overview

SynMem is a browser extension that provides AI agents with memory capabilities by capturing and storing web content locally. This privacy policy explains what data we collect, how it's stored, and your rights regarding your data.

## What Data We Collect

### Browsing Data (Local Only)
- **Page Content**: Text content from web pages you visit when capture is enabled
- **Conversation History**: Chat conversations from AI platforms (ChatGPT, Claude, Gemini) when you choose to capture them
- **URLs**: Web addresses of captured pages
- **Timestamps**: When pages were captured

### No Personal Data Collection
- We do **NOT** collect your name, email, or personal identifiers
- We do **NOT** collect login credentials or passwords
- We do **NOT** collect payment information
- We do **NOT** collect browser history beyond what you explicitly capture

## How Data is Stored

### Local Storage Only
- **All data is stored locally** on your device
- Data is stored in a local SQLite database
- Your data **never leaves your device** unless you explicitly export it
- No data is transmitted to external servers
- No cloud synchronization in the free version

### Encryption
- Sensitive data (cookies, sessions) is encrypted using AES-256-GCM
- Master password protection is available for additional security
- Encryption keys are derived using Argon2

## Third-Party Sharing

**We do NOT share your data with third parties.**

- No analytics services
- No advertising networks
- No data brokers
- No social media tracking

## Permissions Justification

SynMem requests the following browser permissions:

| Permission | Purpose |
|------------|---------|
| `activeTab` | Access current tab content when capture is triggered |
| `storage` | Store captured data locally |
| `tabs` | Navigate and manage browser tabs for automation |
| `nativeMessaging` | Communicate with local MCP server |
| `clipboardRead/Write` | Copy/paste automation features |

## GDPR Compliance

For users in the European Union:

### Your Rights
- **Right to Access**: Export all your data at any time
- **Right to Erasure**: Delete all stored data with one click
- **Right to Portability**: Export data in standard formats (JSON)
- **Right to Restrict Processing**: Disable capture at any time

### Data Processing
- All processing happens **locally** on your device
- No data controller or processor outside your device
- You maintain full control over your data

## Data Retention

- Data is retained locally until you delete it
- No automatic data expiration
- You can clear all data from extension settings
- Uninstalling the extension removes all stored data

## Children's Privacy

SynMem is not intended for use by children under 13 years of age. We do not knowingly collect personal information from children.

## Changes to This Policy

We may update this privacy policy from time to time. Changes will be posted in the extension update notes and on this page.

## Contact

For privacy concerns or questions:
- **GitHub Issues**: [github.com/iberi22/synmem/issues](https://github.com/iberi22/synmem/issues)
- **Repository**: [github.com/iberi22/synmem](https://github.com/iberi22/synmem)

## Summary

| Aspect | Status |
|--------|--------|
| Data storage | 🟢 Local only |
| Third-party sharing | 🟢 None |
| Cloud sync | 🟡 Optional (Premium only) |
| Analytics | 🟢 None |
| Encryption | 🟢 AES-256-GCM |
| GDPR compliant | 🟢 Yes |
| User data control | 🟢 Full control |

---

*SynMem - Open source AI browser memory extension*
