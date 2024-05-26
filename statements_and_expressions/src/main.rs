fn main() {
    let x = (let y = 6);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
