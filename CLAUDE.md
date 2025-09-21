# ZebratronGameSystem - Claude Configuration

## Project Overview
A custom 8-bit style game system inspired by classic retro consoles, built for web browsers using WebAssembly with a focus on making scrolling games easy to develop.

## Build Commands
```bash
# Install dependencies
npm install

# Build the WebAssembly core (use rustup instead of Homebrew Rust)
PATH="$HOME/.rustup/toolchains/stable-aarch64-apple-darwin/bin:$PATH" wasm-pack build --target web --out-dir ../runtime/pkg

# Alternative: Build WebAssembly using npm script
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

## Rust/WebAssembly Setup Notes
**Important**: This system requires rustup (not Homebrew Rust) for WebAssembly compilation.

### Issue: Homebrew Rust vs Rustup
- Homebrew's Rust installation doesn't include wasm32-unknown-unknown target
- The system PATH prioritizes `/opt/homebrew/bin/rustc` over rustup's toolchain
- This causes wasm-pack build failures

### Recommended Solution: Unlink Homebrew Rust
**Best approach** - Remove the PATH conflict entirely:
```bash
# Unlink Homebrew's rust package (keeps rustup tool)
brew unlink rust

# Verify rustup is working
which rustc  # Should show ~/.rustup/toolchains/stable-aarch64-apple-darwin/bin/rustc
rustc --version
```

After unlinking, WebAssembly builds work automatically:
```bash
cd /Users/brad/Code/ZebratronGameSystem/core
wasm-pack build --target web --out-dir ../runtime/pkg  # No PATH override needed!
```

### Fallback Solution: Manual PATH Override
If you need to keep Homebrew Rust for some reason:
```bash
cd /Users/brad/Code/ZebratronGameSystem/core
PATH="$HOME/.rustup/toolchains/stable-aarch64-apple-darwin/bin:$PATH" wasm-pack build --target web --out-dir ../runtime/pkg
```

### Verify Setup
```bash
# Check rustup installation
rustup show

# Should show:
# - installed toolchains: stable-aarch64-apple-darwin
# - installed targets: aarch64-apple-darwin, wasm32-unknown-unknown

# Test WebAssembly build
cd core && wasm-pack build --target web --out-dir ../runtime/pkg
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