fn main() {
    let mut done = false;
    let mut x = 5;

    while !done{
        x += x - 3;
        println!("{}", x );

        if x % 5 == 0 {
            done = true;
        }
    }
    iterate();
}

fn iterate() {
    let mut v = vec![1,2,3];

    for i in &v {
        println!("a reference to {}", i);
    }
}
