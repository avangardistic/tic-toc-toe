use std::io;

const ROWS: usize = 3;
const COLUMNS: usize = 3;
const PLAYERS: &[&str; 2] = &["X", "O"];

struct Game {
    board: [[Option<&'static str>; COLUMNS]; ROWS],
    current_player: usize,
}

impl Game {
    fn new() -> Game {
        Game {
            board: [[None; COLUMNS]; ROWS],
            current_player: 0,
        }
    }

    fn display(&self) {
        for row in 0..ROWS {
            for column in 0..COLUMNS {
                match self.board[row][column] {
                    Some(player) => print!(" {} ", player),
                    None => print!(" . "),
                }
                if column != COLUMNS - 1 {
                    print!("|");
                }
            }
            println!("");
            if row != ROWS - 1 {
                println!("---+---+---");
            }
        }
        println!("");
    }

    fn update(&mut self, row: usize, column: usize) {
        self.board[row][column] = Some(PLAYERS[self.current_player]);
        self.current_player = (self.current_player + 1) % 2;
    }

    fn winner(&self) -> Option<&'static str> {
        // Check rows
        for row in 0..ROWS {
            let player = self.board[row][0];
            if player != None && self.board[row].iter().all(|&p| p == player) {
                return player;
            }
        }

        // Check columns
        for column in 0..COLUMNS {
            let player = self.board[0][column];
            if player != None && self.board.iter().all(|row| row[column] == player) {
                return player;
            }
        }

        // Check diagonals
        let player = self.board[0][0];
        if player != None && self.board.iter().enumerate().all(|(i, row)| row[i] == player) {
            return player;
        }

        let player = self.board[0][COLUMNS - 1];
        if player != None && self.board.iter().enumerate().all(|(i, row)| row[COLUMNS - 1 - i] == player) {
            return player;
        }

        None
    }

    fn full(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|&p| p != None))
    }
}

fn main() {
    let mut game = Game::new();

    loop {
        game.display();
        println!("Player {} turn", PLAYERS[game.current_player]);
        println!("Enter row and column (0, 1, 2): ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim
