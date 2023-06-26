fn main() {
    let number = 3;

    if number % 5 == 0 {
        println!("number is divisible by 5");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("condition was false");
    }


    let condition = true;
    let number = if condition{
        5
    } else {
        6
    };

    println!("The value of number is : {}", number);


    // 반복문
    loop {
        println!("hello");
        break;
    }

    //while
    let mut number = 3;
    while number != 0 {
        println!("{}", number);

        number = number - 1;
    }

    println!("LiftOff!");

    //while colection
    let a = [10, 20, 30, 40, 50];
        let mut index = 0;

    while index < 5 {
        println!("the value is : {}", a[index]);

        index = index + 1;
    }

    //for each
    for element in a.iter(){
        println!("the value is : {}", element);
    }

    //for
    for n in (1..4).rev() {
        println!("{}!", n);
    }
    println!("LiftOff!");
}
