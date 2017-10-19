use std::io;

fn count_digits(n: i32) -> usize {
    if n == 0 {
        return 1;
    }
    
    let mut n = n;
    
    let mut count = 0;
    while n != 0 {
        count += 1;
        n /= 10;
    }
    
    count
}

fn main() {
    let mut line = String::new();
    
    io::stdin().read_line(&mut line).expect("Incorrect string input");
    
    let number = line.parse::<i32>().expect("Failed to parse string to i32");
    
    println!("{} has {} digits", number, count_digits(number));
}
