
// This file is for testing the borrow checker and ownership rules of Rust. 
// not a reference but the original heap memory location of the data.
fn print_null(v: Vec<i32>) -> Vec<i32> {
    println!("The sum of the vector is: {}", v[0] * v[1]);
    return v
}

fn print_null2( vr: &Vec<i32>) {
    println!("The sum of the vector is: {}", (*vr)[0] * (*vr)[1]);
}

fn print_null3(v: &Vec<i32>) {
    println!("The sum of the vector is: {}", v[0] * v[1]);
}

fn counter(v2: &Vec<i32>, val:i32) -> usize {
    v2.into_iter().filter(|&&x|x == val).count()
}

pub fn main() {
    let mut v = Vec::new();
    for i in 100..1000 {
        v.push(i);
    }
    // v using no less than 3600 bytes of memory
    // transfer ownership to print_null
    v = print_null(v);

    println!("{} {}", v[0], v[1]);

    // V still exists
    print_null2(&v);
    println!("{} {}", v[0], v[1]);

    // same as print_null2 but with dereferencing
    print_null3(&v);
    println!("{} {}", v[0], v[1]);

    println!("\nAnother");
    let v2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    for &item in &v2 {
        let res = counter(&v, item);
        println!("The count of {} in v is: {}", item, res);
    }
}