mod areas_volumes;
pub use areas_volumes::*;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let area_rec = rectangle_area(x, y);
    let area_kind = match kind {
        GeometricalShapes::Square => square_area(a) as f64,
        GeometricalShapes::Circle => circle_area(a),
        GeometricalShapes::Rectangle => rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => triangle_area(a, b) as f64,
    };
    (area_kind * times as f64) <= area_rec as f64
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let volume = parallelepiped_volume(x, y, z);
    let area_kind = match kind {
        GeometricalVolumes::Cube => cube_volume(a) as f64,
        GeometricalVolumes::Cone => cone_volume(a, b) as f64,
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c) as f64,
        GeometricalVolumes::Sphere => sphere_volume(a) as f64,
        GeometricalVolumes::TriangularPyramid => triangular_pyramid_volume(a as f64, b) as f64,
    };
    (area_kind * times as f64) <= volume as f64
}
