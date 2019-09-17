pub struct Parameters {
    pub maze_size: usize,
    pub program_name: String,
    pub maze_name: String,
    pub micromouse_root: String,
    pub build_dir: String,
    pub maze_file: String,
    pub search_route_file: String,
    pub shortest_route_file: String,
    pub solver_dir: String,
    pub program: String,
}

impl Parameters {
    pub fn new() -> Parameters {
        let args: Vec<String> = std::env::args().collect();
        let maze_size: usize = 16;
        let program_name = "examples/search_exsample".to_string();
        let maze_name = format!("maze{}.txt", if args.len() < 2 { "0000" } else { &args[1] });
//        let maze_name = maze_name.to_string();
        let micromouse_root = format!("{}/micromouse/", dirs::home_dir().unwrap().display());
        let build_dir = "./build/".to_string();
        let maze_file = format!("{}/maze_data/{}", micromouse_root, maze_name);
        let search_route_file = format!("{}/search.csv", build_dir);
        let shortest_route_file = format!("{}/shortest.csv", build_dir);
        let solver_dir = format!("{}/maze_solver/", micromouse_root);
        let program = format!("{}/{}", build_dir, program_name);

        Parameters {
            maze_size: maze_size,
            program_name: program_name,
            maze_name: maze_name,
            micromouse_root: micromouse_root,
            build_dir: build_dir,
            maze_file: maze_file,
            search_route_file: search_route_file,
            shortest_route_file: shortest_route_file,
            solver_dir: solver_dir,
            program: program,
        }
    }
}