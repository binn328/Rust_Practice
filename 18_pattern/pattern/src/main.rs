// 패턴 매칭
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x)
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

// | 을 이용해 여러 패턴을 매칭시킬 수 있다.
fn multi_pattern() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    // 해당 코드는 one or two를 출력한다.
}

// ..=을 이용해 범위매칭할 수 있다.
// 이제 ... 못씀
fn area_pattern() {
    let x = 5;

    match x {
        1 ..= 5 => println!("one through five"),
        _ => println!("something else"),
    }

    // 1 2 3 4 5 중 하나라면 첫번째 갈래와 매칭된다.

    // char 값의 범위를 이용할 수도 있다.
    let x = 'c';

    match x {
        'a' ..= 'j' => println!("early ASCII letter"),
        'k' ..= 'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

}

struct Point {
    x: i32,
    y: i32,
}
// 구조체를 해체할 수 있다.
fn disasambly_structure() {
    

    let p = Point {x: 0, y: 7 };

    let Point{ x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // x: x 처럼 적는다면 약식으로 적을 수 있다.
    let Point{ x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };

    // Point 구조체를 3가지 경우의 수로 나눈 match문
    match p {
        Point { x, y: 0} => println!("On the x axix at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

// 열거형을 해체할 수도 있다.
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn disasambly_enum() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, 
                y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r, 
                g, 
                b
            )
        }
    }
}

// 패턴과 매칭하려는 값이 참조자를 포함하고 있다면 
// 패턴 내에서 &를 이용해 값으로부터 참조자를 해제해야한다.
// 이렇게하면 참조자를 가지는 변수 대신에, 참조자가 가리키는 값을 갖는 변수를 가져올 수 있다.
// 참조자를 반복하는 반복자에서 클로저를 이용할 때,
// 해당 클로저 내에서 참조자가 아닌 가리키는 값을 사용하길 원할 경우에 유용하다.

fn dereference() {
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points
        .iter()
        // &를 뺀다면 타입 불일치 문제가 발생된다.
        .map(|&Point { x, y} | x * x + y * y)
        .sum();
}

// 구조체와 튜플 해체
// 패턴해체를 복잡하게 섞고, 비교하고, 중첩시킬 수 있다.
// 튜플내에 구조체와 튜플을 가지는, 중첩된 구조에서 본래 값을 얻는 해체를 보여준다.
fn complex_deassamble() {
    let ((feet, inches), Point {x, y}) = 
        ((3, 10), Point { x: 3, y: -10});
}

// 패턴에서 값 무시하기
// _를 사용하거나 _로 시작하는 이름을 사용하거나 ..을 사용하는 방법들이 있다.

// 1. _을 이용해 전체 값 무시하기
// match 문 등에서 사용할 수 있다.

// 2. 함수 시그니쳐에서 _ 사용하기
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn use_underbar() {
    // 첫 인자로 전달된 3이 완벽하게 무시된다.
    foo(3, 4);
}

// 3. 중첩된 _를 이용해 값의 일부를 무시하기
fn overlap_underbar() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match(setting_value, new_setting_value) {
        // 내부값이 필요하지 않은 경우
        // 둘 다 Some인 경우 동작하지 않음
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        // 둘 중 하나가 None인 경우가 표현된다
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

// 4. 튜플의 여러 부분을 무시하기위해 _ 여러개 쓰기
fn ignore_tuple() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // 2와 4번째 요소가 무시된다.
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }
}

// 5. 언더스코어로 시작하는 이름은 안쓰는 변수 오류를 발생시키지 않는다.

fn unused_variable() {
    let _x = 4;
    let y = 9;
}

// 6. ..을 이용해 값의 나머지 부분 무시하기

fn ignore_another() {
    struct Point_six {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point_six { x: 0, y: 0, z: 0 };

    match origin {
        // x 외에 다른 부분은 무시한다.
        // _y, _z 를 쓰는거보다 간단하다!
        Point_six { x, .. } => println!("x is {}", x),
    }
}

// 7. ref와 ref mut를 이용해 패턴 내에서 참조자 생성하기
// ref를 이용하면 패턴 내 변수로 값의 소유권이 이동하지 않도록 할 수 있다
fn ref_mut() {
    let mut robot_name = Some(String::from("Bors"));

    match robot_name {
        // 여기서 robot_name의 소유권이 name으로 넘어간다.
        // Some(&name) 을 사용하면 다른 패턴이 되어버린다.
        // 때문에 ref 키워드를 사용해야한다.
        // 이러면 빌려온다.
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }

    // 값을 변경하려면 ref mut를 쓰면 된다.
    match robot_name {
        Some(ref mut name) => *name = String::from("Another name"),
        None => (),
    }

    // 소유권이 없으므로 여기서 사용할 수 없다.
    println!("robot_name is: {:?}", robot_name);
}

// 8. 매치가드를 이용한 추가조건
// 매치가드는 match가드 뒤에 추가로 붙는 if 조건이다.
fn match_guard() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

// 9. @ 바인딩
// at 연산자인 @은 해당 값이 패턴과 매칭되는지 확인하면서 해당 값을 가지는
// 변수를 생성할 수 있도록 해준다.

fn at_binding() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };
    
    match msg {
        // id가 3 ~ 7 사이의 값인지 확인하면서 캡쳐할 수 있다.
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        // 여기서는 id값이 얼마인지는 알 수 없다.
        // 때문에 사용할 수도 없다.
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        // 여기서는 사용할 수 있다. 
        // id: i32 = id 라고 약어로 쓰였기 때문이다.
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
        // @을 사용하면 값을 테스트하고 변수로 저장하는 일을 한번에 할 수 있다.
    }
}