use std::collections::HashSet;

use crate::utils::drawing::clustering;

use geo::Coordinate;
use map_3d::{geodetic2ecef, Ellipsoid};

type Geocentric = (f64, f64, f64);
type Topocentric = (f64, f64);

trait FromKey {
    fn from_key(self) -> [f64; 2];
}

impl FromKey for String {
    fn from_key(self) -> [f64; 2] {
        let mut iter = self.split(',');
        let lat = iter.next().unwrap().parse::<f64>().unwrap();
        let lon = iter.next().unwrap().parse::<f64>().unwrap();
        [lat, lon]
    }
}

fn euclidean_norm2(x: Geocentric) -> f64 {
    x.0 * x.0 + x.1 * x.1 + x.2 * x.2
}

fn dot_product(x: Geocentric, y: Geocentric) -> f64 {
    x.0 * y.0 + x.1 * y.1 + x.2 * y.2
}

fn cross_product(x: Geocentric, y: Geocentric) -> Geocentric {
    (
        x.1 * y.2 - x.2 * y.1,
        x.2 * y.0 - x.0 * y.2,
        x.0 * y.1 - x.1 * y.0,
    )
}

fn normalize(x: Geocentric) -> Geocentric {
    let l = euclidean_norm2(x).sqrt();
    (x.0 / l, x.1 / l, x.2 / l)
}

/// this function finds the intersection of the ray from earth center to earth surface in radians
fn radial_project(p: Geocentric) -> Topocentric {
    // convert geocentric to geodesic
    let t = 1. - Ellipsoid::default().parameters().2;
    (
        (p.2 / (t * t * (p.0 * p.0 + p.1 * p.1).sqrt())).atan(),
        p.1.atan2(p.0),
    )
}

fn compute_plane_center(points: &Vec<Geocentric>) -> Topocentric {
    let mut dir = (0., 0., 0.);
    for (x, y, z) in points {
        dir.0 += x;
        dir.1 += y;
        dir.2 += z;
    }
    radial_project(dir)
}

fn reverse_project(
    p: [f64; 2],
    (plane_center, plane_x, plane_y, plane_z, adjusted_radius): (
        Geocentric,
        Geocentric,
        Geocentric,
        Geocentric,
        f64,
    ),
) -> Geocentric {
    let x = plane_center.0 + (plane_x.0 * p[0] + plane_y.0 * p[1]) * adjusted_radius;
    let y = plane_center.1 + (plane_x.1 * p[0] + plane_y.1 * p[1]) * adjusted_radius;
    let z = plane_center.2 + (plane_x.2 * p[0] + plane_y.2 * p[1]) * adjusted_radius;
    let (lat, lon) = radial_project((x, y, z));
    let s = dot_product((x, y, z), plane_z) / euclidean_norm2((x, y, z)).sqrt();
    (lat.to_degrees(), lon.to_degrees(), s)
}

pub fn project_points(
    input: Vec<[f64; 2]>,
    radius: f64,
    min_points: usize,
    _category: String,
) -> (Vec<[f64; 2]>, [f64; 2]) {
    let points = input
        .iter()
        .map(|&[lat, lon]| {
            geodetic2ecef(lat.to_radians(), lon.to_radians(), 0., Ellipsoid::default())
        })
        .collect();
    let (plane_center_lat, plane_center_lon) = compute_plane_center(&points);
    let plane_center = geodetic2ecef(plane_center_lat, plane_center_lon, 0., Ellipsoid::default());
    let plane_z = (
        plane_center_lat.cos() * plane_center_lon.cos(),
        plane_center_lat.cos() * plane_center_lon.sin(),
        plane_center_lat.sin(),
    );
    let plane_y = normalize((-plane_center.1, plane_center.0, 0.));
    let plane_x = cross_product(plane_z, plane_y);
    let earth_minor = Ellipsoid::default().parameters().1;
    let adjusted_radius = 0.5 * earth_minor * (2. * radius / earth_minor).sin();
    let global_scale = dot_product(plane_center, plane_z) / adjusted_radius;
    let offset_x = dot_product(plane_center, plane_x) / adjusted_radius;
    let output: Vec<Coordinate> = points
        .iter()
        .map(|p| {
            let scale = global_scale / dot_product(*p, plane_z);
            Coordinate {
                x: dot_product(*p, plane_x) * scale - offset_x,
                y: dot_product(*p, plane_y) * scale,
            }
        })
        .collect();

    let point_map = clustering::udc(output.clone(), min_points);

    let (output, best): (Vec<[f64; 2]>, [f64; 2]) = {
        let mut temp_best = [0.0, 0.0];
        let mut best_count = 0;
        let mut seen_points: HashSet<String> = HashSet::new();
        let return_value: (Vec<[f64; 2]>, [f64; 2]) = (
            point_map
                .into_iter()
                .filter_map(|(key, values)| {
                    if values.len() > best_count {
                        temp_best = key.clone().from_key();
                        best_count = values.len();
                    }
                    if values.len() >= min_points {
                        for p in values.into_iter() {
                            seen_points.insert(p);
                        }
                        return Some(key.from_key());
                    }
                    None
                })
                .collect(),
            temp_best,
        );
        println!(
            "Clusters Made: {} | {} points seen",
            return_value.0.len(),
            seen_points.len()
        );
        return_value
    };

    let mut min = 1. / 0.;
    let mut sum = 0.;
    println!(
        "Center: {:?}, {:?}",
        plane_center_lat.to_degrees(),
        plane_center_lon.to_degrees()
    );
    let mut final_output: Vec<[f64; 2]> = Vec::new();
    let (best_lat, best_lon, _) = reverse_project(
        best,
        (plane_center, plane_x, plane_y, plane_z, adjusted_radius),
    );
    for p in output.iter() {
        let (lat, lon, s) = reverse_project(
            *p,
            (plane_center, plane_x, plane_y, plane_z, adjusted_radius),
        );
        final_output.push([lat, lon]);
        if s < min {
            min = s;
        }
        sum += s;
    }
    println!(
        "Worst scaling: {:?} (larger/closer to 1 = better; larger area to cover is worse)",
        min
    );
    println!("Average scaling: {:?}", sum / output.len() as f64);
    println!("Disc scaling: {:?}", adjusted_radius / radius);
    (final_output, [best_lat, best_lon])
}
