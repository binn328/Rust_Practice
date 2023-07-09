/* 스트링을 pig Latin으로 변경하기
 * 각 단어의 첫번째 자음은 단어의 끝으로 이동하고 "ay"를 붙인다.
 * 모음으로 시작하는 단어는 끝에 "hay"를 붙인다.
 * UTF-8 인코딩을 기억할 것!
*/
use std::io;
fn main() {
    let mut text = String::new();
    io::stdin().read_line(&mut text)
        .expect("단어를 입력하세요");

    let text = text.trim();

    println!("{}", pig_latin(text)); 
}

fn pig_latin(text: &str) -> String {
    let first = text.chars().next().unwrap();
    if first == 'a' || first == 'e' || first == 'i' || first == 'o' || first == 'u' {
        format!("{}{}", text, "hay")
    } else {
        let mut char_vec:Vec<char> = text.chars().collect();
        let removed_char = char_vec.remove(0);
        char_vec.push(removed_char);
        char_vec.push('a');
        char_vec.push('y');

        let mut result = String::new();
        for c in char_vec {
            result.push(c);
        }
        result
    }
}
