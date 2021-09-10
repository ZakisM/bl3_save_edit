![](https://img.shields.io/github/downloads/ZakisM/bl3_save_edit/latest/total)
![](https://img.shields.io/github/downloads/ZakisM/bl3_save_edit/total)

# Borderlands 3 Save Editor

A tool to help you modify your Borderlands 3 Saves and Profiles.

Currently it runs on Windows, Mac OS and Linux. It supports modifying PC saves as well as decrypted PS4 saves (and
converting between them).

# Screenshot

<img width="1762" alt="bl3_save_editor_screenshot" src="https://user-images.githubusercontent.com/8143258/132767695-dadff3ba-8328-4513-96dd-c682531e9214.png">

# How to use

Visit [Releases](https://github.com/ZakisM/bl3_save_edit/releases) and download the corresponding version for your
platform.

Unzip and open the editor, then double click to run it. On the first start it will ask you to point it to the
folder/directory where your saves/profiles are stored. Once you have pointed it to a valid folder it will remember this
folder the next time you open the program.

# Notices

The editor will make backups for you before saving your files but I recommend you make your own backups just in case.

# Building from scratch

First you must install [Rust](https://www.rust-lang.org/).

Then, clone the project and run:

`cargo build`

For a release optimized build run:

`cargo build --release`

# Credits

Huge credits to apocalyptech for their editor at https://github.com/apocalyptech/bl3-cli-saveedit. The majority of this
code was based off of their work.

Thanks to HackerSmacker for their PS4
bitmasks: https://github.com/HackerSmacker/CSave/blob/022c4e78ac7fa68e2338804bc0148ac9be3296f7/FileTranslator.c.

Thanks to those who created these docs:

- https://docs.google.com/spreadsheets/d/1XYG30B6CulmcmmVDuq-PkLEJVtjAFacx7cuSkqbv5N4
- https://docs.google.com/spreadsheets/d/16b7bGPFKIrNg_cJm_WCMO6cKahexBs7BiJ6ja0RlD04
- https://docs.google.com/spreadsheets/d/1v-F_3C2ceaFKJae1b6wmbelw_jLjmPPriBLzGTZMqRc

Tool to download data from these docs can be found [here](https://github.com/ZakisM/bl3_save_edit_resource_downloader).

Thanks to the [Ajour](https://github.com/ajour/ajour) project for their methods of bundling Rust Apps as well as updating
them.
