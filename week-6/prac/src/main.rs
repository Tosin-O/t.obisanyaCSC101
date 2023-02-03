use std::io;

fn voter () {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();

    println!("What is your name: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let ch:char = input1.trim().parse().expect("Invalid input");

    println!("What is your email: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let ch:char = input2.trim().parse().expect("Invalid input");

    println!("What is your department: ");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let ch:char = input3.trim().parse().expect("Invalid input");

    println!("What is your state of origin: ");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let ch:char = input4.trim().parse().expect("Invalid input");
}
