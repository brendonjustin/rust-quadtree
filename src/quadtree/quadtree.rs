use geometry::Point;
use geometry::Rect;
use geometry::Size;

/**
 Elements that may be contained by a quadtree node.
 Either child nodes, a single rect, or nothing.
 */
#[deriving(Show)]
pub enum Elements {
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

        let size = Size::new(largerDimen, largerDimen);

        QuadTree::newWithMember(rect.origin, size, rect)
    }

    /**
     Create an empty quadtree with a zero-sized root node.
     */
    pub fn newEmpty() -> QuadTree {
        let origin = Point::new(0., 0.);
        let size = Size::new(0., 0.);
        let tree = QuadTree::new(origin, size, NoElements);

        tree
    }

    /**
     Create a quadtree with a root node with the given origin, size, and child rectangles.
     Child nodes `tl`, `tr`, `br`, and `bl` should form the rect specified by `origin` and `size`.
     */
    fn newWithChildren(origin: Point, size: Size, tl: ~QuadTree, tr: ~QuadTree, br: ~QuadTree, bl: ~QuadTree) -> QuadTree {
        let nodeRect = Rect::new(origin, size);

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
        let qtRect = Rect::new(origin, size);
        assert!(qtRect.contains(&insertRect),
            "QuadTree node constructed by newWithMember not able to contain the rect it is passed in.");
        let tree = QuadTree { rect: qtRect, elements: Member(insertRect) };

        tree
    }

    /**
     Create a quadtree with only a specified size and position.
     */
    fn newWithSize(origin: Point, size: Size) -> QuadTree {
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
    pub fn insertRect(self, toInsert: Rect) -> (bool, QuadTree) {
        if self.rect.width() == 0.0 {
            return (true, QuadTree::newAutosized(toInsert))
        }

        let rectsInChildren = self.rectsInChildNodesIntersectedByRect(&toInsert);
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
            let left = node.rect.minX() < toInsert.minX();
            let top = node.rect.minX() < toInsert.minY();

            let (tl, tr, bl, br) =
            match (left, top) {
                (true, true) => (QuadTree::newWithSize(origin.subtract(wPoint).add(hPoint), size),
                                 QuadTree::newWithSize(origin.subtract(hPoint), size),
                                 QuadTree::newWithSize(origin.subtract(wPoint), size),
                                 node),
                (true, false) => (QuadTree::newWithSize(origin.subtract(wPoint), size),
                                  node,
                                  QuadTree::newWithSize(origin.subtract(wPoint).subtract(hPoint), size),
                                  QuadTree::newWithSize(origin.subtract(hPoint), size)),
                (false, true) => (QuadTree::newWithSize(origin.add(hPoint), size),
                                  QuadTree::newWithSize(origin.add(hPoint).add(wPoint), size),
                                  node,
                                  QuadTree::newWithSize(origin.add(wPoint), size)),
                (false, false) => (node,
                                   QuadTree::newWithSize(origin.add(wPoint), size),
                                   QuadTree::newWithSize(origin.add(hPoint), size),
                                   QuadTree::newWithSize(origin.add(wPoint).add(hPoint), size)),
            };

            node = QuadTree::newWithChildren(tl.rect.origin,
                Size::new(width * 2., height * 2.),
                ~tl, ~tr, ~br, ~bl);

            bigEnough = node.rect.contains(&toInsert);
        }

        let origin = node.rect.origin;
        let size = node.rect.size;
        node = match node.elements {
            Children(tl, tr, br, bl) => QuadTree::newWithChildren(
                origin,
                size,
                ~tl.insertRectIfIntersects(toInsert),
                ~tr.insertRectIfIntersects(toInsert),
                ~bl.insertRectIfIntersects(toInsert),
                ~br.insertRectIfIntersects(toInsert)),
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
                QuadTree::newWithChildren(
                    origin,
                    size,
                    ~QuadTree::newWithSize(tlo, hSize).insertRectIfIntersects(rect).insertRectIfIntersects(toInsert),
                    ~QuadTree::newWithSize(tro, hSize).insertRectIfIntersects(rect).insertRectIfIntersects(toInsert),
                    ~QuadTree::newWithSize(bro, hSize).insertRectIfIntersects(rect).insertRectIfIntersects(toInsert),
                    ~QuadTree::newWithSize(blo, hSize).insertRectIfIntersects(rect).insertRectIfIntersects(toInsert),)
            }
            NoElements => QuadTree::newWithMember(origin, size, toInsert),
        };

        (true, node)
    }

    /**
     Insert a rectangle into the node IFF the rectangle intersects the node.
     Does nothing if `toInsert` intersects our existing member rect.
     */
    fn insertRectIfIntersects(self, toInsert: Rect) -> QuadTree {
        if self.rect.intersects(&toInsert) {
            let origin = self.rect.origin;
            let size = self.rect.size;
            match self.elements {
                Children(~tl, ~tr, ~br, ~bl) => {
                    let (tl, tr, br, bl) = (tl.insertRectIfIntersects(toInsert),
                                            tr.insertRectIfIntersects(toInsert),
                                            br.insertRectIfIntersects(toInsert),
                                            bl.insertRectIfIntersects(toInsert),);
                    QuadTree::newWithChildren(origin, size, ~tl, ~tr, ~br, ~bl)
                },
                Member(rect) => {
                    if rect.intersects(&toInsert) {
                        self
                    } else {
                        let (tl, tr, br, bl) = QuadTree::makeChildrenForRect(&self.rect);

                        let (tl, tr, br, bl) = (tl.insertRectIfIntersects(rect),
                                                tr.insertRectIfIntersects(rect),
                                                br.insertRectIfIntersects(rect),
                                                bl.insertRectIfIntersects(rect),);
                        let (tl, tr, br, bl) = (tl.insertRectIfIntersects(toInsert),
                                                tr.insertRectIfIntersects(toInsert),
                                                br.insertRectIfIntersects(toInsert),
                                                bl.insertRectIfIntersects(toInsert),);

                        QuadTree::newWithChildren(origin, size, ~tl, ~tr, ~br, ~bl)
                    }
                },
                NoElements => QuadTree::newWithMember(origin, size, toInsert),
            }
        } else {
            self
        }
    }

    /**
     Create four nodes suitable for use as children, covering the passed in rect.
     */
    fn makeChildrenForRect(rect: &Rect) -> (~QuadTree, ~QuadTree, ~QuadTree, ~QuadTree,) {
        let origin = rect.origin;
        let size = rect.size;

        let newSize = Size::new(size.width / 2., size.height / 2.);
        let wPoint = Point::new(newSize.width, 0.);
        let hPoint = Point::new(0., newSize.height);

        let (tl, tr, br, bl) = (QuadTree::newWithSize(origin, newSize),
                                QuadTree::newWithSize(origin.add(wPoint), newSize),
                                QuadTree::newWithSize(origin.add(wPoint).add(hPoint), newSize),
                                QuadTree::newWithSize(origin.add(hPoint), newSize),);

        (~tl, ~tr, ~br, ~bl)
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
