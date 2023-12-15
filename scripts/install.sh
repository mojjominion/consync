#!/usr/bin/sh
#

GITHUB_PKG_URL=https://api.github.com/repos/mojjominion/consync/releases/latest
FORMAT="tar.gz"
BIN_NAME=consync

create_uninstall_binary() {
	cat >uninstall.sh <<EOF
#!/usr/bin/sh
echo "${BIN_NAME} removed from ~/bin/"
rm ~/bin/${BIN_NAME}
rm ~/bin/${BIN_NAME}_uninstall

systemctl --user disable consync.service
systemctl --user stop consync.service
systemctl --user daemon-reload
EOF
}

clean() {
	rm -rf -- $BIN_NAME
}
create_systemd_service() {
	echo "Creating systemd service..."

	# Specify the user and service details
	SERVICE_NAME=$BIN_NAME
	SERVICE_EXECUTABLE="$HOME/bin/$SERVICE_NAME"
	SERVICE_DESCRIPTION="dot_configs watcher"

	# Create the service unit file
	echo "[Unit]
    Description=${SERVICE_DESCRIPTION}

    [Service]
    ExecStart=${SERVICE_EXECUTABLE}
    Restart=on-failure
    RestartSec=3

    [Install]
    WantedBy=default.target
    " >~/.config/systemd/user/${SERVICE_NAME}.service

	# Enable and start the service
	systemctl --user enable "${SERVICE_NAME}"
	systemctl --user start "${SERVICE_NAME}"

	echo "Done..."
}

create_launchd_service() {
	echo "Creating launchd service..."

	# Specify the user and service details
	SERVICE_NAME=$BIN_NAME
	SERVICE_EXECUTABLE="$HOME/bin/$SERVICE_NAME"
	SERVICE_DESCRIPTION="dot_configs watcher"

	# Create the service plist file
	SERVICE_PLIST="~/Library/LaunchAgents/${SERVICE_NAME}.plist"
	cat >"$SERVICE_PLIST" <<EOF
    <?xml version="1.0" encoding="UTF-8"?>
    <plist version="1.0">
        <dict>
            <key>Label</key>
            <string>${SERVICE_NAME}</string>
            <key>ProgramArguments</key>
            <array>
                <string>${SERVICE_EXECUTABLE}</string>
            </array>
            <key>RunAtLoad</key>
            <true/>
            <key>KeepAlive</key>
            <true/>
            <key>StandardOutPath</key>
            <string>/tmp/${SERVICE_NAME}.out</string>
            <key>StandardErrorPath</key>
            <string>/tmp/${SERVICE_NAME}.err</string>
        </dict>
    </plist>
EOF

	# Load and start the service
	launchctl load -w "$SERVICE_PLIST"
	launchctl start "$SERVICE_NAME"

	echo "Done..."
}

# Function to unpack the downloaded tar.gz file
unpack() {
	FILE=$(echo *.tar.gz)
	NAME=${FILE%.tar.gz}
	echo "Unpacking $NAME.$FORMAT ..."

	# clean pre-existing dir
	clean

	tar -xvzf $NAME.$FORMAT && mv $NAME $BIN_NAME
	rm $NAME.$FORMAT
}

# Function to install the binary
install_binary() {

	EXECUTABLE="${1:-$PWD/$BIN_NAME}"/$BIN_NAME
	UNINSTALL_EXECUTABLE=$PWD/uninstall.sh

	ls consync/

	echo "Installing '${BIN_NAME}'..."

	install -d ~/bin/
	mkdir -p ~/.config/$BIN_NAME
	install -- $EXECUTABLE ~/bin/$BIN_NAME
	install -- $UNINSTALL_EXECUTABLE ~/bin/${BIN_NAME}_uninstall

	# remove uninstall script
	rm $UNINSTALL_EXECUTABLE

	printf "Done... "
	printf "'${BIN_NAME}' installed in ~/bin/ \n\n"

}

# Function to download and install the binary for Linux
download_install() {
	# Download the tar.gz file
	curl -s $GITHUB_PKG_URL --progress-bar |
		grep "browser_download_url.*$1*" |
		cut -d : -f 2,3 |
		tr -d \" |
		wget -i -
}

# Function to download and install the binary
download_and_install_binary() {
	# Determine the appropriate download command based on the operating system
	create_service="create_systemd_service"
	if [[ "$(uname)" == "Linux" ]]; then
		pkg="linux"
		create_service="create_systemd_service"
	elif [[ "$(uname)" == "Darwin" ]]; then
		pkg="apple"
		create_service="create_launchd_service"
	else
		echo "Unsupported operating system"
		exit 1
	fi

	# Download the binary
	download_install $pkg
	# Unpack and install the binary
	unpack
	create_uninstall_binary
	install_binary
	$create_service
	clean
}

# Run the script
download_and_install_binary
