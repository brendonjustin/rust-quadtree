extern mod quadtree;

fn main() {
    let origin = quadtree::Point { 0 as f32, 0 as f32 };
    let size = quadtree::Size { 1 as f32, 1 as f32 };
    let rect = quadtree::Rect::new(origin, size);
    let qt = quadtree::QuadTree::newAutosized(rect);
}