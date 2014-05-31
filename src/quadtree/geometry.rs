#[deriving(Clone, PartialEq, Show)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[deriving(Clone, PartialEq, Show)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[deriving(Clone, PartialEq, Show)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    pub fn add(&self, addPoint: Point) -> Point {
        Point::new(self.x + addPoint.x, self.y + addPoint.y)
    }

    pub fn subtract(&self, offsetPoint: Point) -> Point {
        Point::new(self.x - offsetPoint.x, self.y - offsetPoint.y)
    }
}

impl Size {
    pub fn new(width: f64, height: f64) -> Size {
        Size { width: width, height: height }
    }
}

impl Rect {
    pub fn new(origin: Point, size: Size) -> Rect {
        Rect { origin: origin, size: size }
    }

    /// Find which the rect has an origin farther to the left.
    pub fn min_x_rect<'a>(rect1: &'a Rect, rect2: &'a Rect) -> (&'a Rect, &'a Rect) {
        if rect1.min_x() <= rect2.min_x() {
            (rect1, rect2)
        } else {
            (rect2, rect1)
        }
    }

    /// Find which the rect has an origin with a lower y value.
    pub fn min_y_rect<'a>(rect1: &'a Rect, rect2: &'a Rect) -> (&'a Rect, &'a Rect) {
        if rect1.min_y() <= rect2.min_y() {
            (rect1, rect2)
        } else {
            (rect2, rect1)
        }
    }

    /**
     Check if this rect entirely contains another rect.
     */
    pub fn contains(&self, rect: &Rect) -> bool {
        // Find which the rect has an origin farther to the left.
        let (minXRect, otherRect) = Rect::min_x_rect(self, rect);

        // If the rectangles don't intersect, one cannot be contained in the other.
        self.intersects(rect)
        && (minXRect.max_x() >= otherRect.max_x() && minXRect.max_y() >= otherRect.max_y())
    }

    /**
     Check if this rect and another rect intersect.
     */
    pub fn intersects(&self, rect: &Rect) -> bool {
        let (minXRect, otherRect) = Rect::min_x_rect(self, rect);

        let intersects: bool = (minXRect.max_x() >= otherRect.min_x())
            && ((minXRect.min_y() >= otherRect.min_y() && minXRect.min_y() <= otherRect.max_y())
                || (minXRect.min_y() <= otherRect.min_y() && minXRect.max_y() >= otherRect.min_y()));

        intersects
    }

    /**
     Get the intersection with another rect.
     */
    pub fn intersect(&self, rect: &Rect) -> Option<Rect> {
        if !self.intersects(rect) {
            return None;
        }

        let (minXRect, otherXRect) = Rect::min_x_rect(self, rect);
        let (minYRect, otherYRect) = Rect::min_y_rect(self, rect);
        let commonXStart = otherXRect.min_x();
        let commonYStart = otherYRect.min_y();

        let commonXEnd = minXRect.max_x().min(otherXRect.max_x());
        let commonYEnd = minYRect.max_y().min(otherYRect.max_y());

        let width = commonXEnd - commonXStart;
        let height = commonYEnd - commonYStart;

        Some(Rect::new(Point::new(commonXStart, commonYStart), Size::new(width, height)))
    }

    pub fn max_x(&self) -> f64 {
        self.origin.x + self.size.width
    }

    pub fn max_y(&self) -> f64 {
        self.origin.y + self.size.height
    }

    pub fn min_x(&self) -> f64 {
        self.origin.x
    }

    pub fn min_y(&self) -> f64 {
        self.origin.y
    }

    pub fn height(&self) -> f64 {
        self.size.height
    }

    pub fn width(&self) -> f64 {
        self.size.width
    }
}
