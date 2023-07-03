#!/usr/bin/sh
#
rm /usr/local/bin/consync
echo "'consync' removed from /usr/local/bin/"

systemctl --user disable consync.service
systemctl --user stop consync.service
systemctl --user daemon-reload
