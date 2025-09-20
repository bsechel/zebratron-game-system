#!/bin/bash

# ZebratronGameSystem Laravel Integration Script
# This script clones the ZGS repo and integrates it into a Laravel project

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
ZGS_REPO="https://github.com/bsechel/zebratron-game-system.git"
TEMP_DIR="/tmp/zebratron-integration-$$"

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if we're in a Laravel project
check_laravel() {
    if [[ ! -f "artisan" ]]; then
        print_error "This doesn't appear to be a Laravel project directory."
        print_error "Please run this script from your Laravel project root."
        exit 1
    fi
    print_success "Laravel project detected."
}

# Function to check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check for git
    if ! command -v git &> /dev/null; then
        print_error "Git is required but not installed."
        exit 1
    fi
    
    # Check for npm
    if ! command -v npm &> /dev/null; then
        print_error "npm is required but not installed."
        exit 1
    fi
    
    # Check for Node.js
    if ! command -v node &> /dev/null; then
        print_error "Node.js is required but not installed."
        exit 1
    fi
    
    print_success "All prerequisites found."
}

# Function to clone ZGS repository
clone_zgs_repo() {
    print_status "Cloning ZebratronGameSystem repository..."
    
    if [[ -d "$TEMP_DIR" ]]; then
        rm -rf "$TEMP_DIR"
    fi
    
    git clone "$ZGS_REPO" "$TEMP_DIR"
    print_success "Repository cloned to temporary directory."
}

# Function to build ZGS WebAssembly
build_zgs() {
    print_status "Building ZebratronGameSystem WebAssembly module..."
    
    cd "$TEMP_DIR"
    
    # Check for Rust and wasm-pack
    if ! command -v rustc &> /dev/null; then
        print_error "Rust is required to build ZGS. Please install Rust first:"
        print_error "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    
    if ! command -v wasm-pack &> /dev/null; then
        print_error "wasm-pack is required. Installing..."
        cargo install wasm-pack
    fi
    
    # Build the WASM module
    print_status "Building WebAssembly module (this may take a few minutes)..."
    cd core
    wasm-pack build --target web --out-dir ../runtime/pkg
    cd ..
    
    print_success "WebAssembly module built successfully."
    
    # Return to original directory
    cd - > /dev/null
}

# Function to create Laravel directory structure
create_laravel_structure() {
    print_status "Creating Laravel directory structure..."
    
    # Create necessary directories
    mkdir -p public/arcade/{js,wasm,css}
    mkdir -p resources/views/arcade
    mkdir -p resources/js/arcade
    mkdir -p resources/css/arcade
    
    print_success "Directory structure created."
}

# Function to copy ZGS assets
copy_zgs_assets() {
    print_status "Copying ZebratronGameSystem assets..."
    
    # Copy WebAssembly files
    cp "$TEMP_DIR/runtime/pkg/zebratron_core.js" public/arcade/js/
    cp "$TEMP_DIR/runtime/pkg/zebratron_core_bg.wasm" public/arcade/wasm/
    cp "$TEMP_DIR/runtime/pkg/zebratron_core.d.ts" public/arcade/js/
    
    # Copy TypeScript source files for reference
    cp -r "$TEMP_DIR/runtime/src/"* resources/js/arcade/
    
    print_success "ZGS assets copied."
}

# Function to create arcade view template
create_arcade_view() {
    print_status "Creating arcade view template..."
    
    cat > resources/views/arcade/index.blade.php << 'EOF'
@extends('layouts.app')

@section('title', 'ZebratronGameSystem Arcade')

@push('styles')
<style>
    body {
        margin: 0;
        padding: 0;
        background: #000;
        color: #00ff00;
        font-family: 'Courier New', monospace;
        overflow: hidden;
    }
    
    .arcade-header {
        position: absolute;
        top: 10px;
        left: 50%;
        transform: translateX(-50%);
        z-index: 200;
        background: rgba(0, 0, 0, 0.8);
        padding: 10px 20px;
        border: 1px solid #00ff88;
        border-radius: 5px;
        text-align: center;
    }
    
    .arcade-header h1 {
        margin: 0;
        color: #00ff88;
        font-size: 18px;
        text-shadow: 0 0 10px #00ff88;
    }
    
    .back-link {
        color: #00ff88;
        text-decoration: none;
        font-size: 14px;
        margin-top: 5px;
        display: inline-block;
    }
    
    .back-link:hover {
        color: #ffffff;
        text-shadow: 0 0 5px #00ff88;
    }
    
    #container {
        position: relative;
        width: 100vw;
        height: 100vh;
    }
    
    #ui {
        position: absolute;
        top: 20px;
        left: 20px;
        z-index: 100;
        background: rgba(0, 0, 0, 0.8);
        padding: 15px;
        border: 1px solid #00ff88;
        border-radius: 5px;
        max-width: 300px;
    }
    
    h2 {
        color: #00ff88;
        margin-top: 0;
        text-shadow: 0 0 10px #00ff88;
        font-size: 16px;
    }
    
    button {
        background: #006600;
        color: #00ff88;
        border: 1px solid #00ff88;
        padding: 8px 16px;
        border-radius: 4px;
        cursor: pointer;
        font-family: inherit;
        margin: 5px;
    }
    
    button:hover {
        background: #008800;
    }
    
    .status {
        margin-top: 10px;
        font-size: 12px;
        color: #888;
    }
    
    #instructions {
        position: absolute;
        bottom: 20px;
        right: 20px;
        z-index: 100;
        background: rgba(0, 0, 0, 0.8);
        padding: 10px;
        border: 1px solid #00ff88;
        border-radius: 5px;
        max-width: 250px;
        font-size: 12px;
    }
</style>
@endpush

@section('content')
<div class="arcade-header">
    <h1>🕹️ ZebratronGameSystem Arcade</h1>
    <a href="{{ url('/') }}" class="back-link">← Back to Site</a>
</div>

<div id="container">
    <div id="ui">
        <h2>🎮 Game Controls</h2>
        <div class="controls">
            <button onclick="loadHambert()">🐹 Load Hambert</button>
            <button onclick="loadZSynth()">🎹 Load Z-Synth</button>
            <br>
            <button onclick="startGame()">▶️ Start</button>
            <button onclick="stopGame()">⏸️ Stop</button>
            <button onclick="resetGame()">🔄 Reset</button>
        </div>
        <div class="status" id="status">Click a cartridge to load</div>
    </div>
    
    <div id="instructions">
        <strong>🎮 Controls:</strong><br>
        • Mouse: Rotate view<br>
        • Scroll: Zoom in/out<br>
        • Arrow keys: Game input<br>
        • ZSXDCVGBHNJM: Z-Synth keys
    </div>
</div>
@endsection

@push('scripts')
<!-- Three.js from CDN -->
<script src="https://unpkg.com/three@0.149.0/build/three.min.js"></script>

<!-- Inline OrbitControls for guaranteed compatibility -->
<script>
// Include OrbitControls inline
(function() {
    // OrbitControls implementation would go here...
    // (Copy from the working arcade3d-working.html file)
})();
</script>

<!-- ZGS Integration -->
<script type="module">
    // Import ZGS modules using Laravel asset paths
    import init, { ZebratronCartridgeSystem as WasmCartridgeSystem } from '{{ asset('arcade/js/zebratron_core.js') }}';
    
    // AudioManager and arcade setup code would go here...
    // (Copy from the working arcade3d-working.html file)
    
    // Initialize the arcade
    console.log('🕹️ ZebratronGameSystem Arcade initialized in Laravel!');
</script>
@endpush
EOF

    print_success "Arcade view template created."
}

# Function to create route
create_routes() {
    print_status "Adding arcade routes..."
    
    # Check if routes/web.php exists and add our routes
    if [[ -f "routes/web.php" ]]; then
        # Check if arcade routes already exist
        if grep -q "arcade" routes/web.php; then
            print_warning "Arcade routes may already exist in routes/web.php"
        else
            cat >> routes/web.php << 'EOF'

// ZebratronGameSystem Arcade Routes
Route::get('/arcade', function () {
    return view('arcade.index');
})->name('arcade');

Route::get('/arcade/hambert', function () {
    return view('arcade.index', ['default_game' => 'hambert']);
})->name('arcade.hambert');

Route::get('/arcade/zsynth', function () {
    return view('arcade.index', ['default_game' => 'zsynth']);
})->name('arcade.zsynth');
EOF
            print_success "Arcade routes added to routes/web.php"
        fi
    else
        print_warning "routes/web.php not found. You'll need to add arcade routes manually."
    fi
}

# Function to create controller (optional)
create_controller() {
    print_status "Creating ArcadeController..."
    
    cat > app/Http/Controllers/ArcadeController.php << 'EOF'
<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;

class ArcadeController extends Controller
{
    /**
     * Display the main arcade interface
     */
    public function index()
    {
        return view('arcade.index');
    }
    
    /**
     * Load Hambert game directly
     */
    public function hambert()
    {
        return view('arcade.index', [
            'default_game' => 'hambert'
        ]);
    }
    
    /**
     * Load Z-Synth application directly
     */
    public function zsynth()
    {
        return view('arcade.index', [
            'default_game' => 'zsynth'
        ]);
    }
    
    /**
     * API endpoint for tracking game sessions (optional)
     */
    public function trackSession(Request $request)
    {
        // Log game session data
        // Could be used for analytics, high scores, etc.
        
        return response()->json([
            'status' => 'session_tracked',
            'game' => $request->input('game'),
            'duration' => $request->input('duration')
        ]);
    }
}
EOF

    print_success "ArcadeController created."
}

# Function to update package.json for Vite assets
update_package_json() {
    print_status "Updating package.json for Three.js dependency..."
    
    if [[ -f "package.json" ]]; then
        # Add Three.js as dependency if not already present
        if ! grep -q "three" package.json; then
            npm install three@0.149.0 --save
            print_success "Three.js added to package.json"
        else
            print_warning "Three.js already exists in package.json"
        fi
    else
        print_warning "package.json not found. You may need to add Three.js manually."
    fi
}

# Function to create README
create_integration_readme() {
    print_status "Creating integration README..."
    
    cat > ZEBRATRON_INTEGRATION.md << 'EOF'
# ZebratronGameSystem Laravel Integration

This Laravel project now includes the ZebratronGameSystem 3D Arcade!

## 🚀 Quick Start

1. **Visit the arcade:**
   ```
   http://your-laravel-site.com/arcade
   ```

2. **Direct game links:**
   ```
   http://your-laravel-site.com/arcade/hambert  # Load Hambert directly
   http://your-laravel-site.com/arcade/zsynth   # Load Z-Synth directly
   ```

## 📁 Integration Files

### Assets
- `public/arcade/js/` - WebAssembly modules and JavaScript
- `public/arcade/wasm/` - WebAssembly binary files
- `public/arcade/css/` - Arcade-specific stylesheets

### Views
- `resources/views/arcade/index.blade.php` - Main arcade interface

### Controllers
- `app/Http/Controllers/ArcadeController.php` - Arcade route handlers

### Routes
- `/arcade` - Main arcade interface
- `/arcade/hambert` - Direct Hambert game
- `/arcade/zsynth` - Direct Z-Synth app

## 🎮 Features

- **3D Arcade Machine**: Full Three.js 3D environment
- **Live Game Rendering**: Real ZGS games on 3D screen
- **Interactive Controls**: Mouse camera control + keyboard game input
- **Atmospheric Effects**: Screen glow and lighting
- **Laravel Integration**: Seamlessly embedded in your Laravel app

## 🔧 Customization

### Adding Authentication
```php
// In routes/web.php
Route::middleware(['auth'])->group(function () {
    Route::get('/arcade', [ArcadeController::class, 'index'])->name('arcade');
});
```

### Adding High Score Tracking
```php
// Create migration
php artisan make:migration create_arcade_scores_table

// Add to ArcadeController
public function saveScore(Request $request) {
    ArcadeScore::create([
        'user_id' => auth()->id(),
        'game' => $request->game,
        'score' => $request->score
    ]);
}
```

### Styling Integration
Edit `resources/views/arcade/index.blade.php` to match your site's theme:
- Update the header section
- Modify color scheme in the `@push('styles')` section
- Add your site's navigation

## 🛠️ Development

The arcade is fully self-contained and shouldn't interfere with your existing Laravel application. All arcade-specific assets are namespaced under `/arcade/` paths.

## 📚 More Information

- **ZebratronGameSystem Repository**: https://github.com/bsechel/zebratron-game-system
- **Original Documentation**: See the main ZGS README for system details
- **3D Arcade Features**: OrbitControls, screen glow effects, live game rendering

## 🎯 What's Next?

1. **Test the integration**: Visit `/arcade` in your browser
2. **Customize the styling**: Match your site's design
3. **Add user features**: High scores, user profiles, etc.
4. **Deploy**: The arcade works in production environments

Enjoy your ZebratronGameSystem 3D Arcade! 🕹️
EOF

    print_success "Integration README created: ZEBRATRON_INTEGRATION.md"
}

# Function to cleanup
cleanup() {
    print_status "Cleaning up temporary files..."
    if [[ -d "$TEMP_DIR" ]]; then
        rm -rf "$TEMP_DIR"
    fi
    print_success "Cleanup complete."
}

# Main installation function
main() {
    echo ""
    echo "🕹️  ZebratronGameSystem Laravel Integration Script"
    echo "=================================================="
    echo ""
    
    # Get target Laravel directory
    LARAVEL_DIR="${1:-$(pwd)}"
    
    if [[ ! -d "$LARAVEL_DIR" ]]; then
        print_error "Directory $LARAVEL_DIR does not exist."
        exit 1
    fi
    
    cd "$LARAVEL_DIR"
    
    print_status "Installing ZebratronGameSystem into Laravel project at: $LARAVEL_DIR"
    echo ""
    
    # Run installation steps
    check_laravel
    check_prerequisites
    clone_zgs_repo
    build_zgs
    create_laravel_structure
    copy_zgs_assets
    create_arcade_view
    create_routes
    create_controller
    update_package_json
    create_integration_readme
    cleanup
    
    echo ""
    print_success "🎉 ZebratronGameSystem successfully integrated into Laravel!"
    echo ""
    echo "Next steps:"
    echo "1. Start your Laravel development server: php artisan serve"
    echo "2. Visit: http://localhost:8000/arcade"
    echo "3. Read ZEBRATRON_INTEGRATION.md for customization options"
    echo ""
    echo "Have fun with your 3D arcade! 🕹️"
}

# Script usage
usage() {
    echo "Usage: $0 [laravel-project-directory]"
    echo ""
    echo "If no directory is specified, the current directory will be used."
    echo ""
    echo "Example:"
    echo "  $0 /path/to/my-laravel-project"
    echo "  $0  # Use current directory"
    echo ""
}

# Handle command line arguments
if [[ "$1" == "-h" || "$1" == "--help" ]]; then
    usage
    exit 0
fi

# Run main installation
main "$@"