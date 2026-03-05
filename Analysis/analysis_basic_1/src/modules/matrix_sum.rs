pub fn matrics_sum_main() {
    // Y:  Matrix 3 X 3 =>  | n=3 |
    let a:[Vec<i32>;3] = [vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let b:[Vec<i32>;3] = [vec![9, 8, 7], vec![6, 5, 4], vec![3, 2, 1]];
    // array of vectors

    let mut count:u16 = 0;
    let n: usize = 3; // for indexes use usize => memory safe.
    let mut c:Vec<Vec<i32>> = vec![vec![0; n]; n]; // G: Empty matrix (each row has n=zeros and total rows are n)

    for i in 0..n {
        for j in 0..n {
            c[i][j] = a[i][j] + b[i][j];
            count += 1;
        }
    }

    println!("Resualt Matrix =>");
    for row in &c {
        println!("{:?}", row);
    }
    println!("Total Elements => {}", count);
}
