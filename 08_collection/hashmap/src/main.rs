// hashmap
// hashmap은 use문으로 가져와야한다.
use std::collections::HashMap;

fn main() {
    // 간단한 해시맵의 생성
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 튜플의 벡터에 대해 collect 메소드를 이용해 생성할 수도 있다.
    // collect 메소드는 데이터를 모아서 여러 컬렉션타입으로 만들어준다.

    // 각 팀 이름과 점수에 대한 초기 데이터가 있다면
    // 이를 zip 메소드를 이용해 한쌍의 튜플을 만드는 것이 가능하다.
    // 그런 다음 collect 메소드를 이용해 튜플의 벡터를 hashmap으로 바꿀 수 있다.
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // 이때는 타입을 명시해줄 필요가 있다.
    // 명시하지않으면 collect는 무엇으로 만들어줘야할지 모른다.
    // <_, _> 을 통해 벡터로부터 타입을 추론하는것이 가능하다.
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();


    // i32처럼 Copy 트레잇이 구현되어있다면, 그 값들은 해시맵안으로 복사된다.
    // String처럼 소유자가 있는 값에 대해서는 해시맵이 소유자로 변한다.
    
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    // 지금부터 field_name과 field_value는 유효하지않다.
    println!("{}, {}", field_name, field_value);
    // & 연산자를 사용할 수도 있지만, 
    // 이 경우엔 원본값이 해시맵이 살아있는동안 유효할 필요가 있다.



    // 해시맵 값에 접근하기
    let team_name = String::from("Blue");
    // get은 Option<&V>를 반환한다.
    // 따라서 존재하는 값은 Some(&10)으로 반환될 것이다.
    // 이후 score를 사용하려면 match를 이용해 처리해줄 필요가 있다.
    let score = scores.get(&team_name);

    // for를 이용해 반복작업도 가능하다.
    for(key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 해시맵 갱신하기
    // 해시맵에서 하나의 키는 하나의 값만 가질 수 있다.
    // 따라서 이 새로운 값을 덮어씌울지, 있다면 삽입하지 않을지를 정해야한다.
    // 기본적으로 insert를 통해 삽입할 시 덮어씌운다.

    // 없을때만 삽입하고 싶다면 entry라는 API를 사용해야한다.
    // or_insert 메소드는 해당 키가 존재할 경우 관련된 Entry키에 대한
    // 값을 반환하도록 정의되어있다.
    // 반대의 경우엔 주어진 값을 삽입한 후 수정된 Entry키를 반환한다.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // Blue는 이미 값이 존재하기에 50으로 덮어씌어지지 않는다.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // 예전 값을 기초로 값을 갱신하기
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // or_insert() 는 키에 대한 값의 가변 참조자를 반환한다.
    // 때문에 이를 받은 count를 역참조한 값에 +1을 한다.
    // for가 끝나면 count는 scope를 벗어나 사라지며 
    // 덕분에 수정해도 안전하다.
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    

    // HashMap은 해시 함수를 사용한다.
    // 만약 다른 해시 함수를 사용하고 싶다면,
    // 특정 Hasher로 변경하는 것이 가능하다.
    // Hasher는 BuildHasher 트레잇을 구현한 타입을 말한다.
    // crates.io에서 이미 만들어진 알고리즘들을 지원하는 라이브러리를 제공한다.

}

