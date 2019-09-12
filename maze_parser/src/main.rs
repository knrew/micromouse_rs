use maze_parser::*;

fn main() {
    const MAZE_FOLDER_PATH: &str = "./micromouse_maze_tool/mazefiles/text/";
    const OUTPUT_FOLDER_PATH: &str = "./maze_data/";
    const MAZE_NAME: &str = "maze.txt";

    let maze_path = format!("{}/{}", MAZE_FOLDER_PATH.to_string(), MAZE_NAME);
    let output_path = format!("{}/{}", OUTPUT_FOLDER_PATH.to_string(), MAZE_NAME);

    println!("input:{} -> output:{}", maze_path, output_path);

    match process(&maze_path, &output_path) {
        Ok(_) => {}
        Err(e) => panic!(e),
    }
}

fn process(input_file: &str, output_file: &str) -> Result<(), String> {
    let raw_maze = match file_io::read(&input_file) {
        Ok(o) => o,
        Err(e) => return Err(e)
    };

    println!("{}", (raw_maze.len() - 1) / 2);

    let maze = parser::parse(&raw_maze, (raw_maze.len() - 1) / 2);

    println!("");
    wall::Wall::print_hex(&maze);

    match file_io::write(&output_file, &maze) {
        Ok(_) => {}
        Err(e) => return Err(format!("{}", e))
    }

    Ok(())
}