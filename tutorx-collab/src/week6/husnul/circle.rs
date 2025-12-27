use super::shape::Shape;

pub struct Circle{
    pub radius: f32
}


impl Shape for Circle {
    fn area(self) -> f32{
        3.14*self.radius*self.radius
    }

    fn perimeter(self) -> f32{
        3.14*2_f32*self.radius
    }
}