#![allow(unused)]
use std::io;

fn main(){
    println!("Enter name");
    let mut name_input = String::new();
    io::stdin().read_line(&mut name_input).expect("Failed to read input");
    let _name:String = name_input.trim().parse().expect("invalid input for name");
    println!("Your name is {} ",_name);

    println!("Enter Year of birth");
    let mut dob_input = String::new();
    io::stdin().read_line(&mut dob_input).expect("Failed to read input");
    let _dob: i64 = dob_input.trim().parse().expect("invalid input for dob");
    println!("You were born in {} ",_dob);

    println!("Enter email address");
    let mut email_input = String::new();
    io::stdin().read_line(&mut email_input).expect("Failed to read input");
    let _e:String = email_input.trim().parse().expect("invalid input for email");
    println!("Your email is {},",_e);

    println!("Enter your phone number");
    let mut number_input = String::new();
    io::stdin().read_line(&mut number_input).expect("Failed to read input");
    let _n: i64 = number_input.trim().parse().expect("invalid input for number");
    println!("Your phone number is {} ",_n);

    println!("Enter number of children");
    let mut children_input = String::new();
    io::stdin().read_line(&mut children_input).expect("Failed to read input");
    let _c: i64 = children_input.trim().parse().expect("invalid input for Number of children");
    println!("You have {} children ",_c);

    println!("Enter number of siblings");
    let mut siblings_input = String::new();
    io::stdin().read_line(&mut siblings_input).expect("Failed to read input");
    let _s:i64 = siblings_input.trim().parse().expect("invalid input for Number of siblings");
    println!("You have {} siblings ",_s);

    println!("Enter your medeical diagnosis");
    let mut diagnosis_input = String::new();
    io::stdin().read_line(&mut diagnosis_input).expect("Failed to read input");
    let _d:String = diagnosis_input.trim().parse().expect("invalid input for Medical diagnosis");
    println!("YOu are diagnosed with {} ",_d);


    println!("Enter viilage of residence");
    let mut village_input = String::new();
    io::stdin().read_line(&mut village_input).expect("Failed to read input");
    let _v:String = village_input.trim().parse().expect("invalid input for village of residence");
    println!("You live in {} ",_v);


   







}