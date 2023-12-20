##UNTESTED PROTOYPE MAIN LOOP
##CROSSTERM sends event in raw mode. We make sure no modifiers are pressed while the key is pressed.

use crossterm::{
    event::{self, Event as CEvent, KeyEvent},
    terminal::{enable_raw_mode, disable_raw_mode},
};

fn main() {
    enable_raw_mode().unwrap();

    loop {
        match event::read() {
            Ok(CEvent::Key(event)) => {
                if let KeyEvent {
                    code: KeyCode::Char(c),
                    modifiers: KeyModifiers::NONE,
                } = event
                {
                    println!("You pressed: {}", c);
                    // Process the character here
                }
            }
            Ok(_) => {}
            Err(_) => break,
        }
    }

    disable_raw_mode().unwrap();
}
