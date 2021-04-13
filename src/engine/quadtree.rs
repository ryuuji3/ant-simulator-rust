use super::{ Container, Entity, Rectangle, Shape};

#[derive(Debug)]
pub struct QuadTree<T> {
    bounds: Rectangle,
    capacity: usize,
    entities: Vec<Box<dyn Entity<T>>>,

    quadrants: Option<Vec<QuadTree<T>>>,
}

impl<T> QuadTree<T> {
    pub fn new(bounds: Rectangle, capacity: usize) -> QuadTree<T> {
        QuadTree {
            bounds,
            capacity,
            entities: vec![],
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
                0.0)
            )
            .collect();

        let quadrants: Vec<QuadTree<T>> = bounds
                .iter()
                .map(|&bounds| QuadTree::new(bounds, self.capacity))
                .collect();

        self.quadrants = Some(quadrants);
    }
}

impl<T> Container<T> for QuadTree<T> {
    fn query(&self, bounds: &Shape) -> Vec<&Box<dyn Entity<T>>> {
        let mut found = vec![];

        if !bounds.intersects(&Shape::Rectangle(self.bounds)) {
            return found;
        } else {
            self.entities
                .iter()
                .filter(|entity| bounds.contains(&entity.get_position()))
                .for_each(|entity| found.push(entity));
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

    fn insert(&mut self, entity: Box<dyn Entity<T>>) {
        if !Shape::Rectangle(self.bounds).contains(&entity.get_position()) {
            return;
        }

        if self.entities.len() < self.capacity {
            self.entities.push(entity);
            return;
        }
        
        if self.quadrants.is_none() {
            self.create_quadrants()
        }

        match &mut self.quadrants {
            Some(quadrants) => 
                quadrants
                .iter_mut()
                .find(|quadrant| Shape::Rectangle(quadrant.bounds).contains(&entity.get_position()))
                .map(|quadrant| quadrant.insert(entity)),
            _ => unreachable!("Quadrants not initialized!"),
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::Point;

    use super::*;

    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    struct TestEntity {
        position: Point,
    }

    impl Entity<TestEntity> for TestEntity {
        fn get_position(&self) -> Point {
            self.position
        }

        fn tick(&mut self, tree: &dyn Container<TestEntity>) {
            todo!()
        }
    } 

    #[test]
    fn insert_items() {
        let bounds = Rectangle::new(0.0, 0.0, 200.0, 200.0, 0.0);
        let mut tree: QuadTree<TestEntity> = QuadTree::new(bounds, 4);

        let inside_entites: Vec<TestEntity> = vec![
            Point { x: 0.0, y: 0.0, },
            Point { x: 25.3, y: 40.9 },
            Point { x: -54.2, y: -90.5 },
            Point { x: 100.0, y: 100.0 },
            Point { x: -100.0, y: -100.0 },
        ].iter().map(|&position| TestEntity { position }).collect();

        let outside_entities: Vec<TestEntity> = vec![
            Point { x: 200.0, y: 200.0 },
        ].iter().map(|&position| TestEntity { position }).collect();

        inside_entites
            .iter()
            .for_each(|entity| tree.insert(Box::new(entity.clone())));

        outside_entities
            .iter()
            .for_each(|entity| tree.insert(Box::new(entity.clone())));

        let result_entity_references = tree.query(&Shape::Rectangle(bounds));
        // TODO: Sort by points

        let expected_results: Vec<Point> = inside_entites.iter().map(|entity| entity.get_position()).collect();
        let actual_results: Vec<Point> = result_entity_references.iter().map(|entity| entity.get_position()).collect();

        assert_eq!(actual_results, expected_results);
    }
}