// 라이프타임 subtyping : 한 라이프타임이 다른 라이프타임보다 오래 사는 것을 보장하기
// 라이프타임 바운드 : 제네릭 타입을 가리키는 참조자를 위한 라이프타임 명시
// 트레잇 객체 라이프타임 추론 : 컴파일러는 어떻게 트레잇 객체의 라이프타임을 추론하며
//  언제 이들을 명시할 필요가 있는지에 대하여

// 라이프타임 subtyping
// 우리가 Parser를 작성한다고 해보자.
// 파싱 중인 String에 대한 참조자를 가지는 Context 구조체를 사용한다.
// 이 String을 파싱하고 성공여부를 반환하는 Parser를 작성하자.
// Parser는 파싱을 위해 Context를 빌릴 필요가 있을 것이다.

struct Context<'s>(&'s str);

struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    // 이 함수는 성공때는 아무것도 안하고 실패 시 파싱에 실패한 스트링 슬라이스를 반환한다.
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

struct Ref<'a, T: 'a>(&'a T);

struct StaticRef<T: 'static>(&'static T);


// 트레잇 객체 라이프타임의 추론
trait Red {}

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> {}

fn main() {
    let num = 5;
    let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;
}

/*  트레잇 객체의 기본 라이프타임은 'static 이다.
    &'a Trait 혹은 &'a mut Trait 를 쓴 경우는 'a이다.
    단일 T: 'a 구절을 쓴 경우엔 'a이다.
    여러개의 T: 'a 같은 구절을 쓴 경우 기본 라이프타임이 없다.
    우리가 명시해주어야한다. */