use std::io;

fn main() {
    let mut input = String::new();
    println!("변환할 단위를 선택하세요");
    println!("1. C' -> F' / 2. F' -> C");
    
    // 변환할 단위를 선택하기
    io::stdin().read_line(&mut input)
        .expect("readline failed");

    let input: u32 = input.trim().parse()
        .expect("Please type a number!");

    //선택한 것에 따라 동작을 수행하기
    let mut temperature = String::new();
    print!(">>");

    //1이면 c -> f
    if input == 1 {
        io::stdin().read_line(&mut temperature)
            .expect("readline failed");

        let temperature: f64 = temperature.trim().parse()
            .expect("Please type a number!");

        println!("{}C' -> {}F'", temperature, c_to_f(temperature));
    } else if input == 2 {
        //2면 f -> c
        io::stdin().read_line(&mut temperature)
            .expect("readline failed");

        let temperature: f64 = temperature.trim().parse()
            .expect("Please type a number!");

        println!("{}F' -> {}C'", temperature, f_to_c(temperature)); 
    } else {
        println!("type valid number");
    }
    
}

fn c_to_f(temperature: f64) -> f64 {
    temperature * (9.0 / 5.0) + 32.0
}

fn f_to_c(temperature: f64) -> f64 {
    (temperature - 32.0) * (5.00 / 9.0)
}
