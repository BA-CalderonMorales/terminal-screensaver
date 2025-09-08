#!/bin/bash

echo "=== Terminal Screensaver Script Integration Demo ==="
echo ""
echo "This demo showcases how the terminal screensaver can:"
echo "- Execute any script or command"
echo "- Integrate with development workflows"
echo "- Provide extensible keyboard shortcuts"
echo "- Maintain professional appearance"
echo ""
echo "Configuration loaded from demo-config.toml:"
echo "- D: Development Environment Status"
echo "- G: Git Quick Actions"
echo "- ESC: Exit"
echo "- H: Toggle help display"
echo ""
echo "Starting screensaver..."
echo "Try pressing D or G to run scripts!"
sleep 2

# Run the screensaver with the enhanced config
cargo run --release -- -c demo-config.toml
