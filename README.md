# ZEX

## What is ZEX?

It's a HEX viewer, soon editor, maybe one day it will be a disassembler also.

## RoadMap

 - [X] Simple interface
 - [X] Hex viewer
 - [X] ASCII viewer
 - [ ] Command Line
 - [X] Hex Editor
 - [ ] ? Disassembler ?

## Deployment

**NOTE**</br>
YOU SHOULD HAVE RUST AND CARGO TO INSTALL THIS HEX VIEWER

### Rust Installation

Try run: `$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

And see official [guide](https://www.rust-lang.org/tools/install).

For `cargo` try this:
 - Ubuntu / Debian `$ sudo apt-get install cargo`
 - Arch `$ sudo pacman -S cargo`

Or see official [guide](https://github.com/rust-lang/cargo)

### Installation

1. Download or clone this repo:
	- Git: `$ git clone https://github.com/MrZloHex/zex.git`
2. Change working directory to *lscc*:
	- `$ cd zex`
3. Run *installation* script:
	- `$ ./deployment.sh -i`
	- **NOTE** You need to have **sudo** access.

### Uninstallation

1. Change working directory to *lscc*:
	- `$ cd zex`
2. Run *uninstallation* script:
	- `$ ./deployment.sh -u`
3. Go out from directory:
	- `$ cd ..`

## Usage

ZEX have to modes to interact with file and hexadecimal view:
 - Normal Mode
 - Command Mode

### Normal Mode

__Normal Mode__ is for viewing hex dump and ASCII tranclation of bytes.</br>
You can move arcoss dump with _Arrows_ or _Vim-like_ buttons:


&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;🡑&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;k</br>
🡐&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;🡒&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;h&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;l</br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;🡓&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;j</br>

To return in __Normal Mode__ from another just push `esc`.

### Command Mode

__Command Mode__ is for executing commands which you are writing.</br>
To enter in __Command Mode__ press `:`.

Below is a list of available commands.

### Commands

| Mnemonic | Description   |
|:--------:|:-------------:|
| :q	   | Quit from zex |

