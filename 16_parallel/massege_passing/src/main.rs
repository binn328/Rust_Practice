// 메시지 패싱을 이용해 스레드간 데이터 전송하기
//
// 메모리를 공유하거나 메시지를 보냄으로써 스레드간 동시성을 유지하며 통신이 가능하다.
//
// 러스트에선 메시지 패싱을 위해 채널 개념을 사용한다.

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // tx는 transmitter, rx는 receiver를 의미한다.
    // mpsc는 복수 생산자, 단수 소비자를 나타낸다. (multiple producer, single consumer)
    // 소비자는 단 하나만 존재할 수 있음을 나타낸다.
    //
    // mpsc::channel()은 송신자, 수신자 튜플을 반환한다.
    let (tx, rx) = mpsc::channel();

    // 스레드를 생성하고 hi 를 보내기
    // send 메소드를 이용해 전송할 수 있으며 Result<T, E>를 반환한다.
    // 때문에 수신자가 이미 drop 된 상태면 에러를 반환할 것이다.
    // 에러가 발생하면 panic! 하도록 unwrap을 사용했다.
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        
        // val을 채널로 내보낸 후 사용을 시도하면 에러가 난다.
        // 다른 스레드에서 사용 중인데 값이 달라져버리면 문제가 발생할 수 있기 때문이다.
        // 소유권 시스템이 잘 동작한다.
        //println!("val is {}", val);
    });

    // 메인 스레드에서 메시지를 수신한다.
    let received = rx.recv().unwrap();
    println!("Got: {}", received);


    example01();
}

// 1초의 텀을 두고 메시지를 연속으로 전송한다.
fn example01() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

