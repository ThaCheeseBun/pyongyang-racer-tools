# pyongyang-racer-tools
Extract and convert assets from the 2012 hit game Pyongyang Racer.
Also supports repackaging of extracted folder.

# Why?
modding, duh.

# IMPORTANT
This tool are for the `common.dat` and `1.dat` files that are stored in a proprietary format, **NOT** `sound.dat` or `symbol.dat`. They are normal `.swf` files. Rename them and open with [JPEXS FFdec](https://github.com/jindrapetrik/jpexs-decompiler/) to modify those.

# Usage
Unpack an asset archive:
```bash
$ pyongyang-racer-tools unpack <file>
```
Repack an asset archive (edited files will probably work, added will not be used and removed will break the game):
```bash
$ pyongyang-racer-tools pack <folder>
```
Convert an asset file (only works for `.box`, `.obj` and `.map` right now):
```bash
$ pyongyang-racer-tools convert <file>
```
Support for converting the file back is planned but not prioritized.

# Building
Install Rust using [rustup](https://rustup.rs/) or any other method if you know what you are doing.

Clone the repository:
```bash
$ git clone https://github.com/ThaCheeseBun/pyongyang-racer-tools
```
Change directory:
```bash
$ cd pyongyang-racer-tools
```
Build the release binary:
```bash
$ cargo build --release
```
Binary is located at `target/release/pyongyang-racer-tools`.
