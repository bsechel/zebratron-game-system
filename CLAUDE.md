# ZebratronGameSystem - Claude Configuration

## Project Overview
A custom 8-bit style game system inspired by NES/Master System, built for web browsers using WebAssembly with a focus on making scrolling games easy to develop.

## Build Commands
```bash
# Install dependencies
npm install

# Build the WebAssembly core
npm run build:wasm

# Build the JavaScript runtime
npm run build

# Run development server
npm run dev

# Run tests
npm test

# Lint code
npm run lint

# Type check
npm run typecheck
```

## Development Guidelines
- Core system written in Rust, compiled to WebAssembly
- JavaScript/TypeScript API layer for browser integration
- Developer tools written in TypeScript/React
- All graphics assets should be 8-bit style with limited palettes
- Audio should use chip-tune style synthesis
- Focus on 60fps performance in browsers

## Architecture
- `/core/` - Rust WebAssembly implementation
- `/runtime/` - JavaScript/TypeScript browser runtime
- `/tools/` - Asset pipeline and development tools
- `/examples/` - Sample games and demos
- `/docs/` - Documentation and tutorials