use num::complex::Complex;
fn main() {
    let input: [f64; 5] = [2.3, 7.8, 8.9, 9.8, 2.7];
    let size = input.len();
    let mut finished_dft: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); size];
    let e = std::f64::consts::E; //This is my Euler's Constant
    let i = Complex::new(0.0, 1.0); //This is my Imaginary Number
    let pi = std::f64::consts::PI; //This is Pi
    for k in 0..size {
        let mut result = Complex::new(0.0, 0.0); // Initialize X_k
        for n in 0..size {
            result += Complex::new(e, 0.0).powc(-2.0 * i * pi * k as f64 * n as f64 / size as f64); //ti
            
        }   
        finished_dft[k] = result;
    }
    println!("DFT Results:");
    for (k, value) in finished_dft.iter().enumerate() {
        println!("X[{}] = {}", k + 1, value);
    }
}