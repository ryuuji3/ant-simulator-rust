use super::{ Point, Rectangle };

pub enum Shape {
    Rectangle(Rectangle),
}

impl Shape {
    pub fn contains(&self, point: &Point) -> bool {
        use Shape::*;

        match *self {
            Rectangle(ref rectangle) => {
                // factor out the rotation by applying same rotation in reverse to point 
                let rotated_point = point.rotate_around_origin(-rectangle.rotation);

                // can now apply normal logic to rotated point
                rectangle.top_left.x <= rotated_point.x && rotated_point.x <= rectangle.top_right.x
                && rectangle.top_left.y <= rotated_point.y && rotated_point.y <= rectangle.bottom_left.y
            },
            _ => unimplemented!("shape.contains not implemented for shape")
        }
    }

    pub fn intersects(&self, range: &Shape) -> bool {
        use Shape::*;

        match *self {
            Rectangle(ref rectangle) => {
                match range {
                    Rectangle(range) => {
                        let rotated_range = range.rotate(-rectangle.rotation);

                        rotated_range
                            .get_vertexes()
                            .iter()
                            .map(|corner| self.contains(corner))
                            .any(|has_intersection| has_intersection == true)
                    },
                    _ => unimplemented!("rectangle doesn't implement intersects for shape"),
                }
            },
            _ => unimplemented!("shape.intersects not implemented for shape")
        }
    }
}