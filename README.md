# consync

## Description

Simple tool to auto add configuration files (`.conf, .config, .conf`) to the your managed repository on trigger of following events

```rust

    /// An event describing creation operations on files.
    ///
    /// This event is about the creation of files, folders, or other structures but not about e.g.
    /// writing new content into them.
  - Create(CreateKind),

    /// An event describing mutation of content, name, or metadata.
    ///
    /// This event is about the mutation of files', folders', or other structures' content, name
    /// (path), or associated metadata (attributes).
  - Modify(ModifyKind),

    /// An event describing removal operations on files.
    ///
    /// This event is about the removal of files, folders, or other structures but not e.g. erasing
    /// content from them. This may also be triggered for renames/moves that move files _out of the
    /// watched subpath_.
    ///
    /// Some editors also trigger Remove events when saving files as they may opt for removing (or
    /// renaming) the original then creating a new file in-place.
  - Remove(RemoveKind),
```
[source](https://docs.rs/notify/latest/notify/event/enum.EventKind.html)

## Usage

### Installing

You can run this bash script to install the tool


```sh
curl -L https://raw.githubusercontent.com/mojjominion/consync/master/scripts/install.sh | bash
```

> **_Note:_**   this script will create a systemd service for the current user which will run in the background


This script will do following three things:

1. download and install the binary from github release
2. create `systemd` service to keep the tool running in the background
3. and finally install `consync_uninstall` to uninstall the tool

To check that sevice is running you can use this command 

```sh
ps -eo 'tty,pid,comm' | grep "consync"
```

For more information refer to the script [here](./scripts/install.sh)



### Uninstalling

To undo whatever the installation script did in the system you can run this bash script


```sh
consync_uninstall
```

 > This script will remove the systemd service created for the current user.



## Support

At the moment this tool only supports [chezmoi](https://github.com/twpayne/chezmoi) config manager. In future it might have some configuration to run custom scripts on those events

