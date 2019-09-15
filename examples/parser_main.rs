use micromouse_rs::*;

fn main() {
    let micromouse_root: &str = &format!("{}/micromouse/", dirs::home_dir().unwrap().display());
    let maze_folder_path: &str = &format!("{}/micromouse_maze_tool/mazefiles/text/", micromouse_root);
    let output_dir: &str = &format!("{}/micromouse_rs/maze_data/", micromouse_root);
    let readme_file: &str = &format!("{}/README.txt", output_dir);

    let maze_names = match get_maze_names(maze_folder_path) {
        Ok(o) => o,
        Err(e) => panic!(e)
    };

    let mut readme: Vec<String> = Vec::new();
    readme.push(format!("source : https://github.com/micromouseonline/micromouse_maze_tool"));
    for (i, n) in maze_names.iter().enumerate() {
        readme.push(format!("{:0>4} : {}", i, n));

        let maze_path = format!("{}/{}", maze_folder_path.to_string(), n);
        let output_path = format!("{}/maze{:0>4}.txt", output_dir.to_string(), i);

        match process(&maze_path, &output_path) {
            Ok(_) => {}
            Err(e) => panic!(e),
        }
    }

    match write_strs(readme_file, &readme) {
        Ok(_) => {}
        Err(e) => panic!(e),
    }
}

fn process(input_file: &str, output_file: &str) -> Result<(), String> {
    let raw_maze = match read(&input_file) {
        Ok(o) => o,
        Err(e) => return Err(e)
    };

//    println!("{}", (raw_maze.len() - 1) / 2);

    let maze = parse(&raw_maze, (raw_maze.len() - 1) / 2);

//    println!("");
//    wall::Wall::print_hex(&maze);

    match write_maze(&output_file, &maze) {
        Ok(_) => {}
        Err(e) => return Err(format!("{}", e))
    }

    Ok(())
}

fn get_maze_names(folder_path: &str) -> Result<Vec<String>, String> {
    let mut maze_names: Vec<String> = Vec::new();

    let dirs = match std::fs::read_dir(folder_path) {
        Ok(o) => o,
        Err(e) => return Err(format!("{}", e)),
    };
    for d in dirs {
        let s = match d {
            Ok(o) => o,
            Err(e) => return Err(format!("{}", e)),
        };
        let s = format!("{}", s.path().display());
        let parsed: Vec<&str> = s.rsplit("/").collect();
        maze_names.push(parsed[0].to_string());
    }

    Ok(maze_names)
}

pub fn parse(raw_maze: &Vec<String>, maze_size: usize) -> wall::Maze {
    let mut maze: wall::Maze = Vec::new();
    maze.resize_with(maze_size, Default::default);
    for m in maze.iter_mut() { for _i in 0..maze_size { m.push(wall::Wall::new()); } }

    let mut r: usize = 0;
    for (row, line) in raw_maze.iter().enumerate() {
//        println!("{}", line);

        if row != 0 && row % 2 == 0 { r += 1; }

        if row % 2 == 0 {
            let line: Vec<&str> = line.split('o').collect();

            let mut c: usize = 0;
            for s in line {
                if s.to_string() == "".to_string() { continue; }
                if s.to_string() == "---".to_string() {
                    if r <= maze_size - 1 { maze[r][c].n = true; }
                    if r >= 1 && r <= maze_size { maze[r - 1][c].s = true; }
                    c += 1;
                } else if s.to_string() == "   ".to_string() {
                    c += 1;
                }
            }
        } else {
            let mut c: usize = 0;
            let mut count = 0;
            for (_, s) in line.chars().enumerate() {
                if s == '|' {
                    if c <= maze_size - 1 { maze[r][c].w = true; }
                    if c >= 1 && c <= maze_size { maze[r][c - 1].e = true; }
                    c += 1;
                    count = 0;
                }
                if s == ' ' {
                    if count == 3 {
                        c += 1;
                        count = 0;
                    } else {
                        count += 1;
                    }
                }
            }
        }
    }

    maze
}

fn read(maze_file: &str) -> Result<Vec<String>, String> {
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

fn write_strs(file_name: &str, strings: &Vec<String>) -> Result<(), std::io::Error> {
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

fn write_maze(file_name: &str, maze: &wall::Maze) -> Result<(), std::io::Error> {
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