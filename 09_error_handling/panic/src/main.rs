fn main() {
    //panic! 매크로를 실행하면 실패메시지를 출력하고 청소 후 종료된다.
    // 청소과정은 unwinding을 시작한다. 근데 이건 일이 많다.
    // 그냥 abort 하고 싶다면 Cargo.toml 안의 적합한 [profile]섹션에
    // panic = 'abort' 를 추가해주면 된다.
    // ex. 
    // [profile.release]
    // panic = 'abort'

    // 직접 호출하기
    // panic!("crash and burn");
    
    // 라이브러리로 부터 발생시키기
    let v = vec![1, 2, 3];

    // out of index 패닉을 발생시킨다.
    // RUST_BACKTRACE=1 cargo run 을 하면 BackTrace가 활성화된다.
    // BackTrace 는 위에서부터 작성한 코드가 보일때까지 읽어나가면된다.
    // 그곳이 바로 오류가 발생한 곳이다.
    v[99];
    

    
}
