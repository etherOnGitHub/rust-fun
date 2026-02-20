fn func1() {
    let x = 3 + 5;
    println!("The value of x is: {}", x);
}

fn func2(name:String) {
    println!("Hello, {}!", name);
}

fn func3(x:i32, y: i32) {
    let sum = x + y;
    println!("The sum of {} and {} is: {}", x, y, sum);
}

pub fn main() {
    func1();
    func2("etherchild".to_string());
    func3(10, 20);

    // closure
    let closure = |x: i32| x * 2;
    println!("The result of the closure is: {}", closure(5));

    let closure2 = |x| {
        let mut result: i32 = x;
        result += 10;
        result += 6;
        result
    };

    println!("The result of the second closure is: {}", closure2(5));
}