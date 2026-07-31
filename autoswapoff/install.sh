#!/bin/bash

SERVICE_NAME="autoswapoff"
BINARY_PATH="/usr/local/bin/$SERVICE_NAME"

if [ "$EUID" -eq 0 ]; then
    echo "This script must not be run as sudo, please re-run as \"./install\""
    exit 1
fi

cd "$(dirname "$0")"
set -e

sudo systemctl stop "$SERVICE_NAME" || true

echo "Building executable..."
cargo clippy --release
cargo build --release

echo "Copying files..."
sudo cp -i "target/release/$SERVICE_NAME" "$BINARY_PATH"
sudo cp -i "example_systemd.service" "/etc/systemd/system/$SERVICE_NAME.service"

echo "Updating systemctl..."
sudo systemctl daemon-reload
sudo systemctl enable "$SERVICE_NAME"
sudo systemctl restart "$SERVICE_NAME"

echo "Done, checking status:"
sudo systemctl status "$SERVICE_NAME" --no-pager
