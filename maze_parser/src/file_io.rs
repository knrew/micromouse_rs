use crate::wall;

pub fn read(maze_file: &str) -> Result<Vec<String>, String> {
    let reader = match std::fs::File::open(maze_file) {
        Ok(f) => f,
        Err(e) => return Err(format!("{}", e)),
    };

    use std::io::BufRead;
    let reader = std::io::BufReader::new(reader).lines();

    let mut maze: Vec<String> = Vec::new();

    for r in reader {
        let line = match r {
            Ok(s) => s,
            Err(e) => return Err(format!("{}", e)),
        };
        maze.push(line);
    }

    Ok(maze)
}

pub fn write_strs(file_name: &str, strings: &Vec<String>) -> Result<(), std::io::Error> {
    use std::io::Write;
    let file = match std::fs::File::create(file_name) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut writer = std::io::BufWriter::new(file);

    for s in strings.iter() {
        let buf = format!("{}\n", s);
        match writer.write(buf.as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
    }

    Ok(())
}

pub fn write_maze(file_name: &str, maze: &wall::Maze) -> Result<(), std::io::Error> {
    use std::io::Write;
    let file = match std::fs::File::create(file_name) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut writer = std::io::BufWriter::new(file);

    for i in maze.iter() {
        for j in i.iter() {
            match writer.write(format!("{:x} ", j.to_bit()).as_bytes()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        }
        match writer.write("\n".to_string().as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
    }

    Ok(())
}