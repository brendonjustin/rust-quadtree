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
     Create a quadtree with a root node with the given origin and size.
     */
    pub fn new(origin: Point, size: Size, elems: Elements) -> QuadTree {
        let tree = 
        match elems {
            Children(tl, tr, br, bl) => QuadTree::newWithChildren(origin, size, tl, tr, br, bl),
            Member(rect) => QuadTree::newWithMember(origin, size, rect),
            NoElements => QuadTree::newWithSize(origin, size),
        };

        tree
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

        let size = Size { width: largerDimen, height: largerDimen };

        QuadTree::newWithMember(rect.origin, size, rect)
    }

    /**
     Create an empty quadtree with a zero-sized root node.
     */
    pub fn newEmpty() -> QuadTree {
        let origin = Point { x: 0 as f32, y: 0 as f32 };
        let size = Size { width: 0 as f32, height: 0 as f32 };
        let tree = QuadTree::new(origin, size, NoElements);

        tree
    }

    /**
     Create a quadtree with a root node with the given origin, size, and child rectangles.
     Child nodes `tl`, `tr`, `br`, and `bl` should form the rect specified by `origin` and `size`.
     */
    fn newWithChildren(origin: Point, size: Size, tl: ~QuadTree, tr: ~QuadTree, br: ~QuadTree, bl: ~QuadTree) -> QuadTree {
        let nodeRect = Rect { origin: origin, size: size };

        // Assert that our rect and our childrens' rects match up.
        assert!(tl.rect.minX() == bl.rect.minX(), "Top left and bottom left children should have same minX coordinate.");
        assert!(tl.rect.maxX() == bl.rect.maxX(), "Top left and bottom left children should have same maxX coordinate.");
        assert!(tl.rect.maxY() == bl.rect.minY(), "Top left and bottom left children should have same maxY and minY coordinates, respectively.");

        assert!(tl.rect.maxX() == tr.rect.minX(), "Top left and top right children should share their maxX and minX coordinates, respectively.");
        assert!(tl.rect.minY() == tr.rect.minY(), "Top left and top right children should have same minY coordinate.");
        assert!(tl.rect.maxY() == tr.rect.maxY(), "Top left and top right children should have same maxY coordinate.");

        assert!(bl.rect.maxX() == br.rect.minX(), "Bottom left and bottom right children should share their maxX and minX coordinates, respectively.");
        assert!(bl.rect.minY() == br.rect.minY(), "Bottom left and bottom right children should have same minY coordinate.");
        assert!(bl.rect.maxY() == br.rect.maxY(), "Bottom left and bottom right children should have same maxY coordinate.");

        assert!(tr.rect.minX() == br.rect.minX(), "Top right and bottom right children should have same minX coordinate.");
        assert!(tr.rect.maxX() == br.rect.maxX(), "Top right and bottom right children should have same maxX coordinate.");
        assert!(tr.rect.maxY() == br.rect.minY(), "Top right and bottom right children should have same maxY and minY coordinates, respectively.");

        assert!(nodeRect.minX() == tl.rect.minX(), "Tree node's minX should match its top left child's minX coordinate.");
        assert!(nodeRect.maxX() == tr.rect.maxX(), "Tree node's maxX should match its top right child's maxX coordinate.");
        assert!(nodeRect.minY() == tl.rect.minY(), "Tree node's minY should match its top left child's minY coordinate.");
        assert!(nodeRect.maxY() == bl.rect.maxY(), "Tree node's maxY should match its top left child's maxY coordinate.");

        let tree = QuadTree { rect: nodeRect, elements: Children(tl, tr, br, bl) };

        tree
    }

    /**
     Create a quadtree with a root node with the given origin, size, and member rectangle.
     */
    fn newWithMember(origin: Point, size: Size, insertRect: Rect) -> QuadTree {
        let node = QuadTree::newWithSize(origin, size);

        let (success, tree) = node.insertRect(insertRect);
        assert!(success, "Failed inserting a node into an empty quadtree.");

        tree
    }

    /**
     Create a quadtree with only a specified size and position.
     */
    fn newWithSize(origin: Point, size: Size) -> QuadTree {
        let nodeRect = Rect { origin: origin, size: size };
        let tree = QuadTree { rect: nodeRect, elements: NoElements };

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