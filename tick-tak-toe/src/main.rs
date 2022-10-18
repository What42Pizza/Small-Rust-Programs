// created 09/05/22



use std::error::Error;
use std::io::{self, Write};

pub mod input;
pub mod errors;



#[derive(Clone, PartialEq)]
enum Slot {
    Empty,
    X,
    O,
}

impl Slot {
    
    fn to_char (&self) -> char {
        match self {
            Slot::Empty => ' ',
            Slot::X => 'X',
            Slot::O => 'O',
        }
    }
    
    fn to_player_type (&self) -> PlayerType {
        match self {
            Slot::Empty => PlayerType::Neither,
            Slot::X => PlayerType::X,
            Slot::O => PlayerType::O,
        }
    }
    
    fn is_empty (&self) -> bool {
        match self {
            Slot::Empty => true,
            Slot::X => false,
            Slot::O => false,
        }
    }
    
}


#[derive(Clone)]
struct Board {
    slots: [[Slot; 3]; 3]
}

impl Board {
    
    fn new() -> Board {
        Board {
            slots: [
                [Slot::Empty, Slot::Empty, Slot::Empty],
                [Slot::Empty, Slot::Empty, Slot::Empty],
                [Slot::Empty, Slot::Empty, Slot::Empty],
            ]
        }
    }
    
    fn get_slot (&self, pos: &BoardPos) -> Result<&Slot, Box<dyn Error>> {
        pos.is_valid()?;
        Ok(&self.slots[pos.x][pos.y])
    }
    
    fn set_slot (&mut self, pos: &BoardPos, slot: Slot) -> Result<(), Box<dyn Error>> {
        pos.is_valid()?;
        self.slots[pos.x][pos.y] = slot;
        Ok(())
    }
    
    fn is_full (&self) -> bool {
        for x in 0..3 {
            for y in 0..3 {
                if let Slot::Empty = self.slots[x][y] {return false;}
            }
        }
        true
    }
    
}



struct BoardPos {
    x: usize,
    y: usize,
}

impl BoardPos {
    
    fn new (x: usize, y:usize) -> BoardPos {
        BoardPos {
            x: x,
            y: y,
        }
    }
    
    fn is_valid (&self) -> Result<(), Box<dyn Error>> {
        if self.x > 2 {
            return errors::InvalidDataError::new("x was found to be ".to_owned() + &self.x.to_string());
        }
        if self.y > 2 {
            return errors::InvalidDataError::new("y was found to be ".to_owned() + &self.y.to_string());
        }
        Ok(())
    }
    
}



enum PlayerType {
    X,
    O,
    Neither,
}

impl PlayerType {
    
    fn to_str (&self) -> &str {
        match self {
            PlayerType::X => "X",
            PlayerType::O => "O",
            PlayerType::Neither => "Neither",
        }
    }
    
    fn to_slot (&self) -> Slot {
        match self {
            PlayerType::X => Slot::X,
            PlayerType::O => Slot::O,
            PlayerType::Neither => Slot::Empty,
        }
    }
    
    fn next_player (&self) -> PlayerType {
        match self {
            PlayerType::X => PlayerType::O,
            PlayerType::O => PlayerType::X,
            PlayerType::Neither => {
                panic!("cannot take the next player of Neither.");
            }
        }
    }
    
}










fn main() -> Result<(), Box<dyn Error>> {
    io::stdout().flush().unwrap();
    
    let mut board = Board::new();
    let mut current_player = PlayerType::X;
    let mut winning_player: Option<PlayerType> = None;
    
    while winning_player.is_none() {
        winning_player = process_turn (&mut board, &mut current_player)?;
        current_player = current_player.next_player();
    }
    
    print_winning_player (&winning_player.unwrap());
    Ok(())
    
}



fn process_turn (board: &mut Board, current_player: &mut PlayerType) -> Result<Option<PlayerType>, Box<dyn Error>> {
    println!("\n\n\nPlayer {}'s turn.", current_player.to_str());
    print_board (&board);
    
    let pos = get_placement_pos (&board, &current_player)?;
    board.set_slot(&pos, current_player.to_slot())?;
    
    Ok(get_winning_player (&board))
}



fn get_placement_pos (board: &Board, current_player: &PlayerType) -> Result<BoardPos, Box<dyn Error>> {
    loop {
        
        println!("\nX position:");
        let x_pos = input::get_int (1, 3)? - 1;
        println!("\nY position:");
        let y_pos = input::get_int (1, 3)? - 1;
        let pos = BoardPos::new(x_pos, y_pos);
        
        let existing_slot = board.get_slot(&pos)?;
        if !existing_slot.is_empty() {
            println!("That slot is already taken, please choose a different one.");
            continue;
        }
        
        let mut example_board = board.clone();
        example_board.set_slot(&pos, current_player.to_slot())?;
        println!("\nAre you sure you want to make this move:");
        print_board (&example_board);
        let confirmed = input::get_bool()?;
        
        if !confirmed {
            println!("\nCurrent board:");
            print_board(&board);
            continue;
        }
        return Ok(pos);
        
    }
}



fn get_winning_player (board: &Board) -> Option<PlayerType> {
    let mut winner_option;
    
    winner_option = get_winning_player_column (&board, 0);
    if let Some(winner) = winner_option {return Some(winner)};
    winner_option = get_winning_player_column (&board, 1);
    if let Some(winner) = winner_option {return Some(winner)};
    winner_option = get_winning_player_column (&board, 2);
    if let Some(winner) = winner_option {return Some(winner)};
    
    winner_option = get_winning_player_row (&board, 0);
    if let Some(winner) = winner_option {return Some(winner)};
    winner_option = get_winning_player_row (&board, 1);
    if let Some(winner) = winner_option {return Some(winner)};
    winner_option = get_winning_player_row (&board, 2);
    if let Some(winner) = winner_option {return Some(winner)};
    
    if !board.slots[1][1].is_empty() {
        if
            (board.slots[0][0] == board.slots[1][1] && board.slots[1][1] == board.slots[2][2]) ||
            (board.slots[2][0] == board.slots[1][1] && board.slots[1][1] == board.slots[0][2])
        {
            return Some(board.slots[1][1].to_player_type());
        }
    }
    
    if board.is_full() {
        return Some(PlayerType::Neither);
    }
    
    None
    
}

fn get_winning_player_column (board: &Board, i: usize) -> Option<PlayerType> {
    if let Slot::Empty = &board.slots[0][i] {return None;}
    if board.slots[0][i] == board.slots[1][i] && board.slots[1][i] == board.slots[2][i] {
        return Some(board.slots[0][i].to_player_type());
    }
    None
}

fn get_winning_player_row (board: &Board, i: usize) -> Option<PlayerType> {
    if let Slot::Empty = &board.slots[i][0] {return None;}
    if board.slots[i][0] == board.slots[i][1] && board.slots[i][1] == board.slots[i][2] {
        return Some(board.slots[i][0].to_player_type());
    }
    None
}





fn print_board (board: &Board) {
    println!("  1 2 3");
    print_row(&board, 0);
    println!("  -+-+-");
    print_row(&board, 1);
    println!("  -+-+-");
    print_row(&board, 2);
}

fn print_row (board: &Board, row_num: usize) {
    let slot_0_char = board.slots[0][row_num].to_char();
    let slot_1_char = board.slots[1][row_num].to_char();
    let slot_2_char = board.slots[2][row_num].to_char();
    println!("{} {}|{}|{}", row_num + 1, slot_0_char, slot_1_char, slot_2_char);
}



fn print_winning_player (winning_player: &PlayerType) {
    match winning_player {
        PlayerType::X => {
            println!("Player X won!");
        },
        PlayerType::O => {
            println!("Player Y won!");
        },
        PlayerType::Neither => {
            println!("Neither player won!");
        },
    }
}