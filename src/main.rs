use rand::Rng;
use read_input::prelude::*;

struct Board {
    state: [char; 9],
    count: usize,
}

impl Board {
    fn new() -> Self {
        Self {
            state: ['1', '2', '3', '4', '5', '6', '7', '8', '9'],
            count: 0,
        }
    }
}

fn main() {
    println!(
        "Welcome to my simple tic-tac-toe game\nYou are X and the computer is O.\nLet's play!"
    );
    let mut board = Board::new();
    loop {
        draw_board(&board);
        take_input(&mut board);
        if let Some(value) = check_winner(&board) {
            draw_board(&board);
            declare_winner(value);
        }
        if board.count == 9 {
            println!("The Board is full.\nHence a Draw!!");
            std::process::exit(0);
        }
        computer_play(&mut board);
        if let Some(value) = check_winner(&board) {
            draw_board(&board);
            declare_winner(value);
        }
        if board.count == 9 {
            println!("The Board is full.\nHence a Draw!!");
            std::process::exit(0);
        }
    }
}

fn draw_board(board: &Board) {
    print!("\n-------------\n|");
    for i in board.state[0..3].iter() {
        print!(" {i} |");
    }
    print!("\n-------------\n|");
    for i in board.state[3..6].iter() {
        print!(" {i} |");
    }
    print!("\n-------------\n|");
    for i in board.state[6..9].iter() {
        print!(" {i} |");
    }
    print!("\n-------------\n");
}

fn take_input(board: &mut Board) {
    loop {
        print!("Enter which square to mark: ");
        let input: usize = input::<usize>().get();
        if input > 0 && input <= 9 {
            if board.state[input - 1] != 'X' && board.state[input - 1] != 'O' {
                board.state[input - 1] = 'X';
                board.count += 1;
                break;
            } else {
                println!("Input position already taken, try again!!");
            }
        } else {
            println!("Choose any number on the board!");
        }
    }
}

fn check_winner(board: &Board) -> Option<char> {
    let mut offset: usize = 0;
    let state = &board.state;
    for i in 0..3 {
        if state[i + offset] == state[i + offset + 1] && state[i + offset] == state[i + offset + 2]
        {
            return Some(state[i + offset]);
        } else if state[i] == state[i + 3] && state[i] == state[i + 6] {
            return Some(state[i]);
        }
        offset += 2;
    }
    if state[0] == state[4] && state[0] == state[8] {
        return Some(state[0]);
    } else if state[2] == state[2 + 2] && state[2] == state[2 + 4] {
        return Some(state[2]);
    }
    None
}

fn declare_winner(player: char) {
    if player == 'X' {
        println!("The player has won!!\n");
    } else {
        println!("The computer has won!!\n");
    }
    std::process::exit(0);
}

fn computer_play(board: &mut Board) {
    let mut rng = rand::thread_rng();
    loop {
        let input: usize = rng.gen_range(1..9); //inefficient but it works
        if board.state[input - 1] != 'X' && board.state[input - 1] != 'O' {
            board.state[input - 1] = 'O';
            board.count += 1;
            break;
        }
    }
}
