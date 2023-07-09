#!/usr/bin/sh

BIN_NAME=consync

echo "'${BIN_NAME}' removed from ~/bin/"

rm ~/bin/$BIN_NAME
rm ~/bin/${BIN_NAME}_uninstall

systemctl --user disable consync.service
systemctl --user stop consync.service
systemctl --user daemon-reload
