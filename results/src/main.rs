use std::fs::File;

fn main() {
    let f = File::open("hello2.txt").expect("file was not found");

    // let foo = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("file not found");
    //     }
    // };
}
