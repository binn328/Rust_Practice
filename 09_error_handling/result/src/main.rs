// Result로 복구 가능한 에러 만들기
//
// enum Result는 OK와 Err을 가진다.
use std::io;
use std::fs::File;
use std::io::Read;
use std::io::ErrorKind;
fn main() {
    // Result를 반환하도록 해보자
    // 반환타입을 알고 싶다면 아무 타입이나 명시하면 컴파일러가 알려줌
    // let f: u32 = File::open("hello.txt");
    // Result<std::fs::File, std::io::Error> 가 반환된다.

    /* 
    // match를 통해 처리하기
    let f = File::open("hello.txt");

    // Result 와 variants 는 prelude에서 가져오므로 앞에 안붙여도됨
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problen opening the file: {:?}", error)
        },
    };
    */

    // 다른 형식의 에러를 다른 방식으로 처리하기
    
    let f = File::open("hello.txt");

    let f = match f {
        // Ok면 파일을 반환
        Ok(file) => file,
        // NotFound 에러면 파일 생성을 시도
        // if 문을 통해 Match문 갈래를 더 안전하게 실행할 수 있다.
        // 이를 매치 가드라고 부른다.
        // ref를 사용해 if문으로 소유권이 넘어가지 않는다.
        // &대신 사용하는 이유는 나중에 알아보자
        // &는 참조자를 매치하고 값을 제공하고, 
        // ref는 값을 매치하여 그 참조자를 제공한다.
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                // 파일이 생성에 성공했으면 그 파일을 반환
                Ok(fc) => fc,
                // 실패했다면 오류를 반환
                Err(e) => {
                    panic!(
                        "Tried to create file, but there was a problen: {:?}", 
                        e
                    )
                },
            }
        },
        // 다른 오류라면 오류를 반환
        Err(error) => {
            panic!(
            "There was a problen opening the file: {:?}",
                error
            )
        },
    };

    // match는 잘 동작하지만 꽤 장황하다.
    // 숏컷으로 unwrap과 expect를 사용가능하다.

    // unwrap은 Ok면 그 값을, Err이면 패닉을 일으킨다.
    let f = File::open("hello.txt").unwrap();
    
    // expect는 에러메시지를 선택할 수 있다.
    // 에러의 발생지점을 알기가 좋다.
    let f = File::open("hello.txt").expect("Error Message");


    
    
}

// 함수가 문제없이 실행되면 String을, 실패하면 io::Error를 반환한다.
//
fn read_username_from_file() -> Result<String, io::Error> {
    // 해당 함수에서 에러를 처리하는 대신, 처리할 함수를 호출해서 에어를 반환할 수도 있다.
    // 파일을 연다.
    let f = File::open("hello.txt");
    
    // 결과를 f에 저장하고 match를 시작
    let mut f = match f {
        // 성공이면 file을 반환하고 계속
        Ok(file) => file,
        // 실패면 함수를 끝내고 에러를 반환
        Err(e) => return Err(e),
    };

    // 새로운 스트링을 생성
    let mut s = String::new();

    // 파일의 콘텐츠를 읽어 s에 삽입
    match f.read_to_string(&mut s) {
        // 성공 시 읽어들인 이름을 OK(s)로 싸서 반환하고 함수종료
        Ok(_) => Ok(s),
        // 실패 시 에러값을 반환
        // 마지막 표현식이기때문에 return은 필요없다.
        Err(e) => Err(e),
    }
}

// 위의 패턴은 꽤 흔하기때문에 약어 ?를 지원한다.
fn read_username_from_file_advenced() -> Result<String, io::Error> {
    // 성공하면 그 결과를, 실패하면 return Err 한다.
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    // 이 역시 마찬가지다.
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 더 간단하게 할 수도 있다.
fn read_username_from_file_advenced_advenced() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
// ?는 Result를 반환하는 함수에서만 사용할 수 있다.
