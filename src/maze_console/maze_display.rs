use crate::wall;
use crate::maze_console::*;

// new()を呼び出したあとはprint!やprintln!しないこと(ずれるので)
// 文字列の出力が必要ならメンバ関数のprintを使用
pub struct ConsoleMaze {
    display: console_manager::ConsoleManager,
    maze: maze_for_console::MazeForConsole,
}

impl ConsoleMaze {
    pub fn new(size: usize) -> Result<ConsoleMaze, std::io::Error> {
        let mut display = console_manager::ConsoleManager::new();
        let maze = maze_for_console::MazeForConsole::new(size);

        for line in maze.iter() {
            match display.write_new_line_styled_objects(line) {
                Ok(_) => {}
                Err(e) => return Err(e)
            }
        }

        display.write_new_line_str("");

        Ok(ConsoleMaze { display: display, maze: maze })
    }

    pub fn size(&self) -> usize { self.maze.size() }

    // 座標(x, y)の四方の壁を指定して描画
    pub fn set_wall(&mut self, x: usize, y: usize, w: &wall::Wall) -> Result<(), String> {
        match self.maze.set_wall(x, y, w) {
            Ok(_) => {}
            Err(e) => return Err(e),
        };

        for i in 0..3 {
            let l = self.maze.to_line_from_y(y) - 1 + i;
            if l < self.maze.get_num_line() {
                match self.display.write_line_styled_objects(l, &self.maze[l]) {
                    Ok(_) => {}
                    Err(e) => return Err(format!("{}", e)),
                }
            }
        }

        Ok(())
    }

    // 座標(x, y)を訪れた座標として`*`でマーク．引数colorで描画する色を指定
    pub fn visit(&mut self, x: usize, y: usize, color: console::Color) -> Result<(), String> {
        match self.maze.set_by_coordinate(x, y, &console::style('*').fg(color)) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        let l = self.maze.to_line_from_y(y);
        match self.display.write_line_styled_objects(l, &self.maze[l]) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{}", e)),
        }
    }

    // (x0, y0), (x1, y1)間をつなぐ直線を描画(引数colorで色を指定)
    // 隣接しない座標を指定した場合エラー
    pub fn connect(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: console::Color) -> Result<(), String> {
        let y;

        if x0 == x1 {
            let x = self.maze.to_col_from_x(x0);
            y = self.maze.to_line_from_y(std::cmp::max(y0, y1)) + 1;
            match self.maze.set(x, y, &console::style('|').fg(color)) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        } else if y0 == y1 {
            let x = self.maze.to_col_from_x(std::cmp::min(x0, x1)) + 1;
            y = self.maze.to_line_from_y(y0);
            for i in 0..3 {
                match self.maze.set(x + i, y, &console::style('-').fg(color)) {
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

    fn cursor_for_print(&self) -> usize { self.maze.get_num_line() }

    // 迷路の下に1行文字列を表示
    pub fn print(&mut self, s: &str) -> std::io::Result<()> {
        self.display.write_line_str(self.cursor_for_print(), s)
    }
}
