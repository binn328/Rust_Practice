// 스레드를 이용하여 코드를 동시에 실행하기
// 
// 스레드를 사용하면 여러일을 동시에 처리할 수 있지만
// 레이스 컨디션, 데드락 등의 여러 수정하기 힘든 버그들을 야기한다.
//
// 프로그래밍 언어들이 제공하는 스레드를 그린 스레드라고 한다.
// 이 그린 스레드는 운영체제 쓰레드와 M:N 으로 대응된다.
//
// 하지만 러스트는 1:1 스레드 구현만을 제공한다.
// 오버헤드를 댓가로 M:N 스레드를 사용하겠다면 크레이트가 존재하니 그걸 쓰면 된다.

// spawn으로 새로운 스레드를 생성할 수 있다.

use std::thread;
use std::time::Duration;

fn main() {
    // 스레드는 spawn 함수를 호출하고 실행할 코드가 담긴 클로저를 넘긴다.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // 여기에 추가하면 이 스레드가 끝난 후 main 스레드가 실행된다.
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // join을 해주지 않으면 main 스레드가 끝날 때 다른 스레드도 강제 종료된다.
    // join을 해주면 다른 스레드가 끝날 때 까지 해당 스레드를 블락한다.
    // 이를 위해선 클로저를 변수로써 선언해주어야한다.
    
    handle.join().unwrap();

    move_keyword();
}

// move 키워드를 사용하기
//
// 클로저는 v를 캡쳐해 사용할 수가 있다.
// 하지만 스레드가 얼마동안 살아있을지 모르므로, 컴파일러가 에러를 발생시킨다.
//
fn move_keyword() {
    let v = vec![1, 2, 3];

    // move 키워드를 사용하면 클로저가 사용하는 값을 강제로 소유하게 한다.

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // 여기서 드랍시키면 오류가 생길 것이다.
    // drop(v);

    handle.join().unwrap();
}

