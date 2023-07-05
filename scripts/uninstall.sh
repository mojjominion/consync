#!/usr/bin/sh
rm ~/bin/consync
echo "'consync' removed from ~/bin/"

systemctl --user disable consync.service
systemctl --user stop consync.service
systemctl --user daemon-reload
