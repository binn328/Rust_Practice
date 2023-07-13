extern crate trait_;

use trait_::*;


fn main() {
    let tweet = trait_::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably alreay know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());
}

// 트레잇 바운드
// 함수를 호출하기 위해서는 T에 Summarizable 을 구현해야한다.
pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}

// +를 이용하면 여러개의 트레잇 바운드를 설정할 수 있다
// T는 Display와 Clone을, U는 Clone과 Debug를 구현해야한다.
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {

}

// where 를 이용해서 다음 줄에 적을 수도 있다.
fn some_function_visible<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
            U: Clone + Debug 
{

}
