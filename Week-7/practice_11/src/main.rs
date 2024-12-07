fn main() {

    // an array of numbers 
    let numbers = [1,2,3,4,5];
    printlm!("Original array = {:?}", numbers);

    // create a slice of 2nd and 3rd element
    let slice1 = &numbers[1..3];
    println!("2nd and 3rd elements sliced = {:?}", slice1);

    //omit the start index 
    ket slice2 = &numbers[..3];
    //This means the slice starts from index 2 and goes up to 5 (exclusive)
    println!("index 0 to index 3 sliced = {:?}",slice3);

    //omit the start index and the end index 
    // reference the whole array
    let slice4 = &numbers[..];
    // Thus means the slice starts from index 0mand goes up to index 5 (exclusive)
    println!("index 0 to index 5 sliced = {:?}",slice4);
}
