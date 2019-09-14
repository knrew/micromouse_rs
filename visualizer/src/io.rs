use crate::wall;

pub struct Route {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}

impl Route {
    pub fn new() -> Route {
        Route { x: Vec::new(), y: Vec::new() }
    }
}

pub fn read_maze(maze_file: &str) -> Result<Vec<Vec<wall::Wall>>, std::io::Error> {
    let mut ret: Vec<Vec<wall::Wall>> = Vec::new();

    let reader = match std::fs::File::open(maze_file) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    use std::io::BufRead;
    let reader = std::io::BufReader::new(reader).lines();

    for r in reader {
        let line = match r {
            Ok(o) => o,
            Err(e) => return Err(e),
        };

        let splited: Vec<&str> = line.split(" ").collect();

        if splited.len() == 1 { continue; }

        let mut tmp: Vec<wall::Wall> = Vec::new();

        for e in &splited {
            if e.to_string() == "".to_string() { continue; }
            tmp.push(wall::Wall::from_str(e.chars().nth(0).unwrap()).unwrap());
        }

        ret.push(tmp);
    }

    Ok(ret)
}

pub fn read_route(route_file: &str) -> Result<Route, std::io::Error> {
    let reader = match std::fs::File::open(route_file) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    use std::io::BufRead;
    let reader = std::io::BufReader::new(reader).lines();

    let mut route = Route::new();

    for r in reader {
        let line = match r {
            Ok(o) => o,
            Err(e) => return Err(e),
        };
        let line: Vec<&str> = line.split(",").collect();

        let x = line[0].parse::<i32>();
        let y = line[1].parse::<i32>();

        if x.is_ok() && y.is_ok() {
            route.x.push(x.unwrap() as f64 + 0.5);
            route.y.push(y.unwrap() as f64 + 0.5);
        }
    }

    Ok(route)
}

