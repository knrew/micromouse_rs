struct MyDisplay {
    term: console::Term,
    size: usize,
}

impl MyDisplay {
    fn new() -> MyDisplay {
        MyDisplay { term: console::Term::stdout(), size: 0 }
    }

    fn write_new_line(&mut self, s: &str) -> std::io::Result<()> {
        match self.term.write_line(s) {
            Ok(_) => {}
            Err(e) => return Err(e)
        }
        self.size += 1;

        Ok(())
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
}


fn main() {
    println!("----------");
    println!("----------");
    println!("----------");

    let mut display = MyDisplay::new();
    let mut default: Vec<String> = Vec::new();
    for i in 0..16 {
        let mut s: String = String::new();
        for j in 0..16 {
            s = format!("{}{:>3} ", s, i + 16 * j);
        }
        display.write_new_line(&s).unwrap();
        default.push(s);
    }

    for i in 0..16 {
        if (i >= 1) {
            display.write_line(i - 1, &default[i - 1]);
        }

        display.write_line(i, "poyo");

        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    display.write_line(15, &default[15]);
}

