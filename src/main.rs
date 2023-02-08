use std::io;

fn main() {
    println!("Which semester's tuition: 1. Spring 2023, 2. Fall 2023 or Spring 2024");

    let mut semester = String::new();

    io::stdin()
        .read_line(&mut semester)
        .expect("Failed to read line.");

    let semester: u32 = semester.trim().parse().unwrap_or(1);

    match semester {
        1 => println!("You choose Spring 2023."),
        2 => println!("You choose Fall 2023 or Spring 2024."),
        _ => panic!("Please choose 1 or 2."),
    };

    println!("Input your credits for the semester.");

    let mut credit = String::new();

    io::stdin()
        .read_line(&mut credit)
        .expect("Failed to read line.");

    let credit: f32 = credit.trim().parse().unwrap_or(0.0);

    // (2022-2023) Full time: 30,110
    // (2023-2024)            31,310
    // (2022-2023) Part time: 3,495 per credit
    // (2023-2024)            3,635
    if credit >= 9.0 {
        println!("You are a full-time student.");
        if semester == 1 {
            println!("Your tuition is 30,110 dollars.");
        } else {
            println!("Your tuition is 31,310 dollars.");
        }
    } else {
        let mut pt = 0.0;
        println!("You are a part-time student.");
        if semester == 1 {
            pt = 3490.0 * credit;
        } else {
            pt = 3635.0 * credit;
        }
        println!("Your tuition is {pt} dollars.");
    }
}
