#[allow(dead_code)]
pub fn micromouse_root() -> String { format!("{}/../", std::env::current_dir().unwrap().display()) }

#[allow(dead_code)]
pub fn solver_dir() -> String { format!("{}/maze_solver/", micromouse_root()) }

#[allow(dead_code)]
pub fn program_name() -> &'static str { "tests/search_test" }

#[allow(dead_code)]
pub fn maze_size() -> usize { 16 }

#[allow(dead_code)]
pub fn maze_name() -> String {
    let args: Vec<String> = std::env::args().collect();
    format!("maze{}.txt", if args.len() < 2 { "0000" } else { &args[1] })
}

#[allow(dead_code)]
pub fn maze_file() -> String { format!("{}/maze_data/{}", micromouse_root(), maze_name()) }

#[allow(dead_code)]
pub fn build_dir() -> String { String::from("./build/") }

#[allow(dead_code)]
pub fn search_route_file() -> String { format!("{}/search.csv", build_dir()) }

#[allow(dead_code)]
pub fn shortest_route_file() -> String { format!("{}/shortest.csv", build_dir()) }

#[allow(dead_code)]
pub fn program() -> String { format!("{}/{}", build_dir(), program_name()) }