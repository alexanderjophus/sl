use termion::terminal_size;
use std::io::{Write, stdout};
use std::{thread, time};
use termion::{clear, color, cursor, screen};

const TRAIN_11: &str = "      ====        ________                ___________ ";
const TRAIN_12: &str = "  _D _|  |_______/        \\__I_I_____===__|_________| ";
const TRAIN_13: &str = "   |(_)---  |   H\\________/ |   |        =|___ ___|   ";
const TRAIN_14: &str = "   /     |  |   H  |  |     |   |         ||_| |_||   ";
const TRAIN_15: &str = "  |      |  |   H  |__--------------------| [___] |   ";
const TRAIN_16: &str = "  | ________|___H__/__|_____/[][]~\\_______|       |   ";
const TRAIN_17: &str = "  |/ |   |-----------I_____I [][] []  D   |=======|__ ";

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

const COAL01: &str = "                              ";
const COAL02: &str = "                              ";
const COAL03: &str = "    _________________         ";
const COAL04: &str = "   _|                \\_____A ";
const COAL05: &str = " =|                        |  ";
const COAL06: &str = " -|                        |  ";
const COAL07: &str = "__|________________________|_ ";
const COAL08: &str = "|__________________________|_ ";
const COAL09: &str = "   |_D__D__D_|  |_D__D__D_|   ";
const COAL10: &str = "    \\_/   \\_/    \\_/   \\_/";

fn main() {
    // TODO Add flags
    // -n number of carriages
    // -t type of locomotive
    let mut screen = screen::AlternateScreen::from(stdout());
    let mut state = 0 as u8;
    let (width, _) = terminal_size().unwrap();
    let wheel1: [&str; 6] = [WHEEL_11,WHEEL_21,WHEEL_31,WHEEL_41,WHEEL_51,WHEEL_61];
    let wheel2: [&str; 6] = [WHEEL_12,WHEEL_22,WHEEL_32,WHEEL_42,WHEEL_52,WHEEL_62];
    let wheel3: [&str; 6] = [WHEEL_13,WHEEL_23,WHEEL_33,WHEEL_43,WHEEL_53,WHEEL_63];
    for x in 0..=width as u16 {
        // TODO add steam
        println!("{}{}", clear::All, color::Fg(color::AnsiValue(state+1)));
        print_train(&[TRAIN_11, COAL01], width, x, 1);
        print_train(&[TRAIN_12, COAL02], width, x, 2);
        print_train(&[TRAIN_13, COAL03], width, x, 3);
        print_train(&[TRAIN_14, COAL04], width, x, 4);
        print_train(&[TRAIN_15, COAL05], width, x, 5);
        print_train(&[TRAIN_16, COAL06], width, x, 6);
        print_train(&[TRAIN_17, COAL07], width, x, 7);
        print_train(&[wheel1[state as usize], COAL08], width, x, 8);
        print_train(&[wheel2[state as usize], COAL09], width, x, 9);
        print_train(&[wheel3[state as usize], COAL10], width, x, 10);
        state += 1;
        state %= 6;
        screen.flush().unwrap();
        thread::sleep(time::Duration::from_millis(60));
        println!("{}", clear::All);
    }
}

fn print_train(input: &[&str], t_width: u16, offset: u16, row: u16) {
    let mut length = 0;
    // TODO this is ugly as hell - must be a better way to slice a utf8 string
    let out: String = input.iter()
        .map(|item| {
            item.chars().map(|c| {
                length = length + 1;
                if length >= offset {
                    return ' '
                } else {
                    c
                }
            }).collect::<String>()
        })
        .collect::<String>();

    println!("{}{}", cursor::Goto(t_width - offset, row), out);
}