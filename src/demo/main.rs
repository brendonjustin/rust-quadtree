extern crate quadtree;

fn main() {
    let origin = quadtree::geometry::Point::new(0., 0.);
    let size = quadtree::geometry::Size::new(1., 1.);
    let rect = quadtree::geometry::Rect::new(origin, size);
    let qt = quadtree::quadtree::QuadTree::newAutosized(rect);
}
