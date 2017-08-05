extern crate winapi;
extern crate cannon;

use cannon::{Console};
use cannon::input;
use std::thread;
use std::sync::mpsc::channel;

fn main(){
    let mut console = Console::new();
    console.set_should_cls(false);

    let (tx, rx) = channel();

    thread::spawn(move ||{
        let mut console = Console::new();
        console.set_should_cls(false);

        loop{
            let input =  console.poll_input();

            if let Some(i) = input{
                let key_opt = match i.EventType{
                    1 => key_event(i.Event),
                    _ => None,
                };

                if let Some(key) = key_opt{
                    tx.send(key);
                }
            }
        }
    });
    loop {
        println!("{:?}", rx.recv().unwrap());
        console.set_cursor_position(0,0);
    }
}

fn key_event(event: [u32;4]) -> Option<input::Key>{
    //We don't care about releases
    if event[0] == 1{
        return None;
    }

    input::num_to_key(event[2])
}
