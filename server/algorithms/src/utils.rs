use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::{create_dir_all, File};
use std::io::{Result, Write};

use colored::Colorize;
use hashbrown::HashSet;
use model::api::{point_array::PointArray, single_vec::SingleVec};

use crate::rtree::cluster::Cluster;

pub fn debug_hashmap<T, U>(file_name: &str, input: &T) -> Result<()>
where
    U: Debug,
    T: Debug + Clone + IntoIterator<Item = U>,
{
    create_dir_all("./debug_files")?;
    let path = format!("./debug_files/{}", file_name);
    let mut content: String = "".to_string();

    for x in input.clone().into_iter() {
        content = format!("{}\n{:?}\n", content, x);
    }
    content = content.trim_end_matches(",").to_string();
    let mut output = File::create(path)?;
    write!(output, "{}", content)?;
    // println!("Saved {} to file with {} coords", file_name, input.len());
    Ok(())
}

pub fn debug_string(file_name: &str, input: &String) -> Result<()> {
    create_dir_all("./debug_files")?;
    let path = format!("./debug_files/{}", file_name);
    let mut output = File::create(path)?;
    write!(output, "{}", input)?;
    // println!("Saved {} to file with {} coords", file_name, input.len());
    Ok(())
}

pub fn get_sorted<T>(map: &HashMap<String, T>) -> Vec<(String, T)>
where
    T: Clone,
{
    let mut vec: Vec<&String> = map.keys().collect();
    vec.sort();
    vec.into_iter()
        .map(|k| (k.clone(), map.get(k).unwrap().clone()))
        .collect()
}

pub fn centroid(coords: &SingleVec) -> PointArray {
    let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);

    for loc in coords.iter() {
        let lat = loc[0].to_radians();
        let lon = loc[1].to_radians();

        x += lat.cos() * lon.cos();
        y += lat.cos() * lon.sin();
        z += lat.sin();
    }

    let number_of_locations = coords.len() as f64;
    x /= number_of_locations;
    y /= number_of_locations;
    z /= number_of_locations;

    let hyp = (x * x + y * y).sqrt();
    let lon = y.atan2(x);
    let lat = z.atan2(hyp);

    [lat.to_degrees(), lon.to_degrees()]
}

pub fn info_log(file_name: &str, message: String) -> String {
    format!(
        "\r{}{}Z {}  {}{} {}",
        "[".black(),
        chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
        "INFO".green(),
        file_name,
        "]".black(),
        message
    )
}

pub fn _debug_clusters(clusters: &HashSet<Cluster>, file_suffix: &str) {
    let mut point_map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut cluster_map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut unique_map: HashMap<String, HashSet<String>> = HashMap::new();

    for cluster in clusters.iter() {
        cluster_map.insert(
            cluster.point._get_geohash(),
            cluster.all.iter().map(|p| p._get_geohash()).collect(),
        );
        unique_map.insert(
            cluster.point._get_geohash(),
            cluster.points.iter().map(|p| p._get_geohash()).collect(),
        );
        for point in cluster.all.iter() {
            point_map
                .entry(point._get_geohash())
                .and_modify(|f| {
                    f.insert(cluster.point._get_geohash());
                })
                .or_insert_with(|| {
                    let mut set: HashSet<String> = HashSet::new();
                    set.insert(cluster.point._get_geohash());
                    set
                });
        }
    }

    debug_hashmap(&format!("{}_point.txt", file_suffix), &point_map).unwrap();
    debug_hashmap(&format!("{}_cluster.txt", file_suffix), &cluster_map).unwrap();
    debug_hashmap(&format!("{}_unique.txt", file_suffix), &unique_map).unwrap();
}
