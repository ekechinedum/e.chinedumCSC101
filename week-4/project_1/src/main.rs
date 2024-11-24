use std::io;

fn main() {
    
    println!("Enter the value of a:");
    let mut a_input = String::new();
    io::stdin().read_line(&mut a_input).expect("Failed to read input");
    let a: f64 = a_input.trim().parse().expect("invalid input for a");

    println!("Enter the value of b:");
    let mut b_input = String::new();
    io::stdin().read_line(&mut b_input).expect("Failed to read input");
    let b: f64 = b_input.trim().parse().expect("invalid input for b");

    println!("Enter the value of c:");
    let mut c_input = String::new();
    io::stdin().read_line(&mut c_input).expect("Failed to read input");
    let c: f64 = c_input.trim().parse().expect("invalid input for c");


    let discriminant be = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The roots are real and distinct: {:.2} and {:.2}", root1, root2);

    } else if discriminant == 0.0 (

        
    )

}