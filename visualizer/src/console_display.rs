use crate::wall;

pub struct Display {
    pub term: console::Term,
    pub size: usize,
}

impl Display {
    pub fn new() -> Display {
        Display { term: console::Term::stdout(), size: 0 }
    }

    pub fn write_new_line(&mut self, s: &str) -> std::io::Result<()> {
        match self.term.write_line(s) {
            Ok(_) => {}
            Err(e) => return Err(e)
        }
        self.size += 1;

        Ok(())
    }

    pub fn write_line(&mut self, n: usize, s: &str) -> std::io::Result<()> {
        match self.term.move_cursor_up(self.size - n) {
            Ok(_) => {}
            Err(e) => return Err(e)
        };

        match self.term.clear_line() {
            Ok(_) => {}
            Err(e) => return Err(e)
        }

        match self.term.write_line(s) {
            Ok(_) => {}
            Err(e) => return Err(e)
        }

        match self.term.move_cursor_down(self.size - n) {
            Ok(_) => {}
            Err(e) => return Err(e)
        };

        Ok(())
    }
}

pub struct ConsoleMaze {
    pub size: usize,
    pub maze: Vec<String>,
}

impl ConsoleMaze {
    pub fn new(size: usize) -> ConsoleMaze {
        let mut maze: Vec<String> = Vec::new();
        for i in 0..(size * 2 + 1) {
            let mut s = String::new();

            if i % 2 != 0 {
                s = format!("|   ");
                for j in 0..(size - 1) {
                    s = format!("{}    ", s);
                }
                s = format!("{}|", s);
            } else {
                s = format!("o");
                for j in 0..size {
                    if i == 0 || i == size * 2 {
                        s = format!("{}---o", s);
                    } else {
                        s = format!("{}   o", s);
                    }
                }
            }
            maze.push(s);
        }

        ConsoleMaze { maze: maze, size: size }
    }

    fn set(&mut self, x: usize, y: usize, c: char) -> Result<(), String> {
//        if (y > self.size) { return Err("invalid y size.".to_string()); }
        let mut tmp: String = String::new();
        for (i, s) in self.maze[y].chars().enumerate() {
            if (i == x) {
                tmp.push(c);
            } else {
                tmp.push(s);
            }
        }
        self.maze[y] = tmp;
        Ok(())
    }

    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        let x = 2 + x * 3;
        let y = (self.size - y) * 2;
        match self.maze[y].chars().nth(x) {
            Some(c) => Some(c),
            None => None
        }
    }

    pub fn set_coordinate(&mut self, x: usize, y: usize, c: char) {
        let x = 2 + x * 4;
        let y = (self.size - y) * 2 + 1 - 2;
        self.set(x, y, c);
    }

    pub fn set_wall(&mut self, x: usize, y: usize, wall: &wall::Wall) -> Result<(), String> {
        let x = 2 + x * 4;
        let y = (self.size - y) * 2 + 1 - 2;

        if wall.n {
            self.set(x - 1, y - 1, '-');
            self.set(x, y - 1, '-');
            self.set(x + 1, y - 1, '-');
        }
        if wall.e {
            self.set(x + 2, y, '|');
        }
        if wall.s {
            self.set(x - 1, y + 1, '-');
            self.set(x, y + 1, '-');
            self.set(x + 1, y + 1, '-');
        }
        if wall.w {
            self.set(x - 2, y, '|');
        }

        Ok(())
    }
}

