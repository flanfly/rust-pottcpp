fn main() {
    let x;

    {
        let y = 5;
        x = &y;
    }

    println!("x's value is {}", x);
}
