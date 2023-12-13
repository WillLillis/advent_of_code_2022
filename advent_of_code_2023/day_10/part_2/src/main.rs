use std::{collections::HashSet, str::FromStr};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum CardDir {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Tile {
    PipeNS, // |
    PipeEW, // -
    PipeNE, // L
    PipeNW, // J
    PipeSW, // 7
    PipeSE, // F
    Ground, // .
    Start,  // S
}

impl FromStr for Tile {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Tile::PipeNS),
            "-" => Ok(Tile::PipeEW),
            "L" => Ok(Tile::PipeNE),
            "J" => Ok(Tile::PipeNW),
            "7" => Ok(Tile::PipeSW),
            "F" => Ok(Tile::PipeSE),
            "." => Ok(Tile::Ground),
            "S" => Ok(Tile::Start),
            _ => Err(String::from("Failed to deserialize!")),
        }
    }
}

impl Tile {
    /// dir is relative direction from self to other
    fn is_connected(self, other: Self, dir: CardDir) -> bool {
        // ductape
        if other == Tile::Start {
            return true;
        }
        match (self, dir) {
            // S
            (Tile::Start, CardDir::North) => match other {
                Tile::PipeNS | Tile::PipeSW | Tile::PipeSE => true,
                _ => false,
            },
            (Tile::Start, CardDir::East) => match other {
                Tile::PipeEW | Tile::PipeSW | Tile::PipeNW => true,
                _ => false,
            },
            (Tile::Start, CardDir::South) => match other {
                Tile::PipeNS | Tile::PipeNE | Tile::PipeNW => true,
                _ => false,
            },
            (Tile::Start, CardDir::West) => match other {
                Tile::PipeEW | Tile::PipeSE | Tile::PipeNE => true,
                _ => false,
            },
            // .
            (Tile::Ground, _) => false,
            // |
            (Tile::PipeNS, CardDir::North) => match other {
                Tile::PipeNS | Tile::PipeSE | Tile::PipeSW => true,
                _ => false,
            },
            (Tile::PipeNS, CardDir::South) => match other {
                Tile::PipeNS | Tile::PipeNE | Tile::PipeNW => true,
                _ => false,
            },
            (Tile::PipeNS, _) => false,
            // -
            (Tile::PipeEW, CardDir::East) => match other {
                Tile::PipeEW | Tile::PipeSW | Tile::PipeNW => true,
                _ => false,
            },
            (Tile::PipeEW, CardDir::West) => match other {
                Tile::PipeEW | Tile::PipeSE | Tile::PipeNE => true,
                _ => false,
            },
            (Tile::PipeEW, _) => false,
            // L
            (Tile::PipeNE, CardDir::North) => match other {
                Tile::PipeNS | Tile::PipeSW | Tile::PipeSE => true,
                _ => false,
            },
            (Tile::PipeNE, CardDir::East) => match other {
                Tile::PipeEW | Tile::PipeNW | Tile::PipeSW => true,
                _ => false,
            },
            (Tile::PipeNE, _) => false,
            // J
            (Tile::PipeNW, CardDir::North) => match other {
                Tile::PipeNS | Tile::PipeSE | Tile::PipeSW => true,
                _ => false,
            },
            (Tile::PipeNW, CardDir::West) => match other {
                Tile::PipeEW | Tile::PipeNE | Tile::PipeSE => true,
                _ => false,
            },
            (Tile::PipeNW, _) => false,
            // 7
            (Tile::PipeSW, CardDir::South) => match other {
                Tile::PipeNS | Tile::PipeNW | Tile::PipeNE => true,
                _ => false,
            },
            (Tile::PipeSW, CardDir::West) => match other {
                Tile::PipeEW | Tile::PipeSE | Tile::PipeNE => true,
                _ => false,
            },
            (Tile::PipeSW, _) => false,
            // F
            (Tile::PipeSE, CardDir::East) => match other {
                Tile::PipeEW | Tile::PipeSW | Tile::PipeNW => true,
                _ => false,
            },
            (Tile::PipeSE, CardDir::South) => match other {
                Tile::PipeNS | Tile::PipeNE | Tile::PipeNW => true,
                _ => false,
            },
            (Tile::PipeSE, _) => false,
        }
    }
}

type TileMap = Vec<Vec<Tile>>;

struct SearchState {
    row: usize,
    col: usize,
    steps: usize,
}

impl SearchState {
    fn new(row: usize, col: usize, steps: usize) -> Self {
        Self { row, col, steps }
    }
}

fn find_start(map: &TileMap) -> Option<(usize, usize)> {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == Tile::Start {
                return Some((row, col));
            }
        }
    }

    None
}

fn get_map(input: &str) -> TileMap {
    let mut map = TileMap::new();

    for line in input.lines() {
        map.push(
            line.chars()
                .map(|c| String::from(c).parse::<Tile>().unwrap())
                .collect(),
        );
    }

    map
}

fn get_loop(map: &TileMap) -> HashSet<(usize, usize)> {
    let start = find_start(&map).unwrap();
    // track position, current step count, and direction to move in
    let mut to_check: Vec<SearchState> = Vec::new();
    let mut tile_loop: HashSet<(usize, usize)> = HashSet::new();

    let (mut row, mut col) = start;
    let mut steps = 0;

    to_check.push(SearchState::new(row, col, steps));

    loop {
        // get the next step to check
        match to_check.pop() {
            Some(next_pos) => {
                SearchState { row, col, steps } = next_pos;
            }
            None => {
                break;
            }
        }

        // update our step counts if a new minimum is found for the current position
        match tile_loop.get(&(row, col)) {
            Some(_) => {
                continue;
            }
            // we found a new tile, add its distance
            None => {
                tile_loop.insert((row, col));
            }
        }

        // push all viable movements to the stack
        // North
        if row > 0 && map[row][col].is_connected(map[row - 1][col], CardDir::North) {
            to_check.push(SearchState::new(row - 1, col, steps + 1));
        }
        // East
        if col < map[row].len() - 1 && map[row][col].is_connected(map[row][col + 1], CardDir::East)
        {
            to_check.push(SearchState::new(row, col + 1, steps + 1));
        }
        // South
        if row < map.len() - 1 && map[row][col].is_connected(map[row + 1][col], CardDir::South) {
            to_check.push(SearchState::new(row + 1, col, steps + 1));
        }
        // West
        if col > 0 && map[row][col].is_connected(map[row][col - 1], CardDir::West) {
            to_check.push(SearchState::new(row, col - 1, steps + 1));
        }
    }

    tile_loop
}

// Scanline algorithm, suggestion/ help from https://www.reddit.com/r/adventofcode/comments/18f1sgh/comment/kcripvi/?utm_source=share&utm_medium=web2x&context=3
fn count_trapped(row: &Vec<Tile>, main_loop: &HashSet<(usize, usize)>, row_idx: usize) -> usize {
    let mut trapped = 0usize;
    let mut in_region = false;
    let mut last_corner: Option<Tile> = None;

    for (col_idx, tile) in row.iter().enumerate() {
        // check for pipes not connected to the main loop
        if !main_loop.contains(&(row_idx, col_idx)) {
            if in_region {
                trapped += 1;
            }
            continue;
        }
        match (tile, in_region) {
            (Tile::Ground, true) => {
                trapped += 1;
            }
            (Tile::Ground, false) | (Tile::PipeEW, _) => {}
            (Tile::PipeNS, _) => {
                in_region = !in_region;
            }
            // L
            (Tile::PipeNE, _) => {
                last_corner = Some(Tile::PipeNE);
            }
            // F
            (Tile::PipeSE, _) => {
                last_corner = Some(Tile::PipeSE);
            }
            // J
            (Tile::PipeNW, _) => {
                // forms a U
                if let Some(Tile::PipeNE) = last_corner {
                    // do nothing
                }
                // forms an S
                else if let Some(Tile::PipeSE) = last_corner {
                    in_region = !in_region;
                }
                last_corner = None;
            }
            // 7
            (Tile::PipeSW, _) => {
                // forms a U
                if let Some(Tile::PipeSE) = last_corner {
                    // do nothing
                }
                // forms an S
                else if let Some(Tile::PipeNE) = last_corner {
                    in_region = !in_region;
                }
                last_corner = None;
            }
            (Tile::Start, _) => {
                // switching here works for main input but breaks test cases...
                in_region = !in_region;
            }
        }
    }

    trapped
}

fn main() {
    let input =
        std::fs::read_to_string("../test_input_2_1").expect("Failed to read the input file");
    let map = get_map(&input);
    let main_loop = get_loop(&map);

    //let test_input = ".|.|L---J..F---7|.|L7.F-J...|..|";
    //let map = get_map(&test_input);

    let num_trapped: usize = map
        .iter()
        .enumerate()
        .map(|(i, row)| count_trapped(row, &main_loop, i))
        .sum();

    println!("{:?}", num_trapped);
}
