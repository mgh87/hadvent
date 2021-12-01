use tui::{
    style::Color,
    widgets::canvas::{Line, Painter, Shape},
};

/// Shape to draw a rectangle from a `Rect` with the given color
#[derive(Debug, Clone)]
pub struct Triangle {
    pub p1: (f64,f64),
    pub p2: (f64,f64),
    pub p3: (f64,f64),
    pub color: Color,
}

impl Shape for Triangle {
    fn draw(&self, painter: &mut Painter) {
        let lines: [Line; 3] = [
            Line {
                x1: self.p1.0,
                y1: self.p1.1,
                x2: self.p2.0,
                y2: self.p2.1,
                color: self.color,
            },
            Line {
                x1: self.p1.0,
                y1: self.p1.1,
                x2: self.p3.0,
                y2: self.p3.1,
                color: self.color,
            },
            Line {
                x1: self.p2.0,
                y1: self.p2.1,
                x2: self.p3.0,
                y2: self.p3.1,
                color: self.color
            },
        ];
        for line in &lines {
            line.draw(painter);
        }
    }
}
