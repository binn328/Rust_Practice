/* 정수 리스트가 주어졌을 때 
 * 벡터를 이용하여 리스트의 평균값, 중앙값, 최빈값을 반환해라
*/

use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut vec: Vec<i32> = Vec::new();
    for _i in 0..100 {
        vec.push(rand::thread_rng().gen_range(0..50));
    }

    println!("before sorting");
    println!("{:?}", vec);
    
    vec.sort();
    println!("after sorting");
    println!("{:?}", vec);

    println!("average : {:?}, median : {:?}, most_frequently_number : {:?}", 
        get_average(&vec), get_median(&vec), get_most_count(&vec));
    
}

fn get_average(integer_list: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for number in integer_list {
        sum += number;
    }

    let sum = sum as f64;
    let len = integer_list.len() as f64;
    sum / len
}

fn get_median(integer_list: &Vec<i32>) -> i32 {
    let index = integer_list.len() / 2;
    match integer_list.get(index){
        Some(&n) => n,
        None => {
            println!("없는 값입니다.");
            0
        }
    }
}

fn get_most_count(integer_list: &Vec<i32>) -> (i32, i32) {
    let mut counter = HashMap::new();
    for number in integer_list {
        let count = counter.entry(number).or_insert(0);
        *count += 1;
    }

    let mut most_number: (i32, i32) = (0, 0);
    for (number, count) in counter {
        if most_number.1 < count {
            most_number = (*number, count);
        } else {}
    }

    most_number
}
