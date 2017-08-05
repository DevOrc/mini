extern crate winapi;
extern crate cannon;

use cannon::{Console};
use cannon::input;

fn main(){
    let console = Console::new();

    loop{
        let input =  console.poll_input();

        if let Some(i) = input{
            match i.EventType{
                1 => key_event(i.Event),
                _ => (),
            }

        }
    }
}

fn key_event(event: [u32;4]){
    //We don't care about releases
    if event[0] == 1{
        return;
    }

    println!("{:?}", input::num_to_key(event[2]).expect("Key Error"));
}
