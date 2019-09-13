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

pub fn plot_maze(fig: &mut gnuplot::Figure, maze: &wall::Maze, maze_size: i32, show: bool) {
    plot_maze_with_ax(make_ax(fig), maze, maze_size);
    if show { fig.show(); }
}

fn plot_maze_with_ax(ax: &mut gnuplot::Axes2D, maze: &wall::Maze, maze_size: i32) {
    const COLOR: &str = "black";

    for (i, line) in maze.iter().enumerate() {
        for (j, block) in line.iter().enumerate() {
            let x = j as i32;
            let y = maze_size - (i as i32);
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

pub fn plot_route_with_animation(fig: &mut gnuplot::Figure, maze: &wall::Maze, maze_size: i32, route: &io::Route, interval_ms: u64, history: bool) {
    let mut point = io::Route { x: Vec::new(), y: Vec::new() };

    for (i, _) in route.x.iter().enumerate() {
        fig.clear_axes();
        let ax = make_ax(fig);
        plot_maze_with_ax(ax, &maze, maze_size);

        if point.x.len() >= 1 {
            if history {
                ax.points(&point.x, &point.y, &[gnuplot::PointSymbol('O'), gnuplot::Color("blue")]);
            }
        }

        ax.points(&[route.x[i]], &[route.y[i]], &[gnuplot::PointSymbol('O'), gnuplot::Color("red")]);
        fig.show();
        point.x.push(route.x[i]);
        point.y.push(route.y[i]);

        std::thread::sleep(std::time::Duration::from_millis(interval_ms));
    }
}

pub fn plot_routes(fig: &mut gnuplot::Figure, maze: &wall::Maze, maze_size: i32, search: &io::Route, shortest: &io::Route) {
    fig.clear_axes();
    let mut ax = make_ax(fig);
    plot_maze_with_ax(&mut ax, &maze, maze_size);
    ax.points(&search.x, &search.y, &[gnuplot::PointSymbol('O'), gnuplot::Color("blue")]);
    ax.lines(&shortest.x, &shortest.y, &[gnuplot::PointSymbol('O'), gnuplot::Color("red")]);
    fig.show();
}