use super::shape::Shape;
use super::three_d_shape::Shape as Three_D_Shape;

pub struct Square{
    pub length: f32
}

impl Shape for Square {
    fn area(self) -> f32{
        self.length*self.length
    }

    fn perimeter(self) -> f32{
        4_f32*self.length
    }
}

impl Three_D_Shape for Square {
    fn volume(self) -> f32 {
        self.length * self.length * self.length
    }
}