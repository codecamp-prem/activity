fn main() {
    let result = add(49, 98);
    display_result(result);
}

fn display_result(result:i32){
    println!("{:?}",result);
}

fn add(a:i32, b:i32) -> i32 {
    a + b
}