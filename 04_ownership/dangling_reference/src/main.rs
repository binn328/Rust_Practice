fn main() {
    let reference_to_nothing = no_dangle();

}
/*
// 아무것도 가리키지 않는 포인터를 만드는 함수
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
*/
// 해결하려면 그냥 String을 반환하면 된다.
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
