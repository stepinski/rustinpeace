use linreg::{linear_regression, linear_regression_of};



fn main() {
    println!("starting module");

// Example 1: x and y values stored in two different vectors
let xs: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let ys: Vec<f64> = vec![2.0, 4.0, 5.0, 4.0, 5.0];

assert_eq!(Ok((0.6, 2.2)), linear_regression(&xs, &ys));


// Example 2: x and y values stored as tuples
let tuples: Vec<(f32, f32)> = vec![(1.0, 2.0),
                                   (2.0, 4.0),
                                   (3.0, 5.0),
                                   (4.0, 4.0),
                                   (5.0, 5.0)];

assert_eq!(Ok((0.6, 2.2)), linear_regression_of(&tuples));


// Example 3: directly operating on integer (converted to float as required)
let xs: Vec<u8> = vec![1, 2, 3, 4, 5];
let ys: Vec<u8> = vec![2, 4, 5, 4, 5];
let result=linear_regression(&xs, &ys);
let val:  (f32, f32) = result.unwrap();

println!("{:?}",val);
// assert_eq!(Ok((0.6, 2.2)), linear_regression(&xs, &ys));
println!("ending module");
}
