fn main(){
    let mut tup: (i32, f64, u8) = (500, 3.5, 1);

    let (x, y, z) = tup; //destructuring part

    let a = tup.0;
    let b = tup.1;
    let c = tup.2;

    println!("X = {}, Y = {} and Z = {}", x, y, z);

    println!("a = {}, b = {} and c = {}", a, b, c);

    tup.0 = 222;
    println!("a = {}, b = {} and c = {}", a, b, c);
    println!("X = {}, Y = {} and Z = {}", x, y, z);
    println!("tup.0 = {}", tup.0);
}
