pub struct Wall {
    pub n: bool,
    pub e: bool,
    pub s: bool,
    pub w: bool,
    pub has_known_n: bool,
    pub has_known_e: bool,
    pub has_known_s: bool,
    pub has_known_w: bool,
}

pub type Maze = Vec<Vec<Wall>>;

impl Wall {
    pub fn new() -> Wall {
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

    pub fn from_str(s: &str) -> Result<Wall, String> {
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
                    _ => return Err(format!("{} is invalid character!", s))
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

    pub fn to_bit(&self) -> i32 {
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

    pub fn print_hex(maze: &Maze) {
        for i in maze.iter() {
            for (_, j) in i.iter().enumerate() {
                print!("{:x} ", j.to_bit());
            }
            println!("");
        }
    }

    pub fn print_bin(maze: &Maze) {
        for i in maze.iter() {
            for (_, j) in i.iter().enumerate() {
                print!("{:0>8b} ", j.to_bit());
            }
            println!("");
        }
    }

}



