# ws-cleaner

A simple workspace cleaner, with specialized clean patterns referenced in toml files !

## How to install ?

Simply run ```cargo install ws-cleaner``` in your terminal.

## Usage

The first step is to create the `clean.toml` file. It will contain the descriptors of the files that need to be deleted.

Fortunately, some pre-made examples are available: run ```ws-cleaner --init``` to use one.

Once the config file is created, simply run ```ws-cleaner``` to clean your workspace !

If you are not sure about your config, use the `--what-if` option to list the files that would be deleted.