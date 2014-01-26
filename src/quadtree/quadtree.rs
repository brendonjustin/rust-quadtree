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
    None
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
     Create a quadtree with a square root node with the same origin and size
     as the passed in rect.
     */
    fn newAutosized(rect: Rect) -> QuadTree {
        QuadTree { rect: rect, elements: Member(rect), }
    }

    /**
     Create a quadtree with a root node with the given origin, size, and member rectangle.
     If `insertRect` is not entirely contained within the rectangle defined by `origin` and `size`,
     higher level nodes will be created to form a quadtree with a child of the rect specified
     and other children such that the rect is in the tree.
     */
    fn new(origin: Point, size: Size, insertRect: Rect) -> QuadTree {
        let nodeRect = Rect { origin: origin, size: size };
        let node = QuadTree { rect: nodeRect, elements: None };

        node.insertRect(insertRect)
    }

    fn insertRect(self, toInsert: Rect) -> QuadTree {
        fail!("insertRect not implemented!");

        self
    }
}