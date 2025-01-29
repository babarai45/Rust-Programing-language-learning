use std::time::Instant;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const MAX_ITER: usize = 1000;

fn mandelbrot(c: num::Complex<f64>) -> usize {
    let mut z = num::Complex::new(0.0, 0.0);
    for n in 0..MAX_ITER {
        if z.norm_sqr() > 4.0 {
            return n;
        }
        z = z * z + c;
    }
    MAX_ITER
}

fn main() {
    let start = Instant::now();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let real = (x as f64 - WIDTH as f64 / 2.0) * 4.0 / WIDTH as f64;
            let imag = (y as f64 - HEIGHT as f64 / 2.0) * 4.0 / HEIGHT as f64;
            let c = num::Complex::new(real, imag);
            mandelbrot(c);
        }
    }
    let duration = start.elapsed();
    println!("Rust execution time: {:?}", duration);
}
