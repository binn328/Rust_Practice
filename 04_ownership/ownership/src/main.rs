fn main() {
    // 1. 러스트에서 각 값은 owner라는 변수를 가진다.
    // 2. owner는 한 번에 하나만 존재할 수 있다.
    // 3. owner가 scope를 벗어날 때, 그 값은 버려진다(dropped)

    let s = "hello";    //s가 선언되어 유효해졌다.

    // ::은 String 타입 아래의 from() 함수를 사용할 수 있게 해줌
    let s = String::from("hello");
    
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    
    // s라는 변수가 scope를 벗어나면 러스트는 drop()을 호출한다.
    // 보통 }를 만나면 호출된다.

    // 변수와 데이터가 상호작용하는 방법 : move
    
    // 단순 정수값은 스택에서 논다.
    // 5가 복사되어 y에 들어간다.
    let x = 5;
    let y = x;

    // 힙에서는 조금 다르다.
    let s1 = String::from("hello");
    let s2 = s1;
    // s1과 s2가 같은 주소를 가리킨다.
    // scope를 벗어나면 2번 drop() 되면서 이중 메모리 해제가 발생하게 된다.
    // 때문에 러스트에서는 s1이 이후 유효하지 않다고 간주한다.
    // 아래의 명령어는 오류를 발생시킬 것이다.
    //println!("{}", s1);
    
    // 만약 정말 깊은 복사를 하고 싶다면 clone() 을 이용해야한다.
    let s3 = String::from("hi");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);

    // 스칼라값은 그냥 복사해도 잘 동작한다.
}
