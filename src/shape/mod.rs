use crate::canvas::Canvas;

trait Shape {
    fn draw(&self, canvas: &mut Canvas);

    fn draw_filled(&self, canvas: &mut Canvas);
}

struct Line2D {

}

impl Shape for Line2D {

    fn draw(&self, _: &mut Canvas) {
    }
    
    fn draw_filled(&self, c: &mut Canvas) {
        self.draw(c)
    }
}
