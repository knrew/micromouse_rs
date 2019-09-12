use crate::wall;

pub fn parse(raw_maze: &Vec<String>, maze_size: usize) -> wall::Maze {
    let mut maze: wall::Maze = Vec::new();
    maze.resize_with(maze_size, Default::default);
    for m in maze.iter_mut() { for _i in 0..maze_size { m.push(wall::Wall::new()); } }

    let mut r: usize = 0;
    for (row, line) in raw_maze.iter().enumerate() {
//        println!("{}", line);

        if row != 0 && row % 2 == 0 { r += 1; }

        if row % 2 == 0 {
            let line: Vec<&str> = line.split('o').collect();

            let mut c: usize = 0;
            for s in line {
                if s.to_string() == "".to_string() { continue; }
                if s.to_string() == "---".to_string() {
                    if r <= maze_size - 1 { maze[r][c].n = true; }
                    if r >= 1 && r <= maze_size { maze[r - 1][c].s = true; }
                    c += 1;
                } else if s.to_string() == "   ".to_string() {
                    c += 1;
                }
            }
        } else {
            let mut c: usize = 0;
            let mut count = 0;
            for (_, s) in line.chars().enumerate() {
                if s == '|' {
                    if c <= maze_size - 1 { maze[r][c].w = true; }
                    if c >= 1 && c <= maze_size { maze[r][c - 1].e = true; }
                    c += 1;
                    count = 0;
                }
                if s == ' ' {
                    if count == 3 {
                        c += 1;
                        count = 0;
                    } else {
                        count += 1;
                    }
                }
            }
        }
    }

    maze
}