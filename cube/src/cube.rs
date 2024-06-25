
use itertools::{repeat_n, Itertools};

extern crate nalgebra as na;



pub struct Cube {
    vertices: Vec<na::Vector3<f64>>,
    edges: Vec<[u8;2]>
}

impl Cube {
    pub fn new(scale: f64) -> Self {
        let vertices = Self::get_vertices(scale);
        println!("{:?}", vertices);
        let vertices = Self::rot_x(vertices, 0.5);
        println!("{:?}", vertices);
        let edges = Vec::new();
        let cube = Cube{
            vertices,
            edges
        };
        cube
    }
    
    pub fn rot_x(mut vertices: Vec<na::Vector3<f64>>, t: f64) -> Vec<na::Vector3<f64>> {
        for idx in 0..vertices.len() {
            let y = vertices[idx][1];
            let z = vertices[idx][2];
            vertices[idx][1] = y*f64::cos(t) - z*f64::sin(t);
            vertices[idx][2] = z*f64::cos(t) - y*f64::sin(t);
        }
        vertices
    }
    
    pub fn get_projection(&mut self) -> Vec<na::Vector2<f64>> {
        
    }
    
    pub fn get_vertices(scale: f64) -> Vec<na::Vector3<f64>> {
        let mut vertices = Vec::new();
        let neg: f64 = -1.0 * scale;
        let pos: f64 = 1.0 * scale;
        
        let nums = repeat_n(vec![pos,neg].into_iter(), 3).multi_cartesian_product();
        for num in nums {
            vertices.push(na::Vector3::new(num[0], num[1], num[2]));
        }
        
        vertices
    }
}