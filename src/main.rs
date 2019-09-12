fn main() {
    const MAZE_SIZE: i32 = 16;
    const MAZE_FILE: &str = "poyo.txt";
    const SEARCH_ROUTE_FILE: &str = "search_route.csv";
    const OPTIMAL_ROUTE_FILE: &str = "optimal_route.csv";

    // read maze data
    let maze = match read_maze(MAZE_FILE) {
        Ok(o) => o,
        Err(e) => panic!("{}", e)
    };

//    let (x, y) = match read_route(SEARCH_ROUTE_FILE) {
//        Ok(o) => o,
//        Err(e) => panic!("{}", e)
//    };

    let mut fig = gnuplot::Figure::new();
    plot_maze(&mut fig.axes2d(), &maze, MAZE_SIZE);
    fig.show();
//
//    {
//        let mut point_x: Vec<f64> = Vec::new();
//        let mut point_y: Vec<f64> = Vec::new();
//        for (i, _) in x.iter().enumerate() {
//            fig.clear_axes();
//            let mut ax = fig.axes2d();
//            plot_maze(&mut ax, &maze, MAZE_SIZE);
//            ax.points(&[x[i]], &[y[i]], &[gnuplot::PointSymbol('O'), gnuplot::Color("red")]);
//
//            if point_x.len() >= 1 {
//                ax.points(&point_x, &point_y, &[gnuplot::PointSymbol('O'), gnuplot::Color("blue")]);
//            }
//
//            fig.show();
//            point_x.push(x[i]);
//            point_y.push(y[i]);
//        }
//    }
//
//    let (x, y) = match read_route(OPTIMAL_ROUTE_FILE) {
//        Ok(o) => o,
//        Err(e) => panic!("{}", e)
//    };
//
//    fig.clear_axes();
//    let mut ax = fig.axes2d();
//    plot_maze(&mut ax, &maze, MAZE_SIZE);
//    ax.points(&x, &y, &[gnuplot::PointSymbol('O'), gnuplot::Color("blue")]);
//    ax.lines(&x, &y, &[gnuplot::PointSymbol('O'), gnuplot::Color("red")]);
//    fig.show();

    println!("fin.");
}

#[allow(dead_code)]
struct Wall {
    n: bool,
    e: bool,
    s: bool,
    w: bool,
    has_known_n: bool,
    has_known_e: bool,
    has_known_s: bool,
    has_known_w: bool,
}

impl Wall {
    fn new() -> Wall {
        Wall {
            n: false,
            e: false,
            s: false,
            w: false,
            has_known_n: false,
            has_known_e: false,
            has_known_s: false,
            has_known_w: false,
        }
    }

    fn from_str(s: &str) -> Result<Wall, String> {
        if s.len() != 1 { return Err(format!("Input ONE character![{}]", s)); }

        let num = match s.parse::<u8>() {
            Ok(o) => o,
            Err(_) => {
                match s {
                    "a" => 10,
                    "b" => 11,
                    "c" => 12,
                    "d" => 13,
                    "e" => 14,
                    "f" => 15,
                    _ => return Err("Invalid character!".to_string())
                }
            }
        };

        let wall_bin = format!("{:0>4b}", num);

        let mut wall = Wall::new();
        wall.n = if wall_bin.chars().nth(3) == Some('1') { true } else { false };
        wall.e = if wall_bin.chars().nth(2) == Some('1') { true } else { false };
        wall.s = if wall_bin.chars().nth(1) == Some('1') { true } else { false };
        wall.w = if wall_bin.chars().nth(0) == Some('1') { true } else { false };

        Ok(wall)
    }

    #[allow(dead_code)]
    fn to_bit(&self) -> i32 {
        let mut ret: i32 = 0;

        fn pow(a: i32, b: i32) -> i32 { if b == 0 { 1 } else { a * pow(a, b - 1) } }

        if self.n { ret += 1; }
        if self.e { ret += 2; };
        if self.s { ret += pow(2, 2); };
        if self.w { ret += pow(2, 3); };
        if self.has_known_n { ret += pow(2, 4); };
        if self.has_known_e { ret += pow(2, 5); };
        if self.has_known_s { ret += pow(2, 6); };
        if self.has_known_w { ret += pow(2, 7); };

        ret
    }
}

fn read_maze(maze_file: &str) -> Result<Vec<Vec<Wall>>, std::io::Error> {
    let mut ret: Vec<Vec<Wall>> = Vec::new();

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

        let mut tmp: Vec<Wall> = Vec::new();

        for e in &splited {
            if e.to_string() == "".to_string() { continue; }
            tmp.push(Wall::from_str(e).unwrap());
        }

        ret.push(tmp);
    }

    Ok(ret)
}

fn read_route(route_file: &str) -> Result<(Vec<f64>, Vec<f64>), std::io::Error> {
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

    Ok((x, y))
}

fn plot_maze(ax: &mut &mut gnuplot::Axes2D, maze: &Vec<Vec<Wall>>, maze_size: i32) {
    for (i, line) in maze.iter().enumerate() {
        for (j, block) in line.iter().enumerate() {
            let x = j as i32;
            let y = maze_size - (i as i32);
            if block.n {
                ax.lines([x, x + 1].iter(), [y, y].iter(), &[gnuplot::Color("black")]);
            }
            if block.e {
                ax.lines([x + 1, x + 1].iter(), [y, y - 1].iter(), &[gnuplot::Color("black")]);
            }
            if block.s {
                ax.lines([x, x + 1].iter(), [y - 1, y - 1].iter(), &[gnuplot::Color("black")]);
            }
            if block.w {
                ax.lines([x, x].iter(), [y, y - 1].iter(), &[gnuplot::Color("black")]);
            }
        }
    }
}
