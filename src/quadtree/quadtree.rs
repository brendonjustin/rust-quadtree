use geometry::Point;
use geometry::Rect;
use geometry::Size;

/**
 Elements that may be contained by a quadtree node.
 Either child nodes, a single rect, or nothing.
 */
enum Elements {
    /// Children are top left, top right, bottom right, and bottom left, respectively.
    Children(~QuadTree, ~QuadTree, ~QuadTree, ~QuadTree),
    /// A single rectangle.
    Member(Rect),
    /// Nothing.
    NoElements,
}

/**
 A quadtree node that can contain either one rectangle,
 or exactly four child nodes.
 */
pub struct QuadTree {
    rect: Rect,
    elements: Elements,
}

impl QuadTree {
    /**
     Create an empty quadtree with a zero-sized root node.
     */
    pub fn new() -> QuadTree {
        let origin = Point { x: 0 as f32, y: 0 as f32 };
        let size = Size { width: 0 as f32, height: 0 as f32 };
        let nodeRect = Rect { origin: origin, size: size };
        let node = QuadTree { rect: nodeRect, elements: NoElements };

        node
    }

    /**
     Create a quadtree with a root node with the same origin and a square
     size with side length matching the longer dimension of `rect`.
     */
    pub fn newAutosized(rect: Rect) -> QuadTree {
        let largerDimen = if rect.size.width > rect.size.height {
            rect.size.width
        } else {
            rect.size.height
        };

        let square = Rect { origin: rect.origin, size: Size { width: largerDimen, height: largerDimen } };

        QuadTree { rect: square, elements: Member(rect), }
    }

    /**
     Create a quadtree with a root node with the given origin, size, and member rectangle.
     If `insertRect` is not entirely contained within the rectangle defined by `origin` and `size`,
     higher level nodes will be created to form a quadtree with a child of the rect specified
     and other children such that the rect is in the tree.
     */
    pub fn newWithSize(origin: Point, size: Size, insertRect: Rect) -> QuadTree {
        let nodeRect = Rect { origin: origin, size: size };
        let node = QuadTree { rect: nodeRect, elements: NoElements };

        let (success, tree) = node.insertRect(insertRect);

        assert!(success, "Failed inserting a node into an empty quadtree.");

        tree
    }

    /**
     Insert a rectangle into the quadtree. If `toInsert` overlaps another rectangle
     already in the tree, the return value will be (false, self). 
     If the root node is zero-sized, the resulting tree will have a square root node
     large enough to hold `toInsert`.
     */
    pub fn insertRect(self, toInsert: Rect) -> (bool, QuadTree) {
        if self.rect.size.width == 0.0 {
            return (true, QuadTree::newAutosized(toInsert))
        }

        fail!("insertRect not implemented for non-zero sized trees!");

        (false, self)
    }

    /**
     Find all of the rects in `self`, or its children, that are in nodes
     intersected by the given rect.
     */
    pub fn rectsInChildNodesIntersectedByRect(&self, testRect: &Rect) -> ~[Rect] {
        // If the test rect doesn't intersect us, then it can't intersect
        // any rects that we have.
        match self.rect.intersect(testRect) {
            Some(intersection) => {
                let mut rects = ~[];
                let mut nodesToCheck = ~[self];

                while nodesToCheck.len() > 0 {
                    let mut newNodesToCheck = ~[];

                    for node in nodesToCheck.iter() {
                        match node.elements {
                            Children(~ref tl, ~ref tr, ~ref br, ~ref bl) => {
                                let mut check = ~[];
                                let intersection = &intersection;
                                if tl.rect.intersects(intersection) {
                                    check.push(tl);
                                }
                                if tr.rect.intersects(intersection) {
                                    check.push(tr);
                                }
                                if br.rect.intersects(intersection) {
                                    check.push(br);
                                }
                                if bl.rect.intersects(intersection) {
                                    check.push(bl);
                                }
                                newNodesToCheck.push_all(check);
                            }
                            Member(memberRect) => rects.push(memberRect),
                            NoElements => ()
                        };
                    }

                    nodesToCheck = newNodesToCheck;
                }

                rects
            },
            None => ~[],
        }
    }
}