use visualizer::*;

fn main() {
    const MAZE_SIZE: i32 = 16;
    const DEFAULT_MAZE_FILE: &str = "../../maze_data/maze0000.txt";
    const DEFAULT_SEARCH_ROUTE_FILE: &str = "../../maze_solver/search.csv";
    const DEFAULT_OPTIMAL_ROUTE_FILE: &str = "../../maze_solver/shortest.csv";

//    let args: Vec<String> = std::env::args().collect();
//    for (i, arg) in args {}

    let maze = match io::read_maze(DEFAULT_MAZE_FILE) {
        Ok(o) => o,
        Err(e) => panic!("{}", e)
    };

    let search = match io::read_route(DEFAULT_SEARCH_ROUTE_FILE) {
        Ok(o) => o,
        Err(e) => panic!("{}", e)
    };

    let shortest = match io::read_route(DEFAULT_OPTIMAL_ROUTE_FILE) {
        Ok(o) => o,
        Err(e) => panic!("{}", e)
    };

    let mut fig = gnuplot::Figure::new();

    plotter::plot_maze(&mut fig, &maze, MAZE_SIZE, true);

    plotter::plot_route_with_animation(&mut fig, &maze, MAZE_SIZE, &search, 0, true);

    plotter::plot_routes(&mut fig, &maze, MAZE_SIZE, &search, &shortest);
}