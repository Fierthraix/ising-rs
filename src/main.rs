use rand::Rng;

const SIZE: usize = 10;
const T: f64 = 2.5;

fn main(){
    let mut matrix = Matrix::<Spin>::initialize(SIZE);
    let size = matrix.1;

    let mut rng = rand::thread_rng();

    // Run the simulation about 100x per dipole
    for i in 0..100*SIZE*SIZE {
        // Select a random row and column
        let i = rng.gen_range::<usize>(0, size);
        let j = rng.gen_range::<usize>(0, size);

        // Computer deltaU

        // If flipping reduces energy then do it
        if deltaU(i, j, Ediff) {
            // Flip the sping
        } else {
            // Use Bolztmann factor to give probability of flipping
            //if rand < exp(-Ediff/T) { Flip It }

        }

    }
}

#[derive(Clone, Copy, Debug)]
enum Spin {
    Up,
    Down
}

impl std::ops::Mult for Spin {
    //Up = +1
    //Down = -1
}

struct Matrix<T>(Vec<Vec<T>>, usize);

impl<T> Matrix<T> {

    fn size(&self) -> usize {
        self.0.iter().map(|row| row.len()).sum()
    }
}

impl Matrix<Spin> {
    /// Generate a random spin square grid
    fn initialize(size: usize) -> Self {
        let mut rng = rand::thread_rng();

        let mut matrix: Matrix<Spin> = Matrix(vec![vec![]; size], size);

        for row in matrix.0.iter_mut() {
            for _ in 0..size {
                row.push(if rng.gen::<bool>() { Spin::Up } else { Spin::Down });
            }
        }

        matrix
    }
    fn deltaU(&mut self, i: usize, j: usize) -> f64 {
        let size = self.1;
        let top = if i == 0 { self.0[size-1][j] } else { self.0[i-1][ j] };
        let bot = if i == size-1 { self.0[0][ j] } else { self.0[i+1][ j] };
        let left = if j == 1 {self.0[0][size-1]} else { self.0[i][j-1] };
        let right = if j == size-1 { self.0[i][ 0] } else { self.0[i][ j+1] };

        2*self.0[i][j]*(top+bot+left+right)
    }
}
