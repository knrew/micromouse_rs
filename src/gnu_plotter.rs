use crate::wall;
use crate::io;

fn make_ax(fig: &mut gnuplot::Figure) -> &mut gnuplot::Axes2D {
    use gnuplot::*;
    let ax = fig.axes2d()
        .set_aspect_ratio(gnuplot::Fix(1.0))
        .set_x_range(gnuplot::Fix(-1.0), gnuplot::Fix(17.0))
        .set_y_range(gnuplot::Fix(-1.0), gnuplot::Fix(17.0));
    ax
}

pub fn plot(maze: &wall::Maze, maze_size: usize, search: &io::Route, shortest: &io::Route, interval_ms: u64, history: bool) {
    let mut fig = gnuplot::Figure::new();
    plot_route_with_animation(&mut fig, maze, maze_size, search, interval_ms, history);
    plot_routes(&mut fig, maze, maze_size, search, shortest);
    plot_maze(&mut fig, maze, maze_size, true);
}

pub fn plot_maze(fig: &mut gnuplot::Figure, maze: &wall::Maze, maze_size: usize, show: bool) {
    plot_maze_with_ax(make_ax(fig), maze, maze_size);
    if show { fig.show(); }
}

fn plot_maze_with_ax(ax: &mut gnuplot::Axes2D, maze: &wall::Maze, maze_size: usize) {
    const COLOR: &str = "black";

    for (i, line) in maze.iter().enumerate() {
        for (j, block) in line.iter().enumerate() {
            let x = j as i32;
            let y = maze_size - i;
            if block.n {
                ax.lines([x, x + 1].iter(), [y, y].iter(), &[gnuplot::Color(COLOR)]);
            }
            if block.e {
                ax.lines([x + 1, x + 1].iter(), [y, y - 1].iter(), &[gnuplot::Color(COLOR)]);
            }
            if block.s {
                ax.lines([x, x + 1].iter(), [y - 1, y - 1].iter(), &[gnuplot::Color(COLOR)]);
            }
            if block.w {
                ax.lines([x, x].iter(), [y, y - 1].iter(), &[gnuplot::Color(COLOR)]);
            }
        }
    }
}

pub fn plot_route_with_animation(fig: &mut gnuplot::Figure, maze: &wall::Maze, maze_size: usize, route: &io::Route, interval_ms: u64, history: bool) {
    struct RouteF64 {
        x: Vec<f64>,
        y: Vec<f64>,
    }
    let mut point = RouteF64 { x: Vec::new(), y: Vec::new() };

    for (i, _) in route.x.iter().enumerate() {
        fig.clear_axes();
        let ax = make_ax(fig);

        if point.x.len() >= 1 {
            if history {
//                let mut has_fill = [false; 64 * 64];
//                for (j, _) in point.x.iter().enumerate() {
//                    if !has_fill[point.x[j] as usize + 64 * point.y[j] as usize] {
//                        has_fill[point.x[j] as usize + 64 * point.y[j] as usize] = true;
//                        ax.fill_between(&[point.x[j] - 0.5, point.x[j] + 0.5], &[point.y[j] - 0.5, point.y[j] - 0.5], &[point.y[j] + 0.5, point.y[j] + 0.5], &[gnuplot::Color("blue"), gnuplot::FillAlpha(0.3), gnuplot::FillRegion(gnuplot::FillRegionType::Between)]);
//                    }
//                }

                ax.points(&point.x, &point.y, &[gnuplot::PointSymbol('O'), gnuplot::Color("blue")]);
            }
        }

        ax.points(&[route.x[i] as f64 + 0.5], &[route.y[i] as f64 + 0.5], &[gnuplot::PointSymbol('O'), gnuplot::Color("red")]);
        point.x.push(route.x[i] as f64 + 0.5);
        point.y.push(route.y[i] as f64 + 0.5);

        plot_maze_with_ax(ax, &maze, maze_size);

        fig.show();

        std::thread::sleep(std::time::Duration::from_millis(interval_ms));
    }
}

pub fn plot_routes(fig: &mut gnuplot::Figure, maze: &wall::Maze, maze_size: usize, search: &io::Route, shortest: &io::Route) {
    fig.clear_axes();
    let mut ax = make_ax(fig);
    plot_maze_with_ax(&mut ax, &maze, maze_size);

//    let mut has_fill = [false; 64 * 64];
//    for (j, _) in search.x.iter().enumerate() {
//        if !has_fill[search.x[j] as usize + 64 * search.y[j] as usize] {
//            has_fill[search.x[j] as usize + 64 * search.y[j] as usize] = true;
//            ax.fill_between(&[search.x[j] - 0.5, search.x[j] + 0.5], &[search.y[j] - 0.5,search.y[j] - 0.5], &[search.y[j] + 0.5, search.y[j] + 0.5], &[gnuplot::Color("blue"), gnuplot::FillAlpha(0.3), gnuplot::FillRegion(gnuplot::FillRegionType::Between)]);
//        }
//    }
    let x: Vec<f64> = search.x.iter().map(|z| z.clone() as f64 + 0.5).collect();
    let y: Vec<f64> = search.y.iter().map(|z| z.clone() as f64 + 0.5).collect();
    ax.points(&x, &y, &[gnuplot::PointSymbol('O'), gnuplot::Color("blue")]);

    let x: Vec<f64> = shortest.x.iter().map(|z| z.clone() as f64 + 0.5).collect();
    let y: Vec<f64> = shortest.y.iter().map(|z| z.clone() as f64 + 0.5).collect();
    ax.lines(&x, &y, &[gnuplot::PointSymbol('O'), gnuplot::Color("red")]);

    fig.show();
}