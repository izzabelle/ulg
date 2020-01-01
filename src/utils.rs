use mint::Point2;

/// a hexagon index in a map using axial indexing (q/r)
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct AxialIndex {
    pub q: isize,
    pub r: isize,
}

impl AxialIndex {
    /// converts the axial index into a pixel location for drawing
    pub fn hex_to_pixel(&self, size: f32, offset: Option<&Point2<f32>>) -> Point2<f32> {
        let x = size * (3.0_f32.sqrt() * self.q as f32 + 3.0_f32.sqrt() / 2.0 * self.r as f32);
        let y = size * 3.0 / 2.0 * self.r as f32;
        match offset {
            None => Point2 { x, y },
            Some(offset) => Point2 { x: x + offset.x, y: y + offset.y },
        }
    }

    /// gets an axial index from a given position
    pub fn from_pixel(point: Point2<f32>, size: f32) -> Self {
        let point = Point2 { x: point.x, y: point.y };
        let q = (3.0_f32.sqrt() / 3.0 * point.x - 1.0 / 3.0 * point.y) / size;
        let r = (3.0 / 3.0 * point.y) / size;
        Self { q: q.floor() as isize, r: r.floor() as isize }
    }
}

// moves the origin on the screen from top left to center
pub fn point_from_center(coord: Point2<f32>, window_dimensions: (f32, f32)) -> Point2<f32> {
    let center = Point2 { x: window_dimensions.0 / 2.0, y: window_dimensions.1 / 2.0 };
    let (x, y): (f32, f32);

    if coord.x <= center.x {
        x = -(center.x - coord.x);
    } else {
        x = coord.x - center.x;
    }

    if center.y <= center.y {
        y = -(center.y - coord.y);
    } else {
        y = coord.y - center.y;
    }

    Point2 { x, y }
}
