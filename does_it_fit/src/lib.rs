mod areas_volumes;
pub use crate::areas_volumes::*;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let holder = areas_volumes::rectangle_area(x,y);
    match kind {
        areas_volumes::GeometricalShapes::Square => holder as f64 >= times as f64 * areas_volumes::square_area(a) as f64,
        areas_volumes::GeometricalShapes::Circle => holder as f64 >= times as f64 * areas_volumes::circle_area(a),
        areas_volumes::GeometricalShapes::Rectangle => holder as f64 >= times as f64 * areas_volumes::rectangle_area(a,b) as f64,
        areas_volumes::GeometricalShapes::Triangle => holder as f64 >= times as f64 * areas_volumes::triangle_area(a,b) as f64,
    }
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let holder = areas_volumes::parallelepiped_volume(x,y,z);
    match kind {
        areas_volumes::GeometricalVolumes::Cube => holder as f64 >= times as f64 * areas_volumes::cube_volume(a) as f64,
        areas_volumes::GeometricalVolumes::Sphere => holder as f64 >= times as f64 * areas_volumes::sphere_volume(a),
        areas_volumes::GeometricalVolumes::Parallelepiped => holder as f64 >= times as f64 * areas_volumes::parallelepiped_volume(a,b,c) as f64,
        areas_volumes::GeometricalVolumes::Cone => holder as f64 >= times as f64 * areas_volumes::cone_volume(a,b),
        areas_volumes::GeometricalVolumes::TriangularPyramid => holder as f64 >= times as f64 * areas_volumes::triangular_pyramid_volume(a as f64,b),
    }
}