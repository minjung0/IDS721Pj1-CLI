use std::io;

fn main() {
    println!("Input your credits for Spring 2023");

    let mut credit = String::new();

    io::stdin()
        .read_line(&mut credit)
        .expect("Failed to read line");
    
    let credit: f32 = credit.trim().parse().unwrap_or(0.0);

    // Full time: 30,110
    // Part time: 3,495 per credit
    if credit >= 9.0 {
        println!("You are a full-time student.");
        println!("Your tuition is 30,110 dollars ")
    }
    else {
        let pt= 3495.0 * credit;
        println!("You are a part-time student.");
        println!("Your tuition is {} dollars", pt)
    }
}
