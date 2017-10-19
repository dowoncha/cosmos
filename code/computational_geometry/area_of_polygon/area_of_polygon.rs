struct Point(f64, f64);

const SIZE: usize = 100000;


fn calculate_area(a: &Point, b: &Point, c: &Point) -> f64 {
    ((a.0 - b.0) * (c.1 - b.1) - (a.1 - b.1) * (c.0 - b.0)).abs() / 2.0
}

fn main() {
    use std::io::{stdin, Read};
    
    let mut s = String::new();
    
    // Read number of points
    println!("Enter number of points");
    stdin().read_line(&mut s).expect("Incorrect string input");

    let n = s.parse::<i32>().unwrap();
    let mut points = Vec::new();

    // Read points
    for i in 0..n {
        let mut line = String::new();
        stdin().read_to_string(&mut line);
        // Split line into words and create into points
        let words: Vec<String> = line.lines().map(|line| {
            line.split_whitespace().collect()
        }).collect();
        
        let x = words[0].parse::<f64>().unwrap();
        let y = words[1].parse::<f64>().unwrap();
    
        points.push(Point(x, y));
    }
    
    let mut answer = 0.0;
    
    for i in 2..n {
        let index = i as usize;
    
        answer += calculate_area(&points[0], &points[index-1], &points[index]);
    }
    
    println!("The answer is: {}", answer);
}
