use visualizer::wall::Wall;

fn main() {
    const MAZE_SIZE: i32 = 16;
    const MAZE_NAME: &str = "maze0000.txt";
    const PROGRAM_NAME: &str = "examples/search_exsample";

    let micromouse_root: &str = &format!("{}/micromouse/", dirs::home_dir().unwrap().display());
    let build_dir: &str = "./build/";
    let maze_file: &str = &format!("{}/maze_data/{}", micromouse_root, MAZE_NAME);
    let search_route_file: &str = &format!("{}/search.csv", build_dir);
    let shortest_route_file: &str = &format!("{}/shortest.csv", build_dir);
    let solver_dir: &str = &format!("{}/maze_solver/", micromouse_root);
    let program: &str = &format!("{}/{}", build_dir, PROGRAM_NAME);

    let mut display = visualizer::console_display::Display::new();
    let maze = visualizer::io::read_maze(maze_file).expect("failed to read maze_file.");

    if true { //make solver
        process("mkdir", &["-p", build_dir].to_vec(), "./").expect("failed to mkdir build");
        process("cmake", &[solver_dir].to_vec(), build_dir).expect("failed to cmake");
        process("make", &[].to_vec(), build_dir).expect("failed to make");
    }

    process(&program, &[maze_file, search_route_file, shortest_route_file].to_vec(), "./").expect("");

    let search = visualizer::io::read_route(search_route_file).expect("failed to search route file.");
    let shortest = visualizer::io::read_route(shortest_route_file).expect("failed to shortest route file.");

    let mut console_maze = visualizer::console_display::ConsoleMaze::new(MAZE_SIZE as usize);

    for (i, line) in maze.iter().enumerate() {
        for (j, w) in line.iter().enumerate() {
            let x = j;
            let y = MAZE_SIZE as usize - 1 - i;
            console_maze.set_wall(x, y, w);
        }
    }

    println!("----------");
    println!("----------");
    println!("----------");

    for (i, m) in console_maze.maze.iter().enumerate() {
        display.write_new_line(&format!("{}", m)).expect("aaaaa");
    }

    let mut count = search.x.len();
    display.write_new_line(&format!("{}", count)).expect("aaaaa");

    for (i, _) in search.y.iter().enumerate() {
        let x = (search.x[i] - 0.5) as usize;
        let y = (search.y[i] - 0.5) as usize;
        console_maze.set_coordinate(x, y, '*');

        let line = (MAZE_SIZE as usize - y) * 2 - 1;
        display.write_line(line, &console_maze.maze[line]);

        count -= 1;
        display.write_line(MAZE_SIZE as usize * 2 + 1, &format!("{}", count)).expect("aaaaaaaa");
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    return;

    println!("----------");
    println!("----------");
    println!("----------");

    let mut display = visualizer::console_display::Display::new();
    let mut default: Vec<String> = Vec::new();
    for i in 0..16 {
        let mut s: String = String::new();
        for j in 0..16 {
            s = format!("{}{:>3} ", s, i + 16 * j);
        }
        display.write_new_line(&s).expect("poyo");
        default.push(s);
    }

    for i in 0..16 {
        if i >= 1 {
            display.write_line(i - 1, &default[i - 1]).expect("poyo");
        }

        display.write_line(i, "poyo").expect("poyo");

        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    display.write_line(15, &default[15]).expect("poyo");
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

