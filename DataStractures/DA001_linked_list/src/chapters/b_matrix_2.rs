use std::ops::{Add, Mul};

pub type Matrix<T> = Vec<Vec<T>>;

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>, String>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Default,
    // Copy => resue the values like n[i][k] easily.
    // Add  => use sum + someting
    // Mul  => we can use *
    // Default => create initial value acording ot T  GEN:
    //      eg: if T: i32 = default => 0
    //          if T: f32 = default => 0.0
{
    if a.is_empty() || b.is_empty() {
        return Err("Empty Matrix".into());
    }

    let a_rows = a.len();
    let a_cols = a[0].len();
    let b_rows = b.len();
    let b_cols = b[0].len();
    // X_rows =>  total number of inner vector elements
    // X_cols => total number of elements in first inner vector | in rust all must be same so only a[0] is enough

    // a.iter will give..
    // a = vec![         Ty: each line in loop
    //      vec![1,2,3], <=
    //      vec![4,5,6], <=
    //      vec![7,8,9], <=
    //    ]
    // NAV:
    //    .all(conduction) mean:
    //    check all row if they satisfy the conduction ?
    //   ID:
    //    | row | => litrally mean
    //    for row in a { ... }
    // ---
    //  combine:
    //  !a.iter().all(|row| row.len() == b_cols) :
    //  mean if any row has different lenght => error
    //  Y: how?
    //      op of a.iter moved into row =>
    //      each row.len() compaired to
    //      a_cols => which is a[0].len() {compaired with 1st col only}
    //
    // fn check_matrix_matching(
    //     a: &Matrix<T>,
    //     b: &Matrix<T>,
    //     a_cols: usize,
    //     b_cols: usize,
    // ) -> Result<String> {
    //     if !a.iter().all(|row| row.len() == a_cols) || !b.iter().all(|row| row.len() == b_cols) {
    //         return Err("Non Rectangular Matrix".into());
    //     }
    // }
    if !a.iter().all(|row| row.len() == a_cols) || !b.iter().all(|row| row.len() == b_cols) {
        return Err("Non Rectangular Matrix".into());
    }

    // check if both are same matrix.
    if a_cols != b_rows {
        return Err("Matrix Dimentinos Does Not Match With each others...".into());
    }

    // vec![vec![x;n1];n2] replace x with T and n1 and n2 with row and calls.
    // ANY SIZE MATRIX =>
    let mut result = vec![vec![T::default(); b_cols]; a_rows];

    for i in 0..a_rows {
        for j in 0..b_cols {
            let mut sum = T::default();
            for k in 0..a_cols {
                sum = sum + (a[i][k] * b[k][j]);
            }
            result[i][j] = sum;
        }
    }
    Ok(result)
}

// what we are donign here is
// if we ahve 2 matrix like
// ----------------
//  x = Row * Cols
// ----------------
//  a =  2  *  3
//  b =  3  *  2
// ----------------
//  => multiply
//
//  1 2 3
//  4 5 6
//  X
//  1 4
//  2 5
//  3 6
//
//  so we will do:
//  ID:  1.
//  ___________________
//  i [0] [0]
//  =>
//   1 * 1 + -> k0
//   2 * 2 + -> k1
//   3 * 3   -> k2
//   --------
//  = 1 + 4 + 9 => 14
//  ___________________
//  i [0] [1]
//  =>
//   1 * 4 +
//   2 * 5 +
//   3 * 6
//   --------
//  = 4 + 10 + 18 => 32
//  ___________________
//  and so on...
//
//  FINAL: so we get  2*2
//  -------
//  |14|32|
//  -------
//  |32|82|
//  -------
