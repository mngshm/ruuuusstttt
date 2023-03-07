fn main(){
    let mut s = String::from("Baka Baka");

    let _w = word(&s);

    s.clear();

    println!("The first word is {}: ", _w);
}

fn word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];            
        }
    }

    &s[..]
}
