
pub fn run_nested(){
    let n = 5;
    let mut count = 0;

    for i in 0..n{
        for j in 0..n{
            println!("i={} | j={}",i,j);
            count +=1;
        }
        println!("----------")
    }
    println!("Steps ={}",count);
}
