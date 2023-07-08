// 여기에 다 선언하면 보기가 힘들어진다.
// 그냥 ;을 붙여두면 선언이 client.rs 에 있다는 것을 의미한다.
// 즉 다른 파일로 분리할 수 있다.
// pub 키워드로 모듈을 public으로 만들어줄 수 있다.
// 기본적으로 모든 코드들은 private이다.
pub mod client;

pub mod network;



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    // 이 scope 안에는 client가 없다.
    // 따라서 포함시켜주어야한다.
    // :: 를 통해 root 경로에서부터 찾아간다고 알려줄 수 있다.
    // ::client::connect();
    // 혹은 super 키워드를 통해 한 계층을 거슬러 올라갈 수도 있다.
    use super::client;
    // tests 모듈 내에서는 보통 use super:: 뭐시기가 좋은 선택이 된다.

    #[test]
    fn it_works() {
       client::connect(); 
    }
}
