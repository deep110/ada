use crate::canvas::Canvas;
use crate::Color;

pub trait Shape {
    fn draw(&self, canvas: &mut Canvas, color: Color);

    fn draw_filled(&self, canvas: &mut Canvas, color: Color);
}

struct Line2D {

}

impl Shape for Line2D {

    fn draw(&self, _: &mut Canvas, color: Color) {
        
    }
    
    fn draw_filled(&self, canvas: &mut Canvas, color: Color) {
        self.draw(canvas, color)
    }
}
