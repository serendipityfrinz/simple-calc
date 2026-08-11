# simple-calc [![Crates.io](https://img.shields.io/crates/v/simple-calc.svg)](https://crates.io/crates/simple-calc) [![Downloads](https://img.shields.io/crates/d/simple-calc.svg)](https://crates.io/crates/simple-calc) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
***A (not so) simple CLI calculator made in Rust.***

## ℹ️ Info
This calculator contains the four basic math operations, rounding, other stuff inside advanced mode.

Great if you need a reliable, light calculator to use with assignments or quick calculations.
###### ⚠️ (P.S. don't use this for solving something as hard as the Riemann Hypothesis.)

#### Note from creator
*I don't think simple-calc is so simple anymore...*


## 📦 Dependencies
* [clearscreen](https://crates.io/crates/clearscreen) (4.0.6)
* [colored](https://crates.io/crates/colored) (3.1.1)
* [inquire](https://crates.io/crates/inquire) (0.9.4)


## ⚙️ Usage
Just run `simple-calc`, easy as that.
***Note: it does not have CLI arguments.***


## 📥 Installation
Enter this command: 
```bash
cargo install simple-calc
```
Cargo will fetch crates and compile the binary.
After that, it's ready for use! :D


## 🎬 Demo
<p align="center">
  <img src="assets/demo.gif" alt="simple-calc demo" width="800"><br>
  <sub><i>A demo of simple-calc :)</i></sub>
</p>


## 🐧🪟🍎 Support
### Windows: 
It should work on Windows because it uses `std::thread`, `std::time::Duration`,
clearscreen, colored, and inquire, which is cross-platform. However it is not tested
because i use Gentoo Linux (btw).

### Linux:
It definitely works there because that's where i made simple-calc in (i use Gentoo btw).

### MacOS:
Untested, but it should compile and run smoothly since macOS shares standard UNIX runtime primitives with Linux.


## 📄 License
Licensed under ![MIT License](LICENSE).
