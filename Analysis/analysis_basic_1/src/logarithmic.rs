
pub fn run_logarithmic(){
    let mut n=64;
    let mut count = 0;

    while n > 1{
        println!("n={}",n);
        n/=2;
        count +=1;
    }
    println!("----------------------------\nSteps for n/=2 => {}\n----------------------------",count);


    let mut n2 = 81;
    let mut count = 0;

    while n2 > 1{
        println!("n2={}",n2);
        n2/=3;
        count +=1;
    }
    println!("----------------------------\nSteps for n/=3 => {}\n----------------------------",count);
}


