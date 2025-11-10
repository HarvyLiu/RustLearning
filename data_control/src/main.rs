fn main() {
    //regular vars
    let my_num: i32 = 5;          // integer
    let my_double: f64 = 5.99;    // float
    let my_letter: char = 'D';    // character
    let my_bool: bool = true;     // boolean
    let my_text: &str = "Hello";  // string
    println!("{}\n
              {}\n
              {}\n
              {}\n
              {}\n", my_num, my_double, my_letter, my_bool, my_text);
    //const e.g:
    const def_num: i32 = 100; //integer
    const def_float: f64 = 3.14; //64-bit float
    const def_str: &str = "This string is unchangable"; //string


    //CASTING:
    // assign a floating point f64 value to decimal variable
    let decimal: f32 = 64.31;

    // convert decimal variable to u16 integer type using as keyword
    let integer = decimal as u16;

    println!("decimal = {}", decimal);
    println!("integer = {}", integer);


    //Compound data_types
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; //array
    println!("{:?}", numbers);
    let fruits: [&str; 3] = ["Apples", "Bananas", "Peaches"];
    println!("{:?}", fruits);
    println!("{}", fruits[0]);
    let people = ("Henry", 32, true);
    
}
