use std::collections::HashMap;

use super::{ Rectangle, Shape, Point, Rotation };

#[derive(Debug)]
pub struct QuadTree<T> {
    bounds: Rectangle,
    capacity: usize,
    entities: HashMap<T, Point>,

    quadrants: Option<Vec<QuadTree<T>>>,
}

impl<T: std::hash::Hash + Eq> QuadTree<T> {
    pub fn new(bounds: Rectangle, capacity: usize) -> QuadTree<T> {
        QuadTree {
            bounds,
            capacity,
            entities: HashMap::new(),
            quadrants: None,
        }
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
                Rotation::new(0.0))
            )
            .collect();

        let quadrants = bounds
                .iter()
                .map(|&bounds| QuadTree::new(bounds, self.capacity))
                .collect();

        self.quadrants = Some(quadrants);
    }

    fn query(&self, bounds: &Rectangle) -> Vec<&T> {
        let mut found = vec![];

        if !bounds.intersects(&self.bounds) {
            return found;
        } else {
            self.entities
                .iter()
                .filter(|(_, position)| bounds.contains(&position))
                .for_each(|(entity, _)| found.push(entity));
        }

        match &self.quadrants {
            Some(quadrants) =>
                quadrants
                .iter()
                .map(|tree| tree.query(bounds))
                .for_each(|points| found.extend(points)),
            None => (),
        }

        return found;
    }

    fn insert(&mut self, item: T, position: Point) {
        if !self.bounds.contains(&position) {
            return;
        }

        if self.entities.len() < self.capacity {
            self.entities.insert(item, position);
            return;
        }
        
        if self.quadrants.is_none() {
            self.create_quadrants()
        }

        match &mut self.quadrants {
            Some(quadrants) => 
                quadrants
                .iter_mut()
                .find(|quadrant| quadrant.bounds.contains(&position))
                .map(|quadrant| quadrant.insert(item, position)),
            _ => unreachable!("Quadrants not initialized!"),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Hash, Eq, Copy)]
    struct TestItem;

    #[test]
    fn insert_items() {
        let bounds = Rectangle::new(0.0, 0.0, 200.0, 200.0, Rotation::new(0.0));
        let mut tree: QuadTree<TestItem> = QuadTree::new(bounds, 4);

        let inside_entites: Vec<(TestItem, Point)> = vec![
            Point { x: 0.0, y: 0.0, },
            Point { x: 25.3, y: 40.9 },
            Point { x: -54.2, y: -90.5 },
            Point { x: 100.0, y: 100.0 },
            Point { x: -100.0, y: -100.0 },
        ].iter().map(|&point| (TestItem, point)).collect();

        let outside_entities: Vec<(TestItem, Point)> = vec![
            Point { x: 200.0, y: 200.0 },
        ].iter().map(|&point| (TestItem, point)).collect();

        inside_entites
            .iter()
            .for_each(|(entity, position)| tree.insert(*entity, *position));

        outside_entities
            .iter()
            .for_each(|(entity, position)| tree.insert(*entity, *position));

        let mut expected_results: Vec<TestItem> = inside_entites
            .iter()
            .map(|(entity, _)| *entity)
            .collect();
        let mut actual_results: Vec<TestItem> = tree.query(&bounds)
            .iter()
            .map(|&&entity| entity)
            .collect(); // TODO: Sort by points

        assert_eq!(actual_results.sort(), expected_results.sort());
    }
}