// OOP는 보통 객체, 캡슐화, 상속을 지원한다.
// 러스트에서 지원하는 객체지향적 특성을 살펴보자
//
// 객체는 데이터 및 이 데이터를 활용하는 프로시저: 메소드를 가진다.
// 러스트에서 구조체와 열거형은 데이터를 가지고 imple로 메소드를 제공하니 러스트는 객체를 제공한다.
//
// 캡슐화는 객체를 이용하는 코드에서 객체의 상세 구현을 접근할 수 없다는 것이다.
// 러스트에서 pub를 통해 공개와 비공개를 정할 수 있다.
// 기본적으로 모두 비공개이다.

// 벡터와 벡터의 평균을 가지는 변수를 가진다.
// 이를 사용하는 사람이 매번 평균을 계산할 필요는 없다.
// 얘가 알아서 한다.

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// 상속은 러스트에서 반드시 제공하지는 않는다.
// 보통 코드 재사용을 위해 상속을 이용한다.
// 러스트에서는 트레잇 기본 메소드의 구현을 이용해 코드를 공유할 수 있다.
// 다른 이유로는 자식 타입을 부모 타입처럼 사용하기 위함에 있다.
// 이를 다형성이라고도 부르며 여러 객체들이 일정한 특성을 공유한다면 
// 이들을 런타임에 서로 바꿔 대입하며 사용할 수 있음을 의미한다.
//
// 최근에는 상속의 인기가 떨어지고 있다.
// 불필요한 코드까지 공유할 위험이 있기 때문이다.
// 또한 유연성을 저하시킬 수 있다.

pub trait Draw {
    fn draw(&self);
}

// Draw 트레잇을 구현하는 트레잇 객체들의 벡터를 가지는 Screen
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

// 제네릭과 트레잇 바운드를 사용한 run 메소드
impl<T> Screen<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 대충 버튼 그리는 코드 
    }
}

