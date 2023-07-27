use crate::utils::Area;

mod loader;
mod quadtree;
mod utils;

fn main() {
    let mut points = loader::load_csv("data.csv");
    let area = Area::new(0.0, 1.0, 0.0, 1.0);
    let mut quad = quadtree::Quadtree::new(2000, area);
    quad.build(&mut points, quad.area);
    // quad.nodes contains all nodes and leads of quadtree along areas.
    // quad.indices contains ranges indices of points in leafs.
    // The modified points vector has already a cache memory friendly layout.
}
