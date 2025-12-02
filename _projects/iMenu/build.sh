#!/usr/bin/env bash
# Build script for iMenu - Creates distribution package
# Detects shell and OS, cleans dist/, and rebuilds everything

set -e

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Detect OS
detect_os() {
    case "$(uname -s)" in
        Linux*)
            echo "linux"
            ;;
        Darwin*)
            echo "darwin"
            ;;
        MINGW*|MSYS*|CYGWIN*)
            echo "windows"
            ;;
        *)
            echo "unknown"
            ;;
    esac
}

# Detect architecture
detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64)
            echo "amd64"
            ;;
        arm64|aarch64)
            echo "arm64"
            ;;
        arm*)
            echo "arm"
            ;;
        *)
            echo "$(uname -m)"
            ;;
    esac
}

OS=$(detect_os)
ARCH=$(detect_arch)

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔨 Building iMenu Distribution Package"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📋 Detected:"
echo "   OS: $OS"
echo "   Architecture: $ARCH"
echo "   Shell: Bash"
echo ""

# Check if Go is installed
if ! command -v go &> /dev/null; then
    echo "❌ Go is not installed. Please install Go first."
    echo "   Visit: https://go.dev/dl/"
    exit 1
fi

echo "✅ Go found: $(go version)"
echo ""

# Clean dist directory
echo "🧹 Cleaning dist directory..."
rm -rf dist/
echo "✅ Cleaned"
echo ""

# Recreate dist structure
echo "📁 Creating distribution structure..."
mkdir -p dist/bin
mkdir -p dist/lib
mkdir -p dist/docs
echo "✅ Structure created"
echo ""

# Update Go dependencies
echo "📦 Updating Go dependencies..."
go mod tidy
echo "✅ Dependencies updated"
echo ""

# Build executables
echo "🔨 Building executables..."
echo ""

# Build prompt-wizard
echo "   Building prompt-wizard..."
if [ "$OS" = "windows" ]; then
    go build -o dist/bin/prompt-wizard.exe ./cmd/prompt-wizard
    if [ -f "dist/bin/prompt-wizard.exe" ]; then
        echo "   ✅ Built: dist/bin/prompt-wizard.exe"
    else
        echo "   ❌ Failed to build prompt-wizard.exe"
        exit 1
    fi
else
    go build -o dist/bin/prompt-wizard ./cmd/prompt-wizard
    if [ -f "dist/bin/prompt-wizard" ]; then
        echo "   ✅ Built: dist/bin/prompt-wizard"
    else
        echo "   ❌ Failed to build prompt-wizard"
        exit 1
    fi
fi

# Build prompt-huh (optional)
if [ -f "cmd/prompt-huh/main.go" ]; then
    echo "   Building prompt-huh..."
    if [ "$OS" = "windows" ]; then
        go build -o dist/bin/prompt-huh.exe ./cmd/prompt-huh 2>/dev/null || echo "   ⚠️  prompt-huh build skipped (optional)"
    else
        go build -o dist/bin/prompt-huh ./cmd/prompt-huh 2>/dev/null || echo "   ⚠️  prompt-huh build skipped (optional)"
    fi
fi

echo ""

# Copy wrapper scripts to dist/lib
echo "📋 Copying wrapper scripts..."
cp wizard.sh dist/lib/
cp wizard.ps1 dist/lib/
chmod +x dist/lib/wizard.sh
echo "✅ Wrapper scripts copied"
echo ""

# Copy documentation to dist/docs
echo "📚 Copying documentation..."
if [ -d "docs" ]; then
    cp docs/*.md dist/docs/ 2>/dev/null || true
    echo "✅ Documentation copied"
else
    echo "⚠️  No docs directory found"
fi
echo ""

# Copy dist README if it exists
if [ -f "dist/README.md" ]; then
    echo "✅ Distribution README exists"
elif [ -f "README.md" ]; then
    # If there's a main README, we could copy it, but dist has its own
    echo "📝 Note: dist/README.md should exist for distribution package"
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Build complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📦 Distribution package created in: dist/"
echo ""
echo "📁 Structure:"
echo "   dist/"
echo "   ├── bin/          # Executables"
echo "   ├── lib/          # Wrapper scripts"
echo "   ├── docs/         # Documentation"
echo "   └── README.md     # Package README"
echo ""
echo "💡 Usage:"
echo "   cd dist"
echo "   source lib/wizard.sh"
echo ""

