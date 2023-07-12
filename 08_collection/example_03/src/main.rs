// 해시맵과 벡터를 이용하여
// 사용자가 회사 내 부서에 대한 피고용인 이름을 추가할 수 있도록 하는
// 텍스트 인터페이스를 만들어보기
// 입력1 : Add Sally to Engineering
// 입력2 : Add Amir to Sales
// 그리고 사용자가 각 부서의 모든 사람들에 대한 리스트나
// 알파벳 순으로 정렬된 부서별 모든 사람들 리스트를 조회할 수 있도록 하기

//hashmap<부서, Vec<사람>> 으로 만들자!

use std::collections::HashMap;
use std::io::stdin;
use std::fmt::Write;
fn main() {
    run();

    
    
}

fn run() {
    // 입력 버퍼
    let mut input_buffer = String::new();

   
    // 정보가 저장될 해시맵, HashMap<부서이름, Vec<사람목록>> 이다.
    let mut database:HashMap<String, Vec<String>> = HashMap::new();


    // 1.   명령어를 입력한다.
    // 1.1. 명령어가 Add로 시작한다면
    //      to 이전까지의 단어를 가져와서 name 에 삽입
    //      to 이후 단어를 가져와서 department 에 삽입
    //      database.get(department)를 한다.
    //      있다면 가져온 vec에 name을 삽입
    //      없다면 새로 생성하고 name을 삽입
    //
    // 1.2. 명령어가 List로 시작한다면
    //      of 이후의 단어를 가져와 department에 삽입
    //      database.get(department)를 해서 알파벳순으로 sort한다.
    //      그리고 그걸 리스트로 출력한다.
    //
    //      만약 of 이후 단어가 All이면
    //      database 내의 모든 department를 가져온다.
    //      알파벳순으로 각 department의 모든 사람의 이름을 출력
    loop {
        println!("명령어를 입력하세요 \nex1. Add Sally to Engineering");
        println!("ex2. List of Engineering");
        println!("ex3. List of All");
        println!("종료는 Quit 입니다.");
        input_buffer.clear();
        stdin().read_line(&mut input_buffer)
            .expect("명령어 입력에 실패");
        
        // 입력을 공백을 기준으로 쪼갠다.
        let mut input = input_buffer.split_ascii_whitespace();
        
        
        
        match input.next().expect("잘못된 명령어입니다.") {
            "Add" => add_to_database(&mut input, &mut database),
            "List" => {
                for value in database.values_mut() {
                    value.sort();
                }
                list_of_database(&mut input, &mut database);
            },
            "Quit" => {
                println!("프로그램을 종료합니다.");
                break;
            }
            _ => println!("잘못된 명령어입니다."),
        }
    }

}

fn add_to_database(input: &mut std::str::SplitAsciiWhitespace, database: &mut HashMap<String, Vec<String>>) {
    let name = String::from(input.next()
        .expect("이름을 읽어오는데 실패했습니다.").trim());
    input.next()
        .expect("to를 제거하는데 실패했습니다.");
    let department = String::from(input.next()
        .expect("부서를 읽어오는데 실패했습니다.").trim());

    // 이미 존재하면 
    if database.contains_key(&department) {
        database.get_mut(&department)
            .expect("직원 목록을 가져오는데 실패했습니다.").push(String::from(&name));
        
    } else {
        database.insert(String::from(&department), Vec::new());
        database.get_mut(&department)
            .expect("직원 목록을 가져오는데 실패했습니다.").push(String::from(&name));
    }
    println!("{}을/를 {}에 추가했습니다.\n", name, department);
}

fn list_of_database(input: &mut std::str::SplitAsciiWhitespace, database: &mut HashMap<String, Vec<String>>) {
    input.next().unwrap();
    let department = input.next().expect("잘못된 명령어입니다.");
    match department {
        "All" => print_all(database),
        _ => {
            let mut output_buffer = String::new();
            if database.contains_key(department) {
                writeln!(output_buffer, "[{department}]").unwrap();
                for name in database.get(department).unwrap() {
                    writeln!(output_buffer, "{name}").unwrap();
                }
                println!("{output_buffer}");
            } else {
                println!("해당 부서를 찾을 수 없습니다.");
            }
        },
    }

}

fn print_all(database: &HashMap<String, Vec<String>>) {
    // key 리스트 변수 생성
    let mut key_list = Vec::new();
    // key 리스트를 생성
    for key in database.keys() {
        key_list.push(key);
    }
    // 정렬시킴
    key_list.sort();

    // 출력 버퍼
    let mut output_buffer = String::new();

    // 부서 내의 사원을 부서의 알파벳 순으로 버퍼에 저장
    for key in key_list {
        writeln!(output_buffer, "[{key}]").unwrap();
        for name in database.get(key).unwrap() {
            writeln!(output_buffer, "{name}").unwrap();
        }
    }
    println!("{output_buffer}");
}

