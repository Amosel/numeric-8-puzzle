use inquire::Select;
use rand::Rng;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

trait FromChar {
    fn from_char(c: char) -> Result<Self, &'static str>
    where
        Self: Sized;
}
impl FromChar for Direction {
    fn from_char(c: char) -> Result<Self, &'static str> {
        match c {
            'L' | 'l' => Ok(Direction::Left),
            'R' | 'r' => Ok(Direction::Right),
            'U' | 'u' => Ok(Direction::Up),
            'D' | 'd' => Ok(Direction::Down),
            _ => Err("Invalid direction character"),
        }
    }
}

trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Left => "Left".to_string(),
            Direction::Right => "Right".to_string(),
            Direction::Up => "Up".to_string(),
            Direction::Down => "Down".to_string(),
        }
    }
}
trait ToChar {
    fn to_char(&self) -> char;
}

impl ToChar for Direction {
    fn to_char(&self) -> char {
        match self {
            Direction::Left => 'L',
            Direction::Right => 'R',
            Direction::Up => 'U',
            Direction::Down => 'D',
        }
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Direction::Left, Direction::Left)
            | (Direction::Right, Direction::Right)
            | (Direction::Up, Direction::Up)
            | (Direction::Down, Direction::Down) => true,
            _ => false,
        }
    }
}

fn find_position(puzzle: &[[u8; 3]; 3], value: u8) -> (usize, usize) {
    if value >= 8 {
        unreachable!()
    }
    for (row_idx, row) in puzzle.iter().enumerate() {
        for (col_idx, &piece) in row.iter().enumerate() {
            if piece == 0 {
                return (row_idx, col_idx);
            }
        }
    }
    unreachable!()
}
fn find_empty_position(puzzle: &[[u8; 3]; 3]) -> (usize, usize) {
    return find_position(puzzle, 0);
}

fn get_valid_moves(puzzle: &[[u8; 3]; 3]) -> Vec<Move> {
    let empty_position = find_empty_position(puzzle);

    let mut valid_moves = Vec::new();

    // down
    if empty_position.0 != 0 {
        valid_moves.push(Move {
            direction: Direction::Down,
            from: (empty_position.0 - 1, empty_position.1),
            to: empty_position,
            piece: puzzle[empty_position.0 - 1][empty_position.1],
        });
    }
    // up
    if empty_position.0 < 2 {
        valid_moves.push(Move {
            direction: Direction::Up,
            from: (empty_position.0 + 1, empty_position.1),
            to: empty_position,
            piece: puzzle[empty_position.0 + 1][empty_position.1],
        })
    }
    // right
    if empty_position.1 < 2 {
        valid_moves.push(Move {
            direction: Direction::Left,
            from: (empty_position.0, empty_position.1 + 1),
            to: empty_position,
            piece: puzzle[empty_position.0][empty_position.1 + 1],
        })
    }
    // left
    if empty_position.1 != 0 {
        valid_moves.push(Move {
            direction: Direction::Right,
            from: (empty_position.0, empty_position.1 - 1),
            to: empty_position,
            piece: puzzle[empty_position.0][empty_position.1 - 1],
        })
    }
    valid_moves
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    from: (usize, usize),
    to: (usize, usize),
    piece: u8,
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.direction.to_char(), self.piece)
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.direction == other.direction && self.piece == other.piece
    }
}

fn shuffle_puzzle(puzzle: &mut [[u8; 3]; 3], nr_moves: usize) {
    let mut rng = rand::thread_rng();

    let directions = ['L', 'R', 'U', 'D'];
    let mut counter = 0;
    while counter != nr_moves {
        let random_direction: usize = rng.gen_range(0..4);
        let direction = Direction::from_char(directions[random_direction]).unwrap();
        if let Some(m) = get_valid_moves(&puzzle)
            .iter()
            .find(|v| v.direction == direction)
        {
            puzzle[m.from.0][m.from.1] = 0;
            puzzle[m.to.0][m.to.1] = m.piece;
            counter += 1;
        }
    }
}

fn print_puzzle(puzzle: &[[u8; 3]; 3]) {
    for row in puzzle.iter() {
        for &piece in row.iter() {
            if piece == 0 {
                print!("_  ");
            } else {
                print!("{:<3}", piece);
            }
        }
        println!();
    }
}

fn is_success(puzzle: &[[u8; 3]; 3]) -> bool {
    let mut success = true;
    let mut prev = 0;
    for row in puzzle.iter() {
        for value in row.iter() {
            if *value == 0 {
                break;
            }
            if prev + 1 != *value {
                success = false;
                break;
            }
            prev = *value;
        }
    }

    success
}

fn main() {
    let mut puzzle: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 0]];
    shuffle_puzzle(&mut puzzle, 1);
    let mut valid_moves = get_valid_moves(&puzzle);
    let mut success = false;
    while !success {
        println!("Welcome to the Numeric 8 Puzzle!, {}", success);
        println!("_______");
        print_puzzle(&puzzle);
        println!("_______");
        println!("Please select an option:");
        print!("Enter your choice: ");
        let mut opts: Vec<String> = valid_moves
            .iter()
            .map(|mv| format!("Move {} to {}", mv.piece, mv.direction.to_string()))
            .collect();
        opts.push("Exit".to_string());
        let exit_index = opts.len() - 1;

        match Select::new("What's your next move?", opts).raw_prompt() {
            Ok(choice) => {
                if choice.index == exit_index {
                    println!("Exiting the program...");
                    break;
                }
                println!("{} Chosen", choice);
                let selected_move = &valid_moves[choice.index];
                puzzle[selected_move.from.0][selected_move.from.1] = 0;
                puzzle[selected_move.to.0][selected_move.to.1] = selected_move.piece;
                success = is_success(&puzzle);
                if !success {
                    valid_moves = get_valid_moves(&puzzle);
                } else {
                    println!("Success");
                }
            }
            Err(_) => println!("There was an error, please try again"),
        }
    }
}
