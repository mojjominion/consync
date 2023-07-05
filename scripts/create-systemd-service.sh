#!/bin/bash
# Specify the user and service details
SERVICE_NAME="consync"
SERVICE_EXECUTABLE="$HOME/bin/consync"
SERVICE_DESCRIPTION="dot_configs watcher"

# Create the service unit file
echo "[Unit]
Description=${SERVICE_DESCRIPTION}

[Service]
Type=oneshot
RemainAfterExit=yes
ExecStart=${SERVICE_EXECUTABLE}
RestartSec=3

[Install]
WantedBy=default.target
" >~/.config/systemd/user/${SERVICE_NAME}.service

# Enable and start the service
systemctl --user enable "${SERVICE_NAME}"
systemctl --user start "${SERVICE_NAME}"
