#!/usr/bin/env bash
# Bootstrap GitHub SSH authentication setup for system restore
# Creates SSH key for GitHub authentication and configures Git to use SSH

set -e

SSH_KEY_PATH="$HOME/.ssh/github_pi"
SSH_CONFIG="$HOME/.ssh/config"
GIT_EMAIL="${GIT_EMAIL:-$(git config --global user.email 2>/dev/null || echo 'user@example.com')}"
GIT_REPO_DIR="$HOME/_playground"

# Source wizard helper functions
IWIZARD_FUNCTIONS="$HOME/_playground/projects/iMenu/iwizard-functions.sh"
if [ -f "$IWIZARD_FUNCTIONS" ]; then
    source "$IWIZARD_FUNCTIONS"
else
    echo "⚠️  Warning: iwizard-functions.sh not found at $IWIZARD_FUNCTIONS" >&2
    echo "   Wizard functionality may not work properly" >&2
fi

# Show help by default
show_help() {
    cat << EOF
GitHub SSH Bootstrap Script

Usage: $0 [command]

Commands:
  setup              Full setup: SSH key, config, and git remote
  status             Show current status (key, remotes, repo, etc.)
  secrets            Setup or remove git-crypt for encrypted secrets
  remove-key         Remove SSH key
  remove-remote      Remove/detach from git remote(s)
  remove-repo        Remove local git repository (.git directory)
  help               Show this help

Examples:
  $0 setup           # Full setup
  $0 status          # Show current status
  $0 secrets         # Setup or remove git-crypt for secrets
  $0 remove-key      # Remove SSH key
  $0 remove-remote   # Remove git remotes
  $0 remove-repo     # Remove local git repo
EOF
}

# Show status
show_status() {
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "📊 GitHub SSH Status"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    
    # SSH Key Status
    echo "🔑 SSH Key:"
    if [ -f "$SSH_KEY_PATH" ]; then
        echo "  ✅ Key exists: $SSH_KEY_PATH"
        KEY_PERMS=$(stat -c "%a" "$SSH_KEY_PATH" 2>/dev/null || stat -f "%OLp" "$SSH_KEY_PATH" 2>/dev/null || echo "unknown")
        if [ "$KEY_PERMS" = "600" ]; then
            echo "  ✅ Permissions: $KEY_PERMS (correct)"
        else
            echo "  ⚠️  Permissions: $KEY_PERMS (should be 600)"
        fi
        if [ -f "$SSH_KEY_PATH.pub" ]; then
            echo "  ✅ Public key exists"
            echo "  📋 Public key fingerprint:"
            ssh-keygen -lf "$SSH_KEY_PATH.pub" 2>/dev/null | sed 's/^/     /' || echo "     (could not read)"
        else
            echo "  ⚠️  Public key missing"
        fi
    else
        echo "  ❌ Key not found: $SSH_KEY_PATH"
    fi
    echo ""
    
    # SSH Config Status
    echo "⚙️  SSH Config:"
    if [ -f "$SSH_CONFIG" ]; then
        if grep -q "Host github.com" "$SSH_CONFIG" 2>/dev/null; then
            echo "  ✅ GitHub config present in $SSH_CONFIG"
            echo "  📋 Config:"
            grep -A 4 "Host github.com" "$SSH_CONFIG" | sed 's/^/     /'
        else
            echo "  ⚠️  GitHub config not found in $SSH_CONFIG"
        fi
    else
        echo "  ⚠️  SSH config file not found: $SSH_CONFIG"
    fi
    echo ""
    
    # Local Git Repository Status
    echo "📂 Local Git Repository:"
    if [ -d "$GIT_REPO_DIR/.git" ]; then
        echo "  ✅ Repository exists: $GIT_REPO_DIR"
        cd "$GIT_REPO_DIR"
        CURRENT_BRANCH=$(git branch --show-current 2>/dev/null || echo "unknown")
        echo "  📋 Current branch: $CURRENT_BRANCH"
        
        # Local commit info
        if git rev-parse HEAD &>/dev/null; then
            LAST_COMMIT=$(git log -1 --oneline 2>/dev/null | head -1)
            echo "  📋 Last commit: $LAST_COMMIT"
        else
            echo "  ⚠️  No commits yet"
        fi
        
        # Local branch info
        LOCAL_BRANCHES=$(git branch 2>/dev/null | wc -l)
        echo "  📋 Local branches: $LOCAL_BRANCHES"
    else
        echo "  ❌ No git repository found at $GIT_REPO_DIR"
    fi
    echo ""
    
    # Remote Git Repository Status
    echo "🌐 Remote Git Repository:"
    if [ -d "$GIT_REPO_DIR/.git" ]; then
        cd "$GIT_REPO_DIR"
        REMOTES=$(git remote)
        if [ -n "$REMOTES" ]; then
            for REMOTE in $REMOTES; do
                REMOTE_URL=$(git remote get-url "$REMOTE" 2>/dev/null)
                echo "  📋 Remote '$REMOTE': $REMOTE_URL"
                
                # Check if remote is reachable
                if git ls-remote "$REMOTE" &>/dev/null; then
                    echo "     ✅ Remote is reachable"
                else
                    echo "     ⚠️  Remote is not reachable"
                fi
            done
            
            # Upstream tracking
            if git rev-parse --abbrev-ref --symbolic-full-name @{u} &>/dev/null; then
                UPSTREAM=$(git rev-parse --abbrev-ref --symbolic-full-name @{u} 2>/dev/null)
                echo "  📋 Upstream tracking: $UPSTREAM"
            else
                echo "  ⚠️  No upstream branch configured"
            fi
        else
            echo "  ⚠️  No remotes configured"
        fi
    else
        echo "  ⚠️  No local repository (cannot check remotes)"
    fi
    echo ""
    
    # GitHub SSH Connection Test
    echo "🔌 GitHub SSH Connection:"
    if [ -f "$SSH_KEY_PATH" ]; then
        # Capture output and exit code without triggering set -e
        set +e
        TEST_OUTPUT=$(timeout 5 ssh -o ConnectTimeout=5 -o BatchMode=yes -T git@github.com 2>&1)
        EXIT_CODE=$?
        set -e
        
        if echo "$TEST_OUTPUT" | grep -qi "successfully authenticated"; then
            echo "  ✅ SSH connection successful"
            GITHUB_USER=$(echo "$TEST_OUTPUT" | grep -oP "(?<=Hi )\w+" || echo "unknown")
            echo "  📋 Authenticated as: $GITHUB_USER"
        elif echo "$TEST_OUTPUT" | grep -qi "permission denied"; then
            echo "  ⚠️  Permission denied (key may not be added to GitHub)"
            echo "     Add key at: https://github.com/settings/keys"
            echo ""
            echo "  📋 Your public key (copy and add to GitHub):"
            cat "$SSH_KEY_PATH.pub" | sed 's/^/     /'
        elif echo "$TEST_OUTPUT" | grep -qi "host key verification failed"; then
            echo "  ⚠️  Host key verification failed"
            echo "     Run: ssh-keyscan github.com >> ~/.ssh/known_hosts"
        elif [ $EXIT_CODE -eq 124 ]; then
            echo "  ⚠️  Connection test timed out"
            echo "     Check network connectivity"
        else
            echo "  ⚠️  Connection test failed (exit code: $EXIT_CODE)"
            if [ -n "$TEST_OUTPUT" ]; then
                echo "     Output: $(echo "$TEST_OUTPUT" | head -1)"
            fi
            echo "     Run manually: ssh -T git@github.com"
        fi
    else
        echo "  ⚠️  Cannot test (SSH key not found)"
    fi
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

# Individual setup functions

# Setup SSH key
setup_ssh_key() {
    # Check if SSH key already exists
    if [ -f "$SSH_KEY_PATH" ]; then
        echo "⚠️  SSH key already exists at $SSH_KEY_PATH"
        read -p "Recreate SSH key? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "ℹ️  Skipping SSH key generation"
            return 0
        else
            echo "🗑️  Removing existing key..."
            rm -f "$SSH_KEY_PATH" "$SSH_KEY_PATH.pub"
        fi
    fi
    
    # Generate SSH key if it doesn't exist
    if [ ! -f "$SSH_KEY_PATH" ]; then
        echo "🔐 Generating SSH key pair..."
        mkdir -p "$HOME/.ssh"
        chmod 700 "$HOME/.ssh"
        ssh-keygen -t ed25519 -C "$GIT_EMAIL" -f "$SSH_KEY_PATH" -N ""
        echo "✅ SSH key created: $SSH_KEY_PATH"
        
        # Fix SSH key permissions
        chmod 600 "$SSH_KEY_PATH"
        chmod 644 "$SSH_KEY_PATH.pub"
    else
        echo "✅ Using existing SSH key"
    fi
    
    # Display public key
    echo ""
    echo "📋 Your public SSH key:"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    cat "$SSH_KEY_PATH.pub"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
}

# Setup SSH config
setup_ssh_config() {
    echo "⚙️  Configuring SSH for GitHub..."
    mkdir -p "$HOME/.ssh"
    
    # Check if GitHub config already exists
    if grep -q "Host github.com" "$SSH_CONFIG" 2>/dev/null; then
        echo "⚠️  GitHub SSH config already exists in $SSH_CONFIG"
        read -p "Update configuration? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            # Remove existing GitHub config block
            sed -i '/^Host github\.com$/,/^$/d' "$SSH_CONFIG"
            echo "🗑️  Removed existing GitHub configuration"
        else
            echo "ℹ️  Keeping existing SSH config"
            return 0
        fi
    fi
    
    # Add GitHub SSH config if not present
    if ! grep -q "Host github.com" "$SSH_CONFIG" 2>/dev/null; then
        cat >> "$SSH_CONFIG" << EOF
Host github.com
    HostName github.com
    User git
    IdentityFile ~/.ssh/github_pi
    IdentitiesOnly yes

EOF
        echo "✅ SSH config updated"
    fi
    
    chmod 600 "$SSH_CONFIG"
}

# Setup local git repository
setup_local_repo() {
    if [ -d "$GIT_REPO_DIR/.git" ]; then
        echo "✅ Local git repository already exists at $GIT_REPO_DIR"
        return 0
    fi
    
    echo "📂 Setting up local git repository..."
    mkdir -p "$GIT_REPO_DIR"
    cd "$GIT_REPO_DIR"
    
    # Initialize git repository
    git init
    echo "✅ Git repository initialized"
    
    # Set default branch to main
    git branch -M main 2>/dev/null || git checkout -b main 2>/dev/null
    
    # Configure git user if not set
    if ! git config user.name &>/dev/null; then
        GIT_NAME=$(git config --global user.name 2>/dev/null || echo "")
        if [ -z "$GIT_NAME" ]; then
            read -p "Enter your name for git commits: " GIT_NAME
            git config user.name "$GIT_NAME"
            git config --global user.name "$GIT_NAME"
        else
            git config user.name "$GIT_NAME"
        fi
    fi
    
    if ! git config user.email &>/dev/null; then
        GIT_EMAIL=$(git config --global user.email 2>/dev/null || echo "")
        if [ -z "$GIT_EMAIL" ]; then
            read -p "Enter your email for git commits: " GIT_EMAIL
            git config user.email "$GIT_EMAIL"
            git config --global user.email "$GIT_EMAIL"
        else
            git config user.email "$GIT_EMAIL"
        fi
    fi
    
    echo "✅ Local git repository configured"
}

# Individual remove functions

# Remove SSH key
remove_ssh_key() {
    if [ -f "$SSH_KEY_PATH" ] || [ -f "$SSH_KEY_PATH.pub" ]; then
        echo "⚠️  WARNING: This will remove the SSH key at $SSH_KEY_PATH"
        read -p "Are you sure? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            rm -f "$SSH_KEY_PATH" "$SSH_KEY_PATH.pub"
            echo "✅ SSH key removed"
            echo "   Note: You may want to remove it from GitHub: https://github.com/settings/keys"
        else
            echo "❌ Cancelled"
            return 1
        fi
    else
        echo "ℹ️  No SSH key found at $SSH_KEY_PATH"
    fi
}

# Remove SSH config
remove_ssh_config() {
    if [ -f "$SSH_CONFIG" ]; then
        if grep -q "Host github.com" "$SSH_CONFIG" 2>/dev/null; then
            echo "⚠️  This will remove GitHub SSH config from $SSH_CONFIG"
            read -p "Are you sure? (y/N): " -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                # Remove existing GitHub config block
                sed -i '/^Host github\.com$/,/^$/d' "$SSH_CONFIG"
                echo "✅ GitHub SSH config removed from $SSH_CONFIG"
            else
                echo "❌ Cancelled"
                return 1
            fi
        else
            echo "ℹ️  No GitHub SSH config found in $SSH_CONFIG"
        fi
    else
        echo "ℹ️  SSH config file not found: $SSH_CONFIG"
    fi
}

# Remove local git repository
remove_local_repo() {
    if [ -d "$GIT_REPO_DIR/.git" ]; then
        echo "⚠️  WARNING: This will remove the git repository at $GIT_REPO_DIR"
        echo "   This will NOT delete your files, only the .git directory"
        read -p "Are you sure? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            rm -rf "$GIT_REPO_DIR/.git"
            echo "✅ Git repository removed from $GIT_REPO_DIR"
            echo "   Your files are still intact"
        else
            echo "❌ Cancelled"
            return 1
        fi
    else
        echo "ℹ️  No git repository found at $GIT_REPO_DIR"
    fi
}

# Remove git remote
remove_remote() {
    if [ -d "$GIT_REPO_DIR/.git" ]; then
        cd "$GIT_REPO_DIR"
        REMOTES=$(git remote)
        if [ -n "$REMOTES" ]; then
            echo "📋 Current remotes:"
            git remote -v
            echo ""
            read -p "Remove all remotes? (y/N): " -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                for REMOTE in $REMOTES; do
                    git remote remove "$REMOTE"
                    echo "✅ Removed remote: $REMOTE"
                done
                echo "✅ All remotes removed"
            else
                echo "❌ Cancelled"
                return 1
            fi
        else
            echo "ℹ️  No remotes configured"
        fi
    else
        echo "ℹ️  No git repository found at $GIT_REPO_DIR"
    fi
}

# Check if repository exists on GitHub
check_repo_exists() {
    local GITHUB_USER=$1
    local REPO_NAME=$2
    
    # Try to check via git ls-remote (works if repo exists and is accessible)
    set +e
    git ls-remote "git@github.com:$GITHUB_USER/$REPO_NAME.git" &>/dev/null
    local EXISTS=$?
    set -e
    
    if [ $EXISTS -eq 0 ]; then
        return 0  # Repository exists
    else
        return 1  # Repository doesn't exist or not accessible
    fi
}

# Create repository on GitHub using GitHub CLI
create_repo_with_gh() {
    local GITHUB_USER=$1
    local REPO_NAME=$2
    local IS_PRIVATE=$3
    
    if ! command -v gh &> /dev/null; then
        return 1
    fi
    
    # Check if gh is authenticated (should already be done in setup_remote, but double-check)
    if ! gh auth status &>/dev/null; then
        return 1
    fi
    
    # Create repository
    set +e
    if [ "$IS_PRIVATE" = "true" ]; then
        gh repo create "$REPO_NAME" --private --source=. --remote=origin --push &>/dev/null
    else
        gh repo create "$REPO_NAME" --public --source=. --remote=origin --push &>/dev/null
    fi
    local RESULT=$?
    set -e
    
    if [ $RESULT -eq 0 ]; then
        return 0
    else
        return 1
    fi
}

# Create repository on GitHub using API
create_repo_with_api() {
    local GITHUB_USER=$1
    local REPO_NAME=$2
    local IS_PRIVATE=$3
    local GITHUB_TOKEN=$4
    
    if [ -z "$GITHUB_TOKEN" ]; then
        return 1
    fi
    
    local VISIBILITY="public"
    if [ "$IS_PRIVATE" = "true" ]; then
        VISIBILITY="private"
    fi
    
    set +e
    RESPONSE=$(curl -s -w "\n%{http_code}" -X POST \
        -H "Accept: application/vnd.github.v3+json" \
        -H "Authorization: token $GITHUB_TOKEN" \
        https://api.github.com/user/repos \
        -d "{\"name\":\"$REPO_NAME\",\"private\":$IS_PRIVATE,\"auto_init\":false}" 2>&1)
    HTTP_CODE=$(echo "$RESPONSE" | tail -1)
    set -e
    
    if [ "$HTTP_CODE" = "201" ]; then
        return 0
    else
        return 1
    fi
}

# Setup git remote
setup_remote() {
    cd "$GIT_REPO_DIR"
    
    # Check if remotes already exist
    REMOTES=$(git remote 2>/dev/null)
    if [ -n "$REMOTES" ]; then
        echo "📋 Existing remotes found:"
        git remote -v
        echo ""
        read -p "Remove existing remotes and set up new one? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            for REMOTE in $REMOTES; do
                git remote remove "$REMOTE"
                echo "✅ Removed remote: $REMOTE"
            done
        else
            echo "ℹ️  Keeping existing remotes"
            return 0
        fi
    fi
    
    # Get GitHub username
    GITHUB_USER=$(git config --global user.name 2>/dev/null || echo "")
    if [ -z "$GITHUB_USER" ]; then
        read -p "Enter your GitHub username: " GITHUB_USER
        if [ -z "$GITHUB_USER" ]; then
            echo "⚠️  No username provided, skipping remote setup"
            return 1
        fi
    else
        echo "ℹ️  Detected GitHub username: $GITHUB_USER"
        read -p "Is this correct? (Y/n): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Nn]$ ]]; then
            read -p "Enter your GitHub username: " GITHUB_USER
        fi
    fi
    
    # Prompt for repository name
    DEFAULT_REPO="Skinny-Pi"
    read -p "Enter repository name [$DEFAULT_REPO]: " REPO_NAME
    REPO_NAME="${REPO_NAME:-$DEFAULT_REPO}"
    
    # Check if repository already exists
    echo ""
    echo "🔍 Checking if repository exists on GitHub..."
    if check_repo_exists "$GITHUB_USER" "$REPO_NAME"; then
        echo "✅ Repository already exists: $GITHUB_USER/$REPO_NAME"
    else
        echo "ℹ️  Repository doesn't exist yet: $GITHUB_USER/$REPO_NAME"
        echo ""
        read -p "Create repository on GitHub? (Y/n): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Nn]$ ]]; then
            # Ask for visibility
            read -p "Make repository private? (y/N): " -n 1 -r
            echo
            IS_PRIVATE="false"
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                IS_PRIVATE="true"
            fi
            
            # Check if GitHub CLI is available and authenticated
            if command -v gh &> /dev/null; then
                # Check if gh is authenticated
                if ! gh auth status &>/dev/null; then
                    echo ""
                    echo "🔐 GitHub CLI not authenticated"
                    echo "   Running: gh auth login"
                    echo ""
                    if gh auth login; then
                        echo "✅ GitHub CLI authenticated"
                    else
                        echo "⚠️  GitHub CLI authentication failed or cancelled"
                        echo "   Will try alternative methods..."
                    fi
                    echo ""
                fi
            fi
            
            # Try to create using GitHub CLI
            echo ""
            echo "🔨 Creating repository on GitHub..."
            if create_repo_with_gh "$GITHUB_USER" "$REPO_NAME" "$IS_PRIVATE"; then
                echo "✅ Repository created using GitHub CLI"
                # gh repo create already sets up the remote and pushes
                CURRENT_BRANCH=$(git branch --show-current 2>/dev/null || echo "main")
                git branch --set-upstream-to=origin/$CURRENT_BRANCH 2>/dev/null || true
                echo ""
                echo "📋 Current remotes:"
                git remote -v
                return 0
            else
                # Try using API with token from environment or secrets
                GITHUB_TOKEN="${GITHUB_TOKEN:-}"
                if [ -f "$HOME/.secrets" ]; then
                    # Try to get token from secrets file
                    GITHUB_TOKEN=$(grep "^GITHUB_TOKEN=" "$HOME/.secrets" 2>/dev/null | cut -d'=' -f2- | tr -d '"' | tr -d "'" || echo "")
                fi
                
                if [ -n "$GITHUB_TOKEN" ]; then
                    echo "  🔑 Using GitHub token from environment/secrets..."
                    if create_repo_with_api "$GITHUB_USER" "$REPO_NAME" "$IS_PRIVATE" "$GITHUB_TOKEN"; then
                        echo "✅ Repository created using GitHub API"
                    else
                        echo "⚠️  Failed to create repository via API"
                        echo "   You'll need to create it manually"
                    fi
                else
                    echo "⚠️  Cannot create repository automatically"
                    echo "   Options:"
                    echo "   1. Install GitHub CLI: apt install gh && gh auth login"
                    echo "   2. Set GITHUB_TOKEN environment variable"
                    echo "   3. Add GITHUB_TOKEN to ~/.secrets file"
                    echo "   4. Create manually at: https://github.com/new"
                fi
            fi
        fi
    fi
    
    # Add remote if it doesn't exist
    if ! git remote | grep -q "^origin$"; then
        REMOTE_URL="git@github.com:$GITHUB_USER/$REPO_NAME.git"
        git remote add origin "$REMOTE_URL"
        echo "✅ Added remote 'origin': $REMOTE_URL"
    else
        # Update remote URL if it's different
        CURRENT_URL=$(git remote get-url origin 2>/dev/null || echo "")
        NEW_URL="git@github.com:$GITHUB_USER/$REPO_NAME.git"
        if [ "$CURRENT_URL" != "$NEW_URL" ]; then
            git remote set-url origin "$NEW_URL"
            echo "✅ Updated remote 'origin': $NEW_URL"
        fi
    fi
    
    # Set upstream branch
    CURRENT_BRANCH=$(git branch --show-current 2>/dev/null || echo "main")
    echo "🔧 Setting upstream branch to origin/$CURRENT_BRANCH"
    git branch --set-upstream-to=origin/$CURRENT_BRANCH 2>/dev/null || echo "⚠️  Upstream will be set on first push"
    
    echo ""
    echo "📋 Current remotes:"
    git remote -v
    echo ""
    
    # Check if repository exists before showing next steps
    if ! check_repo_exists "$GITHUB_USER" "$REPO_NAME"; then
        echo "📝 Next steps:"
        echo "   1. Create the repository on GitHub: https://github.com/new"
        echo "      Name: $REPO_NAME"
        echo "      Don't initialize with README (we already have one)"
        echo ""
        echo "   2. Push to GitHub:"
        echo "      git add ."
        echo "      git commit -m 'Initial $REPO_NAME repository setup'"
        echo "      git push -u origin $CURRENT_BRANCH"
    else
        echo "✅ Repository is ready! Push your code:"
        echo "   git push -u origin $CURRENT_BRANCH"
    fi
}

# Helper function to parse JSON result
parse_json_result() {
    local json="$1"
    local key="$2"
    python3 -c "import sys, json; data=json.load(sys.stdin); print(data.get('$key', ''))" <<< "$json" 2>/dev/null || echo ""
}

# Helper function to parse multiselect result
parse_multiselect_result() {
    local json="$1"
    local key="$2"
    python3 -c "import sys, json; data=json.load(sys.stdin); print('\\n'.join(data.get('$key', [])))" <<< "$json" 2>/dev/null || echo ""
}

# Git-crypt complete setup - does all steps
setup_git_crypt() {
    # Install git-crypt if not already installed
    if ! command -v git-crypt &> /dev/null; then
        echo "📦 Installing git-crypt..."
        if command -v apt &> /dev/null; then
            if [ "$EUID" -eq 0 ]; then
                apt update -qq && apt install -y git-crypt
            else
                sudo apt update -qq && sudo apt install -y git-crypt
            fi
            if [ $? -ne 0 ]; then
                echo "❌ Failed to install git-crypt"
                return 1
            fi
            echo "✅ git-crypt installed"
            echo ""
        else
            echo "❌ Cannot install git-crypt: apt not found"
            echo "   Please install git-crypt manually"
            return 1
        fi
    fi
    
    cd "$GIT_REPO_DIR"
    if [ ! -d ".git" ]; then
        echo "❌ Not a git repository. Run 'Setup local git repository' first."
        return 1
    fi
    
    echo "🔐 Setting up git-crypt..."
    echo ""
    
    # Initialize git-crypt
    git_crypt_init
    echo ""
    
    # Setup .gitattributes
    git_crypt_setup_gitattributes
    echo ""
    
    # Update .gitignore
    git_crypt_update_gitignore
    echo ""
    
    # Copy secrets file to repo
    git_crypt_copy_secrets
    echo ""
    
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "✅ git-crypt setup complete!"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "📝 Next steps:"
    echo "   1. Edit .secrets file if needed:"
    echo "      nano $GIT_REPO_DIR/.secrets"
    echo ""
    echo "   2. Add and commit:"
    echo "      cd $GIT_REPO_DIR"
    echo "      git add .gitattributes .secrets"
    echo "      git commit -m 'Add encrypted .secrets with git-crypt'"
    echo ""
    echo "   3. Push to GitHub:"
    echo "      git push origin main"
    echo ""
}

# Git-crypt complete removal - does all steps
remove_git_crypt() {
    cd "$GIT_REPO_DIR"
    
    echo "🗑️  Removing git-crypt..."
    echo ""
    
    # Remove git-crypt initialization
    git_crypt_remove_init
    echo ""
    
    # Restore .gitignore
    git_crypt_restore_gitignore
    echo ""
    
    # Remove .gitattributes entries
    git_crypt_remove_gitattributes
    echo ""
    
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "✅ git-crypt removal complete!"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

# Git-crypt setup functions
git_crypt_init() {
    cd "$GIT_REPO_DIR"
    if [ ! -d ".git" ]; then
        echo "❌ Not a git repository"
        return 1
    fi
    
    if git-crypt status &>/dev/null; then
        echo "⚠️  git-crypt already initialized"
        return 0
    fi
    
    echo "🔐 Initializing git-crypt..."
    git-crypt init
    echo "✅ git-crypt initialized"
}

git_crypt_setup_gitattributes() {
    cd "$GIT_REPO_DIR"
    
    if [ ! -f ".gitattributes" ]; then
        cat > ".gitattributes" << 'EOF'
# Encrypted files with git-crypt
.secrets filter=git-crypt diff=git-crypt
*.secrets filter=git-crypt diff=git-crypt
EOF
        echo "✅ Created .gitattributes"
    else
        if grep -q "^\.secrets" ".gitattributes" 2>/dev/null; then
            echo "✅ .secrets already in .gitattributes"
        else
            cat >> ".gitattributes" << 'EOF'

# Encrypted files with git-crypt
.secrets filter=git-crypt diff=git-crypt
*.secrets filter=git-crypt diff=git-crypt
EOF
            echo "✅ Added .secrets to .gitattributes"
        fi
    fi
}

git_crypt_update_gitignore() {
    cd "$GIT_REPO_DIR"
    
    if [ ! -f ".gitignore" ]; then
        echo "❌ .gitignore not found"
        return 1
    fi
    
    # Remove .secrets from .gitignore (we want to track encrypted version)
    if grep -q "^\.secrets$" ".gitignore" 2>/dev/null; then
        cp ".gitignore" ".gitignore.bak"
        sed -i '/^\.secrets$/d' ".gitignore"
        echo "✅ Removed .secrets from .gitignore"
        echo "   Backup saved to .gitignore.bak"
    else
        echo "✅ .secrets not in .gitignore (or already removed)"
    fi
    
    # Add note about encrypted secrets
    if ! grep -q "# Encrypted .secrets is tracked" ".gitignore" 2>/dev/null; then
        cat >> ".gitignore" << 'EOF'

# Note: .secrets is encrypted with git-crypt and IS tracked in git
# The encrypted version is safe to commit
EOF
        echo "✅ Added note about encrypted .secrets"
    fi
}

git_crypt_copy_secrets() {
    local SECRETS_FILE="$HOME/.secrets"
    local REPO_SECRETS="$GIT_REPO_DIR/.secrets"
    
    if [ -f "$SECRETS_FILE" ]; then
        cp "$SECRETS_FILE" "$REPO_SECRETS"
        chmod 600 "$REPO_SECRETS"
        echo "✅ Copied secrets file to repository"
    else
        echo "⚠️  Secrets file not found at $SECRETS_FILE"
        echo "   Creating template..."
        cat > "$REPO_SECRETS" << 'EOF'
# Unified Secrets File
# This file is encrypted with git-crypt
# Add your passwords/keys here
# 
# Format: KEY=VALUE (one per line)
# Comments start with #

# ============================================
# MQTT Broker
# ============================================
MQTT_PASSWORD=
MQTT_USERNAME=mqtt

# ============================================
# GitHub (if needed)
# ============================================
# GITHUB_TOKEN=

# ============================================
# Other APIs / Services
# ============================================
# API_KEY_SERVICE1=
# API_KEY_SERVICE2=
EOF
        chmod 600 "$REPO_SECRETS"
        echo "✅ Created template .secrets file"
    fi
}

# Git-crypt removal functions
git_crypt_remove_init() {
    cd "$GIT_REPO_DIR"
    if git-crypt status &>/dev/null; then
        echo "⚠️  WARNING: This will remove git-crypt encryption"
        echo "   Encrypted files will become unencrypted"
        read -p "Continue? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            git-crypt unlock 2>/dev/null || true
            rm -f .git/git-crypt/keys/default
            echo "✅ git-crypt removed"
        else
            echo "❌ Cancelled"
            return 1
        fi
    else
        echo "ℹ️  git-crypt not initialized"
    fi
}

git_crypt_restore_gitignore() {
    cd "$GIT_REPO_DIR"
    
    if [ -f ".gitignore.bak" ]; then
        read -p "Restore .gitignore from backup? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            mv ".gitignore.bak" ".gitignore"
            echo "✅ Restored .gitignore from backup"
        fi
    else
        # Add .secrets back to .gitignore
        if ! grep -q "^\.secrets$" ".gitignore" 2>/dev/null; then
            echo ".secrets" >> ".gitignore"
            echo "✅ Added .secrets back to .gitignore"
        fi
    fi
    
    # Remove note about encrypted secrets
    sed -i '/# Note: .secrets is encrypted with git-crypt/d' ".gitignore" 2>/dev/null || true
    sed -i '/# The encrypted version is safe to commit/d' ".gitignore" 2>/dev/null || true
}

git_crypt_remove_gitattributes() {
    cd "$GIT_REPO_DIR"
    
    if [ -f ".gitattributes" ]; then
        # Remove git-crypt lines
        sed -i '/^\.secrets filter=git-crypt/d' ".gitattributes" 2>/dev/null || true
        sed -i '/^# Encrypted files with git-crypt/d' ".gitattributes" 2>/dev/null || true
        sed -i '/^\*\.secrets filter=git-crypt/d' ".gitattributes" 2>/dev/null || true
        
        # Remove empty lines at end
        sed -i -e :a -e '/^\n*$/{$d;N;ba' -e '}' ".gitattributes" 2>/dev/null || true
        
        # Remove file if empty
        if [ ! -s ".gitattributes" ]; then
            rm -f ".gitattributes"
            echo "✅ Removed .gitattributes (was empty)"
        else
            echo "✅ Removed git-crypt entries from .gitattributes"
        fi
    fi
}

# Git-crypt wizard
git_crypt_wizard() {
    local ACTION="${1:-}"  # Accept action as parameter (Setup or Remove)
    local result_file=$(mktemp)
    local action_result
    local exit_code
    
    # Step 1: Choose action (Setup or Remove) - only if not provided as parameter
    if [ -z "$ACTION" ]; then
        iwizard_run_inline '[
          {
            "type": "select",
            "title": "Git-Crypt Secrets Management",
            "description": "Choose an action",
            "key": "action",
            "options": ["Setup", "Remove"]
          }
        ]' "$result_file"
        exit_code=$?
        
        if [ $exit_code -ne 0 ] || [ ! -f "$result_file" ]; then
            rm -f "$result_file"
            echo "❌ Wizard cancelled" >&2
            return 1
        fi
        
        action_result=$(cat "$result_file")
        rm -f "$result_file"
        
        ACTION=$(parse_json_result "$action_result" "action")
        
        if [ -z "$ACTION" ]; then
            echo "❌ No action selected" >&2
            return 1
        fi
        
        echo ""
    fi
    
    # Step 2: Show multiselect with all options for the selected action
    local options_json
    local title
    local description
    
    if [ "$ACTION" = "Setup" ]; then
        title="Select setup options"
        description="Choose what to configure (A to select all/none)"
        options_json='[
          "Initialize git-crypt",
          "Setup .gitattributes",
          "Update .gitignore",
          "Copy secrets file to repo"
        ]'
    else
        title="Select removal options"
        description="Choose what to remove (A to select all/none)"
        options_json='[
          "Remove git-crypt initialization",
          "Restore .gitignore",
          "Remove .gitattributes entries"
        ]'
    fi
    
    # Build the multiselect JSON
    local multiselect_json=$(cat <<EOF
[
  {
    "type": "multiselect",
    "title": "$title",
    "description": "$description",
    "key": "options",
    "options": $options_json
  }
]
EOF
)
    
    result_file=$(mktemp)
    iwizard_run_inline "$multiselect_json" "$result_file"
    exit_code=$?
    
    if [ $exit_code -ne 0 ] || [ ! -f "$result_file" ]; then
        rm -f "$result_file"
        echo "❌ Wizard cancelled" >&2
        return 1
    fi
    
    local options_result=$(cat "$result_file")
    rm -f "$result_file"
    
    local SELECTED=$(parse_multiselect_result "$options_result" "options")
    
    if [ -z "$SELECTED" ]; then
        echo "❌ No options selected" >&2
        return 1
    fi
    
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    if [ "$ACTION" = "Setup" ]; then
        echo "🔐 Setting up git-crypt..."
    else
        echo "🗑️  Removing git-crypt..."
    fi
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    
    # Check prerequisites for setup
    if [ "$ACTION" = "Setup" ]; then
        if ! command -v git-crypt &> /dev/null; then
            echo "❌ git-crypt is not installed"
            echo "   Install with: sudo apt install git-crypt"
            return 1
        fi
        
        cd "$GIT_REPO_DIR"
        if [ ! -d ".git" ]; then
            echo "❌ Not a git repository. Run 'git init' first."
            return 1
        fi
    else
        cd "$GIT_REPO_DIR"
    fi
    
    # Execute selected options
    if [ "$ACTION" = "Setup" ]; then
        if echo "$SELECTED" | grep -q "Initialize git-crypt"; then
            git_crypt_init
        fi
        
        if echo "$SELECTED" | grep -q "Setup .gitattributes"; then
            git_crypt_setup_gitattributes
        fi
        
        if echo "$SELECTED" | grep -q "Update .gitignore"; then
            git_crypt_update_gitignore
        fi
        
        if echo "$SELECTED" | grep -q "Copy secrets file to repo"; then
            git_crypt_copy_secrets
        fi
        
        echo ""
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo "✅ Setup complete!"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo ""
        echo "📝 Next steps:"
        echo "   1. Edit .secrets file if needed:"
        echo "      nano $GIT_REPO_DIR/.secrets"
        echo ""
        echo "   2. Add and commit:"
        echo "      cd $GIT_REPO_DIR"
        echo "      git add .gitattributes .secrets"
        echo "      git commit -m 'Add encrypted .secrets with git-crypt'"
        echo ""
        echo "   3. Push to GitHub:"
        echo "      git push origin main"
        echo ""
    else
        if echo "$SELECTED" | grep -q "Remove git-crypt initialization"; then
            git_crypt_remove_init
        fi
        
        if echo "$SELECTED" | grep -q "Restore .gitignore"; then
            git_crypt_restore_gitignore
        fi
        
        if echo "$SELECTED" | grep -q "Remove .gitattributes entries"; then
            git_crypt_remove_gitattributes
        fi
        
        echo ""
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo "✅ Removal complete!"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    fi
}

# Main wizard - first asks Setup/Remove, then shows multiselect with all options
main_wizard() {
    local result_file=$(mktemp)
    local action_result
    local exit_code
    
    # Step 1: Choose action (Setup or Remove)
    iwizard_run_inline '[
      {
        "type": "select",
        "title": "GitHub Bootstrap",
        "description": "Choose an action",
        "key": "action",
        "options": ["Setup", "Remove"]
      }
    ]' "$result_file"
    exit_code=$?
    
    if [ $exit_code -ne 0 ] || [ ! -f "$result_file" ]; then
        rm -f "$result_file"
        echo "❌ Wizard cancelled" >&2
        return 1
    fi
    
    action_result=$(cat "$result_file")
    rm -f "$result_file"
    
    local ACTION=$(parse_json_result "$action_result" "action")
    
    if [ -z "$ACTION" ]; then
        echo "❌ No action selected" >&2
        return 1
    fi
    
    echo ""
    
    # Step 2: Show multiselect with all options for the selected action
    local options_json
    local title
    local description
    
    if [ "$ACTION" = "Setup" ]; then
        title="Select setup options"
        description="Choose what to configure (A to select all/none)"
        options_json='[
          "Generate SSH key",
          "Configure SSH config",
          "Setup local git repository",
          "Setup git remote",
          "Setup git-crypt"
        ]'
    else
        title="Select removal options"
        description="Choose what to remove (A to select all/none)"
        options_json='[
          "Remove SSH key",
          "Remove SSH config",
          "Remove local git repository",
          "Remove git remote",
          "Remove git-crypt"
        ]'
    fi
    
    # Build the multiselect JSON
    local multiselect_json=$(cat <<EOF
[
  {
    "type": "multiselect",
    "title": "$title",
    "description": "$description",
    "key": "options",
    "options": $options_json
  }
]
EOF
)
    
    result_file=$(mktemp)
    iwizard_run_inline "$multiselect_json" "$result_file"
    exit_code=$?
    
    if [ $exit_code -ne 0 ] || [ ! -f "$result_file" ]; then
        rm -f "$result_file"
        echo "❌ Wizard cancelled" >&2
        return 1
    fi
    
    local options_result=$(cat "$result_file")
    rm -f "$result_file"
    
    local SELECTED=$(parse_multiselect_result "$options_result" "options")
    
    if [ -z "$SELECTED" ]; then
        echo "❌ No options selected" >&2
        return 1
    fi
    
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    if [ "$ACTION" = "Setup" ]; then
        echo "🔑 Setting up GitHub SSH..."
    else
        echo "🗑️  Removing GitHub SSH components..."
    fi
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    
    # Execute selected options
    if [ "$ACTION" = "Setup" ]; then
        if echo "$SELECTED" | grep -q "Generate SSH key"; then
            setup_ssh_key
            echo ""
        fi
        
        if echo "$SELECTED" | grep -q "Configure SSH config"; then
            setup_ssh_config
            echo ""
        fi
        
        if echo "$SELECTED" | grep -q "Setup local git repository"; then
            setup_local_repo
            echo ""
        fi
        
        if echo "$SELECTED" | grep -q "Setup git remote"; then
            # Ensure we're in the repo directory
            if [ -d "$GIT_REPO_DIR/.git" ]; then
                cd "$GIT_REPO_DIR"
                # Fix non-standard remote name (Cursor expects 'origin')
                if git remote 2>/dev/null | grep -q "^main$" && ! git remote 2>/dev/null | grep -q "^origin$"; then
                    echo "⚠️  Remote named 'main' detected (non-standard)"
                    echo "🔧 Renaming remote 'main' → 'origin' (for Cursor compatibility)"
                    git remote rename main origin
                fi
            fi
            setup_remote
            echo ""
        fi
        
        # Git-crypt setup
        if echo "$SELECTED" | grep -q "Setup git-crypt"; then
            setup_git_crypt
            echo ""
        fi
        
        # Configure Cursor git.path if workspace file exists
        WORKSPACE_FILE="$HOME/.vscode/RPi-Full.code-workspace"
        if [ -f "$WORKSPACE_FILE" ]; then
            echo "⚙️  Configuring Cursor git path..."
            if ! grep -q '"git.path"' "$WORKSPACE_FILE"; then
                # Add git.path to workspace settings
                sed -i '/"git.enabled":/i\    "git.path": "/usr/bin/git",' "$WORKSPACE_FILE"
                echo "✅ Added git.path to workspace settings"
            else
                echo "ℹ️  git.path already configured in workspace"
            fi
            echo ""
        fi
        
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo "✅ Setup complete!"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo ""
        
        # Show next steps if SSH key was generated
        if echo "$SELECTED" | grep -q "Generate SSH key"; then
            echo "📝 Next steps:"
            echo "   1. Go to: https://github.com/settings/keys"
            echo "   2. Click 'New SSH key'"
            echo "   3. Title: 'Pi (github_pi)'"
            echo "   4. Key type: 'Authentication Key'"
            echo "   5. Paste the public key shown above"
            echo "   6. Click 'Add SSH key'"
            echo ""
            echo "🧪 Test your connection:"
            echo "   ssh -T git@github.com"
            echo ""
        fi
        
        if echo "$SELECTED" | grep -q "Setup git remote"; then
            echo "🚀 Push to GitHub:"
            echo "   git push origin main"
            echo ""
        fi
        
        
        echo "⚠️  Cursor Setup:"
        echo "   - Restart Cursor after running this script"
        echo "   - Sign in to GitHub: Ctrl+Shift+P → 'GitHub: Sign In'"
        echo "   - This enables Background Agents and git integration"
        echo ""
        
        return 0
    else
        if echo "$SELECTED" | grep -q "Remove SSH key"; then
            remove_ssh_key
            echo ""
        fi
        
        if echo "$SELECTED" | grep -q "Remove SSH config"; then
            remove_ssh_config
            echo ""
        fi
        
        if echo "$SELECTED" | grep -q "Remove local git repository"; then
            remove_local_repo
            echo ""
        fi
        
        if echo "$SELECTED" | grep -q "Remove git remote"; then
            remove_remote
            echo ""
        fi
        
        # Git-crypt removal
        if echo "$SELECTED" | grep -q "Remove git-crypt"; then
            remove_git_crypt
            echo ""
        fi
        
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo "✅ Removal complete!"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        
        return 0
    fi
}

# Handle subcommands
# Default to wizard if no argument or empty argument
if [ -z "${1}" ]; then
    set -- wizard
fi

case "$1" in
    setup)
        # Full setup - run all setup functions
        echo "🔑 Bootstrapping GitHub SSH authentication..."
        echo ""
        setup_ssh_key
        setup_ssh_config
        setup_local_repo
        echo ""
        # Setup remote if repo exists
        if [ -d "$GIT_REPO_DIR/.git" ]; then
            cd "$GIT_REPO_DIR"
            # Fix non-standard remote name (Cursor expects 'origin')
            if git remote 2>/dev/null | grep -q "^main$" && ! git remote 2>/dev/null | grep -q "^origin$"; then
                echo "⚠️  Remote named 'main' detected (non-standard)"
                echo "🔧 Renaming remote 'main' → 'origin' (for Cursor compatibility)"
                git remote rename main origin
            fi
            setup_remote
        else
            echo "📦 No git repository found. Setting up remote..."
            setup_remote
        fi
        echo ""
        # Configure Cursor git.path if workspace file exists
        WORKSPACE_FILE="$HOME/.vscode/RPi-Full.code-workspace"
        if [ -f "$WORKSPACE_FILE" ]; then
            echo "⚙️  Configuring Cursor git path..."
            if ! grep -q '"git.path"' "$WORKSPACE_FILE"; then
                sed -i '/"git.enabled":/i\    "git.path": "/usr/bin/git",' "$WORKSPACE_FILE"
                echo "✅ Added git.path to workspace settings"
            else
                echo "ℹ️  git.path already configured in workspace"
            fi
            echo ""
        fi
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo "✅ GitHub SSH setup complete!"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo ""
        echo "📝 Next steps:"
        echo "   1. Go to: https://github.com/settings/keys"
        echo "   2. Click 'New SSH key'"
        echo "   3. Title: 'Pi (github_pi)'"
        echo "   4. Key type: 'Authentication Key'"
        echo "   5. Paste the public key shown above"
        echo "   6. Click 'Add SSH key'"
        echo ""
        echo "🧪 Test your connection:"
        echo "   ssh -T git@github.com"
        echo ""
        echo "🚀 Push to GitHub:"
        echo "   git push origin main"
        echo ""
        echo "⚠️  Cursor Setup:"
        echo "   - Restart Cursor after running this script"
        echo "   - Sign in to GitHub: Ctrl+Shift+P → 'GitHub: Sign In'"
        echo "   - This enables Background Agents and git integration"
        echo ""
        exit 0
        ;;
    status)
        show_status
        exit 0
        ;;
    secrets)
        git_crypt_wizard
        exit 0
        ;;
    remove-key|delete-key)
        if [ -f "$SSH_KEY_PATH" ] || [ -f "$SSH_KEY_PATH.pub" ]; then
            echo "⚠️  WARNING: This will remove the SSH key at $SSH_KEY_PATH"
            read -p "Are you sure? (y/N): " -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                rm -f "$SSH_KEY_PATH" "$SSH_KEY_PATH.pub"
                echo "✅ SSH key removed"
                echo "   Note: You may want to remove it from GitHub: https://github.com/settings/keys"
            else
                echo "❌ Cancelled"
            fi
        else
            echo "ℹ️  No SSH key found at $SSH_KEY_PATH"
        fi
        exit 0
        ;;
    remove-remote|detach-remote)
        if [ -d "$GIT_REPO_DIR/.git" ]; then
            cd "$GIT_REPO_DIR"
            REMOTES=$(git remote)
            if [ -n "$REMOTES" ]; then
                echo "📋 Current remotes:"
                git remote -v
                echo ""
                read -p "Remove all remotes? (y/N): " -n 1 -r
                echo
                if [[ $REPLY =~ ^[Yy]$ ]]; then
                    for REMOTE in $REMOTES; do
                        git remote remove "$REMOTE"
                        echo "✅ Removed remote: $REMOTE"
                    done
                    echo "✅ All remotes removed"
                else
                    echo "❌ Cancelled"
                fi
            else
                echo "ℹ️  No remotes configured"
            fi
        else
            echo "ℹ️  No git repository found at $GIT_REPO_DIR"
        fi
        exit 0
        ;;
    remove-repo|delete-repo)
        if [ -d "$GIT_REPO_DIR/.git" ]; then
            echo "⚠️  WARNING: This will remove the git repository at $GIT_REPO_DIR"
            echo "   This will NOT delete your files, only the .git directory"
            read -p "Are you sure? (y/N): " -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                rm -rf "$GIT_REPO_DIR/.git"
                echo "✅ Git repository removed from $GIT_REPO_DIR"
                echo "   Your files are still intact"
            else
                echo "❌ Cancelled"
            fi
        else
            echo "ℹ️  No git repository found at $GIT_REPO_DIR"
        fi
        exit 0
        ;;
    wizard)
        main_wizard
        WIZARD_EXIT=$?
        # Wizard now handles everything internally, just exit with its status
        exit $WIZARD_EXIT
        ;;
    help|--help|-h)
        show_help
        exit 0
        ;;
    *)
        echo "❌ Unknown command: $1"
        echo ""
        show_help
        exit 1
        ;;
esac




