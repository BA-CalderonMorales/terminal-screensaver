# Path configuration
PATH=$PATH:/c/Go/bin

# NVM configuration
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"

# Bun configuration
export BUN_INSTALL="$HOME/.bun"
export PATH="$BUN_INSTALL/bin:$PATH"

# Simple Git branch detection
parse_git_branch() {
    git rev-parse --git-dir > /dev/null 2>&1 && git branch 2> /dev/null | sed -e '/^[^*]/d' -e 's/* \(.*\)/ [\1]/'
}

# Simplified color prompt with Git integration - Professional version
PS1='\[\033[01;32m\][me]\[\033[00m\]:\[\033[01;34m\]\w\[\033[33m\]$(parse_git_branch)\[\033[00m\]\$ '

# Environment settings with smart history
export EDITOR='code --wait'
export VISUAL='code --wait'
export HISTSIZE=50000
export HISTFILESIZE=100000
export HISTCONTROL=ignoreboth:erasedups
export HISTIGNORE="ls:ll:la:cd:pwd:exit:date:* --help:history:clear"
export HISTTIMEFORMAT="%F %T "

# Smart history search with arrow keys
bind '"\e[A": history-search-backward'
bind '"\e[B": history-search-forward'

# Better tab completion
bind "set completion-ignore-case on"
bind "set completion-map-case on"
bind "set show-all-if-ambiguous on"
bind "set mark-symlinked-directories on"

# Help function to show all aliases and their descriptions
function aliases() {
    echo "🚀 Available Aliases and Functions:"
    echo ""
    echo "📁 Directory Navigation:"
    echo "  ..     -> cd .."
    echo "  ...    -> cd ../.."
    echo "  ....   -> cd ../../.."
    echo "  .....  -> cd ../../../.."
    echo "  ~      -> cd ~"
    echo "  -      -> cd to last directory"
    echo ""
    echo "📋 Listing Commands:"
    echo "  l      -> ls -CF"
    echo "  ll     -> ls -lah (detailed list)"
    echo "  la     -> ls -A (show hidden)"
    echo ""
    echo "🗂️  Directory Management:"
    echo "  mkcd   -> Create and enter directory"
    echo "  mark   -> Bookmark current directory"
    echo "  marks  -> Show bookmarked directories"
    echo "  jump   -> Jump to bookmarked directory"
    echo ""
    echo "🔍 Search Tools:"
    echo "  f      -> Find files (usage: f pattern)"
    echo "  r      -> Recursive grep (usage: r pattern)"
    echo ""
    echo "🌿 Git Workflow (Super Fast):"
    echo "  g      -> git (shorthand)"
    echo "  gs     -> git status (short format)"
    echo "  gl     -> git log (graph, last 10)"
    echo "  gla    -> git log --all (last 20)"
    echo "  gc     -> git commit"
    echo "  qc     -> Quick commit with timestamp"
    echo "  gco    -> git checkout"
    echo "  gcb    -> git checkout -b (new branch)"
    echo "  newb   -> Create new branch from main/master"
    echo "  switchb-> Interactive branch switcher (needs fzf)"
    echo "  delb   -> Safe branch deletion with confirmation"
    echo "  gb     -> git branch"
    echo "  current-> Show current branch name"
    echo "  recent -> Show recent branches"
    echo "  cleanup-> Delete merged branches"
    echo "  gf     -> git fetch --all --prune"
    echo "  gpu    -> git push -u origin current-branch"
    echo ""
    echo "⚙️  Development Workflow:"
    echo "  serve  -> Start Python HTTP server"
    echo "  ports  -> Show open ports"
    echo "  myip   -> Show public IP"
    echo "  pyenv  -> Activate Python virtual environment"
    echo "  nodeenv-> Use node version from .nvmrc"
    echo "  reload -> Reload this bashrc file"
    echo "  editbash-> Edit bashrc in VS Code"
    echo ""
    echo "🏗️  Project Management:"
    echo "  initproj-> Create new project (basic, node, python, rust, go)"
    echo ""
    echo "⏱️  Utility Functions:"
    echo "  timer  -> Time a command execution"
    echo "  extract-> Extract any archive file"
    echo ""
    echo "🗂️  Directory Shortcuts:"
    echo "  dev    -> cd /c/dev-environment"
    echo "  proj   -> cd /c/Projects"
    echo "  repos  -> cd ~/usr_repos"
    echo "  desk   -> cd ~/Desktop"
    echo "  down   -> cd ~/Downloads"
    echo ""
    echo "💡 Pro Tip: Most Git commands work with partial matches!"
}

alias help='aliases'

# Better directory navigation
alias ..='cd ..'
alias ...='cd ../..'
alias ....='cd ../../..'
alias .....='cd ../../../..'
alias ~='cd ~'
alias -- -='cd -'

# Enhanced listing
alias ls='ls -F --color=auto'
alias ll='ls -lah'
alias la='ls -A'
alias l='ls -CF'
alias dir='dir --color=auto'

# Powerful Git aliases for rapid workflow
alias g='git'
alias gs='git status -sb'
alias gl='git log --oneline --graph --decorate -10'
alias gla='git log --oneline --graph --decorate --all -20'
alias gp='git pull'
alias gpr='git pull --rebase'
alias gpp='git pull && git push'
alias gpu='git push -u origin $(git branch --show-current)'
alias gc='git commit'
alias gca='git commit -a'
alias gcm='git commit -m'
alias gco='git checkout'
alias gcb='git checkout -b'
alias gb='git branch'
alias gbd='git branch -d'
alias gbD='git branch -D'
alias gba='git branch -a'
alias gd='git diff'
alias gdc='git diff --cached'
alias ga='git add'
alias gaa='git add -A'
alias grh='git reset --hard'
alias grs='git reset --soft'
alias gst='git stash'
alias gstp='git stash pop'
alias gstl='git stash list'
alias gf='git fetch --all --prune'

# Branch management shortcuts
alias current='git branch --show-current'
alias recent='git for-each-ref --sort=-committerdate refs/heads/ --format="%(refname:short) %(committerdate:relative)"'
alias cleanup='git branch --merged | grep -v "\*\|main\|master\|develop" | xargs -n 1 git branch -d'

# Directory shortcuts with quick access
alias dev='cd /c/dev-environment'
alias proj='cd /c/Projects'
alias repos='cd ~/usr_repos'
alias desk='cd ~/Desktop'
alias down='cd ~/Downloads'

# Quick environment switching
alias pyenv='source venv/bin/activate 2>/dev/null || source .venv/bin/activate 2>/dev/null || echo "No Python venv found"'
alias nodeenv='nvm use 2>/dev/null || echo "No .nvmrc found, using default node"'
alias reload='source ~/.bashrc && echo "✅ Bashrc reloaded!"'
alias editbash='code ~/.bashrc'

# Project workflow helpers
alias serve='python -m http.server'
alias ports='netstat -tulanp'
alias myip='curl http://ipecho.net/plain; echo'
alias weather='curl wttr.in'
alias speed='curl -s https://raw.githubusercontent.com/sivel/speedtest-cli/master/speedtest.py | python -'

# Process management shortcuts
alias psg='ps aux | grep'
alias killport='function _killport(){ lsof -ti:$1 | xargs kill -9; }; _killport'
alias listening='lsof -i -P -n | grep LISTEN'

# System monitoring
alias meminfo='free -h'
alias diskinfo='df -h'
alias cpuinfo='lscpu'
alias sysload='uptime'

# Quick file operations
alias cp='cp -i'
alias mv='mv -i'
alias rm='rm -i'
alias mkdir='mkdir -pv'
alias path='echo -e ${PATH//:/\\n}'
alias now='date +"%T"'
alias nowdate='date +"%d-%m-%Y"'

# Quick text processing
alias count='wc -l'
alias urlencode='python3 -c "import sys, urllib.parse as ul; print(ul.quote_plus(sys.argv[1]))"'
alias urldecode='python3 -c "import sys, urllib.parse as ul; print(ul.unquote_plus(sys.argv[1]))"'

# Advanced search
function f() { find . -iname "*$1*" ${2:-.}; }
function r() { grep "$1" ${2:-.} -R; }

# Create and enter directory
function mkcd() { mkdir -p "$@" && cd "$_"; }

# Extract any archive
function extract() {
    if [ -f $1 ]; then
        case $1 in
            *.tar.bz2)   tar xjf $1     ;;
            *.tar.gz)    tar xzf $1     ;;
            *.bz2)       bunzip2 $1     ;;
            *.rar)       unrar e $1     ;;
            *.gz)        gunzip $1      ;;
            *.tar)       tar xf $1      ;;
            *.tbz2)      tar xjf $1     ;;
            *.tgz)       tar xzf $1      ;;
            *.zip)       unzip $1       ;;
            *.Z)         uncompress $1  ;;
            *.7z)        7z x $1        ;;
            *)          echo "'$1' cannot be extracted" ;;
        esac
    else
        echo "'$1' is not a valid file"
    fi
}

# Advanced productivity functions

# Quick commit with automated message
function qc() {
    if [ -z "$1" ]; then
        git add -A && git commit -m "Quick commit: $(date '+%Y-%m-%d %H:%M')"
    else
        git add -A && git commit -m "$*"
    fi
}

# Create new branch and switch to it with optional base branch
function newb() {
    local branch_name="$1"
    local base_branch="${2:-main}"
    
    if [ -z "$branch_name" ]; then
        echo "Usage: newb <branch-name> [base-branch]"
        return 1
    fi
    
    git checkout "$base_branch" && git pull && git checkout -b "$branch_name"
}

# Safe branch deletion with confirmation
function delb() {
    local branch="$1"
    if [ -z "$branch" ]; then
        echo "Usage: delb <branch-name>"
        return 1
    fi
    
    echo "Are you sure you want to delete branch '$branch'? (y/N)"
    read -r response
    if [[ "$response" =~ ^[Yy]$ ]]; then
        git branch -D "$branch" && echo "✅ Branch '$branch' deleted"
    else
        echo "❌ Branch deletion cancelled"
    fi
}

# Interactive branch switcher using fzf (if available)
function switchb() {
    if command -v fzf >/dev/null; then
        local branch
        branch=$(git branch | grep -v '^\*' | sed 's/^[[:space:]]*//' | fzf --height=20% --reverse --border)
        if [ -n "$branch" ]; then
            git checkout "$branch"
        fi
    else
        echo "Install fzf for interactive branch switching: https://github.com/junegunn/fzf"
        git branch
    fi
}

# Project initialization with templates
function initproj() {
    local name="$1"
    local type="${2:-basic}"
    
    if [ -z "$name" ]; then
        echo "Usage: initproj <project-name> [type]"
        echo "Types: basic, node, python, rust, go"
        return 1
    fi
    
    mkdir -p "$name" && cd "$name"
    git init
    
    case "$type" in
        node)
            npm init -y
            echo "node_modules/\n.env\n*.log" > .gitignore
            ;;
        python)
            echo -e "__pycache__/\n*.pyc\n.env\nvenv/\n.venv/\n*.egg-info/" > .gitignore
            touch requirements.txt
            ;;
        rust)
            cargo init --name "$name"
            echo "target/\nCargo.lock" > .gitignore
            ;;
        go)
            go mod init "$name"
            echo "*.exe\n*.log" > .gitignore
            ;;
        *)
            echo "*.log\n.env\n.DS_Store\nThumbs.db" > .gitignore
            ;;
    esac
    
    echo "# $name\n\nInitialized on $(date)" > README.md
    git add . && git commit -m "Initial commit: $type project setup"
    
    if command -v code >/dev/null; then
        code .
    fi
    
    echo "✅ Project '$name' ($type) initialized!"
}

# Performance-optimized timer for command execution
function timer() {
    [[ $# -eq 0 ]] && { echo "Usage: timer <command>"; return 1; }
    local start=$(date +%s)
    "$@"
    local end=$(date +%s)
    printf "Execution time: %d seconds\n" $((end - start))
}

# Intelligent directory bookmarking system
[[ ! -f ~/.dirs ]] && touch ~/.dirs

alias mark='pwd >> ~/.dirs'
alias marks='sort -u ~/.dirs'
alias jump='cd "$(sort -u ~/.dirs | fzf --height=40% --reverse --border --preview "ls -la {}")" 2>/dev/null || echo "fzf not available"'

# Advanced bookmark management
function unmark() {
    local current_dir=$(pwd)
    grep -v "^${current_dir}$" ~/.dirs > ~/.dirs.tmp && mv ~/.dirs.tmp ~/.dirs
    echo "Removed: $current_dir"
}

function clearmarks() {
    > ~/.dirs
    echo "All bookmarks cleared"
}

# Optimized environment initialization
[[ -f /etc/profile.d/git-sdk.sh ]] && source /etc/profile.d/git-sdk.sh 2>/dev/null

# Efficient NVM setup with lazy loading
if [[ -d "$HOME/.nvm" ]]; then
    export NVM_DIR="$HOME/.nvm"
    [[ -s "$NVM_DIR/nvm.sh" ]] && source "$NVM_DIR/nvm.sh" --no-use
    [[ -s "$NVM_DIR/bash_completion" ]] && source "$NVM_DIR/bash_completion"
fi

# Clean, professional terminal initialization
if [[ $- == *i* ]]; then
    printf "\nTerminal ready | %s | %s\n" "$(date '+%H:%M:%S')" "$(basename "$PWD")"
    printf "Type 'help' for available commands\n\n"
fi
