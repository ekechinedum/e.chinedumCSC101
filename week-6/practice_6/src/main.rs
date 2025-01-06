fn main(){
    let n1 = "Electrical".to_string();
    let n2 = "Electronic".to_string();
    let n3 = "Engineering".to_string();
    let n4 = n1 + &n2 + &n3; // n2 & n3 referenec is passed

    // About Electrical/Electronic
    println!("\nThe {} is informed by th aspiration to train electrical/electronic proffesionals in the area of design, building and maintenance of electrical control systems,", n4);

    let w1 = "Computer".to_string();
    let w2 = "Science".to_String();
    let w3 = w1 + &w2; //w2 reference is passed
    println!();
    println!("{} is aimed at developing competent, creative,
    innovative, entrepreneurial and ethically-minded persons,
    capable of creating valur in the diverse fields of 
    computer Science, ",w3);


}