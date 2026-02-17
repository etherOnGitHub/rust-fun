pub fn main() {
    println!("This is the rusty module's main function!");

    let x:bool = true;

    if x {
        println!("x is true");
    } else {
        println!("x is false");
    }

    let y:i8 = 6;
    if y == 4 {
        println!("y is 4");
    } else if y > 4 {
        println!("y is greater than 4");
        } else {
        println!("y is not 4");
    }

    while x {
        println!("x is false, looping...");
        println!("yeah nah im not looping forever, im gonna go ahead and terminate the loop here");
        break
    }

    for i in 0..8 {
        println!("i is: {}", i);
    }

    let bruh = "bruh";
    match bruh {
        "bruh" => println!("bruh does match"),
        _ => println!("not bruh"),
    };

    let a_word = "word";
    let word_word = match a_word {
        "word" => "word",
        _ => "not word",
    };

    println!("The value of word_word is: {}", word_word);
}