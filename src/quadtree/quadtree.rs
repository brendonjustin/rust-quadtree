use geometry::Point;
use geometry::Rect;
use geometry::Size;

use std::vec::Vec;

/**
 Elements that may be contained by a quadtree node.
 Either child nodes, a single rect, or nothing.
 */
#[deriving(Show)]
pub enum Elements {
    /// Children are top left, top right, bottom right, and bottom left, respectively.
    Children(Box<QuadTree>, Box<QuadTree>, Box<QuadTree>, Box<QuadTree>),
    /// A single rectangle.
    Member(Rect),
    /// Nothing.
    NoElements,
}

/**
 A quadtree node that can contain either one rectangle,
 or exactly four child nodes.
 */
#[deriving(Show)]
pub struct QuadTree {
    pub rect: Rect,
    pub elements: Elements,
}

impl QuadTree {
    /**
     Create a quadtree with a root node with the given origin and size.
     */
    pub fn new(origin: Point, size: Size, elems: Elements) -> QuadTree {
        let tree =
        match elems {
            Children(tl, tr, br, bl) => QuadTree::new_with_children(origin, size, tl, tr, br, bl),
            Member(rect) => QuadTree::new_with_member(origin, size, rect),
            NoElements => QuadTree::new_with_size(origin, size),
        };

        tree
    }

    /**
     Create a quadtree with a root node with the same origin and a square
     size with side length matching the longer dimension of `rect`.
     */
    pub fn new_autosized(rect: Rect) -> QuadTree {
        let largerDimen = if rect.size.width > rect.size.height {
            rect.size.width
        } else {
            rect.size.height
        };

        let size = Size::new(largerDimen, largerDimen);

        QuadTree::new_with_member(rect.origin, size, rect)
    }

    /**
     Create an empty quadtree with a zero-sized root node.
     */
    pub fn new_empty() -> QuadTree {
        let origin = Point::new(0., 0.);
        let size = Size::new(0., 0.);
        let tree = QuadTree::new(origin, size, NoElements);

        tree
    }

    /**
     Create a quadtree with a root node with the given origin, size, and child rectangles.
     Child nodes `tl`, `tr`, `br`, and `bl` should form the rect specified by `origin` and `size`.
     */
    fn new_with_children(origin: Point, size: Size, tl: Box<QuadTree>, tr: Box<QuadTree>, br: Box<QuadTree>, bl: Box<QuadTree>) -> QuadTree {
        let nodeRect = Rect::new(origin, size);

        // Assert that our rect and our childrens' rects match up.
        assert!(tl.rect.min_x() == bl.rect.min_x(), "Top left and bottom left children should have same min_x coordinate.");
        assert!(tl.rect.max_x() == bl.rect.max_x(), "Top left and bottom left children should have same max_x coordinate.");
        assert!(tl.rect.max_y() == bl.rect.min_y(), "Top left and bottom left children should have same max_y and min_y coordinates, respectively.");

        assert!(tl.rect.max_x() == tr.rect.min_x(), "Top left and top right children should share their max_x and min_x coordinates, respectively.");
        assert!(tl.rect.min_y() == tr.rect.min_y(), "Top left and top right children should have same min_y coordinate.");
        assert!(tl.rect.max_y() == tr.rect.max_y(), "Top left and top right children should have same max_y coordinate.");

        assert!(bl.rect.max_x() == br.rect.min_x(), "Bottom left and bottom right children should share their max_x and min_x coordinates, respectively.");
        assert!(bl.rect.min_y() == br.rect.min_y(), "Bottom left and bottom right children should have same min_y coordinate.");
        assert!(bl.rect.max_y() == br.rect.max_y(), "Bottom left and bottom right children should have same max_y coordinate.");

        assert!(tr.rect.min_x() == br.rect.min_x(), "Top right and bottom right children should have same min_x coordinate.");
        assert!(tr.rect.max_x() == br.rect.max_x(), "Top right and bottom right children should have same max_x coordinate.");
        assert!(tr.rect.max_y() == br.rect.min_y(), "Top right and bottom right children should have same max_y and min_y coordinates, respectively.");

        assert!(nodeRect.min_x() == tl.rect.min_x(), "Tree node's min_x should match its top left child's min_x coordinate.");
        assert!(nodeRect.max_x() == tr.rect.max_x(), "Tree node's max_x should match its top right child's max_x coordinate.");
        assert!(nodeRect.min_y() == tl.rect.min_y(), "Tree node's min_y should match its top left child's min_y coordinate.");
        assert!(nodeRect.max_y() == bl.rect.max_y(), "Tree node's max_y should match its top left child's max_y coordinate.");

        let tree = QuadTree { rect: nodeRect, elements: Children(tl, tr, br, bl) };

        tree
    }

    /**
     Create a quadtree with a root node with the given origin, size, and member rectangle.
     */
    fn new_with_member(origin: Point, size: Size, insertRect: Rect) -> QuadTree {
        let qtRect = Rect::new(origin, size);
        assert!(qtRect.contains(&insertRect),
            "QuadTree node constructed by new_with_member not able to contain the rect it is passed in.");
        let tree = QuadTree { rect: qtRect, elements: Member(insertRect) };

        tree
    }

    /**
     Create a quadtree with only a specified size and position.
     */
    fn new_with_size(origin: Point, size: Size) -> QuadTree {
        let nodeRect = Rect::new(origin, size);
        let tree = QuadTree { rect: nodeRect, elements: NoElements };

        tree
    }

    /**
     Insert a rectangle into the quadtree. If `toInsert` overlaps another rectangle
     already in the tree, the return value will be (false, self).
     If the root node is zero-sized, the resulting tree will have a square root node
     large enough to hold `toInsert`.
     */
    pub fn insert_rect(self, toInsert: Rect) -> (bool, QuadTree) {
        if self.rect.width() == 0.0 {
            return (true, QuadTree::new_autosized(toInsert))
        }

        let rectsInChildren = self.rects_in_child_nodes_intersected_by_rect(&toInsert);
        if rectsInChildren.len() > 0 {
            return (false, self)
        }

        let mut node = self;
        let mut bigEnough = node.rect.contains(&toInsert);

        while !bigEnough {
            let width = node.rect.width();
            let height = node.rect.height();
            let origin = node.rect.origin;
            let size = node.rect.size;

            let wPoint = Point::new(width, 0.);
            let hPoint = Point::new(0., height);

            // Check if the rects to insert extends to the left or "above" our origin,
            // i.e. has a lower x or y coordinate in its origin.
            // Use this information to determine if we must grow the tree left, up, right, or down.
            let left = node.rect.min_x() < toInsert.min_x();
            let top = node.rect.min_x() < toInsert.min_y();

            let (tl, tr, bl, br) =
            match (left, top) {
                (true, true) => (QuadTree::new_with_size(origin.subtract(wPoint).add(hPoint), size),
                                 QuadTree::new_with_size(origin.subtract(hPoint), size),
                                 QuadTree::new_with_size(origin.subtract(wPoint), size),
                                 node),
                (true, false) => (QuadTree::new_with_size(origin.subtract(wPoint), size),
                                  node,
                                  QuadTree::new_with_size(origin.subtract(wPoint).subtract(hPoint), size),
                                  QuadTree::new_with_size(origin.subtract(hPoint), size)),
                (false, true) => (QuadTree::new_with_size(origin.add(hPoint), size),
                                  QuadTree::new_with_size(origin.add(hPoint).add(wPoint), size),
                                  node,
                                  QuadTree::new_with_size(origin.add(wPoint), size)),
                (false, false) => (node,
                                   QuadTree::new_with_size(origin.add(wPoint), size),
                                   QuadTree::new_with_size(origin.add(hPoint), size),
                                   QuadTree::new_with_size(origin.add(wPoint).add(hPoint), size)),
            };

            node = QuadTree::new_with_children(tl.rect.origin,
                Size::new(width * 2., height * 2.),
                box tl, box tr, box br, box bl);

            bigEnough = node.rect.contains(&toInsert);
        }

        let origin = node.rect.origin;
        let size = node.rect.size;
        node = match node.elements {
            Children(tl, tr, br, bl) => QuadTree::new_with_children(
                origin,
                size,
                box tl.insert_rect_if_intersects(toInsert),
                box tr.insert_rect_if_intersects(toInsert),
                box bl.insert_rect_if_intersects(toInsert),
                box br.insert_rect_if_intersects(toInsert)),
            Member(rect) => {
                let hw = size.width / 2.;
                let hh = size.height / 2.;
                let wp = Point::new(hw, 0.);
                let hp = Point::new(0., hh);
                let (tlo, tro, bro, blo) = (
                    origin,
                    origin.add(wp),
                    origin.add(hp),
                    origin.add(wp).add(hp),
                    );
                let hSize = Size::new(hw, hh);
                QuadTree::new_with_children(
                    origin,
                    size,
                    box QuadTree::new_with_size(tlo, hSize).insert_rect_if_intersects(rect).insert_rect_if_intersects(toInsert),
                    box QuadTree::new_with_size(tro, hSize).insert_rect_if_intersects(rect).insert_rect_if_intersects(toInsert),
                    box QuadTree::new_with_size(bro, hSize).insert_rect_if_intersects(rect).insert_rect_if_intersects(toInsert),
                    box QuadTree::new_with_size(blo, hSize).insert_rect_if_intersects(rect).insert_rect_if_intersects(toInsert),)
            }
            NoElements => QuadTree::new_with_member(origin, size, toInsert),
        };

        (true, node)
    }

    /**
     Insert a rectangle into the node IFF the rectangle intersects the node.
     Does nothing if `toInsert` intersects our existing member rect.
     */
    fn insert_rect_if_intersects(self, toInsert: Rect) -> QuadTree {
        if self.rect.intersects(&toInsert) {
            let origin = self.rect.origin;
            let size = self.rect.size;
            match self.elements {
                Children(box tl, box tr, box br, box bl) => {
                    let (tl, tr, br, bl) = (tl.insert_rect_if_intersects(toInsert),
                                            tr.insert_rect_if_intersects(toInsert),
                                            br.insert_rect_if_intersects(toInsert),
                                            bl.insert_rect_if_intersects(toInsert),);
                    QuadTree::new_with_children(origin, size, box tl, box tr, box br, box bl)
                },
                Member(rect) => {
                    if rect.intersects(&toInsert) {
                        self
                    } else {
                        let (tl, tr, br, bl) = QuadTree::make_children_for_rect(&self.rect);

                        let (tl, tr, br, bl) = (tl.insert_rect_if_intersects(rect),
                                                tr.insert_rect_if_intersects(rect),
                                                br.insert_rect_if_intersects(rect),
                                                bl.insert_rect_if_intersects(rect),);
                        let (tl, tr, br, bl) = (tl.insert_rect_if_intersects(toInsert),
                                                tr.insert_rect_if_intersects(toInsert),
                                                br.insert_rect_if_intersects(toInsert),
                                                bl.insert_rect_if_intersects(toInsert),);

                        QuadTree::new_with_children(origin, size, box tl, box tr, box br, box bl)
                    }
                },
                NoElements => QuadTree::new_with_member(origin, size, toInsert),
            }
        } else {
            self
        }
    }

    /**
     Create four nodes suitable for use as children, covering the passed in rect.
     */
    fn make_children_for_rect(rect: &Rect) -> (Box<QuadTree>, Box<QuadTree>, Box<QuadTree>, Box<QuadTree>,) {
        let origin = rect.origin;
        let size = rect.size;

        let newSize = Size::new(size.width / 2., size.height / 2.);
        let wPoint = Point::new(newSize.width, 0.);
        let hPoint = Point::new(0., newSize.height);

        let (tl, tr, br, bl) = (QuadTree::new_with_size(origin, newSize),
                                QuadTree::new_with_size(origin.add(wPoint), newSize),
                                QuadTree::new_with_size(origin.add(wPoint).add(hPoint), newSize),
                                QuadTree::new_with_size(origin.add(hPoint), newSize),);

        (box tl, box tr, box br, box bl)
    }

    /**
     Find all of the rects in `self`, or its children, that are in nodes
     intersected by the given rect.
     */
    pub fn rects_in_child_nodes_intersected_by_rect(&self, testRect: &Rect) -> Vec<Rect> {
        // If the test rect doesn't intersect us, then it can't intersect
        // any rects that we have.
        match self.rect.intersect(testRect) {
            Some(intersection) => {
                let mut rects = Vec::new();
                let mut nodesToCheck = vec!(self);

                while nodesToCheck.len() > 0 {
                    let mut newNodesToCheck = Vec::new();

                    for node in nodesToCheck.iter() {
                        match node.elements {
                            Children(box ref tl, box ref tr, box ref br, box ref bl) => {
                                let intersection = &intersection;
                                if tl.rect.intersects(intersection) {
                                    newNodesToCheck.push(tl);
                                }
                                if tr.rect.intersects(intersection) {
                                    newNodesToCheck.push(tr);
                                }
                                if br.rect.intersects(intersection) {
                                    newNodesToCheck.push(br);
                                }
                                if bl.rect.intersects(intersection) {
                                    newNodesToCheck.push(bl);
                                }
                            }
                            Member(memberRect) => rects.push(memberRect),
                            NoElements => ()
                        };
                    }

                    nodesToCheck = newNodesToCheck;
                }

                rects
            },
            None => Vec::new(),
        }
    }
}
