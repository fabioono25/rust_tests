fn main(){
    //let mut a = 2147483648; //out of range for i32
    //let mut a = 2147483647;  // attempt to add with overflow 
    
    let mut a:i64 = 2147483647;
    println!("{}", a);

    a = a + 1;
    println!("A = {}", a); //2147483648
}