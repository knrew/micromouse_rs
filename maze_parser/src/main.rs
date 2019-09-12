fn main() {
    const MAZE_NAME: &str = "japan2017eq.txt";
    const OUTPUT: &str = "/home/ryunosuke/Music/poyo.txt";

    let raw_maze = match read(MAZE_NAME) {
        Ok(o) => o,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let mut maze: Vec<Vec<Wall>> = Vec::new();
    maze.resize_with(16, Default::default);
    for m in maze.iter_mut() { for _i in 0..16 { m.push(Wall::new()); } }

    let mut r: usize = 0;
    for (row, line) in raw_maze.iter().enumerate() {
        println!("{}", line);

        if row != 0 && row % 2 == 0 { r += 1; }

        if row % 2 == 0 {
            let line: Vec<&str> = line.split('o').collect();

            let mut c: usize = 0;
            for s in line {
                if s.to_string() == "".to_string() { continue; }
                if s.to_string() == "---".to_string() {
                    if r <= 15 { maze[r][c].n = true; }
                    if r >= 1 && r <= 16 { maze[r - 1][c].s = true; }
                    c += 1;
                } else if s.to_string() == "   ".to_string() {
                    c += 1;
                }
            }
        } else {
            let mut c: usize = 0;
            let mut count = 0;
            for (_, s) in line.chars().enumerate() {
                if s == '|' {
                    if c <= 15 { maze[r][c].w = true; }
                    if c >= 1 && c <= 16 { maze[r][c - 1].e = true; }
                    c += 1;
                    count = 0;
                }
                if s == ' ' {
                    if count == 3 {
                        c += 1;
                        count = 0;
                    } else {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("");
    for i in maze.iter() {
        for (_, j) in i.iter().enumerate() {
            print!("{:x} ", j.to_bit());
        }
        println!("");
    }

    use std::io::Write;
    let file = match std::fs::File::create(OUTPUT) {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let mut writer = std::io::BufWriter::new(file);

    for i in maze.iter() {
        for j in i.iter() {
          writer.write(format!("{:x} ", j.to_bit()).as_bytes()).expect("writing failed.");
        }
        writer.write("\n".to_string().as_bytes()).expect("writing failed.");
    }
}

#[allow(dead_code)]
struct Wall {
    n: bool,
    e: bool,
    s: bool,
    w: bool,
    has_known_n: bool,
    has_known_e: bool,
    has_known_s: bool,
    has_known_w: bool,
}

impl Wall {
    fn new() -> Wall {
        Wall {
            n: false,
            e: false,
            s: false,
            w: false,
            has_known_n: false,
            has_known_e: false,
            has_known_s: false,
            has_known_w: false,
        }
    }

    #[allow(dead_code)]
    fn from_str(s: &str) -> Result<Wall, String> {
        if s.len() != 1 { return Err("Input ONE character!".to_string()); }

        let num = match s.parse::<u8>() {
            Ok(o) => o,
            Err(_) => {
                match s {
                    "a" => 10,
                    "b" => 11,
                    "c" => 12,
                    "d" => 13,
                    "e" => 14,
                    "f" => 15,
                    _ => return Err("Invalid character!".to_string())
                }
            }
        };

        let wall_bin = format!("{:0>4b}", num);

        let mut wall = Wall::new();
        wall.n = if wall_bin.chars().nth(3) == Some('1') { true } else { false };
        wall.e = if wall_bin.chars().nth(2) == Some('1') { true } else { false };
        wall.s = if wall_bin.chars().nth(1) == Some('1') { true } else { false };
        wall.w = if wall_bin.chars().nth(0) == Some('1') { true } else { false };

        Ok(wall)
    }

    #[allow(dead_code)]
    fn to_bit(&self) -> i32 {
        let mut ret: i32 = 0;

        fn pow(a: i32, b: i32) -> i32 { if b == 0 { 1 } else { a * pow(a, b - 1) } }

        if self.n { ret += 1; }
        if self.e { ret += 2; };
        if self.s { ret += pow(2, 2); };
        if self.w { ret += pow(2, 3); };
        if self.has_known_n { ret += pow(2, 4); };
        if self.has_known_e { ret += pow(2, 5); };
        if self.has_known_s { ret += pow(2, 6); };
        if self.has_known_w { ret += pow(2, 7); };

        ret
    }
}

fn read(maze_name: &str) -> Result<Vec<String>, String> {
    let maze_path = match get_maze_path(maze_name) {
        Some(s) => s,
        None => return Err("".to_string())
    };

    let reader = match std::fs::File::open(maze_path) {
        Ok(f) => f,
        Err(e) => return Err(format!("{}", e)),
    };

    use std::io::BufRead;
    let reader = std::io::BufReader::new(reader).lines();

    let mut maze: Vec<String> = Vec::new();

    for r in reader {
        let line = match r {
            Ok(s) => s,
            Err(e) => return Err(format!("{}", e)),
        };
        maze.push(line);
    }

    Ok(maze)
}

fn get_maze_path(maze_name: &str) -> Option<String> {
    let file_name = match dirs::home_dir() {
        Some(s) => s,
        None => return None
    };

    let file_name = match file_name.to_str() {
        Some(s) => s,
        None => return None
    };

    let ret = format!("{}/Documents/rust_projects/maze_parser/micromouse_maze_tool/mazefiles/text/{}", file_name.to_string(), maze_name.to_string());

    Some(ret)
}