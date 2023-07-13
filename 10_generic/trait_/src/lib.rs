// 트레잇으로 공유동작을 정의하기
//
// 트레잇은 추상 메소드를 정의할 수 있다.
// 다른 언어에서 인터페이스라고 불린다.

// Summarizable 트레잇을 선언했다.
// 이 트레잇을 사용하려면 summary() 메소드를 구현해야한다.
// 여러개의 메소드를 가질 수 있다.
// 기본 동작을 구현하는 것도 가능하다.
pub trait Summarizable {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn author_summary(&self) -> String {
        format!("@{}", self.author)
    }
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
/* 만약 기본 정의를 사용하고자 한다면 빈 impl 블록을 명시하면 된다.
* impl Summarizable for NewsArticle {}
*   
*/

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
