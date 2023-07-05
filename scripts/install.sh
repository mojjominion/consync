#!/usr/bin/sh

trap 'exit' INT TERM

BIN_NAME=consync
EXECUTABLE=$PWD/target/release/$BIN_NAME
UNINSTALL_EXECUTABLE=$PWD/scripts/uninstall.sh

echo "Installing '${BIN_NAME}'..."

install -d ~/bin/
install -- $EXECUTABLE ~/bin/$BIN_NAME
install -- $UNINSTALL_EXECUTABLE ~/bin/${BIN_NAME}_uninstall

printf "Done... "
printf "'${BIN_NAME}' installed in ~/bin/ \n\n"

echo "Creating systemd service..."
bash $PWD/scripts/create-systemd-service.sh
echo "Done..."
