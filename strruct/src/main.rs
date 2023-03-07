#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32, 
}

fn main(){
    let rect1 = Rectangle {
        width: 20, 
        height: 32,
    };

    dbg!(&rect1);
}
