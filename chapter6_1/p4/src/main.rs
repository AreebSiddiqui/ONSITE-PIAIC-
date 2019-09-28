#[derive(Debug)]
pub enum Message {
    Quit(u32),
    Move {a:u32,b:u32},
    Colour(i32,i32,i32),
    Write(String),
}



// struct QuitMessage; //unit struct

// struct MoveMessage {
//     x:i32,
//     y:i32
// }

// struct ColourMessage (i32,i32,i32); //tuple struct
// struct WriteMessage (String);

impl Message {
    fn call (&self){
        println!("{:#?}",self);
    }
}

fn main () {

    let m = Message::Write(String::from("What time is it?"));
    

    m.call();
    

}

