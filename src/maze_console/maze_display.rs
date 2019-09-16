use crate::wall;

struct ConsoleManager {
    term: console::Term,
    size: usize,
}

impl ConsoleManager {
    fn new() -> ConsoleManager {
        ConsoleManager { term: console::Term::stdout(), size: 0 }
    }

    fn write_new_line(&mut self, s: &str) -> std::io::Result<()> {
        match self.term.write_line(s) {
            Ok(_) => {}
            Err(e) => return Err(e)
        }
        self.size += 1;

        Ok(())
    }

    fn write_new_styled_objects(&mut self, objects: &Vec<console::StyledObject<char>>) -> std::io::Result<()> {
        let mut s = String::new();
        for o in objects.iter() {
            s = format!("{}{}", s, o);
        }

        self.write_new_line(&s)
    }

    fn write_line(&mut self, n: usize, s: &str) -> std::io::Result<()> {
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

    fn write_styled_objects(&mut self, n: usize, objects: &Vec<console::StyledObject<char>>) -> std::io::Result<()> {
        let mut s = String::new();
        for o in objects.iter() {
            s = format!("{}{}", s, o);
        }

        self.write_line(n, &s)
    }
}

struct MazeForConsole {
    pub maze: Vec<Vec<console::StyledObject<char>>>,
    pub size: usize,
}

impl MazeForConsole {
    fn new(size: usize) -> MazeForConsole {
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

        MazeForConsole { maze: maze, size: size }
    }

    fn get_num_line(&self) -> usize { self.size * 2 + 1 }

    fn get_num_col(&self) -> usize { self.size * 4 + 1 }

    fn set(&mut self, x: usize, y: usize, c: &console::StyledObject<char>) -> Result<(), String> {
        if x >= self.get_num_col() || y >= self.get_num_line() {
            return Err("invalid range.".to_string());
        }
        self.maze[y][x] = c.clone();
        Ok(())
    }

//    pub fn get(&self, x: usize, y: usize) -> maze_console::StyledObject<char> {
//        self.maze[(self.maze_size - y) * 2][2 + x * 3]
//    }

    fn set_by_coordinate(&mut self, x: usize, y: usize, c: &console::StyledObject<char>) -> Result<(), String> {
        let x = 2 + x * 4;
        let y = (self.size - y) * 2 + 1 - 2;
        self.set(x, y, &c)
    }

    fn set_wall(&mut self, x: usize, y: usize, wall: &wall::Wall) -> Result<(), String> {
        let x = 2 + x * 4;
        let y = (self.size - y) * 2 + 1 - 2;

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

    fn size(&self) -> usize { self.size }
}

//MazeDisplay::new()を呼び出したあとはprintしないこと(ずれるので)
pub struct MazeDisplay {
    display: ConsoleManager,
    maze: MazeForConsole,
}

impl MazeDisplay {
    pub fn new(size: usize) -> Result<MazeDisplay, std::io::Error> {
        let mut md = MazeDisplay { display: ConsoleManager::new(), maze: MazeForConsole::new(size) };

        for (_, line) in md.maze.maze.iter().enumerate() {
            match md.display.write_new_styled_objects(line) {
                Ok(_) => {}
                Err(e) => return Err(e)
            }
        }

        Ok(md)
    }

    pub fn size(&self) -> usize { self.maze.size() }

    pub fn set_wall(&mut self, x: usize, y: usize, w: &wall::Wall) -> Result<(), String> {
        match self.maze.set_wall(x, y, w) {
            Ok(_) => {}
            Err(e) => return Err(e),
        };

        for i in 0..3 {
            let l = (self.size() - y) * 2 - 2 + i;
            if l < self.maze.get_num_line() {
                match self.display.write_styled_objects(l, &self.maze.maze[l]) {
                    Ok(_) => {}
                    Err(e) => return Err(format!("{}", e)),
                }
            }
        }

        Ok(())
    }

    pub fn visit(&mut self, x: usize, y: usize, c: &console::StyledObject<char>) -> Result<(), String> {
        match self.maze.set_by_coordinate(x, y, c) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        let l = (self.size() - y) * 2 - 1;
        match self.display.write_styled_objects(l, &self.maze.maze[l]) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{}", e)),
        }
    }

    pub fn connect(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) -> Result<(), String> {
        let y;
        if x0 == x1 {
            let x = 2 + x0 * 4;
            y = (self.size() - std::cmp::max(y0, y1)) * 2;
            match self.maze.set(x, y, &console::style('|').red()) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        } else if y0 == y1 {
            let x = 2 + std::cmp::min(x0, x1) * 4 + 2;
            y = (self.size() - y0) * 2;
            for i in 0..3 {
                match self.maze.set(x + i, y, &console::style('-').red()) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }
        } else {
            return Err("invalid arguments.".to_string());
        }

        let l = y.clone();
        match self.display.write_styled_objects(l, &self.maze.maze[l]) {
            Ok(_) => {}
            Err(e) => return Err(format!("{}", e)),
        };

        Ok(())
    }
}