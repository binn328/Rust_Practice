use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

// 유닛타입 ()을 반환, 
// 에러타입은 특성 오브젝트 Box를 사용
// 지금은 Error 특성을 구현하는 타입을 반환한다고만 알고있자.
// 17장 가서 배움

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// 튜플로 반환하는 것 보다는 구조체로 반환하는 것이 더 좋다.
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // 오류가 나면 문자열 리터럴을 반환한다.
    // 잘 동작하면 Config 객체를 반환한다.
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name")
        };

        // 환경변수를 읽어와 사용하기
        // 설정되어있다면 false, 설정이 안된상태면 true를 반환
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
/* 이 함수는 단순히 Config 객체를 반환한다.
* 때문에 이럴거면 그냥 Config의 new 함수로 만들어버리는게 효율적이다.
fn parse_config(args: &[String]) -> Config {
    // clone 메소드는 조금 비효율적이지만 간단하다.
    // 지금은 약간의 비용만 드므로 그냥 이거 쓰자
    // 나중에 효율적인 방법을 배워보자

    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
*/

// 실패 테스트 작성하기
//
// 반환된 벡터가 contents의 문자열을 참조한다.
// 이 벡터는 contents만큼 오래 생존할 것이다.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
// TDD 절차
// 1. 실패할 테스트를 작성하고, 의도한대로 실패하는지 실행
// 2. 해당 테스트를 통과할 수 있도록 코드를 작성
// 3. 코드를 다듬으며 통과할 수 있도록 수정
// 4. 새 기능 작성 시 1번으로 복귀

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

// 클로저와 반복자는 존나 빠르다
