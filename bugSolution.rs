fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let x = vec[0]; // Clone the value instead of borrowing
    vec.push(3);
    println!("Value of x: {}", x);
}