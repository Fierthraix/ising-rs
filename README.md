# ising-rs
Simple simulation of the 2D Ising model using a Monte Carlo simulation of the Metropolis algorithm

## Installation
1. [Install Rust first](https://rustup.rs/)
2. `git clone https://github.com/Fierthraix/ising-rs`
3. `cd ising-rs`
4. `cargo build --release`
5. `./target/release/ising`

## Flags
* `-i` The number of iterations to run the program per diopole. As only one dipole is modified at a time the program will run iterations$\times$size$^2$.
* `-s` Change the size of the square grid
* `-T` Set the temperature in units of $\epsilon/k_B$
* `-p` Save png images using the given basename instead of printing unicode arrows
* `-b` Change the basename/pick a folder for saving png images
* `-v` Change the verbosity. When this flag is used once then `iterations` images will be printed. When called twice or more every single matrix state is printed (even ones with no change).

## Examples
Make a 1000$\times$1000 grid iterate 100 times and save the evolution in `/tmp/ising/`
```bash
cargo run --release -- -p -b "/tmp/ising/run_1" -s 1000 -T 1.5 -v -i 100
```
The `-p` means we'll be saving png images, `-b` tells us to save the images in `/tmp/ising` and to use `run_1` as the basename. This means that the first image will be `run_1_1.5_00000000000000000000.png` and the last will be `run_1_1.5_00000000000010000000.png`. The temperature is 1.5, the grid size is 100. The `-v` means we will print create 100 (`-i 100`) images. If we called `-v` twice there would be 100$\times$1000$^2$=100,000,000 images produced.
