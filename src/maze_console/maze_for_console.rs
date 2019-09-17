use crate::wall;

pub struct MazeForConsole {
    maze: Vec<Vec<console::StyledObject<char>>>,
    has_wall: Vec<Vec<bool>>,
    size: usize,
}

impl std::ops::Index<usize> for MazeForConsole {
    type Output = Vec<console::StyledObject<char>>;

    fn index(&self, i: usize) -> &Vec<console::StyledObject<char>> {
        &self.maze[i]
    }
}

impl MazeForConsole {
    pub fn new(size: usize) -> MazeForConsole {
        let mut maze: Vec<Vec<console::StyledObject<char>>> = Vec::new();
        maze.resize(size * 2 + 1, Vec::new());

        let mut has_wall: Vec<Vec<bool>> = Vec::new();
        has_wall.resize(size * 2 + 1, Vec::new());

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
                has_wall[i].push(if c == ' ' { false } else { true })
            }
        }

        MazeForConsole { maze: maze, has_wall: has_wall, size: size }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Vec<console::StyledObject<char>>> {
        self.maze.iter()
    }

    pub fn size(&self) -> usize { self.size }

    pub fn to_col_from_x(&self, x: usize) -> usize { 2 + x * 4 }

    pub fn to_line_from_y(&self, y: usize) -> usize { (self.size - y) * 2 - 1 }

    pub fn get_num_line(&self) -> usize { self.size * 2 + 1 }

    pub fn get_num_col(&self) -> usize { self.size * 4 + 1 }

    // 壁があっても上書き許可(壁描画用)
    fn forced_set(&mut self, col: usize, line: usize, c: &console::StyledObject<char>) -> Result<(), String> {
        if col >= self.get_num_col() || line >= self.get_num_line() {
            return Err("invalid range.".to_string());
        }

        self.maze[line][col] = c.clone();
        Ok(())
    }

    // 壁があれば描画されない(上書き禁止)
    pub fn set(&mut self, col: usize, line: usize, c: &console::StyledObject<char>) -> Result<(), String> {
        if self.has_wall[line][col] { return Ok(()); }
        self.forced_set(col, line, c)
    }

    pub fn set_by_coordinate(&mut self, x: usize, y: usize, c: &console::StyledObject<char>) -> Result<(), String> {
        self.forced_set(self.to_col_from_x(x), self.to_line_from_y(y), &c)
    }

    pub fn set_wall(&mut self, x: usize, y: usize, wall: &wall::Wall) -> Result<(), String> {
        let x = self.to_col_from_x(x);
        let y = self.to_line_from_y(y);

        if wall.n {
            for i in 0..3 {
                self.has_wall[y - 1][x - 1 + i] = true;
                match self.forced_set(x - 1 + i, y - 1, &console::style('-')) {
                    Ok(_) => {}
                    Err(e) => return Err(e)
                };
            }
        }
        if wall.e {
            self.has_wall[y][x + 2] = true;
            match self.forced_set(x + 2, y, &console::style('|')) {
                Ok(_) => {}
                Err(e) => return Err(e)
            };
        }
        if wall.s {
            for i in 0..3 {
                self.has_wall[y + 1][x - 1 + i] = true;
                match self.forced_set(x - 1 + i, y + 1, &console::style('-')) {
                    Ok(_) => {}
                    Err(e) => return Err(e)
                };
            }
        }
        if wall.w {
            self.has_wall[y][x - 2] = true;
            match self.forced_set(x - 2, y, &console::style('|')) {
                Ok(_) => {}
                Err(e) => return Err(e)
            };
        }

        Ok(())
    }
}