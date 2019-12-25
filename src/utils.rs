use mint::Point2;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct AxialIndex {
    pub q: isize,
    pub r: isize,
}

impl AxialIndex {
    pub fn hex_to_pixel(&self, size: f32, offset: Option<&Point2<f32>>) -> Point2<f32> {
        let x = size * (3.0_f32.sqrt() * self.q as f32 + 3.0_f32.sqrt() / 2.0 * self.r as f32);
        let y = size * 3.0 / 2.0 * self.r as f32;
        match offset {
            None => Point2 { x, y },
            Some(offset) => Point2 { x: x + offset.x, y: y + offset.y },
        }
    }
}

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
