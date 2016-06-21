use std::env;

fn main() {
    //initialize what to an Option<String>
    let what = env::args().next();

    //Check if what is empty or not and do something
    match what {
        Some(string) => println!("Hello, {}", string),
        None => panic!("Nothing to print passed!")
    }
}
