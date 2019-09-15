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
    pub maze_style: Vec<console::StyledObject<String>>,
    pub maze_str: Vec<String>,
}

impl ConsoleMaze {
    pub fn new(size: usize) -> ConsoleMaze {
        let mut maze_str: Vec<String> = Vec::new();
        let mut maze_style: Vec<console::StyledObject<String>> = Vec::new();
        for i in 0..(size * 2 + 1) {
            let mut s = String::new();

            if i % 2 != 0 {
                s = format!("|   ");
                for _j in 0..(size - 1) {
                    s = format!("{}    ", s);
                }
                s = format!("{}|", s);
            } else {
                s = format!("o");
                for _j in 0..size {
                    if i == 0 || i == size * 2 {
                        s = format!("{}---o", s);
                    } else {
                        s = format!("{}   o", s);
                    }
                }
            }
            maze_style.push(console::style(format!("{}", s)));
            maze_str.push(s);
        }

        ConsoleMaze { maze_style: maze_style, maze_str: maze_str, size: size }
    }

    fn set(&mut self, x: usize, y: usize, c: char) -> Result<(), String> {
        let mut tmp_str: String = String::new();
        let mut tmp_style: console::StyledObject<String>;
        for (i, s) in self.maze_str[y].chars().enumerate() {
            if i == x {
                tmp_str.push(c);
            } else {
                tmp_str.push(s);
            }
        }
        self.maze_str[y] = format!("{}", tmp_str);
        self.maze_style[y] = console::style(format!("{}", tmp_str));

        Ok(())
    }

    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        let x = 2 + x * 3;
        let y = (self.size - y) * 2;
        match self.maze_str[y].chars().nth(x) {
            Some(c) => Some(c),
            None => None
        }
    }

    pub fn set_by_coordinate(&mut self, x: usize, y: usize, c: char) -> Result<(), String> {
        let x = 2 + x * 4;
        let y = (self.size - y) * 2 + 1 - 2;
        self.set(x, y, c)
    }

    pub fn set_wall(&mut self, x: usize, y: usize, wall: &wall::Wall) -> Result<(), String> {
        let x = 2 + x * 4;
        let y = (self.size - y) * 2 + 1 - 2;

        if wall.n {
            match self.set(x - 1, y - 1, '-') {
                Ok(_) => {}
                Err(e) => return Err(e)
            };
            match self.set(x, y - 1, '-') {
                Ok(_) => {}
                Err(e) => return Err(e)
            };
            match self.set(x + 1, y - 1, '-') {
                Ok(_) => {}
                Err(e) => return Err(e)
            };
        }
        if wall.e {
            match self.set(x + 2, y, '|') {
                Ok(_) => {}
                Err(e) => return Err(e)
            };
        }
        if wall.s {
            match self.set(x - 1, y + 1, '-') {
                Ok(_) => {}
                Err(e) => return Err(e)
            };
            match self.set(x, y + 1, '-') {
                Ok(_) => {}
                Err(e) => return Err(e)
            };
            match self.set(x + 1, y + 1, '-') {
                Ok(_) => {}
                Err(e) => return Err(e)
            };
        }
        if wall.w {
            match self.set(x - 2, y, '|') {
                Ok(_) => {}
                Err(e) => return Err(e)
            };
        }

        Ok(())
    }
}

