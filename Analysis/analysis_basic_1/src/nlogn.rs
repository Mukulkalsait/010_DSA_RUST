
pub fn run_n_log_n(){
    let n=8;
    let mut count=0;

    for _ in 0..n { // Y: runs 8 time but withuout changing n...
        println!("n ={}",n); // Y: =>  n is constant = 8 so no mut.
        let mut j = n;

        while j>1 {
            println!("j={} | n ={}",j,n);
            j/=2;
            count +=1
        }
        println!("-------------")
    }
    println!("Total Steps => {}",count);
    println!("Steps per outer loop => {}",count/n);


    println!("
==================
Outer = n ,
Inner = log n,
Total = n × log n 
      → O(n log n) 
==================

so mathematically.
Suppose initial value = n.
After 1 step → n/2
After 2 steps → n/4
After 3 steps → n/8
-------------------
After k steps: n / 2^k
Loop stops when: n / 2^k = 1
Multiply both sides by 2^k we get 
=> n = 2^k
=> 2^k = n => so when log=> 
Take log: k = log₂(n)

=================================
Outer runs {n} times => Each time inner runs log n times => {}.
So total operations: n × log n --> multiplication of frequencies.
",count/n)
}
