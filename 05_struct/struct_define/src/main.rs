// 사용자 계정정보를 저장하는 구조체를 정의
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

fn main() {
    
    
    // 사용하려면 인스턴스화 해주어야한다.
    let user1 = User {
        email: String::from("hello@world.com"),
        username: String::from("name1"),
        active: true,
        sign_in_count: 1,
    };
    
    // .으로 요소에 접근할 수 있으며 변경하려면 mut여야함
    let mut user2 = User {
        email: String::from("HELLO@WORLD.com"),
        username: String::from("NAME2"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("GOODBYE@WORLD.com");

    // 기존에 인스턴스화된 구조체의 값을 이용해 구조체 인스턴스화하기
    // email과 username만 제공하고 나머지는 이미 생성된 user1의 값을 사용
    let user3 = User {
        email: String::from("email3@world.com"),
        username: String::from("name3"),
        ..user1
    };



    // 튜플에 이름을 붙일때도 struct 사용가능
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // 같은 튜플이지만 이름이 다르기에 다른 타입으로 취급된다.
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);


}

// email 과 username 을 넣으면 User를 생성해 반환하는 함수
// 같은 이름을 넣기때문에 헷갈리고 귀찮다.
fn build_user_basic(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// 매개변수와 필드이름이 같으면 알아서 대입해준다!
fn build_user_advenced(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
