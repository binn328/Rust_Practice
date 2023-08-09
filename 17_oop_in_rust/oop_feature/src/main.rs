extern crate oop_feature;
use oop_feature::{Draw, Screen, Button};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 대충 선택박스 그리는 코드 
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            /* 
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Ok"),
            }),
            */
        ],
    };

    screen.run();
}
