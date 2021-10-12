// use ndarray::arr2;

// fn main() {
//     let a = arr2(&[[1, 2, 3],
//                    [4, 5, 6]]);

//     let b = arr2(&[[6, 5, 4],
//                    [3, 2, 1]]);

//     let sum = &a + &b;

//     println!("{}", a);
//     println!("+");
//     println!("{}", b);
//     println!("=");
//     println!("{}", sum);
// }


// use ndarray::arr2;

// fn main() {
//     let a = arr2(&[[1, 2, 3],
//                    [4, 5, 6]]);

//     let b = arr2(&[[6, 3],
//                    [5, 2],
//                    [4, 1]]);

//     println!("{}", a.dot(&b));
// }


// use ndarray::{arr1, arr2, Array1};

// fn main() {
//     let scalar = 4;

//     let vector = arr1(&[1, 2, 3]);

//     let matrix = arr2(&[[4, 5, 6],
//                         [7, 8, 9]]);

//     let new_vector: Array1<_> = scalar * vector;
//     println!("{}", new_vector);

//     let new_matrix = matrix.dot(&new_vector);
//     println!("{}", new_matrix);
// }


// use approx::assert_abs_diff_eq;
// use ndarray::Array;

// fn main() {
//   let a = Array::from(vec![1., 2., 3., 4., 5.]);
//   let b = Array::from(vec![5., 4., 3., 2., 1.]);
//   let mut c = Array::from(vec![1., 2., 3., 4., 5.]);
//   let mut d = Array::from(vec![5., 4., 3., 2., 1.]);

//   let z = a + b;
//   let w =  &c + &d;

//   assert_abs_diff_eq!(z, Array::from(vec![6., 6., 6., 6., 6.]));

//   println!("c = {}", c);
//   c[0] = 10.;
//   d[1] = 10.;

//   assert_abs_diff_eq!(w, Array::from(vec![6., 6., 6., 6., 6.]));

// }


// use ndarray::{array, Array1, ArrayView1};

// fn l1_norm(x: ArrayView1<f64>) -> f64 {
//     x.fold(0., |acc, elem| acc + elem.abs())
// }

// fn l2_norm(x: ArrayView1<f64>) -> f64 {
//     x.dot(&x).sqrt()
// }

// fn normalize(mut x: Array1<f64>) -> Array1<f64> {
//     let norm = l2_norm(x.view());
//     x.mapv_inplace(|e| e/norm);
//     x
// }

// fn main() {
//     let x = array![1., 2., 3., 4., 5.];
//     println!("||x||_2 = {}", l2_norm(x.view()));
//     println!("||x||_1 = {}", l1_norm(x.view()));
//     println!("Normalizing x yields {:?}", normalize(x));
// }


// use nalgebra::Matrix3;

// fn main() {
//     let m1 = Matrix3::new(2.0, 1.0, 1.0, 3.0, 2.0, 1.0, 2.0, 1.0, 2.0);
//     println!("m1 = {}", m1);
//     match m1.try_inverse() {
//         Some(inv) => {
//             println!("The inverse of m1 is: {}", inv);
//         }
//         None => {
//             println!("m1 is not invertible!");
//         }
//     }
// }


extern crate nalgebra;
extern crate serde_json;

use nalgebra::base::DMatrix;

fn main() -> Result<(), std::io::Error> {
    let row_slice: Vec<i32> = (1..5001).collect();
    let matrix = DMatrix::from_row_slice(50, 100, &row_slice);

    // serialize matrix
    let serialized_matrix = serde_json::to_string(&matrix)?;

    // deserialize matrix
    let deserialized_matrix: DMatrix<i32> = serde_json::from_str(&serialized_matrix)?;

    // verify that `deserialized_matrix` is equal to `matrix`
    assert!(deserialized_matrix == matrix);

    Ok(())
}
