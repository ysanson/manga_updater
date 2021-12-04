# Manga Updater

## Introduction

This program help to keep track of ongoing mangas on [manganelo](manganelo.com).
It supports telling updates, adding new mangas, and updating the old ones.  

## Commands
 
- `Init`: Creates a new CSV file to store the mangas.
- `Add [URL]`: adds the URL to the CSV file. It adds the latest chapter while doing so.
- `List`: Lists the mangas and for each of them tells if an update is present or not.
- `Update`: Updates all the mangas to their latest chapters.
- `Export -e [path to folder]`: Exports the CSV file to a specified folder.
- `Import -e [path to file]`: Imports the specified file to the program's CSV. 
- `Open [num]`: Opens the manganelo page of the manga. Combined with -d, opens directly the last chapter.
- `Unread [num]`: Sets the last chapter number of a manga back one time. Useful if the manga has been updated in error.
- `Undo`: Undoes the last write operation on the CSV (the updated mangas goes back to their last states). Only for new mangas and updated lines.

Use `manga_updater -h` for a full list of available commands, options, along with their descriptions.

## Technologies

### Code 
This project is being build using Rust version 1.54.0. 

#### Documentation

Documentation for this project is available in the code directly, and the documentation pages can be generated using the command `cargo doc` from the cargo package manager.

#### Testing

A few unit tests are available, testing the core functionalities. You can run them with the command `cargo test`.

### Database

The program uses a CSV file to store its data. It's a simple CSV with 2 columns:
- url: the URL to the manga page in manganelo.
- last chapter: the last chapter recorded. Useful to tell when a new chapter is available.

## Installation

No precompiled packages are published at the time, though it can be added later. To build this program locally, you will need Rust 1.54.0 or higher, installation instructions can be found [here](https://www.rust-lang.org/tools/install).

1. Clone this repository, or download it as a ZIP file and extract the contents wherever you like.
2. With a terminal, move to the directory, and type the command `cargo build --release`. This will effectively build the program and its dependencies for production. It may take a few minutes to download and build all the dependencies.
3. The build artefact will be located in the `target/release` folder. You can now take this executable, place it wherever you like (and add a new entry to the PATH variable) to launch the program from anywhere.
4. Use `manga_updater init` to create an empty CSV file.
5. Add new entries with `manga_updater add [URL]`. Consult them with `manga_updater (list)` (the list command is optional).
6. ???
7. Enjoy!

## FAQ

##### Why?  
Because why not?

##### What about creating an account?  
Well, it wouldn't be fun, would it?

##### Well, ok... But why in Rust?
I wanted to get my hands on Rust for a long time, and I wanted to create something cool with it.

##### How does it work?
This program is a scraper. Given an input URL (stored in the CSV file), it will parse the HTML page to find the relevant information (the manga title, the last chapter, and the URL to the last chapter). The information is then presented to the user in console format, who can then choose a manga to open. 

## License

Copyright 2021 Yvan SANSON

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

       (http://www.apache.org/licenses/LICENSE-2.0)[http://www.apache.org/licenses/LICENSE-2.0]

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
