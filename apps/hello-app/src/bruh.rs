pub fn main() {

    for i in 0u8..27u8 {
        if i == 0 {
            println!("{} is zero", i);
        } else if i % 2 == 0 {
            println!("{} is even", i);
            match i {
                2 => println!("{} is two", i),
                4 => println!("{} is four", i),
                6 => println!("{} is six", i),
                8 => println!("{} is eight", i),
                10 => println!("{} is ten", i),
                12 => println!("{} is twelve", i),
                14 => println!("{} is fourteen", i),
                16 => println!("{} is sixteen", i),
                18 => println!("{} is eighteen", i),
                20 => println!("{} is twenty", i),
                22 => println!("{} is twenty-two", i),
                24 => println!("{} is twenty-four", i),
                _ => println!("{} is an even number greater than 24", i),
            };
        } else {
            println!("{} is odd", i);
        }
    }
}