use std::io;

fn fib(nth: u32){
    let mut x = (1, 1);
    for _i in 0..nth - 1 {
        x = (x.1, x.0 + x.1)
    }
    println!("The number at position {} in the fibonacci sequence is {}", nth, x.0);
}

fn main(){
    println!("What position in the fibonacci sequence would you like revealed? Enter a positive number.");
    let mut nth_is = String::new();

    io::stdin()
    .read_line(&mut nth_is)
    .expect("Failed to read the line.");

    let nth_of_fib: u32 = nth_is.trim().parse().expect("Required a number.");

    
    
    return fib(nth_of_fib);
}
