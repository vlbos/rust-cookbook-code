// fn main() {
//     let complex_integer = num::complex::Complex::new(10, 20);
//     let complex_float = num::complex::Complex::new(10.1, 20.1);

//     println!("Complex integer: {}", complex_integer);
//     println!("Complex float: {}", complex_float);
// }

// fn main() {
//     let complex_num1 = num::complex::Complex::new(10.0, 20.0); // Must use floats
//     let complex_num2 = num::complex::Complex::new(3.1, -4.2);

//     let sum = complex_num1 + complex_num2;

//     println!("Sum: {}", sum);
// }


// use std::f64::consts::PI;
// use num::complex::Complex;

// fn main() {
//     let x = Complex::new(0.0, 2.0*PI);

//     println!("e^(2i * pi) = {}", x.exp()); // =~1
// }




use num::bigint::{BigInt, ToBigInt};

fn factorial(x: i32) -> BigInt {
    if let Some(mut factorial) = 1.to_bigint() {
        for i in 1..=x {
            factorial = factorial * i;
        }
        factorial
    }
    else {
        panic!("Failed to calculate factorial!");
    }
}

fn main() {
    println!("{}! equals {}", 100, factorial(100));
}
