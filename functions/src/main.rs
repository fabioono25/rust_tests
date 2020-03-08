fn main(){
    println!("Hello World!");

    another_function();
    another_function2(2);
    
    let a = sum(1, 3);
    println!("Sum = {}", a);
}

fn another_function(){
    println!("Another function");
}

fn another_function2(x:i32){
    println!("The value of x = {}", x);
}

fn sum(x:i32, y:i32) -> i32 {
    let b = x + y;

    b * 100

    //you can add the value into a variable and use return variable;
}

