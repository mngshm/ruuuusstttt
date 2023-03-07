fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // integer
    let x = 5; 
    let y = x;

    println!("x = {}, y = {}", x, y);

    // function 
    owfunc();
}

fn owfunc(){
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5; 

    makes_copy(x);
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}
