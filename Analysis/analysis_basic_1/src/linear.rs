pub fn run_linear(){
    let n = 10; 
    let mut count = 0;

    for i in 0..n{
        println!("i = {}", i);
        count +=1;
    }
    println!("Total steps {}",count);
}
