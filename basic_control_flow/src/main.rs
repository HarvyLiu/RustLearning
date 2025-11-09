fn main() {
    let x = 7;
    let y = 5;
    if y > x{
        println!("{} > {}", x, y);
    }else if y == x {
     println!("Oh wow")   
    }else {
        println!("LOL")
    }
    let day = 1;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."),
    }
}
