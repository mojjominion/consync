#!/usr/bin/sh
#
#
chmod +x ./target/debug/consync
sudo mv ./target/debug/consync /usr/local/bin/

echo "'consync' installed in /usr/local/bin/"
echo "Creating systemd service..."
bash ./lib/create-systemd-service.sh
