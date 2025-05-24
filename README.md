# Oscar the Trash CLI
![Picture of Oscar the Grouch from Sesame Street](https://sesameworkshop.org/wp-content/uploads/2023/02/presskit_ss_bio_oscar-560x420.png "Oscar the Grouch")

Your friendly neighborhood CLI to help you sift thru and take care of your FreeDesktop system trash. Whenever you need to interface with your trash, Oscar will be there to lend you a helping hand!

# About
Oscar the Trash CLI is my personal port of [trash-cli](https://github.com/andreafrancia/trash-cli/) to Rust using an improved CLI. Last year I opened [an issue on GitHub](https://github.com/andreafrancia/trash-cli/issues/290) to propose to update the existing trash CLI written in Python to use subcommands instead of have separate commands for each trash operation (put, list, remove, etc). Now that I have time, I have decided to make a spiritual fork of that project written in Rust for the sake of my own learning as well embracing new technology.

Oscar implements the [FreeDesktop.org Trash Specifcation](https://specifications.freedesktop.org/trash-spec/latest/), so will be able to seamlessly integrate into any environment (e.g. GNOME, KDE, etc) that uses that specification. For systems that do not, this will effectively operate as a secondary, independent system trash.

**NOTE:** Currently this CLI only works on the home trash for a given system. Future features I would like to implement include adding a hard drive for all partitions

## Installation

### Distro Package Mangers
If this project goes far enough, I would love to work with various distros to get this included in their package managers

### Cargo (Planned)
Coming soon! Since this is written in Rust, the hope will be to put this on Cargo and make installation as simple as `cargo install oscar-trash-cli`. It will be some time before this is ready for prime time, however, so this may change.

### Build from Source
1. Clone this repository E.g:
`git clone https://github.com/curtiscali/oscar-trash-cli.git`

2. cd into the cloned repo
`cd oscar-trash-cli`

3. Build Oscar in release mode (this makes the CLI a bit faster)
`cargo build -r [-j <number of cores for your CPU goes here>] #-j is completely optional, but will speed up compilation a bit`

4. If compilation succeeds, the compiled binary will be located in `target/release/oscar`
5. To install oscar, simply move the above binary to any directory located in your PATH. E.g;
`sudo mv target/release/oscar /usr/local/bin`

Congratulations! You can now use Oscar! Please see the section below for usage guide

## Usage
**WARNING: This software is under development, so the following may be changed as implementation progresses**

The basic structure of an Oscar invocation is `oscar [command] [arguments]`.

Oscar supports the following commands:
- Put: places a file or directory into the system trash
- Empty: permanently deletes *ALL* contents of the system trash
- List: lists the contents of the trash
- Restore: restores a file/directory in the trash to its original location
- Remove: permanently deletes an individual file from the trash and system

```sh
oscar [-h|--help] # documents all available commands in oscar
oscar <command> [-h|--help] # documents all options for whichever command you entered

oscar put|p [file/directory] # places a file specified by the user into the home trash
oscar empty|e [-y|--yes] # empties the home trash. Permanently deletes all files/directories in the home trash
oscar list|ls [-r|--recursive] # lists all contents of the home trash.
oscar restore|rs [-o|--overwrite] #restores a file from the home trash to its original location
oscar rm [-y|--yes] #permanently deletes an individual file
```