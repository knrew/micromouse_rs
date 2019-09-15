use micromouse_rs::*;

fn main() {
    const MAZE_SIZE: usize = 16;
    const MAZE_NAME: &str = "maze0000.txt";
    const PROGRAM_NAME: &str = "examples/search_exsample";

    let micromouse_root: &str = &format!("{}/micromouse/", dirs::home_dir().unwrap().display());
    let build_dir: &str = "./build/";
    let maze_file: &str = &format!("{}/maze_data/{}", micromouse_root, MAZE_NAME);
    let search_route_file: &str = &format!("{}/search.csv", build_dir);
    let shortest_route_file: &str = &format!("{}/shortest.csv", build_dir);
    let solver_dir: &str = &format!("{}/maze_solver/", micromouse_root);
    let program: &str = &format!("{}/{}", build_dir, PROGRAM_NAME);

    let maze = io::read_maze(maze_file).expect("failed to read maze_file.");

    if false { //make solver
        process("mkdir", &["-p", build_dir].to_vec(), "./").expect("failed to mkdir build");
        process("cmake", &[solver_dir].to_vec(), build_dir).expect("failed to cmake");
        process("make", &[].to_vec(), build_dir).expect("failed to make");
    }

    process(&program, &[maze_file, search_route_file, shortest_route_file].to_vec(), "./").expect("");

    let search = io::read_route(search_route_file).expect("failed to search route file.");
    let shortest = io::read_route(shortest_route_file).expect("failed to shortest route file.");

    println!("----------");
    println!("----------");
    println!("----------");

    let mut maze_display = maze_console::maze_display::MazeDisplay::new(MAZE_SIZE).expect("failed to initialize display.");

    for (i, line) in maze.iter().enumerate() {
        for (j, w) in line.iter().enumerate() {
            let x = j;
            let y = MAZE_SIZE - 1 - i;
            maze_display.set_wall(x, y, w).expect("failed to set wall.");
        }
    }

//    maze_display.set_by_coordinate(0, 0, &maze_console::style('A').blue()).expect("failed to set.");

    for (i, _) in search.y.iter().enumerate() {
        if i >= 1 {
            let x = (search.x[i - 1] - 0.5) as usize;
            let y = (search.y[i - 1] - 0.5) as usize;
            maze_display.set(x, y, &console::style('*').blue()).expect("failed to set.");
        }

        let x = (search.x[i] - 0.5) as usize;
        let y = (search.y[i] - 0.5) as usize;
        maze_display.set(x, y, &console::style('*').red()).expect("failed to set.");

        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    for (i, _) in shortest.y.iter().enumerate() {
        let x = (shortest.x[i] - 0.5) as usize;
        let y = (shortest.y[i] - 0.5) as usize;
        maze_display.set(x, y, &console::style('x').red()).expect("failed to set.");
    }

    return;
}

fn process(program: &str, args: &Vec<&str>, dir: &str) -> Result<std::process::ExitStatus, std::io::Error> {
    match match std::process::Command::new(program)
        .args(args)
        .current_dir(dir)
        .stdout(std::process::Stdio::inherit())
        .spawn() {
        Ok(o) => o,
        Err(e) => return Err(e),
    }.wait() {
        Ok(o) => Ok(o),
        Err(e) => Err(e)
    }
}

