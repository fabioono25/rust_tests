fn main() {

    {
        let x = 1;
    }

    //x is unaccessible here!

    let y = 1;

    if y == 1 {
        let a = 10;

        println!("a = {}", a);
    }
    
    println!("a = {}", a);
}
