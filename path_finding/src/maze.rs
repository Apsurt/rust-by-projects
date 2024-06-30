use rand::Rng;
use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    x: usize,
    y: usize,
    visited: bool,
    walls: [bool; 4], // [down, left, up, right]
}

impl Cell {
    pub fn get_adjecent(&self, n: usize) -> Vec<[usize; 2]> {
        let mut neighbours: Vec<[usize; 2]> = Vec::new();
        if self.y < n-1 {
            let down  = [self.x, self.y+1];
            neighbours.push(down);
        }
        if self.x > 0 {
            let left  = [self.x-1, self.y];
            neighbours.push(left);
        }
        if self.y > 0 {
            let up    = [self.x, self.y-1];
            neighbours.push(up);
        }
        if self.x < n-1 {
            let right = [self.x+1, self.y];
            neighbours.push(right);
        }
        neighbours
    }
}

#[derive(Debug)]
pub struct Maze {
    pub grid: Vec<Vec<Cell>>,
    pub size: usize
}

impl Maze {
    pub fn new(n: usize) -> Maze {
        let grid = Maze::generate_maze(n);
        let maze = Maze {
            grid,
            size: n
        };
        maze
    }

    fn generate_empty_maze(n: usize) -> Vec<Vec<Cell>> {
        let walls = [true; 4];
        let visited = false;
        let mut grid: Vec<Vec<Cell>> = Vec::new();
        for y in 0..n {
            let row: Vec<Cell> = Vec::new();
            grid.push(row);
            for x in 0..n {
                let walls = walls.clone();
                let cell =  Cell{
                    x,
                    y,
                    visited,
                    walls
                };
                grid[y].push(cell);
            }
        }
        grid
    }
    
    fn generate_maze(n: usize) -> Vec<Vec<Cell>> {
        let mut rng = rand::thread_rng();
        
        let mut grid = Maze::generate_empty_maze(n);
        
        let mut frontier: Vec<[usize; 2]> = Vec::new();
        let start_x = rng.gen_range(0..n);
        let start_y = rng.gen_range(0..n);
        
        grid[start_y][start_x].visited = true;
        Maze::add_frontier(&mut frontier, &grid[start_y][start_x], n, &grid);
        
        while !frontier.is_empty() {
            let index = rng.gen_range(0..frontier.len());
            let [x, y] = frontier.swap_remove(index);
            
            let neighbors: Vec<[usize; 2]> = grid[y][x].get_adjecent(n)
                .into_iter()
                .filter(|&[nx, ny]| grid[ny][nx].visited)
                .collect();
            
            if let Some(&[px, py]) = neighbors.choose(&mut rng) {
                Maze::remove_wall(&mut grid, x, y, px, py);
                grid[y][x].visited = true;
                Maze::add_frontier(&mut frontier, &grid[y][x], n, &grid);
            }
        }        
        grid
    }
    
    fn add_frontier(frontier: &mut Vec<[usize; 2]>, cell: &Cell, n: usize, grid: &Vec<Vec<Cell>>) {
        for [nx, ny] in cell.get_adjecent(n) {
            if !grid[ny][nx].visited && !frontier.contains(&[nx, ny]) {
                frontier.push([nx, ny]);
            }
        }
    }
    
    fn remove_wall(grid: &mut Vec<Vec<Cell>>, x1: usize, y1: usize, x2: usize, y2: usize) {
        if x1 < x2 {
            grid[y1][x1].walls[3] = false; // Remove right wall
            grid[y2][x2].walls[1] = false; // Remove left wall
        } else if x1 > x2 {
            grid[y1][x1].walls[1] = false; // Remove left wall
            grid[y2][x2].walls[3] = false; // Remove right wall
        } else if y1 < y2 {
            grid[y1][x1].walls[0] = false; // Remove bottom wall
            grid[y2][x2].walls[2] = false; // Remove top wall
        } else {
            grid[y1][x1].walls[2] = false; // Remove top wall
            grid[y2][x2].walls[0] = false; // Remove bottom wall
        }
    }
    
    pub fn print(&self) {
        for y in 0..=self.size {
            // Print top wall of each cell in the row
            for x in 0..=self.size {
                print!("{}", self.get_corner_char(x, y));
                if x < self.size {
                    let cell = &self.grid[y.min(self.size - 1)][x];
                    print!("{}", if y == self.size || cell.walls[2] { "───" } else { "   " });
                }
            }
            println!();
    
            // Print right wall of each cell in the row
            if y < self.size {
                for _ in 0..2 {
                    for x in 0..=self.size {
                        if x < self.size {
                            let cell = &self.grid[y][x];
                            print!("{}", if cell.walls[1] { "│" } else { " " });
                            print!("   ");
                        } else {
                            print!("│");
                        }
                    }
                    println!();
                }
            }
        }
    }
    
    fn get_corner_char(&self, x: usize, y: usize) -> char {
        //[down, left, up, right]
        let top = y > 0 && y <= self.size && x < self.size && self.grid[y-1][x].walls[0];
        let left = x > 0 && y < self.size && self.grid[y][x-1].walls[3];
        let bottom = y < self.size && x < self.size && self.grid[y][x].walls[2];
        let right = x < self.size && y < self.size && self.grid[y][x].walls[1];
    
        match (top, left, bottom, right) {
            (true, true, true, true) => '┼',
            (true, true, true, false) => '┤',
            (true, true, false, true) => '┴',
            (true, true, false, false) => '┘',
            (true, false, true, true) => '├',
            (true, false, true, false) => '│',
            (true, false, false, true) => '└',
            (true, false, false, false) => '╵',
            (false, true, true, true) => '┬',
            (false, true, true, false) => '┐',
            (false, true, false, true) => '─',
            (false, true, false, false) => '╴',
            (false, false, true, true) => '┌',
            (false, false, true, false) => '╷',
            (false, false, false, true) => '╶',
            (false, false, false, false) => ' ',
        }
    }
}