extern crate kernel32 as console;
extern crate winapi;

pub mod color;
pub mod input;

use winapi::*;

use std::io::Write;

pub struct Console{
    handle: *mut std::os::raw::c_void,
    input_handle: *mut std::os::raw::c_void,
    color: u16,
    should_cls: bool
}

impl Drop for Console{
    fn drop(&mut self) {
        unsafe {
            self.set_color(color::WHITE, color::BLACK);
            if self.should_cls{
                self.clear_screen();
                self.set_cursor_position(0, 1);
            }
            console::CloseHandle(self.handle);
        }
    }
}

impl Console{
    pub fn new() -> Console{
        let console = Console{
            handle: unsafe {
                    console::GetStdHandle(winapi::STD_OUTPUT_HANDLE)
                },
            input_handle: unsafe {
                    console::GetStdHandle(winapi::STD_INPUT_HANDLE)
                },
            color: color::format_color(color::WHITE, color::BLACK),
            should_cls: false
        };

        console
    }

    pub fn set_should_cls(&mut self, val: bool){
        self.should_cls = val;
    }

    pub fn poll_input(&self) -> Option<winapi::INPUT_RECORD>{
        let input = &mut winapi::INPUT_RECORD {Event: [0;4], EventType: 0};
        let read: &mut u32 = &mut 0;
        unsafe {
            console::ReadConsoleInputA(self.input_handle, input, 1, read);
        }
        if *read != 0{
            return Some(*input);
        }
        None
    }

    pub fn get_console_size(&self) -> ConsoleSize{
        let console_info = self.get_console_info();
        let size = console_info.srWindow;
        ConsoleSize{width: size.Right + 1, height: size.Bottom + 1}
    }

    pub(crate) fn get_console_info(&self) -> CONSOLE_SCREEN_BUFFER_INFO{
        let info = &mut CONSOLE_SCREEN_BUFFER_INFO {
            dwSize: COORD {X: 0, Y: 0},
            dwCursorPosition: COORD {X: 0, Y: 0},
            wAttributes: 0,
            srWindow: SMALL_RECT {Left: 0, Right: 0, Top: 0, Bottom: 0},
            dwMaximumWindowSize: COORD {X: 0, Y: 0}
        };
        unsafe {
            console::GetConsoleScreenBufferInfo(self.handle, info);

        }
        *info
    }

    pub fn write(&self, input: &str){
        print!("{}", input);
        Console::flush();
    }

    pub fn clear_screen(&self){
        let size = self.get_console_size();
        unsafe {
            let mut a = 0;
            console::FillConsoleOutputCharacterA(self.handle, 32, (size.width  * size.height) as u32,
                COORD{X: 0, Y: 0}, &mut a);
            console::FillConsoleOutputAttribute(self.handle, self.color, (size.width  * size.height) as u32,
                COORD{X: 0, Y: 0}, &mut a);
            Console::flush();
        }
    }

    pub fn set_color(&mut self, foreground: u16, background: u16){
        let color = color::format_color(foreground, background);

        self.color = color;

        unsafe {
            console::SetConsoleTextAttribute(self.handle, color);
        }
    }

    pub fn write_character(&self, x: i16, y: i16, character: i8){
        unsafe {
            console::FillConsoleOutputCharacterA(self.handle, character, 1,
                COORD{X: x, Y: y}, &mut 0);
            console::FillConsoleOutputAttribute(self.handle, self.color, 1,
                COORD{X: x, Y: y}, &mut 0);
            Console::flush();
        }
    }

    pub fn set_cursor_position(&self, x: i16, y: i16){
        unsafe {
            console::SetConsoleCursorPosition(self.handle, winapi::COORD {X: x, Y: y});
        }
    }

    pub fn flush(){
        let result = std::io::stdout().flush();
        if let Err(e) = result{
            panic!("Error Flushing Console {:?}", e);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConsoleSize{
    pub width: i16,
    pub height: i16
}
