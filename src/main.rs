use std::time::Duration;

use rdev::{listen, Event, EventType};
use terminal_size::{terminal_size, Height, Width};

fn main() {
    if let Err(error) = listen(|d: Event| {
        let size = terminal_size().unwrap();
        let (Width(w), Height(h)) = size;
        match d.event_type {
            EventType::MouseMove { x, y } => {
                std::thread::sleep(Duration::from_millis(1));
                let mut mouse_position_h = (y / h as f64) as usize;
                let mut mouse_position_w = (x / w as f64) as usize;
                let mut screen = vec![vec![" "; w as usize]; h as usize];
                if mouse_position_w >= screen[0].len() {
                    mouse_position_w = screen[0].len() - 1;
                }
                if mouse_position_h >= screen.len() {
                    mouse_position_h = screen.len() - 1;
                }

                println!("{} {}", mouse_position_h, mouse_position_w);
                screen[mouse_position_h as usize][mouse_position_w as usize] = "\x1b[93mX\x1b[0m";
                print!("\x1B[2J\x1B[1;1H");
                for i in screen {
                    println!("{}", i.join("").as_str());
                }
            }
            _ => {}
        }
    }) {
        println!("Error: {:?}", error)
    }
}
