use noise::{NoiseFn, OpenSimplex};
use rand::*;
use voronoi::Point;
use std::ops::{Add,Mul,Sub};
use svg::node::element::path::Data;

pub fn generate_sites_with_open_simplex(box_size: f64, threshold : f32) -> Vec<Point> {
    let ratio = 1.0 / 10.0;
    let (height_offset, width_offset) = ((box_size * ratio) as u32, (box_size * ratio) as u32);
    let (height, width) = ((box_size * (1.0 - ratio)) as u32, (box_size * (1.0 - ratio)) as u32);
    let noise_generator = OpenSimplex::new();
    let mut sites: Vec<Point> = Vec::new();
    for y in (height_offset..=height).map(|y| {y as f64}) {
        for x in (width_offset..=width).map(|y| {y as f64}) {
            // Open Simplex Min/Max: +-0.5439776726541984
            let noise_value = noise_generator.get([x, y]) as f32;
            if noise_value > threshold || noise_value < -threshold {
                sites.push(Point::new(x,y));
            }
        }
    }
    sites
}

pub fn generate_sites_with_random(box_size: f64, partition: Point, variance : f64) -> Vec<Point> {
    let mut sites: Vec<Point> = Vec::new();
    let (step_x, step_y) = (box_size / partition.x() + 1.0,
                            box_size / partition.y() + 1.0);
    let mut rng = rand::thread_rng();
    for y in 1..=100 {
        for x in 1..=100 {
            let x_offset = step_x * variance * (2.0 * rng.gen::<f64>() - 1.0);
            let y_offset = step_y * variance * (2.0 * rng.gen::<f64>() - 1.0);
            sites.push(Point::new(x as f64 * step_x + x_offset,
                                  y as f64 * step_y + y_offset))
        }
    }
    sites
}

pub fn gen_base_box(box_size: f64) -> Data {
    Data::new().move_to((0.0,0.0))
        .line_to((box_size,0.0))
        .line_to((box_size, box_size))
        .line_to((0.0, box_size))
        .close()
}

pub fn get_centroid(polygon: &Vec<Point>) -> Point {
    let xs = polygon.iter().map(|p| {p.x()}).sum::<f64>() / polygon.len() as f64;
    let ys = polygon.iter().map(|p| {p.y()}).sum::<f64>() / polygon.len() as f64;
    Point::new(xs,ys)
}

pub fn move_to_centroid(point: &Point, centroid: Point, scale: f64) -> (f32, f32) {
    let moved_point = point.add(centroid.sub(*point).mul(scale));
    let (x, y) = (moved_point.x(), moved_point.y());
    (x as f32, y as f32)
}