use std::io;
mod my_first_module;

fn main() {
    let mut input: String = String::new();

    println!("Type something here...    ");
    io::stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");


    println!("This is your first input value: {}", input);

    my_first_module::run();
}
