// 러스트의 고오급 기능들
// unsafe 러스트
// unsafe 키워드를 이용해 4가지 동작을 할 수 있다.
/*  1. 로우 포인터를 역참조하기
    2. 안전하지 않은 함수나 메소드 호출하기
    3. 가변 정적 변수의 접근이나 수정
    4. 안전하지 않은 트레잇 구현하기
*/

// unsafe를 사용한다면 그 블럭을 적게 유지하는 것이 후에 편하다.
// 메모리 버그가 발생했을 때 빠르게 디버깅할 수 있다.
// unsafe한 코드를 격리하기 위해서는 안전한 추상화내에
// 있도록 하는 것이 좋다.

// 로우 포인터를 역참조하기
// 로우 포인터도 참조자처럼 불변 혹은 가변이 될 수 있다.
// *const T와 *mut T 라고 쓸 수 있다.
// *이 붙었다고 역참조가 아니다. 
// 그저 타입 이름의 일부일 뿐이다.
// - 로우 포인터는 빌림규칙을 무시할 수 있다.
// - 로우포인터는 유효하지 않은 메모리를 가리킬 수도 있다.
// - 로우 포인터는 Null이 될 수 있다.
// - 로우 포인터는 자동 메모리 정리가 구현되어있지않다.
// 이렇게 생성할 수 있다.
fn low_pointer() {
    // 로우 포인터 자체는 unsafe 밖에서도 생성할 수 있다.
    // 그저 역참조를 못할 뿐이다.
    let mut num = 5;
    // as 를 통해 로우포인터로 캐스팅한다.
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    // 해당 코드의 로우포인터는 유효하다.
}

// 로우 포인터를 생성할 수는 있지만, 가리키는 데이터는 읽을 수 없다.
fn invaild_low_pointer() {
    let address = 0x012345usize;
    let r = address as *const i32;
}

fn unsafe_low_pointer() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 생성은 상관이 없으나, 읽는 것은 문제가 될 수 있다.
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    // C 코드와 상호자용할 때 사용하거나, 
    // 빌림검사기가 이해할 수 없는 안전한 추상화를 만들 때 사용한다.
}


// 안전하지 않은 함수나 메소드 호출하기
unsafe fn dangerous() {}

fn call_dangerous() {
    // 반드시 unsafe 블럭 안에서 호출해야한다.
    unsafe {
        dangerous();
    }
}

// 안전하지 않은 코드에서 안전한 추상화 생성하기
// 표준 함수의 split_at_mut를 공부해보자
fn how_to_use_split_at_mut() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

use std::slice;
// 안전한 러스트만으로 이 함수를 구현하는 것은 불가능하다.
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // 우리는 전혀 다른 부분의 슬라이스를 빌리고 있음을 알 수 있다.
    // 하지만 러스트는 슬라이스 두개를 빌린다는 사실만 알고있다.
    // 코드가 겹치지 않으므로 문제가 없지만, 러스트는 그만큼 똑똑하지는 않다.
    // 따라서 이런상황에는 unsafe를 사용할 수 있다.
    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
    // 이렇게 구현하면 내부는 안전하지 않지만, 외부에선 안전한 함수처럼 사용할수있다.
}

// extern 함수를 이용해 외부 코드 호출하기
fn extern_c() {
    // C에 라이브러리의 abs 함수를 가져온다.
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// 아래 문법을 사용하면 다른 언어에서 러스트의 함수를 호출하는게 가능하다.
// C에서 call_from_c() 함수를 호출하는게 가능하다!
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}


// 가변 정적 변수의 접근 or 수정
// 러스트에서 전역변수는 static 변수라고 불린다.
static HELLO_WORLD: &str = "Hello, world!";

fn static_variable() {
    println!("name is : {}", HELLO_WORLD);
}

// 정적 변수는 가변일 수 있다.
// 하지만 이 경우에는 안전하지 않을 수 있다.
// 따라서 이 변수에 접근하고 수정하는 것은 unsafe이다.
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn static_count() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// 안전하지 않은 트레잇 구현하기
unsafe trait Foo {

}

unsafe impl Foo for i32 {

}
fn main() {
    
}
