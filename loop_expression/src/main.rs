fn main() {
    loop {
        println!("Hello");
        break;
    }

    // demo 2
    let mut i = 3;
    loop {
        println!("{:?}", i);
        i = i - 1;
        if i == 0 {
            break;
        }
    }

    // demo 3
    let mut j = 1;
    loop {
        println!("{:?}", j);
        if j == 4{
            break;
        }
        j = j + 1;
    }
}
