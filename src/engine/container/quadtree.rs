use super::{ Container, Rectangle, Geometry, Entity };

#[derive(Debug)]
pub struct QuadTree {
    bounds: Rectangle,
    capacity: usize,
    entities: Vec<Entity>,

    quadrants: Option<Vec<QuadTree>>,
}

impl QuadTree {
    pub fn new(bounds: Rectangle, capacity: usize) -> QuadTree {
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

        let quadrants = bounds
                .iter()
                .map(|&bounds| QuadTree::new(bounds, self.capacity))
                .collect();

        self.quadrants = Some(quadrants);
    }
}

impl Container for QuadTree {
    fn query(&self, bounds: &Rectangle) -> Vec<&Entity> {
        let mut found = vec![];

        if !bounds.intersects(&self.bounds) {
            return found;
        } else {
            self.entities
                .iter()
                .filter(|&entity| bounds.contains(&entity.position))
                .for_each(|entity| found.push(entity));
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

    fn insert(&mut self, entity: Entity) {
        if !self.bounds.contains(&entity.position) {
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
                .find(|quadrant| quadrant.bounds.contains(&entity.position))
                .map(|quadrant| quadrant.insert(entity)),
            _ => unreachable!("Quadrants not initialized!"),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn insert_items() {
        let bounds = Rectangle::new(0.0, 0.0, 200.0, 200.0, 0.0);
        let mut tree: QuadTree = QuadTree::new(bounds, 4);

        let inside_entites: Vec<Entity> = vec![
            Point { x: 0.0, y: 0.0, },
            Point { x: 25.3, y: 40.9 },
            Point { x: -54.2, y: -90.5 },
            Point { x: 100.0, y: 100.0 },
            Point { x: -100.0, y: -100.0 },
        ].iter().map(|&point| Entity::new(point)).collect();

        let outside_entities: Vec<Entity> = vec![
            Point { x: 200.0, y: 200.0 },
        ].iter().map(|&point| Entity::new(point)).collect();

        inside_entites
            .iter()
            .for_each(|&entity| tree.insert(entity));

        outside_entities
            .iter()
            .for_each(|&entity| tree.insert(entity));

        let actual_results: Vec<Entity> = tree.query(&bounds).iter().map(|&&entity| entity).collect(); // TODO: Sort by points

        assert_eq!(actual_results, inside_entites);
    }
}