pub mod a_matrix_calculations;
pub mod b_matrix_2;

use std::vec;

/// To calclate Matrix:
///  Matrix => n * n and not n * m
///  so n = m | must be true
pub fn run() {
    let mut a = vec![
        vec![1, 2, 3, 4],
        vec![4, 5, 6, 8],
        vec![7, 8, 9, 10],
        vec![11, 12, 13, 14],
    ];
    let b = vec![
        vec![14, 13, 12, 11],
        vec![9, 8, 7, 6],
        vec![6, 5, 4, 3],
        vec![3, 2, 1, 0],
    ];
    let c = vec![
        vec![1, 2, 3, 4],
        vec![4, 5, 6, 7],
        vec![7, 8, 9, 10],
        vec![11, 12, 13, 14],
    ];

    let d = vec![vec![1, 2, 3], vec![4, 5, 6]]; // 2x3

    let e = vec![vec![7, 8], vec![9, 10], vec![11, 12]]; // 3x2

    let flag: bool = a_matrix_calculations::check_matrix(&a);
    if flag {
        // Changing N will change thigns
        // let n: usize = 2; //because its 2*2 array
        // let n: usize = 3; //because its 3*3 array
        let n: usize = 4; //because its 4*4 array

        a_matrix_calculations::sum_of_2d_vec(&a, &b, n);
        a_matrix_calculations::sum_of_2d_vec_optimised(&mut a, &c, n);
        a_matrix_calculations::matrix_multiply(&a, &b, n);
    }

    match b_matrix_2::multiply(&a, &b) {
        Ok(res) => {
            for row in res {
                println!("{:?}", row);
            }
        }
        Err(e) => println!("Error: {}", e),
    };

    match b_matrix_2::multiply(&d, &e) {
        Ok(res) => {
            for row in res {
                println!("{:?}", row);
            }
        }
        Err(e) => println!("Error: {}", e),
    };
}

/// Y: Rhe ∑ shows => LOOP
///
/// k=0
/// ∑ A[i][k] * B[k][j]
/// k=n-1
///
/// meaning:
/// ```rust
///let mut sum = 0;
/// for k in 0..n { sum += A[i][k] * B[k][j]; }
/// C[i][j] = sum;
/// ```
pub fn _loop_explanation_methods() {
    println!("check info with shift + k");
}
