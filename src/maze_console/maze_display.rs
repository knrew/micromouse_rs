use crate::wall;
use crate::maze_console::*;
use console::StyledObject;

//new()を呼び出したあとはprintしないこと(ずれるので)
pub struct ConsoleMaze {
    display: console_manager::ConsoleManager,
    maze: maze_for_console::MazeForConsole,
}

impl ConsoleMaze {
    pub fn new(size: usize) -> Result<ConsoleMaze, std::io::Error> {
        let mut md = ConsoleMaze { display: console_manager::ConsoleManager::new(), maze: maze_for_console::MazeForConsole::new(size) };

        for (_, line) in md.maze.iter().enumerate() {
            match md.display.write_new_line_styled_objects(line) {
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
                match self.display.write_line_styled_objects(l, &self.maze[l]) {
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
        match self.display.write_line_styled_objects(l, &self.maze[l]) {
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
            let x = 2 + std::cmp::min(x0, x1) * 4 + 1;
            y = (self.size() - y0) * 2 - 1;
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
        match self.display.write_line_styled_objects(l, &self.maze[l]) {
            Ok(_) => {}
            Err(e) => return Err(format!("{}", e)),
        };

        Ok(())
    }
}
