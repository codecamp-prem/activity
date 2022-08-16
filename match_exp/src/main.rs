fn main() {
    let some_int = 3;

    match some_int{
        1 => println!("Its 1"),
        2 => println!("Its 2"),
        3 => println!("Its 3"),
        _ => println!("Its something else"),
    }

    let some_boolean = true;
    match some_boolean {
        true => println!("It's true"),
        false => println!("It's false"),
    }
}
