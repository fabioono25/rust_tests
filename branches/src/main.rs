fn main() {
    let mut a = 6;

    if a < 10 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    loop {
        println!("value is {}", a);
        break;
    }

    while a != 0 {
        println!("value inside while is {}", a);

        a = a - 1;
    }

    let b = [1,2,3,4,5];

    for element in b.iter() { //iterate one element at a time
        println!("Element inside for is {}", element);
    }
}
