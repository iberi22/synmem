# Synthetic Memory Browser Agent

## Overview

Agente de navegador con memoria sintética. Proyecto en Rust para navegación web inteligente con memoria persistente.

## Tech Stack

- **Language:** Rust
- **Crates:** 
  - `synmem-browser` - Navegador/UI
  - `synmem-core` - Core del sistema de memoria
  - `synmem-mcp` - MCP server integration

## Project Structure

```
Synthetic Memory Browser Agent/
├── src/                  # Fuente principal
├── crates/
│   ├── synmem-browser/   # UI del navegador
│   ├── synmem-core/      # Memoria sintética core
│   └── synmem-mcp/        # MCP server
├── tests/                # Tests de integración
├── docs/                 # Documentación
└── scripts/              # Scripts de utilidad
```

## Architecture

```
┌─────────────────┐
│  synmem-mcp     │  ← MCP Server (tool interface)
├─────────────────┤
│  synmem-core    │  ← Memory engine
├─────────────────┤
│  synmem-browser │  ← Web browser / UI
└─────────────────┘
```

## Status

🟡 In Development - Rust project activo

## Notes

- Sistema de memoria sintética para agentes IA
- Integración MCP (Model Context Protocol)
- Navegación web inteligente con persistencia

## Links

- .cursorrules para reglas de desarrollo
- docs/ para documentación adicional
