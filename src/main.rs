mod parameters;
mod generator;

use voronoi::{Point, voronoi, make_polygons};
use svg::{Document, node::element::Path, save};

use generator::{gen_base_box, get_centroid, move_to_centroid};

fn main() {
    const BOX_SIZE: f64 = 500.0;
    let sites = generator::generate_sites_with_random(BOX_SIZE, Point::new(34.0, 34.0), 0.5);
    let mut document = Document::new();
    let voronoi = voronoi(sites, BOX_SIZE);
    let voronoi_polygons = make_polygons(&voronoi);
    let mut data = gen_base_box(BOX_SIZE);
    for polygon in voronoi_polygons.iter() {
        let centroid = get_centroid(polygon);
        const SCALE: f64 = 0.1;
        let p0 = move_to_centroid(&polygon[0], centroid, SCALE);
        data = data.move_to(p0);
        for point in polygon.iter().skip(1) {
            data = data.line_to(move_to_centroid(point, centroid, SCALE));
        }
        data = data.close();
    }

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data);

    document = document.add(path);
    match save("voronoi.svg", &document) {
        Ok(ok) => println!("{:?}", ok),
        Err(err) => println!("{:?}", err)
    }
}
