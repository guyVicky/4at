use std::io::{stdout, Result, Write};
use crossterm::{
    cursor::{MoveTo, MoveToNextLine}, event::{self, poll, read, Event, KeyEventKind, KeyModifiers}, style::Print, terminal::{self, ClearType}, QueueableCommand};
use std::thread;
use std::time::Duration;

fn main() -> Result<()>{
    let mut stdout = stdout();

    let _ = terminal::enable_raw_mode();

    stdout.queue( terminal::Clear(ClearType::All))?;
    
    let (mut width, mut height) = terminal::size()?;
    
    let mut message = String::new();
    let mut messages: Vec<String> = Vec::new();
    
    let mut quit = false;


    while !quit {
        while poll(Duration::ZERO)? {
            match read()? {
                Event::Key(event) => {
                    match event.code {
                        event::KeyCode::Backspace => {
                            if event.kind == KeyEventKind::Press {
                                message.pop();
                            }
                        },
                        event::KeyCode::Char(x) => {
                            if (x=='c') && event.modifiers.contains(KeyModifiers::CONTROL) {
                                quit = true;
                            }
                            else if event.kind == KeyEventKind::Press {
                                message.push(x);
                            }
                        },
                        event::KeyCode::Enter => {
                            messages.push(message.clone());
                            message.clear();
                        }
                        _ => {},
                    }
                },
                Event::Resize(nw, nh) => {
                    width = nw;
                    height = nh;
                },
                _ => {},
            }
        }
        let stuff = format!("{}, {}", width, height);
        let bar = format!("{:┄<1$}", "", width as usize);
        
        
    stdout.queue( terminal::Clear(ClearType::All))?;



    stdout.queue(MoveTo(1, 0))?;

    for (i, line) in messages.iter().enumerate() {
        stdout
            .queue(Print(line))?
            .queue(MoveToNextLine(1))?;
    }


    stdout
            .queue(MoveTo(width/2 - stuff.len() as u16/2, height/2))?
            .queue(Print(stuff))?
            .queue(MoveTo(0, height - 2))?
            .queue(Print(bar))?
            .queue(MoveTo(1, height - 1))?
            .queue(Print(">"))?
            .queue(MoveTo(2, height - 1))?
            .queue(Print(&message))?;
    stdout.flush()?;
    thread::sleep(Duration::from_millis(33));
}
    Ok(())
}

// fn print_events() -> Result<()> {
//     loop {
//         match read()? {
//             Event::FocusGained => println!("Focus Gained!"),
//             Event::FocusLost => println!("Focus Lost!"),
//             Event::Key(event) => println!("{:?}", event),
//             Event::Mouse(event) => println!("{:?}", event),
//             // #[cfg(feature = "bracketed-paste")]
//             Event::Paste(data) => println!("{:?}", data),
//             Event::Resize(width, height) => println!("New size {}x{}", width, height)
//         }
//     }
// }