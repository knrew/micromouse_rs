use maze_parser::*;

fn main() {
    const MAZE_FOLDER_PATH: &str = "./micromouse_maze_tool/mazefiles/text/";
    const OUTPUT_FOLDER_PATH: &str = "./maze_data/";
    const README_FILE: &str = "./maze_data/README.txt";

    let maze_names = match get_maze_names(MAZE_FOLDER_PATH) {
        Ok(o) => o,
        Err(e) => panic!(e)
    };

    let mut readme: Vec<String> = Vec::new();
    readme.push(format!("source : https://github.com/micromouseonline/micromouse_maze_tool"));
    for (i, n) in maze_names.iter().enumerate() {
        readme.push(format!("{:>4} : {}", i, n));

        let maze_path = format!("{}/{}", MAZE_FOLDER_PATH.to_string(), n);
        let output_path = format!("{}/maze{:0>4}.txt", OUTPUT_FOLDER_PATH.to_string(), i);

//        println!("input:{} -> output:{}", maze_path, output_path);

        match process(&maze_path, &output_path) {
            Ok(_) => {}
            Err(e) => panic!(e),
        }
    }

    match file_io::write_strs(README_FILE, &readme) {
        Ok(_) => {}
        Err(e) => panic!(e),
    }
}

fn process(input_file: &str, output_file: &str) -> Result<(), String> {
    let raw_maze = match file_io::read(&input_file) {
        Ok(o) => o,
        Err(e) => return Err(e)
    };

//    println!("{}", (raw_maze.len() - 1) / 2);

    let maze = parser::parse(&raw_maze, (raw_maze.len() - 1) / 2);

//    println!("");
//    wall::Wall::print_hex(&maze);

    match file_io::write_maze(&output_file, &maze) {
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