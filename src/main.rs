use png::HasParameters;
use rand::Rng;
use structopt::StructOpt;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

macro_rules! save {
    ($m:expr, $opt:expr, $i:expr) => {
        if $opt.png {
            $m.save(&format!("{}_{}-{:025}.png", $opt.base, $opt.temp, $i))
                .expect("Unable to save image");
        } else {
            println!("{:?}", $m);
        }
        // Print the magnetization percent
        if $opt.mag {
            let mag: usize =
                $m.0.iter()
                    .flat_map(|row| {
                        row.iter().map(|spin| match spin {
                            Spin::Up => 1,
                            Spin::Down => 0,
                        })
                    })
                    .sum();
            let mag = mag as f64 / $opt.size.pow(2) as f64;
            println!("Up Spin: {}%, Down Spin: {}%", mag, 1. - mag);
        }
    };
}

fn main() {
    let opt = Opt::from_args();
    let mut rng = rand::thread_rng();
    let mut matrix = Matrix::<Spin>::initialize(opt.size);

    if opt.verbose >= 1 {
        save!(matrix, opt, 0);
    }

    // Run the simulation about `iters` times per dipole (0 -> iters * size^2)
    for iter in 1..opt.iters * opt.size.pow(2) {
        // Select a random row and column
        let i = rng.gen_range(0, opt.size);
        let j = rng.gen_range(0, opt.size);

        let energy_diff = matrix.delta_u(i, j);
        // If flipping reduces energy then do it
        if energy_diff <= 0. {
            matrix.0[i][j].flip()
        } else {
            // Use Bolztmann factor to give probability of flipping
            if rng.gen::<f64>() < (-energy_diff / opt.temp).exp() {
                matrix.0[i][j].flip()
            }
        }
        // Print every iteration if the user asks
        if opt.verbose == 1 && iter % opt.size.pow(2) == 0 {
            save!(matrix, opt, iter);
        } else if opt.verbose >= 2 {
            save!(matrix, opt, iter);
        }
    }
    save!(matrix, opt, opt.iters * opt.size.pow(2));
}

#[derive(Clone, Copy)]
enum Spin {
    Up,
    Down,
}

impl Spin {
    /// Flip the spin
    fn flip(&mut self) {
        match self {
            Spin::Up => *self = Spin::Down,
            Spin::Down => *self = Spin::Up,
        }
    }

    /// Convert to floating point value
    fn as_f64(&self) -> f64 {
        match self {
            Spin::Up => 1.0,
            Spin::Down => -1.0,
        }
    }
}

impl std::fmt::Debug for Spin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match self {
            Spin::Up => "⬆",   // ⇧
            Spin::Down => "⇩", // ⬇
        };
        write!(f, "{}", c)
    }
}

struct Matrix<T>(Vec<Vec<T>>, usize);

impl<T: std::fmt::Debug> std::fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.0.iter() {
            for item in row.iter() {
                write!(f, "{:?} ", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Matrix<Spin> {
    /// Generate a random spin square grid
    fn initialize(size: usize) -> Self {
        let mut rng = rand::thread_rng();

        let mut matrix: Matrix<Spin> = Matrix(vec![vec![]; size], size);

        for row in matrix.0.iter_mut() {
            for _ in 0..size {
                row.push(if rng.gen::<bool>() {
                    Spin::Up
                } else {
                    Spin::Down
                });
            }
        }

        matrix
    }
    /// Calculate the energy delta using wrapping grid math
    fn delta_u(&self, i: usize, j: usize) -> f64 {
        let size = self.1;
        // All the if statements handle boundary conditions,
        // the branch predictor should speed this up
        let top = if i == 0 {
            self.0[size - 1][j].as_f64()
        } else {
            self.0[i - 1][j].as_f64()
        };
        let bot = if i == size - 1 {
            self.0[0][j].as_f64()
        } else {
            self.0[i + 1][j].as_f64()
        };
        let left = if j == 0 {
            self.0[0][size - 1].as_f64()
        } else {
            self.0[i][j - 1].as_f64()
        };
        let right = if j == size - 1 {
            self.0[i][0].as_f64()
        } else {
            self.0[i][j + 1].as_f64()
        };

        2. * self.0[i][j].as_f64() * (top + bot + left + right)
    }
    /// Save the matrix as a png image
    fn save(&self, name: &str) -> Result<(), std::io::Error> {
        #[allow(clippy::toplevel_ref_arg)]
        let ref mut w = BufWriter::new(File::create(Path::new(name))?);
        let mut encoder = png::Encoder::new(w, self.1 as u32, self.1 as u32);
        encoder
            .set(png::ColorType::Grayscale)
            .set(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        let data: Vec<u8> = self
            .0
            .iter()
            .flat_map(|row| {
                row.iter().map(|spin| match spin {
                    Spin::Up => 0u8,
                    Spin::Down => 255u8,
                })
            })
            .collect();

        writer.write_image_data(&data)?;
        Ok(())
    }
}

#[derive(StructOpt)]
#[structopt(name = "ising")]
/// simulate the 2D Ising model using a Mante Carlo simulation of the Metropolis algorithm
struct Opt {
    /// the temperature
    #[structopt(short = "T", long = "temp", default_value = "2.5")]
    temp: f64,

    /// the grid size
    #[structopt(short = "s", long = "size", default_value = "10")]
    size: usize,

    /// the number of times to run the simulation per dipole
    #[structopt(short = "i", long = "iterations", default_value = "100")]
    iters: usize,

    /// print all of the in-between states
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,

    /// save as a png image
    #[structopt(short = "p", long = "png")]
    png: bool,

    /// give a basename for the png file(s)
    #[structopt(short = "b", long = "base", default_value = "ising-2D")]
    base: String,

    /// print the magnetization for every printed run
    #[structopt(short = "m", long = "mag")]
    mag: bool,
}
