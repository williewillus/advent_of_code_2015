struct Grid {
    main: Vec<Vec<bool>>,
    back_buffer: Vec<Vec<bool>>,
    grid_size: usize,
}

impl Grid {
    fn new(grid_size: usize) -> Self {
        Self {
            main: vec![vec![false; grid_size]; grid_size],
            back_buffer: vec![vec![false; grid_size]; grid_size],
            grid_size
        }
    }

    fn set_corners(&mut self) {
        self.main[0][0] = true;
        self.main[self.grid_size - 1][0] = true;
        self.main[0][self.grid_size - 1] = true;
        self.main[self.grid_size - 1][self.grid_size - 1] = true;
    }

    fn update(&mut self, part2: bool) {
        for i in 0..self.grid_size {
            for j in 0..self.grid_size {
                let mut nbs_on = 0;

                // up
                if j > 0 && self.main[j - 1][i] {
                    nbs_on += 1;
                }

                // down
                if j < self.grid_size - 1 && self.main[j + 1][i] {
                    nbs_on += 1;
                }

                if i > 0 {
                    // left
                    if self.main[j][i - 1] {
                        nbs_on += 1;
                    }

                    // up left
                    if j > 0 && self.main[j - 1][i - 1] {
                        nbs_on += 1;
                    }

                    // down left
                    if j < self.grid_size - 1 && self.main[j + 1][i - 1] {
                        nbs_on += 1;
                    }
                }

                if i < self.grid_size - 1 {
                    // right
                    if self.main[j][i + 1] {
                        nbs_on += 1;
                    }

                    // up right
                    if j > 0 && self.main[j - 1][i + 1] {
                        nbs_on += 1;
                    }

                    // down right
                    if j < self.grid_size - 1 && self.main[j + 1][i + 1] {
                        nbs_on += 1;
                    }
                }

                if self.main[j][i] {
                    self.back_buffer[j][i] = nbs_on == 2 || nbs_on == 3;
                } else {
                    self.back_buffer[j][i] = nbs_on == 3;
                }
            }
        }
        std::mem::swap(&mut self.main, &mut self.back_buffer);
        if part2 {
            self.set_corners();
        }
    }

    fn load(&mut self, lines: &[String]) {
        assert_eq!(lines.len(), self.main.len());
        for (line, row) in lines.iter().zip(self.main.iter_mut()) {
            assert_eq!(line.len(), row.len());
            for (c, dest) in line.chars().zip(row.iter_mut()) {
                *dest = c == '#';
            }
        }
    }

    fn count(&self) -> usize {
        self.main.iter()
            .flat_map(|r| r.iter())
            .filter(|c| **c)
            .count()
    }

    fn show(&self) {
        for row in &self.main {
            for &b in row {
                print!("{}", if b { '#' } else { '.' });
            }
            println!();
        }
    }
}

pub fn run() {
    let lines = crate::util::lines("d18_input.txt").unwrap();
    let mut grid = Grid::new(lines.len());
    grid.load(&lines);
    for _ in 0..100 {
        grid.update(false);
    }
    println!("Part 1: {}", grid.count());

    grid.load(&lines);
    grid.set_corners();
    for _ in 0..100 {
        grid.update(true);
    }
    println!("Part 2: {}", grid.count());
}
