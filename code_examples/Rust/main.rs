// #region 0. Boilerplate
fn main() {
use std::f64::consts::PI;



// #endregion

// Partially Applied/Curried Functions
let add2numbers = |a: i32, b: i32| a + b;
let add2numbersPartial = |a: i32| move |b: i32| a + b;
let add2numbersCurried = |a: i32| move |b: i32| add2numbers(a, b);
let plus5 = add2numbersCurried(5);
let twelve = plus5(7);
let fortyTwo = add2numbersPartial(15)(27);

// Composition
let plus12 = |a: i32| a + 12;
let mult5 = |a: i32| a * 5;
macro_rules! compose { ($f: expr) => { move |g: fn(_) -> _| move |x: _| $f(g(x)) }; }
let plus12mult5 = compose!(mult5)(plus12);

let _ = plus12mult5(1);

// Higher-Order Functions
let characters = 
    [
        "Rusty Venture",
        "Brock Samson",
        "Dean Venture",
        "Hank Venture",
        "Helper Robot",
        "Dr. Orpheus",
        "The Monarch",
        "Dr. Girlfriend"
    ];

let isDirectFamily = |name: &&str| name.contains("Venture");
let directFamily = characters.iter().copied().filter(isDirectFamily).collect::<Vec<&str>>();
let doctorCount = characters.iter().copied().filter(|n| n.contains(&"Dr.")).count();

let charactersByLastName = characters.iter()
    .map(|n| n.split(&" ").last().unwrap()).collect::<Vec<&str>>()
    .as_slice().group_by(|a, b| a == b)
    .map(|ns| (ns.first(), ns.len()));

// Pattern Matching
enum Shape {
    Circle {radius : f64},
    Rectangle {width: f64, height: f64},
    Triangle {side1: f64, side2: f64, side3: f64}
}

impl Shape {
    fn area(self) -> f64 {
        match self {
            Shape::Circle {radius} => PI * radius.powi(2),
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { side1, side2, side3 } => {
                let s = (side1 + side2 + side3) / 2.0;
                f64::sqrt(s *(s-side1) * (s-side2) * (s-side3))
            }
        }
    }
}

_ = Shape::area(Shape::Circle {radius: 2.5});
_ = Shape::area(Shape::Rectangle {width: 10.0, height: 20.0});
_ = Shape::area(Shape::Triangle {side1: 2.0, side2: 3.0, side3: 4.0});






}