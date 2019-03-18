use termion::terminal_size;
use std::io::{Write, stdout};
use std::{thread, time};
use termion::{clear, color, cursor, screen};

const WHEEL_11: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
const WHEEL_12: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
const WHEEL_13: &str = "  \\_/      \\O=====O=====O=====O_/      \\_/            ";

const WHEEL_21: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
const WHEEL_22: &str = " |/-=|___|=O=====O=====O=====O   |_____/~\\___/        ";
const WHEEL_23: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

const WHEEL_31: &str = "__/ =| o |=-O=====O=====O=====O \\ ____Y___________|__ ";
const WHEEL_32: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
const WHEEL_33: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

const WHEEL_41: &str = "__/ =| o |=-~O=====O=====O=====O\\ ____Y___________|__ ";
const WHEEL_42: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
const WHEEL_43: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

const WHEEL_51: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
const WHEEL_52: &str = " |/-=|___|=   O=====O=====O=====O|_____/~\\___/        ";
const WHEEL_53: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

const WHEEL_61: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
const WHEEL_62: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
const WHEEL_63: &str = "  \\_/      \\_O=====O=====O=====O/      \\_/            ";

fn main() {
    let mut screen = screen::AlternateScreen::from(stdout());
    let mut state = 0 as u8;
    let (width, _) = terminal_size().unwrap();
    let wheel1: [&str; 6] = [WHEEL_11,WHEEL_21,WHEEL_31,WHEEL_41,WHEEL_51,WHEEL_61];
    let wheel2: [&str; 6] = [WHEEL_12,WHEEL_22,WHEEL_32,WHEEL_42,WHEEL_52,WHEEL_62];
    let wheel3: [&str; 6] = [WHEEL_13,WHEEL_23,WHEEL_33,WHEEL_43,WHEEL_53,WHEEL_63];
    for x in 58..=width as u16 {
        println!("{}{}", clear::All, color::Fg(color::Red));
        println!("{}{}", cursor::Goto(width-x, 1), "      ====        ________                ___________     ");
        println!("{}{}", cursor::Goto(width-x, 2), "  _D _|  |_______/        \\__I_I_____===__|_________|    ");
        println!("{}{}", cursor::Goto(width-x, 3), "   |(_)---  |   H\\________/ |   |        =|___ ___|      ");
        println!("{}{}", cursor::Goto(width-x, 4), "   /     |  |   H  |  |     |   |         ||_| |_||       ");
        println!("{}{}", cursor::Goto(width-x, 5), "  |      |  |   H  |__--------------------| [___] |       ");
        println!("{}{}", cursor::Goto(width-x, 6), "  | ________|___H__/__|_____/[][]~\\_______|       |      ");
        println!("{}{}", cursor::Goto(width-x, 7), "  |/ |   |-----------I_____I [][] []  D   |=======|__     ");
        println!("{}{}", cursor::Goto(width-x, 8), wheel1[state as usize]);
        println!("{}{}", cursor::Goto(width-x, 9), wheel2[state as usize]);
        println!("{}{}", cursor::Goto(width-x, 10), wheel3[state as usize]);
        state += 1;
        state %= 6;
        screen.flush().unwrap();
        thread::sleep(time::Duration::from_millis(60));
        println!("{}", clear::All);
    }
}
