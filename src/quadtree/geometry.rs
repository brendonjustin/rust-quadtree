pub struct Point {
    x: f32,
    y: f32,
}

pub struct Size {
    width: f32,
    height: f32,
}

pub struct Rect {
    origin: Point,
    size: Size,
}


impl Rect {
    fn new(origin: Point, size: Size) -> Rect {
        Rect { origin: origin, size: size }
    }

    /// Find which the rect has an origin farther to the left.
    fn minXRect<'a>(rect1: &'a Rect, rect2: &'a Rect) -> (&'a Rect, &'a Rect) {
        if (rect1.minX() <= rect2.minX()) {
            (rect1, rect2)
        } else {
            (rect2, rect1)
        }
    }

    /**
     Check if this rect entirely contains another rect.
     */
    fn contains(&self, rect: &Rect) -> bool {
        // Find which the rect has an origin farther to the left.
        let (minXRect, otherRect) = Rect::minXRect(self, rect);

        // If the rectangles don't intersect, one cannot be contained in the other.
        self.intersects(rect)
        && (minXRect.maxX() >= otherRect.maxX() && minXRect.maxY() >= otherRect.maxY())
    }

    /**
     Check if this rect and another rect intersect.
     */
    fn intersects(&self, rect: &Rect) -> bool {
        let (minXRect, otherRect) = Rect::minXRect(self, rect);

        let intersects: bool = ((minXRect.maxX() >= otherRect.minX())
            && ((minXRect.minY() >= otherRect.minY() && minXRect.minY() <= otherRect.maxY())
                || (minXRect.minY() <= otherRect.minY() && minXRect.maxY() >= otherRect.minY())));

        intersects
    }

    fn maxX(&self) -> f32 {
        self.origin.x + self.size.width
    }

    fn maxY(&self) -> f32 {
        self.origin.y + self.size.height
    }

    fn minX(&self) -> f32 {
        self.origin.x
    }

    fn minY(&self) -> f32 {
        self.origin.y
    }
}