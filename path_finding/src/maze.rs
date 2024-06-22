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
        
        let mut visited: Vec<Cell> = Vec::new();
        
        let mut frontier: Vec<Cell> = Vec::new();
        let x = rng.gen_range(0..n);
        let y = rng.gen_range(0..n);
        frontier.push(grid[y][x]);
        
        while frontier.len() > 0 {
            frontier.shuffle(&mut rng);
            let mut current_cell = frontier.pop().unwrap();
            current_cell.visited = true;
            visited.push(current_cell);
            
            let mut adj = current_cell.get_adjecent(n);
            adj.shuffle(&mut rng);
            let mut removed_wall = false;
            for coords in adj {
                let mut neighbour = grid[coords[1]][coords[0]];
                if neighbour.visited && !removed_wall {
                    let x_offset = neighbour.x as i32 -current_cell.x as i32;
                    let y_offset = neighbour.y as i32 -current_cell.y as i32;
                    
                    let idx_current: i32;
                    let idx_neighbour: i32;
                    if x_offset == 0 {
                        idx_current = -y_offset + 1;
                        idx_neighbour = y_offset + 1;
                    }
                    else {
                        idx_current = x_offset + 2;
                        idx_neighbour = -x_offset + 2;
                    }
                    
                    current_cell.walls[idx_current as usize] = !current_cell.walls[idx_current as usize];
                    neighbour.walls[idx_neighbour as usize] = !neighbour.walls[idx_neighbour as usize];
                    
                    grid[neighbour.y][neighbour.x] = neighbour;
                    
                    removed_wall = true;
                }
                else if !neighbour.visited {
                    frontier.push(neighbour);
                }
            }
            
            let (x, y) = (current_cell.x, current_cell.y);
            grid[y][x] = current_cell;
        }
        
        grid
    }
}