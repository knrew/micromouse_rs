use crate::wall;

pub fn plot_maze(ax: &mut &mut gnuplot::Axes2D, maze: &wall::Maze, maze_size: i32) {
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
