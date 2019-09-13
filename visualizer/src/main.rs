use visualizer::*;

fn main() {
    const MAZE_SIZE: i32 = 16;
    //    const MAZE_FILE: &str = "poyo.txt";
    const MAZE_FILE: &str = "../../maze_data/maze.txt";
    const SEARCH_ROUTE_FILE: &str = "../../maze_solver/search_route.csv";
    const OPTIMAL_ROUTE_FILE: &str = "../../maze_solver/optimal_route.csv";

    // read maze data
    let maze = match read_maze(MAZE_FILE) {
        Ok(o) => o,
        Err(e) => panic!("{}", e)
    };

    let (x, y) = match read_route(SEARCH_ROUTE_FILE) {
        Ok(o) => o,
        Err(e) => panic!("{}", e)
    };

    let mut fig = gnuplot::Figure::new();
    fig.clear_axes();
    let ax = fig.axes2d();
    maze_plotter::plot_maze(&mut fig.axes2d(), &maze, MAZE_SIZE);
    fig.show();

//    return;

    let mut point_x: Vec<f64> = Vec::new();
    let mut point_y: Vec<f64> = Vec::new();
    for (i, _) in x.iter().enumerate() {
        fig.clear_axes();
        let mut ax = fig.axes2d();
        maze_plotter::plot_maze(&mut ax, &maze, MAZE_SIZE);
        ax.points(&[x[i]], &[y[i]], &[gnuplot::PointSymbol('O'), gnuplot::Color("red")]);

        if point_x.len() >= 1 {
            ax.points(&point_x, &point_y, &[gnuplot::PointSymbol('O'), gnuplot::Color("blue")]);
        }

        fig.show();
        point_x.push(x[i]);
        point_y.push(y[i]);

//        std::thread::sleep(std::time::Duration::from_millis(500));
    }


    let (x, y) = match read_route(OPTIMAL_ROUTE_FILE) {
        Ok(o) => o,
        Err(e) => panic!("{}", e)
    };

    fig.clear_axes();
    let mut ax = fig.axes2d();
    maze_plotter::plot_maze(&mut ax, &maze, MAZE_SIZE);
    ax.points(&point_x, &point_y, &[gnuplot::PointSymbol('O'), gnuplot::Color("blue")]);
    ax.lines(&x, &y, &[gnuplot::PointSymbol('O'), gnuplot::Color("red")]);
    fig.show();

    println!("fin.");
}

fn read_maze(maze_file: &str) -> Result<Vec<Vec<wall::Wall>>, std::io::Error> {
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

