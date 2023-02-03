use std::io;


fn studentinfo() {

    let mut input5 = String::new();

    println!("Are you a class rep: , if yes enter 1,if no enter 0");
    io::stdin().read_line(&mut input5 ).expect("Failed to read input");
    let rep:i32 = input5.trim().parse().expect("Invalid input");


     let mut input6 = String::new();

     println!("Are you in 100 level: ,if yes enter Y,if no enter N");
     io::stdin().read_line(&mut input6 ).expect("Failed to read input");
     let level:char = input6.trim().parse().expect("Invalid input");

     let mut input7 = String::new();

     println!("What is your cgpa: ");
     io::stdin().read_line(&mut input7 ).expect("Failed to read input");
     let cgpa:f64 = input7.trim().parse().expect("Invalid input");

    if rep == 1
     {
        println!("You are eligible to vote")
     }
    
     
     else if level == 'N'
     {
        println!("You are eligible to vote");
     }
    
     else if cgpa >= 4.0
     {
        println!("You are  eligible to vote");
     }

     else {
        println!("Your are not eligible to vote");
     }
    

}

fn main() {

for i in 1..16{
  let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();

    println!("What is your name: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    

    println!("What is your email: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
   

    println!("What is your department: ");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    

    println!("What is your state of origin: ");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    

  
    studentinfo();
    }
}

