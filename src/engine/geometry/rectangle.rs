use super::{ Point };

// Here's an example of a rectangle with rotation 0 
//
//    top-left────────────────top-right
//           |                |
//           |                |    \
//           |     origin     |     | rotation
//           |                |    \/
//           |                |
//  bottom-left────────────────bottom-right
// 
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Rectangle {
    // center of rectangle
    pub origin: Point,

    pub width: f32,
    pub height: f32,

    pub rotation: f32,

    pub top_right: Point,
    pub bottom_right: Point,
    pub bottom_left: Point,
    pub top_left: Point,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32, rotation: f32) -> Rectangle {
        Rectangle {
            origin: Point::create_point(x, y),
            width,
            height,
            rotation,

            top_right: Point::create_point(x + width / 2.0, y - height / 2.0),
            bottom_right: Point::create_point(x + width / 2.0, y + height / 2.0),
            bottom_left: Point::create_point(x - width / 2.0, y + height / 2.0),
            top_left: Point::create_point(x - width / 2.0, y - height / 2.0),
        }
    }

    pub fn get_x(&self) -> f32 {
        self.origin.x
    }

    pub fn get_y(&self) -> f32 {
        self.origin.y
    }

    pub fn get_vertexes(&self) -> Vec<&Point> {
        Vec::from([
            &self.top_right,
            &self.bottom_right,
            &self.bottom_left,
            &self.top_left,
        ])
    }

    pub fn rotate(&self, rotation: f32) -> Rectangle {
        let new_rotation = self.rotation + rotation;

        Rectangle {
            origin: self.origin,
            width: self.width,
            height: self.height,
            rotation: new_rotation,

            top_right: self.top_right.rotate_around_origin(new_rotation),
            bottom_right: self.bottom_right.rotate_around_origin(new_rotation),
            bottom_left: self.bottom_left.rotate_around_origin(new_rotation),
            top_left: self.top_left.rotate_around_origin(new_rotation),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Shape;

    #[test]
    fn rect_contains_point() {
        let basic_rectangle = Shape::Rectangle(Rectangle::new(0.0,0.0,100.0,100.0,0.0));

        // inside
        assert!(basic_rectangle.contains(&Point::create_point(0.0, 0.0))); 
        assert!(basic_rectangle.contains(&Point::create_point(20.7, 30.4)));
        assert!(basic_rectangle.contains(&Point::create_point(-25.2, 40.9)));

        // edges
        assert!(basic_rectangle.contains(&Point::create_point(50.0, 0.0))); // right center
        assert!(basic_rectangle.contains(&Point::create_point(50.0, 50.0))); // right bottom
        assert!(basic_rectangle.contains(&Point::create_point(0.0, 50.0))); // right center
        assert!(basic_rectangle.contains(&Point::create_point(-50.0, -50.0))); // bottom left
        assert!(basic_rectangle.contains(&Point::create_point(-50.0, 0.0))); // bottom center
        assert!(basic_rectangle.contains(&Point::create_point(-50.0, 50.0))); // top left
        assert!(basic_rectangle.contains(&Point::create_point(0.0, 50.0))); // top center
        assert!(basic_rectangle.contains(&Point::create_point(50.0, -50.0))); // top right

        // outside
        assert!(!basic_rectangle.contains(&Point::create_point(100.0, 50.0))); 
        assert!(!basic_rectangle.contains(&Point::create_point(-100.0, 50.0))); 
        assert!(!basic_rectangle.contains(&Point::create_point(50.0, -100.0))); 
        assert!(!basic_rectangle.contains(&Point::create_point(50.0, 100.0))); 
    }

    #[test]
    fn rotated_rect_contains_point() {
        // technically rotated, but because the rectangle is a square should be virtually the same as if it was 0 degrees
        let rotated_rectangle = Shape::Rectangle(Rectangle::new(0.0,0.0,100.0,100.0,45.0));

        // inside
        assert!(rotated_rectangle.contains(&Point::create_point(0.0, 0.0))); 
        assert!(rotated_rectangle.contains(&Point::create_point(20.7, 30.4)));
        assert!(rotated_rectangle.contains(&Point::create_point(-25.2, 40.9)));

        // edges
        assert!(rotated_rectangle.contains(&Point::create_point(50.0, 0.0))); // right center
        assert!(rotated_rectangle.contains(&Point::create_point(25.0, 25.0))); // bottom-right center
        assert!(rotated_rectangle.contains(&Point::create_point(0.0, 50.0))); // right center
        assert!(rotated_rectangle.contains(&Point::create_point(-25.0, -25.0))); // bottom left
        assert!(rotated_rectangle.contains(&Point::create_point(-50.0, 0.0))); // bottom center
        assert!(rotated_rectangle.contains(&Point::create_point(-25.0, 25.0))); // top left
        assert!(rotated_rectangle.contains(&Point::create_point(0.0, 50.0))); // top center
        assert!(rotated_rectangle.contains(&Point::create_point(25.0, -25.0))); // top right

        // outside
        assert!(!rotated_rectangle.contains(&Point::create_point(100.0, 50.0))); 
        assert!(!rotated_rectangle.contains(&Point::create_point(-100.0, 50.0))); 
        assert!(!rotated_rectangle.contains(&Point::create_point(50.0, -100.0))); 
        assert!(!rotated_rectangle.contains(&Point::create_point(50.0, 100.0))); 
    }

    #[test]
    pub fn rect_intersects() {
        let main_rectangle = Shape::Rectangle(Rectangle::new(0.0, 0.0, 100.0, 100.0, 0.0));
        
        let overlapping_rectangle = Shape::Rectangle(Rectangle::new(25.0, 25.0, 100.0, 100.0, 0.0));
        let edge_overlapping_rectangle = Shape::Rectangle(Rectangle::new(100.0, 100.0, 100.0, 100.0, 0.0));
        let not_overlapping_rectangle = Shape::Rectangle(Rectangle::new(101.0, 101.0, 100.0, 100.0, 0.0));

        assert!(main_rectangle.intersects(&overlapping_rectangle));
        assert!(edge_overlapping_rectangle.intersects(&overlapping_rectangle));

        assert!(!main_rectangle.intersects(&not_overlapping_rectangle));
    }
}