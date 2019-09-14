fn main() {
    const MAZE_SIZE: i32 = 16;
    const MAZE_NAME: &str = "maze0037.txt";
    const PROGRAM_NAME: &str = "examples/search_exsample";

    let micromouse_root: &str = &format!("{}/micromouse/", dirs::home_dir().unwrap().display());
    let build_dir: &str = "./build/";
    let maze_file: &str = &format!("{}/maze_data/{}", micromouse_root, MAZE_NAME);
    let search_route_file: &str = &format!("{}/search.csv", build_dir);
    let shortest_route_file: &str = &format!("{}/shortest.csv", build_dir);
    let solver_dir: &str = &format!("{}/maze_solver/", micromouse_root);
    let program: &str = &format!("{}/{}", build_dir, PROGRAM_NAME);

    let maze = visualizer::io::read_maze(maze_file).expect("failed to read maze_file.");

    if false {
        visualizer::gnu_plotter::plot_maze(&mut gnuplot::Figure::new(), &maze, MAZE_SIZE, true);
        return;
    }

    if true { //make solver
        process("mkdir", &["-p", build_dir].to_vec(), "./").expect("failed to mkdir build");
        process("cmake", &[solver_dir].to_vec(), build_dir).expect("failed to cmake");
        process("make", &[].to_vec(), build_dir).expect("failed to make");
    }

    process(&program, &[maze_file, search_route_file, shortest_route_file].to_vec(), "./").expect("");

    let search = visualizer::io::read_route(search_route_file).expect("failed to search route file.");
    let shortest = visualizer::io::read_route(shortest_route_file).expect("failed to shortest route file.");

    visualizer::gnu_plotter::plot(&maze, MAZE_SIZE, &search, &shortest, 0, true);
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

