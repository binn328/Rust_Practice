// 고급 함수와 클로저

fn main() {
    
}

// 함수 포인터
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn function_pointer() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

// 클로저 이용하기
fn using_closure() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();
}
// 컴파일되면 어차피 같은 코드다.
// 마음에 드는거로 사용하자

// 클로저 반환하기
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}