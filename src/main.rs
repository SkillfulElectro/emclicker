use rdev::{Event, EventType, listen}; 
use std::io::{self, Write};
use enigo::{Enigo , Mouse};

static mut X : f64 = 0.0;
static mut Y : f64 = 0.0;
static mut INTERVAL : u64 = 0;
static mut TYPE : String = String::new();
static mut ADDITIONAL : String = String::new();


static mut IGNORE_POS : bool = false;

static mut TRIGER_STOP : Vec<rdev::Key> = Vec::new();

fn callback(event: Event) {
    match event.event_type {
        EventType::MouseMove { x, y } => {
            unsafe {
                if !IGNORE_POS {
                    print!("Mouse moved to: ({}, {})\r", x, y);
                    io::stdout().flush().unwrap();

                    X = x;
                    Y = y;
                }
            }
        } , 
        EventType::KeyRelease(rdev::Key::Escape) => {
            unsafe {
                if !IGNORE_POS {
                    println!("\nEscape\r");
                    io::stdout().flush().unwrap();
                    println!("selected point ({} , {})" , X , Y);
                    IGNORE_POS = true;
                    println!("clicking process started ...");
                    clicker();
                }
            }
        } ,
        EventType::KeyRelease(rdev::Key::ControlLeft) => {
            unsafe {
                if TRIGER_STOP.contains(&rdev::Key::Alt) {
                    if TRIGER_STOP.contains(&rdev::Key::KeyS) {
                        panic!("clicker terminated");
                    }
                }
                TRIGER_STOP.push(rdev::Key::ControlLeft);
            }
        } ,
        EventType::KeyRelease(rdev::Key::Alt) => {
            unsafe {
                if TRIGER_STOP.contains(&rdev::Key::ControlLeft) {
                    if TRIGER_STOP.contains(&rdev::Key::KeyS) {
                        panic!("clicker terminated");
                    }
                }
                TRIGER_STOP.push(rdev::Key::Alt);
            }
        } ,
        EventType::KeyRelease(rdev::Key::KeyS) => {
            unsafe {
                if TRIGER_STOP.contains(&rdev::Key::Alt) {
                    if TRIGER_STOP.contains(&rdev::Key::ControlLeft) {
                        panic!("clicker terminated");
                    }
                }
                TRIGER_STOP.push(rdev::Key::KeyS);
            }
        } ,
        _ => {
            unsafe {
                TRIGER_STOP = Vec::new();
            }
        }, 
    }
}

fn clicker() {
    unsafe {
        let mut click_count : u64 = 0;
        loop {
            let mut enigo = Enigo::new(&enigo::Settings::default()).expect("failed to simulate the clicks");
            if TYPE == "mouse".to_string() {
                let _ = enigo.move_mouse(X as i32 , Y as i32 , enigo::Coordinate::Abs).expect("failed to move mouse");
                if ADDITIONAL == "left".to_string() {
                    let _ = enigo.button(enigo::Button::Left , enigo::Direction::Click).expect("failed to press mouse button"); 
                } else {
                    let _ = enigo.button(enigo::Button::Right , enigo::Direction::Click).expect("failed to press mouse button");
                }
            }
            click_count += 1;
            print!("{} times clicked !\r", click_count);
            io::stdout().flush().unwrap();


            std::thread::sleep(std::time::Duration::from_millis(INTERVAL));
        }
    }
}

fn main() {
    let mut input = String::new();
    println!("SimpleClicker : simple cli auto clicker\n");
    println!("Enter the click type : (mouse)");

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    unsafe { TYPE = input[0..input.len()].trim().to_string(); }

    println!("enter type of the click in the selected hardware : (for mouse : left , right)");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    unsafe { ADDITIONAL = input[0..input.len()].trim().to_string(); }

    println!("enter interval of clicks in milliseconds : ");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    unsafe {
        INTERVAL = input.trim().parse().expect("invalid interval entered");
    }

    println!("enter the point explicitly or watch cursor position? (pe/wcp)");
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input = input[0..input.len()].trim().to_string();
    if input == "wcp".to_string() {
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error);
        }
    } else {
        println!("enter the x of position :");
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");


        unsafe {
            X = input.trim().parse().expect("invalid float64 number entered");
        }
        println!("enter the y of position :");
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        unsafe {
            Y = input.trim().parse().expect("invalid float64 number entered");
        }
        unsafe {
            IGNORE_POS = true;
            std::thread::spawn(||{
                println!("clicking process started ...");
                clicker();
            });
        }
        

        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error);
        }

    }
}

