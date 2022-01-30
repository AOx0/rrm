# rwm

Inspired by Spoons [rmm][1]. This is a cross-platform Mod Manager for RimWorld intended to work with macOS, linux and Windows

Up to now, you must have `steamcmd` installed and available within your `PATH`.



# Current features demo:

![Example][image-1]



## Installation
Make sure you have `steamcmd` installed and available within your `PATH`. Check Section [`Install steamcmd`][2]

This program is installable with `cargo`. Install `rust` along with `cargo` [here][3].

To install run:

	cargo install --git https://github.com/AOx0/rwm

<br/> <br/>
## Configuration
To set configuration values like game installation path and whether `rwm` should use `more` to display its output or not, you can use the `set` subcommand. The configuration file is available in `$USER_HOME/.rwm/config` on macOS, Linux, and Windows with a JSON format. 

For example, to set the path were RimWorld is installed, you can use:

	rwm set game-path /Applications/RimWorld.app

Or with its alias:

	rwm set path /Applications/RimWorld.app

Help message:

	rwm-set
	Set new configuration values
	
	USAGE:
	    rwm set <OPTION> <VALUE>
	
	OPTIONS:
	    game-path    Set the path where RimWorld is installed [alias: 'path']
	    pager        Set the paging software to use, like bat, more or less [alias: 'paging']
	    use-pager    Set if rwm should use more to display output [values: false, true, 0, 1] [alias: 'use-paging']


<br/><br/> 
## Install `steamcmd`
Information extracted from [SteamCMD website][4]

### \> macOS
You can install `steamcmd` via [Homebrew][5] with:

	brew install steamcmd

Or with the following command, which requires you to manually add it to the `PATH`.

	curl -sqL "https://steamcdn-a.akamaihd.net/client/installer/steamcmd_osx.tar.gz" | tar zxvf -


### \> Windows
1. Create a folder for SteamCMD. For example:

	C:\steamcmd

2. Download SteamCMD for Windows: [https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip][6]
3. Extract the contents of the zip to the folder.

### \> Linux
You can install it via:

	sudo apt install steamcmd

 **Note**: 	If you are using a 64 bit machine, you will need to add multiverse

	sudo add-apt-repository multiverse
	sudo dpkg --add-architecture i386
	sudo apt update
	sudo apt install lib32gcc1 steamcmd 

# To do
Available [here][7]

[1]:	https://github.com/Spoons/rmm "rmm"
[2]:	https://github.com/AOx0/rwm#install--steamcmd
[3]:	https://www.rust-lang.org/tools/install
[4]:	https://developer.valvesoftware.com/wiki/SteamCMD "SteamCMD website"
[5]:	https://brew.sh "Homebrew"
[6]:	https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip
[7]:	https://github.com/AOx0/rwm/projects/1

[image-1]:	./media/Demo1.svg