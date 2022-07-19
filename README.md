<div>
	<img src="https://img.shields.io/badge/GitHub   Version-0.0.1--alpha.9-green">
	</img><img src="https://img.shields.io/badge/Status-Almost Done-yellow">
</div>
<div>
	<a href="https://crates.io/crates/rrm">
		<img src="https://img.shields.io/badge/crates.io%20Version-0.0.1--alpha.9-green"></img>
	</a>
	<img src="https://img.shields.io/badge/Status-Almost Done-yellow">
</div>

<br/>

TODOs are available [here][1]. Discussions, PRs and Issues are open for anyone who is willing to contribute. 

<br/>

<img width="1680" alt="Screen Shot 2022-07-19 at 4 44 18 a m" src="https://user-images.githubusercontent.com/50227494/179720712-69a4d2d1-4cb1-4c48-902e-32644759e8dd.png">

# rrm

Inspired by Spoons [rmm][2]. This is a cross-platform Mod Manager for RimWorld intended to work with macOS, linux and Windows

## Usage
- All documentation, as well as usage examples and help is available in the [Wiki][3]. Or you can ask via [Discussion][4]  
- Mod installation demo available [here][7]

<br/>

## Installation
This program is installable with `cargo`. Install `rust` along with `cargo` [here][5]. It’s bundled with anything it needs to work. 

### Install
To install the `crates.io` pre-release version run:

	cargo install rrm --version 0.0.1-alpha.9

Or install the GitHub version. Although it's not recommended since it has a 'dev' flag which changes the installer behaviour:

	cargo install --git https://github.com/AOx0/rrm

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
[3]:	https://github.com/AOx0/rrm/wiki
[4]:	https://github.com/AOx0/rrm/discussions
[5]:	https://www.rust-lang.org/tools/install
[6]:	https://www.cve.org/CVERecord?id=CVE-2022-21658
[7]:    https://youtu.be/Fp5Y89DeLBY
