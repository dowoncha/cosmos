fn mean(n: &[f64]) -> f64 {
    let sum = n.iter().fold(0.0, |acc, &n| acc + n);
    
    sum / 1.max(n.len()) as f64
}  

fn std_dev(arr: &[f64]) -> f64 {
    let n_mean = mean(arr);
    
    let mut std = 0.0;
   
    for elem in arr {
        std = std + (elem - n_mean) * (elem - n_mean);
    }
    
    (std / arr.len() as f64).sqrt()
}

#[test]
fn test_std_dev() {
    let arr = vec![12.0f64, 65.0, 91., 52., 18., 72.];
    let std = std_dev(&arr);
    
    // Margin of error is 0.01
    assert!((std - 28.41).abs() <= 0.01);
}
