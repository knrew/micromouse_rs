use micromouse_rs::*;

mod parameters;

fn main() {
    let params = parameters::Parameters::new();

    let maze = io::read_maze(&params.maze_file).expect("failed to read maze_file.");

    if false {
        gnu_plotter::plot_maze(&mut gnuplot::Figure::new(), &maze, params.maze_size, true);
        return;
    }

    bash_process::process(&params.program, &[&params.maze_file as &str, &params.search_route_file, &params.shortest_route_file].to_vec(), "./").expect("failed to run solver.");

    let search = io::read_route(&params.search_route_file).expect("failed to search route file.");
    let shortest = io::read_route(&params.shortest_route_file).expect("failed to shortest route file.");

    gnu_plotter::plot(&maze, params.maze_size, &search, &shortest, 0, true);
}