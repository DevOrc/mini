extern crate winapi;
extern crate cannon;

use cannon::{Console};
use cannon::input;

fn main(){
    let console = Console::new();

    loop{
        let input =  console.poll_input();

        if let Some(i) = input{
            println!("{:?}", i);
        }
    }
}
