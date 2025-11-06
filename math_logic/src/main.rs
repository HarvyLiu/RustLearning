fn main() {
    let add = 5 + 3; //add
    let sub = 10 - 4; //minus
    let mul = 6 * 2; //multiply
    let div = 12 / 3; //division
    let rem = 10 % 3; //reminder

    println!("Add: {}", add);
    println!("Sub: {}", sub);
    println!("Mul: {}", mul);
    println!("Div: {}", div);
    println!("Rem: {}", rem);


    println!("{}", add+2);
    let mut x = 10;
    println!("X before adding: {}", x);
    x += 1;
    println!("x after adding: {}", x);
    
    let logged_in: bool = true;
    let is_admin: bool = false;

   println!("Is regular user: {}", logged_in && !is_admin);
   println!("Has any access: {}", logged_in || is_admin);
   println!("Not logged in: {}", !logged_in);

   let age = 20;
   let can_vote = age >=18; //I can let it compare true false in one line
   println!("Can vote = {}", can_vote);

}

