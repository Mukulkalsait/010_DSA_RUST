/// to insure matrix n = m must be true
/// hence we check and verify if the
/// row = cols are true or not from this function.
pub fn check_matrix(a: &[Vec<i32>]) -> bool {
    let rows = a.len();
    let cols = a[0].len();
    println!("_________________________________");
    println!("we have {}=Rows and {}=Clos", rows, cols);

    if rows == cols {
        println!("___________________\nMatrix test passed.\n___________________");
        println!("Initialising main program...\n\n\n=>");
        true
    } else {
        println!(
            "Matrix test Failed.\nPlease make sure the number of rows and number of cols are same."
        );
        false
    }
}

/// G: spaceC =>
/// we are doing 3*3 matrix sum
/// 1. n, count , i, j, => all constant
///    O(0)
/// ----
/// 2. a, b, c => 3*3 matrix
///    => n*n => n square
///    => 3n square +
/// ----
/// 3. Auxilary Memory :
///    x => n*n => n square | is created on runtime.
///    * can it be reduced?
///    * yes 👇
///    * [sum_of_2d_vec_optimised()](sum_of_2d_vec_optimised)
pub fn sum_of_2d_vec(a: &[Vec<i32>], b: &[Vec<i32>], n: usize) {
    let mut count = 0;
    let mut x = vec![vec![0; n]; n]; // Space C => n square
    //
    for i in 0..n {
        for j in 0..n {
            x[i][j] = a[i][j] + b[i][j]; // tiemC => n squ
            count += 1;
        }
    }
    println!("Resualt Matrix");

    for row in &x {
        println!("{:?}", row);
    }
    println!("Total elements operations: {}", count);
    println!("========================================")
    // B:Time C is  [O n squrre]
    // .. code inside nested loop.
}

/// Y: SPACE OPTIMISATION :
///     * instead of x
///     * use a += b method.
///     instead of doing AUXILARY variable x we put sums in a (made it mutable.)
///
/// R: Space C =>
///  - BEFORE:
///    3n square + O(1) + auxilary n Square
///  - AFTER OPTIMISATION:
///    3n square + O(1)
pub fn sum_of_2d_vec_optimised(a: &mut [Vec<i32>], b: &[Vec<i32>], n: usize) {
    for i in 0..n {
        for j in 0..n {
            a[i][j] += b[i][j];
        }
    }
    println!("Resualt Matrix with Auxilary Optimistaion:");
    for line in a {
        println!("{:?}", line);
    }
    println!("========================================")
}

/// R: this is the Multipleacation of  4*4 matirx
pub fn matrix_multiply(a: &[Vec<i32>], b: &[Vec<i32>], n: usize) {
    let mut c = vec![vec![0; n]; n];
    let mut count = 0;

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
                count += 1;
            }
        }
    }

    println!("==================== Matrix Multipleacation ====================");
    for row in &c {
        println!("{:?}", row);
    }
    println!("Total Elementas Operations = {}", count)
}
// ================================================================================================================================================================
