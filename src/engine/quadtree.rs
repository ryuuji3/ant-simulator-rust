use super::{Boundary, Point, Rectangle};

pub struct QuadTree {
    bounds: Rectangle,
    capacity: usize,
    points: Vec<Point>,

    quadrants: Option<Vec<QuadTree>>,
}

impl QuadTree {
    pub fn new(bounds: Rectangle, capacity: usize) -> QuadTree {
        QuadTree {
            bounds,
            capacity,
            points: vec![],
            quadrants: None,
        }
    }

    pub fn insert(&mut self, point: &Point)
    {
        if !self.bounds.contains(point) {
            return;
        }

        if self.points.len() < self.capacity {
            self.points.push(point.clone());
            return;
        }
        
        if self.quadrants.is_none() {
            self.create_quadrants()
        }

        match &mut self.quadrants {
            Some(quadrants) => 
                quadrants
                .iter_mut()
                .for_each(|quadrant| quadrant.insert(point)),
            _ => unreachable!("Quadrants not initialized!"),
        };
    }

    pub fn query(&self, bounds: Rectangle) -> Vec<Point> {
        let mut found = vec![];

        if !bounds.intersects(&self.bounds) {
            return found;
        } else {
            self.points
                .iter()
                .filter(|point| bounds.contains(point))
                .for_each(|&point| found.push(point));
        }

        match & self.quadrants {
            Some(quadrants) =>
                quadrants
                .iter()
                .map(|tree| tree.query(bounds))
                .for_each(|points| found.extend(points)),
            None => (),
        }

        return found;
    }

    fn create_quadrants(&mut self) {
        let width = self.bounds.width / 2.0;
        let height = self.bounds.height / 2.0;

        let bounds: Vec<Rectangle> = self.bounds
            .get_vertexes()
            .iter()
            .map(|&vertex| Rectangle::new(
                vertex.x, 
                vertex.y, 
                if vertex.x.is_sign_negative() { width } else { -width }, 
                if vertex.y.is_sign_negative() { height } else { -height}, 
                0.0)
            )
            .collect();

        let quadrants: Vec<QuadTree> = bounds
                .iter()
                .map(|&bounds| QuadTree::new(bounds, self.capacity))
                .collect();

        self.quadrants = Some(quadrants);
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::Point;

    use super::*;

    #[test]
    fn insert_items() {
        let bounds = Rectangle::new(0.0, 0.0, 200.0, 200.0, 0.0);
        let mut tree = QuadTree::new(bounds, 4);

        let inside_points = vec![
            Point { x: 0.0, y: 0.0, },
            Point { x: 25.3, y: 40.9 },
            Point { x: -54.2, y: -90.5 },
            Point { x: 100.0, y: 100.0 },
            Point { x: -100.0, y: -100.0 },
        ];

        let outside_points = vec![
            Point { x: 200.0, y: 200.0 },
        ];

        inside_points
            .iter()
            .for_each(|point| tree.insert(point));

        outside_points
            .iter()
            .for_each(|point| tree.insert(point));

        assert_eq!(tree.query(bounds), inside_points);
    }
}