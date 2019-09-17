//! コンソールへの描画を制御するクラス
//! `write_new_*`で新しい行へ出力
//! `wirte_*`で`n`で指定した行を書き換える(`n`は上から数えた行数)．

pub struct ConsoleManager {
    term: console::Term,
    size: usize,
}

impl ConsoleManager {
    pub fn new() -> ConsoleManager {
        ConsoleManager { term: console::Term::stdout(), size: 0 }
    }

    pub fn write_line_str(&mut self, n: usize, s: &str) -> std::io::Result<()> {
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

    pub fn write_new_line_str(&mut self, s: &str) -> std::io::Result<()> {
        match self.term.write_line(s) {
            Ok(_) => {}
            Err(e) => return Err(e)
        }
        self.size += 1;

        Ok(())
    }

    pub fn write_line_styled_objects(&mut self, n: usize, objects: &Vec<console::StyledObject<char>>) -> std::io::Result<()> {
        let mut s = String::new();
        for o in objects.iter() {
            s = format!("{}{}", s, o);
        }

        self.write_line_str(n, &s)
    }

    pub fn write_new_line_styled_objects(&mut self, objects: &Vec<console::StyledObject<char>>) -> std::io::Result<()> {
        let mut s = String::new();
        for o in objects.iter() {
            s = format!("{}{}", s, o);
        }

        let s: &str = &s;
        self.write_new_line_str(&s)
    }
}
