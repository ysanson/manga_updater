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

## Technologies

This project is being build using Rust version 1.47.0.  
In addition, the program uses a CSV file to store its data. It's a simple CSV with 2 columns:
- url: the URL to the manga page in manganelo.
- last chapter: the last chapter recorded. Useful to tell when a new chapter is available. 

## FAQ

##### Why?  
Because why not?

##### What about creating an account?  
Well, it wouldn't be fun, would it?

##### Well, ok... But why in Rust?
I wanted to get my hands on Rust for a long time, and I wanted to create something cool with it.
