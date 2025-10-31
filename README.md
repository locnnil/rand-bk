# Rand Bk
[![tests](https://github.com/locnnil/rand-bk/actions/workflows/test.yml/badge.svg)](https://github.com/locnnil/rand-bk/actions/workflows/test.yml)

This tool is designed to launch your terminal emulator with a random background color, chosen from a predefined set of colors.
It is particularly useful for those who enjoy a bit of variety in their terminal aesthetics.

## Motivation

Often you're working on the terminal and then you open multiple tabs or windows, and they all look the same.
This can make it hard to quickly identify which terminal window you're working no what.
By using a random background color, you can easily distinguish between different terminal instances.

## Support list

- Alacritty [✅]
- Ghostty [⌛]
- Kitty [⌛]
- GNOME Terminal [⌛]


## Installation

This application is available via `cargo` and can be installed with the following command:

```shell
cargo install rand-bk
```

## Post Installation

It's kind of weird to launch a terminal from another terminal, so you need to set up a shell alias or function.
A good way to do this on Gnome DE is to create a custom shortcut that calls `rand-bk`.

- Create a custom shortcut in your system settings > keyboard > View and Customize Shortcuts > Custom Shortcuts.
- Click on + icon and fill the form:
  - Name: Rand Bk
  - Command: rand-bk
  - Shortcut: Choose your own shortcut (e.g., `Ctrl + Alt + a`)

