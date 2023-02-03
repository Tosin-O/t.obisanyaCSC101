//Rust program to calculate the roots of a quadratic equation
use std::io;

fn main()
 {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter A: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter B: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let B:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter C: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let b = f32::powf(B,2.0);
    let d:f32 = b - (4.0 * a * c);
    let x1:f32 = (-B + d.powf(0.5))/(2.0 * a);
    let x2:f32 = (-B - d.powf(0.5))/(2.0 * a);


    if d > 0.0  {
      println!("The two real roots are: {} and {}.",x1, x2);
    }

    else if d < 0.0  {
      println!("There are no real roots.");
    }

    else if d == 0.0 {
      println!("There is one root: {}",x1);
    }

}
