
fn main(){


// Method 1: Specifying the variable type explicitly
let age: u32 = 25;
println!("Age: {}", age);

// Method 2: Declaring a mutable variable
let mut count = 0;
println!("Count: {}", count);
count += 1;
println!("Updated Count: {}", count);

// Method 3: Declaring multiple variables at once
let (x, y, z) = (1, 2, 3);
println!("x: {}, y: {}, z: {}", x, y, z);

// Method 4: Declaring The Variable Without Specifying The Type Explicitly
let age = 18;
println!("{}",age);

// Declaring a constant variable with the value "Chandra Prakash"
const NAME: &str = "Chandra Prakash";
println!("{}", NAME);

const username: &f32 = &18.56;
println!("{}",username);



}

