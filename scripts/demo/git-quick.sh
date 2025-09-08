#!/bin/bash
# Demo script: Quick Git Operations

echo "=== Git Quick Actions ==="
echo ""

if git rev-parse --git-dir > /dev/null 2>&1; then
    echo "Current repository: $(basename `git rev-parse --show-toplevel`)"
    echo "Current branch: $(git branch --show-current)"
    echo ""
    echo "Choose an action:"
    echo "1) Show status"
    echo "2) Show recent commits"
    echo "3) Show branches"
    echo "4) Fetch latest"
    echo "5) Return to screensaver"
    echo ""
    read -p "Enter choice (1-5): " choice
    
    case $choice in
        1) git status ;;
        2) git log --oneline -10 ;;
        3) git branch -a ;;
        4) git fetch --all --prune && echo "Fetch completed!" ;;
        5) echo "Returning..." ;;
        *) echo "Invalid choice" ;;
    esac
else
    echo "Not in a git repository!"
fi

echo ""
echo "Press any key to return..."
read -n 1
