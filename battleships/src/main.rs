use rand::Rng;
use std::io;

// Constants for the board size and number of ships
const BOARD_SIZE: usize = 5;
const NUM_SHIPS: usize = 3;

// Struct to represent the game state
struct BattleshipGame {
    board: [[char; BOARD_SIZE]; BOARD_SIZE],
    ships: Vec<Vec<(usize, usize)>>,
}

impl BattleshipGame {
    fn new() -> Self {
        // Initialize an empty board
        let board = [['~'; BOARD_SIZE]; BOARD_SIZE];

        // Place ships randomly without overlapping
        let mut ships = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..NUM_SHIPS {
            let mut ship = Vec::new();
            let mut ship_added = false;

            while !ship_added {
                ship.clear();
                let start_row = rng.gen_range(0..BOARD_SIZE);
                let start_col = rng.gen_range(0..BOARD_SIZE);
                let is_vertical: bool = rng.gen();

                for i in 0..NUM_SHIPS {
                    let row = if is_vertical { start_row + i } else { start_row };
                    let col = if is_vertical { start_col } else { start_col + i };

                    if row < BOARD_SIZE && col < BOARD_SIZE {
                        ship.push((row, col));
                    } else {
                        break;
                    }
                }

                // Check if the ship overlaps with existing ships
                let overlap = ships.iter().any(|s: &Vec<(usize, usize)>| s.iter().any(|&coord| ship.contains(&coord)));
                if !overlap {
                    ships.push(ship.clone());
                    ship_added = true;
                }
            }
        }

        BattleshipGame { board, ships }
    }

    fn display_board(&self) {
        // Display column numbers
        print!("   ");
        for col in 0..BOARD_SIZE {
            print!(" {} ", col);
        }
        println!();

        for (row_num, row) in self.board.iter().enumerate() {
            // Display row numbers
            print!(" {} ", row_num);

            // Display the actual board content
            for &cell in row {
                print!(" {} ", cell);
            }
            println!();
        }
    }

    fn make_guess(&mut self, row: usize, col: usize) -> bool {
        for ship in &mut self.ships {
            if ship.contains(&(row, col)) {
                println!("Hit!");
                self.board[row][col] = 'X';
                ship.retain(|&coord| coord != (row, col));
                if ship.is_empty() {
                    println!("Congratulations! You sunk a battleship!");
                    self.ships.retain(|s| !s.is_empty());
                    if self.ships.is_empty() {
                        println!("Congratulations! You sunk all battleships. You win!");
                        return true;
                    }
                }
                return true;
            }
        }

        println!("Miss!");
        self.board[row][col] = 'O';
        false
    }
}

fn main() {
    println!("Welcome to Battleship!");

    let mut game = BattleshipGame::new();

    loop {
        println!("Current Board:");
        game.display_board();

        println!("Enter your guess (row, col):");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // Parse the user's guess
        let coordinates: Vec<usize> = guess
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if coordinates.len() == 2 {
            let row = coordinates[0];
            let col = coordinates[1];

            if row < BOARD_SIZE && col < BOARD_SIZE {
                if game.make_guess(row, col) {
                }
            } else {
                println!("Invalid coordinates. Try again.");
            }
        } else {
            println!("Invalid input. Enter two space-separated numbers for row and column.");
        }
    }
}

