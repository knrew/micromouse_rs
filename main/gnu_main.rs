use micromouse_rs::*;

mod parameters;
use parameters::*;

fn main() {
    let maze = io::read_maze(&maze_file()).expect("failed to read maze_file.");

    if false {
        gnu_plotter::plot_maze(&mut gnuplot::Figure::new(), &maze, maze_size(), true);
        return;
    }

    bash_process::process(&program(), &[&maze_file() as &str, &search_route_file(), &shortest_route_file()].to_vec(), "./").expect("failed to run solver.");

    let search = io::read_route(&search_route_file()).expect("failed to search route file.");
    let shortest = io::read_route(&shortest_route_file()).expect("failed to shortest route file.");

    gnu_plotter::plot(&maze, maze_size(), &search, &shortest, 0, true);
}