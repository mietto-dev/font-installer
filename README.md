# Font Installer

Shell utility for quickly installing unpackaged fonts on Fedora, written in Rust. Motivated by broken Gnome Fonts utility.

> Note: using this project to brush up my Rust abilities - any suggestions, feel free to open a PR!

Usage:

```bash
# font-installer <dir> (path to your font files), i.e:
font-installer ~/Downloads
```

## Features

- [ ] Install fonts for local user
- [ ] Install fonts system wide
- [ ] Batch install fonts
- [ ] Live preview
- [ ] Pick fonts directly from DB's
- [ ] Nerd Fonts
- [ ] Google Fonts
- [ ] Remove fonts (local / system)

## Demo

(INSERT GIF)

## Installation

From cargo package manager:

```bash
cargo install font-installer
```

## References and Dependencies

Based on [Writing a CLI app in Rust](https://mattgathu.dev/2017/08/29/writing-cli-app-rust.html).

**Args Parsing: Clap**
https://docs.rs/clap/latest/clap/

**Progress Bars, Spinners, etc**
https://crates.io/crates/indicatif

**File Selection**
https://github.com/urbanogilson/lineselect
https://github.com/console-rs/dialoguer

## Commands executed

**System wide installation**, for all users (requires `sudo` permissions):

Create a new directory in the system fonts directory (/usr/local/share/fonts/) to accommodate the new font family, and copy the downloaded fonts (e.g. robofont.ttf files)

```bash
sudo mkdir -p /usr/local/share/fonts/robofont
sudo cp ~/Downloads/robofont.ttf /usr/local/share/fonts/robofont/
```

Set permissions and update SELinux labels:

```bash
sudo chown -R root: /usr/local/share/fonts/robofont
sudo chmod 644 /usr/local/share/fonts/robofont/*
sudo restorecon -vFr /usr/local/share/fonts/robofont
```

Update the font cache:

```bash
sudo fc-cache -v
```

---

**Local installation**, for current user only:

Create local fonts dir and copy font files (ex: `robofont`):

```bash
mkdir -p ~/.local/share/fonts/robofont
cp ~/Downloads/robofont.ttf ~/.local/share/fonts/robofont
```

Update the font cache:

```bash
fc-cache -v
```
