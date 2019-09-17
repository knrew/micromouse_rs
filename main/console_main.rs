use micromouse_rs::*;

mod parameters;

fn main() {
    let params = parameters::Parameters::new();

    bash_process::process(&params.program, &[&params.maze_file as &str, &params.search_route_file, &params.shortest_route_file].to_vec(), "./").expect("failed to run solver.");

    let maze = io::read_maze(&params.maze_file).expect("failed to read maze_file.");
    let search = io::read_route(&params.search_route_file).expect("failed to search route file.");
    let shortest = io::read_route(&params.shortest_route_file).expect("failed to shortest route file.");

    bash_process::process("rm", &[&params.search_route_file as &str, &params.shortest_route_file].to_vec(), "./").expect("failed to delete file.");

    let mut console_maze = maze_console::maze_display::ConsoleMaze::new(params.maze_size).expect("failed to initialize display.");
    console_maze.print("start!").expect("");

    for (i, line) in maze.iter().enumerate() {
        for (j, w) in line.iter().enumerate() {
            let x = j;
            let y = params.maze_size - 1 - i;
            console_maze.set_wall(x, y, w).expect("failed to set wall.");
        }
    }

    for (i, _) in search.y.iter().enumerate() {
        if i >= 1 {
            let x = search.x[i - 1] as usize;
            let y = search.y[i - 1] as usize;
            console_maze.visit(x, y, console::Color::Blue).expect("failed to set.");
        }

        let x = search.x[i] as usize;
        let y = search.y[i] as usize;
        console_maze.visit(x, y, console::Color::Red).expect("failed to set.");

        const RUNNING: [&str; 4] = ["running", "running.", "running..", "running..."];
        console_maze.print(RUNNING[i % 4]).expect("");

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    for (i, _) in shortest.y.iter().enumerate() {
        let x = (shortest.x[i]) as usize;
        let y = (shortest.y[i]) as usize;
        console_maze.visit(x, y, console::Color::Red).expect("failed to set.");
        if i >= 1 {
            console_maze.connect(x, y, shortest.x[i - 1] as usize, shortest.y[i - 1] as usize, console::Color::Red).expect("failed to connect.");
        }
    }

    console_maze.print("finish.").expect("");
}