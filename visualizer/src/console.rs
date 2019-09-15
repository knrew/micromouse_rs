use crate::wall;

pub struct Display {
    term: console::Term,
    size: usize,
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

    pub fn write_new_styled_objects(&mut self, objects: &Vec<console::StyledObject<char>>) -> std::io::Result<()> {
        let mut s = String::new();
        for o in objects.iter() {
            s = format!("{}{}", s, o);
        }

        self.write_new_line(&s)
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

    pub fn write_styled_objects(&mut self, n: usize, objects: &Vec<console::StyledObject<char>>) -> std::io::Result<()> {
        let mut s = String::new();
        for o in objects.iter() {
            s = format!("{}{}", s, o);
        }

        self.write_line(n, &s)
    }
}

pub struct ConsoleMaze {
    pub maze: Vec<Vec<console::StyledObject<char>>>,
    pub maze_size: usize,
}

impl ConsoleMaze {
    pub fn new(size: usize) -> ConsoleMaze {
        let mut maze: Vec<Vec<console::StyledObject<char>>> = Vec::new();
        maze.resize(size * 2 + 1, Vec::new());

        for i in 0..(size * 2 + 1) {
            let mut s;
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

            for c in s.chars() {
                maze[i].push(console::style(c));
            }
        }

        ConsoleMaze { maze: maze, maze_size: size }
    }

    fn set(&mut self, x: usize, y: usize, c: &console::StyledObject<char>) -> Result<(), String> {
        self.maze[y][x] = c.clone();
        Ok(())
    }

//    pub fn get(&self, x: usize, y: usize) -> console::StyledObject<char> {
//        self.maze[(self.maze_size - y) * 2][2 + x * 3]
//    }

    pub fn set_by_coordinate(&mut self, x: usize, y: usize, c: console::StyledObject<char>) -> Result<(), String> {
        let x = 2 + x * 4;
        let y = (self.maze_size - y) * 2 + 1 - 2;
        self.set(x, y, &c)
    }

    pub fn set_wall(&mut self, x: usize, y: usize, wall: &wall::Wall) -> Result<(), String> {
        let x = 2 + x * 4;
        let y = (self.maze_size - y) * 2 + 1 - 2;

        if wall.n {
            match self.set(x - 1, y - 1, &console::style('-')) {
                Ok(_) => {}
                Err(e) => return Err(e)
            };
            match self.set(x, y - 1, &console::style('-')) {
                Ok(_) => {}
                Err(e) => return Err(e)
            };
            match self.set(x + 1, y - 1, &console::style('-')) {
                Ok(_) => {}
                Err(e) => return Err(e)
            };
        }
        if wall.e {
            match self.set(x + 2, y, &console::style('|')) {
                Ok(_) => {}
                Err(e) => return Err(e)
            };
        }
        if wall.s {
            match self.set(x - 1, y + 1, &console::style('-')) {
                Ok(_) => {}
                Err(e) => return Err(e)
            };
            match self.set(x, y + 1, &console::style('-')) {
                Ok(_) => {}
                Err(e) => return Err(e)
            };
            match self.set(x + 1, y + 1, &console::style('-')) {
                Ok(_) => {}
                Err(e) => return Err(e)
            };
        }
        if wall.w {
            match self.set(x - 2, y, &console::style('|')) {
                Ok(_) => {}
                Err(e) => return Err(e)
            };
        }

        Ok(())
    }
}

//struct MazeDisplay {
//    c: ,
//    maze: ConsoleMaze,
//}

