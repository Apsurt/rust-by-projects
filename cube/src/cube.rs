
use itertools::{repeat_n, Itertools};

extern crate nalgebra as na;

pub struct Cube {
    pub vertices: Vec<na::Vector3<f64>>,
    pub edges: Vec<[usize;2]>
}

impl Cube {
    pub fn new(scale: f64, transform: [f64; 3]) -> Self {
            let vertices = Self::get_vertices(scale, transform);
            //let vertices = Self::rot(vertices, 0.5);
            let edges = Self::get_edges();
            Cube { vertices, edges }
        }
    
        fn calculate_center(vertices: &[na::Vector3<f64>]) -> na::Vector3<f64> {
            let sum = vertices.iter().fold(na::Vector3::zeros(), |acc, v| acc + v);
            sum / vertices.len() as f64
        }
        
        pub fn rot_x(vertices: &Vec<na::Vector3<f64>>, t: f64) -> Vec<na::Vector3<f64>> {
            let center = Self::calculate_center(vertices);
            vertices.into_iter().map(|v| {
                let translated = v - center;
                let y = translated.y * f64::cos(t) - translated.z * f64::sin(t);
                let z = translated.y * f64::sin(t) + translated.z * f64::cos(t);
                na::Vector3::new(translated.x, y, z) + center
            }).collect()
        }
        
        pub fn rot_y(vertices: &Vec<na::Vector3<f64>>, t: f64) -> Vec<na::Vector3<f64>> {
            let center = Self::calculate_center(vertices);
            vertices.into_iter().map(|v| {
                let translated = v - center;
                let x = translated.x * f64::cos(t) + translated.z * f64::sin(t);
                let z = -translated.x * f64::sin(t) + translated.z * f64::cos(t);
                na::Vector3::new(x, translated.y, z) + center
            }).collect()
        }
        
        pub fn rot_z(vertices: &Vec<na::Vector3<f64>>, t: f64) -> Vec<na::Vector3<f64>> {
            let center = Self::calculate_center(vertices);
            vertices.into_iter().map(|v| {
                let translated = v - center;
                let x = translated.x * f64::cos(t) - translated.y * f64::sin(t);
                let y = translated.x * f64::sin(t) + translated.y * f64::cos(t);
                na::Vector3::new(x, y, translated.z) + center
            }).collect()
        }
        
        pub fn rot(vertices: &Vec<na::Vector3<f64>>, t: f64) -> Vec<na::Vector3<f64>> {
            let center = Self::calculate_center(&vertices);
            let vertices = vertices.into_iter().map(|v| v - center).collect();
            let vertices = Self::rot_x(&vertices, t);
            let vertices = Self::rot_y(&vertices, t);
            let vertices = Self::rot_z(&vertices, t);
            vertices.into_iter().map(|v| v + center).collect()
        }
    
    pub fn get_projection(&self, distance: f64) -> Vec<(f64, f64)> {
        self.vertices
            .iter()
            .map(|v| {
                let z = v.z + distance;
                let x = v.x / z;
                let y = v.y / z;
                
                let scaled_x = x * 100.0 + 110.0;
                let scaled_y = y * 50.0 + 60.0;
                
                (scaled_x, scaled_y)
            })
            .collect()
    }
    
    pub fn get_vertices(scale: f64, transform: [f64; 3]) -> Vec<na::Vector3<f64>> {
        let mut vertices = Vec::new();
        let neg: f64 = -1.0 * scale;
        let pos: f64 = 1.0 * scale;
        
        let nums = repeat_n(vec![pos,neg].into_iter(), 3).multi_cartesian_product();
        for num in nums {
            vertices.push(na::Vector3::new(
                num[0]+transform[0], 
                num[1]+transform[1], 
                num[2]+transform[2]
            ));
        }
        vertices
    }
    
    fn get_edges() -> Vec<[usize; 2]> {
        vec![
            [0, 1], [1, 3], [3, 2], [2, 0],
            [4, 5], [5, 7], [7, 6], [6, 4],
            [0, 4], [1, 5], [2, 6], [3, 7],
        ]
    }
}