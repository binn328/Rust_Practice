fn main() {
    // vector는 제네릭을 통해 구현되었다.
    // 따라서 생성 시, 타입을 명시해줄 필요가 있다.
    let v: Vec<i32> = Vec::new();

    // 타입 추론을 지원하기 때문에 초기값을 가진 vector를 생성한다면
    // 타입을 명시하지 않아도 컴파일러가 타입을 추론한다.
    // 또한 초기값을 가지는 vector를 만드는 vec![] 매크로를 제공한다.
    let v1 = vec![1, 2, 3];

    // push로 값을 삽입할 수 있다.
    // 이때는 삽입하는 값으로 타입 추론이 가능하다.
    // 따라서 타입을 명시해주지않아도 된다.
    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    // vector가 scope를 벗아난다면 내용물도 모두 드랍된다.
    // vector 각 요소에 대한 참조를 만들 때 복잡할 수 있다.

    // vector의 값을 읽어오기
    let v3 = vec![1, 2, 3, 4, 5];

    // 인덱스 값으로 가져오기
    let third: &i32 = &v[2];
    // 메소드를 이용해 가져오기
    let third: Option<&i32> = v.get(2);

    // 존재하지 않는 값에 접근한다면?
    // 얘는 panic! 을 일으킨다.
    // 다른 값이 들어왔을 때 프로그램을 종료시켜야한다면 사용할 수 있다.
    let dose_not_exist = &v[100];

    // 얘는 None을 반환한다.
    // 이 경우엔 값을 사용하기 전에 None을 처리해줄 필요가 있다.
    // 사용자 친화적이다.
    let does_not_exist = v.get(100);


    // & 빌림자를 사용하여 vector의 내용물을 참조해도 
    // 계속 유효하도록 해준다.
    
    // 하지만 참조자가 존재하는 상황에 원본 vector에 push를 시도하면 
    // 오류가 발생한다.
    // vector에 삽입하는 과정에서 vector의 주소가 변경되는 일이 발생가능하다.
    // 때문에 컴파일러는 오류를 발생시킨다.
    // 주소가 변경되면 first는 유효하지 않은 주소를 가리킬 것이기 때문이다.
    let first = &v3[0];
    v3.push(6);

    // vector 요소 가져와 반복하기
    let v4 = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // vector 요소 변경시키기
    for i in &mut v {
        // 가변 참조자가 가리키는 값을 변경시키기 위해서는
        // * 을 이용해 역참조를 통해 값을 얻어와야한다.
        // C의 포인터와 비슷하다.
        *i += 50;
    }
    
    // enum을 통해 여러 타입 저장하기
    // 기본적으로 vector는 하나의 타입만을 저장할 수 있다.
    // enum을 이용해 가능하다!
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    // 어떤 타입이 들어올 지 알고 있다면 enum을 이용할 수 있다.
    // 그렇지 않다면 trait object를 사용할 수 있다.
    // 17장에서 배움
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];






}
