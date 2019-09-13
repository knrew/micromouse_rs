use crate::wall;

pub struct Route {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
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

    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();

    for r in reader {
        let line = match r {
            Ok(o) => o,
            Err(e) => return Err(e),
        };
        let s: Vec<&str> = line.split(",").collect();
        let tmp = (s[0].parse::<i32>(), s[1].parse::<i32>());

        if tmp.0.is_ok() & &tmp.0.is_ok() {
            x.push(tmp.0.unwrap() as f64 + 0.5);
            y.push(tmp.1.unwrap() as f64 + 0.5);
        }
    }

    Ok(Route { x, y })
}

