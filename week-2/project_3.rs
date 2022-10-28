<<<<<<< HEAD
fn main() {
	let p:f64 = 210000.0;
	let r:f64 = 5.0;
	let t:f64 = 3.0; 

	// compound interest
	let a = p * ( 1.0 - (r / 100.0)) * t;
	println!("Amount is {}", a);
	let d = a - p;
	println!("Depreciation is {}", d);
=======
fn main() {
	let p:f64 = 210000.0;
	let r:f64 = 5.0;
	let t:f64 = 3.0; 

	// compound interest
	let a = p * ( 1.0 - (r / 100.0)) * t;
	println!("Amount is {}", a);
	let d = a - p;
	println!("Depreciation is {}", d);
>>>>>>> 2d384b1a8edc8a1758f98f4e32d47d91b6a1d6ab
}