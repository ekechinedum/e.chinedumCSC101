fn main() {
let v = vec![20,40,60,80];
// vector v owns the object inheap

let v2 = v;
letv2_return = display(v2);
println!("In main {:>}",v);

}
fn display(v:Vec<i32>)->Vec<i32> {
    // returning same vector
    println!("inside display {:?}",v);
    return v;
}