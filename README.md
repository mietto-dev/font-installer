# Font Installer
Shell utility for quickly installing fonts on Fedora, written in Rust.

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

Here

Create local fonts dir and copy font files (ex: `robofont`):
```bash
mkdir -p ~/.local/share/fonts/robofont
cp ~/Downloads/robofont.ttf ~/.local/share/fonts/robofont
```

Update the font cache:
```bash
fc-cache -v
```