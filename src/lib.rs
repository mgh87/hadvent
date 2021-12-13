use tui::{
    style::Color,
    widgets::canvas::{Line, Painter, Shape},
};

/// Shape to draw a rectangle from a `Rect` with the given color
#[derive(Debug, Clone)]
pub struct GeneralSquare {
    pub p1: (f64,f64),
    pub p2: (f64,f64),
    pub p3: (f64,f64),
    pub p4: (f64,f64),
    pub color: Color,
}

impl Shape for GeneralSquare {
    fn draw(&self, painter: &mut Painter) {
        let lines: [Line; 4] = [
            Line {
                x1: self.p1.0,
                y1: self.p1.1,
                x2: self.p2.0,
                y2: self.p2.1,
                color: self.color,
            },
            Line {
                x1: self.p2.0,
                y1: self.p2.1,
                x2: self.p3.0,
                y2: self.p3.1,
                color: self.color,
            },
            Line {
                x1: self.p3.0,
                y1: self.p3.1,
                x2: self.p4.0,
                y2: self.p4.1,
                color: self.color
            },
            Line {
                x1: self.p4.0,
                y1: self.p4.1,
                x2: self.p1.0,
                y2: self.p1.1,
                color: self.color
            }
        ];
        for line in &lines {
            line.draw(painter);
        }
    }
}
