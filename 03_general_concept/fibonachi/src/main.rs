use std::io;

fn main() {
    println!("n을 입력하세요");
    
    let mut n = String::new();
    io::stdin().read_line(&mut n)
        .expect("read_line failed");
    
    let n: u32 = n.trim().parse()
        .expect("숫자를 입력하세요.");

    println!("{}번째 피보나치 수열은 {}입니다.", n, fibo(n));
}

fn fibo(n: u32) -> u32 {
    if n == 1 {
        1
    } else if n == 2 {
        2
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}
