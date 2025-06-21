mod areas_volumes;
use areas_volumes::*;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let rec = rectangle_area(x, y) as f64;
    let area_times = times as f64
        * match kind {
            GeometricalShapes::Square => square_area(a) as f64,
            GeometricalShapes::Circle => circle_area(a),
            GeometricalShapes::Rectangle => rectangle_area(a, b) as f64,
            GeometricalShapes::Triangle => triangle_area(a, b) as f64,
        };
    rec >= area_times
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let rec = parallelepiped_volume(x, y, z) as f64;
    let area_times = times as f64
        * match kind {
            GeometricalVolumes::Cone => cone_volume(a, b),
            GeometricalVolumes::Cube => cube_volume(a) as f64,
            GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c) as f64,
            GeometricalVolumes::Sphere => sphere_volume(a),
            GeometricalVolumes::TriangularPyramid => triangular_pyramid_volume(a as f64, b) as f64,
        };
    rec >= area_times
}
