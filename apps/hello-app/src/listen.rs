
struct Rust {
        age:u8,
        marks:u8,
}

#[derive(Debug)]
enum Rusty {
    Age(u8),
    Marks(u8),
}

pub fn main() {
    let student:Rust = Rust {
        age: 16,
        marks: 100,
    };
    println!("The student's age is: {}", student.age);
    println!("The student's marks are: {}", student.marks);

    let student_enum:Rusty = Rusty::Age(16);
    let student_enum_marks:Rusty = Rusty::Marks(100);
    println!("The student's age from enum is: {:?}, the marks {:?}", student_enum, student_enum_marks);

    let array = [1, 2, 3, 4, 5];
    println!("The array is: {:?}", array);

    for i in array.iter() {
        println!("The value is: {}", i);
    }

    let k = [6; 10];
    for i in k.iter() {
        println!("The value of k is: {:?}", i);
    }
    println!("The length of k is: {}", k.len());

    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("The vector is: {:?}", v);
    v.pop();
    println!("The vector after pop is: {:?}", v);

    // create vec w elements and loop and print numbers divide by 3
    // for other numbers add to another verctor and print

    let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut vec2 = vec![0; 1];

    for i in vec1.iter() {
        if i % 3 == 0 {
            println!{"divide by 3: {}", i};
        } else {
            vec2.push(*i);
        }
    }
    println!("The other vector is: {:?}", vec2);
    println!("The vector is: {:?}", vec1);
}

