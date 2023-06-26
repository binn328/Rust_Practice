fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    
    println!("The value of y is : {}", y);

    let z = five();
    println!("The value of z is {}", z);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x, y is : {}, {}", x, y);
}

fn five() -> i32 {
    5
}
