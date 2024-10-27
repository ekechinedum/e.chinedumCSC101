fn main(){
  let p:i64 = 520000000;

  let r:i64 = 10;
  
  let n = 5;
  
  let a:i64 =p*(1+(r/100)).pow(n);
  
  let ci:i64 = a - p;

  println!("Compuond Interest is {} ", ci );


}