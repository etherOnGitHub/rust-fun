// fn none(data:i32) {
//     println!("The value of data is: {}", data);
// }

// pub fn main() {
//     let x = 5;
//     let y = x;
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
//     none(x);
// }

// pub fn main() {
//     let v = vec![1, 2, 3];
//     println!("The vector is: {}", v[0]);
//     let v2 =  v;
//     println!("The vector is: {}", v2[0]);
//     println!("The vector is: {}", v[0]);
// }

fn print_sum(v:Vec<i32>) {
    println!("The sum of the vector is: {}", v[0] + v[1]);
}

pub fn main() {
    let mut v = Vec::new();
    for i in 100..1000 {
        v.push(i);
    }
    print_sum(v);
    // println!("{}", v[0]);
}