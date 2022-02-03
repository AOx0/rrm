<div><img src="https://img.shields.io/badge/Status-Unstable-red"></img></div>

</br>

TODOs are available [here][1]. Discussions, PRs and Issues are open for anyone who is willing to contribute. 

---- 
# rrm

Inspired by Spoons [rmm][2]. This is a cross-platform Mod Manager for RimWorld intended to work with macOS, linux and Windows

<br/> <br/>
## Installation
This program is installable with `cargo`. Install `rust` along with `cargo` [here][3]. It’s bundled with anything it needs to work. For security reasons, the minimum `rustc` version to compile the program is `1.58.1`.

You can update rustc with:

	rustup update

### Install
To install run:

	cargo install rrm

### Linux additional steps
Additionally, if you are using **_linux_** on a 64 bit machine, you will need to add multiverse with:

	sudo add-apt-repository multiverse
	sudo dpkg --add-architecture i386
	sudo apt update
	sudo apt install lib32gcc1 steamcmd 



<br/> <br/>
## Configuration
To set configuration values like game installation path and whether `rrm` should use `more` to display its output or not, you can use the `set` subcommand. The configuration file is available in `$USER_HOME/.rrm/config` on macOS, Linux, and Windows with a JSON format. 

For example, to set the path were RimWorld is installed, you can use:

	rrm set game-path /Applications/RimWorld.app

Or with its alias:

	rrm set path /Applications/RimWorld.app

Help message:

	rrm-set
	Set new configuration values
	
	USAGE:
	    rrm set <OPTION> <VALUE>
	
	OPTIONS:
	    game-path    Set the path where RimWorld is installed [alias: 'path']
	    pager        Set the paging software to use, like bat, more or less [alias: 'paging']
	    use-pager    Set if rrm should use more to display output [values: false, true, 0, 1] [alias: 'use-paging']

You can bypass configured values with special flags. 
- `-—no-pager`: Disables the pager output display no matter what configurations says. Does not change the configuration’s value.
-  `-—pager`: Enables the pager output display no matter what configurations says. Does not change the configuration’s value.

[1]:	https://github.com/AOx0/rrm/projects/1
[2]:	https://github.com/Spoons/rmm "rmm"
[3]:	https://www.rust-lang.org/tools/install

[image-1]:	./media/Demo1.svg
