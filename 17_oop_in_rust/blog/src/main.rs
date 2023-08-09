// 객체 지향 디자인 패턴 구현하기
//
// state_pattern은 객체 지향 디자인 패턴이다.
// 어떤 값이 상태 객체들의 집합으로 표현되는 일종의 내부상태를 가지며,
// 이 값의 동작은 내부 상테에 기반하여 바뀐다는 것이다.
//
// 1. 블로그 게시물은 빈 초안으로 시작
// 2. 초안이 완료되면 게시물 검토가 요청
// 3. 게시물이 승인되면 게시
// 4. 게시된 게시물만이 내용물을 반환하므로, 승인거부된 게시물은 게시될 수 없다.
extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

