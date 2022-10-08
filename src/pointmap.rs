use std::vec;

use crate::{circle::Circle, rectangle::Rectangle, Shape};

pub struct PointMap<'a, T> {
    bounds: &'a Rectangle,
    points: Vec<Vec<T>>,
    grid_resolution: usize,
}

impl<'a> PointMap<'a, Circle> {
    pub fn new(bounds: &Rectangle, resolution: usize) -> PointMap<Circle> {
        let mut map = vec![Vec::new(); resolution.pow(2)];

        for i in 0..map.len() {
            map[i] = vec![];
        }

        return PointMap {
            bounds: &bounds,
            points: map,
            grid_resolution: resolution,
        };
    }

    pub fn insert(&mut self, circle: Circle) -> Result<usize, Circle> {
        let i = self.get_index(circle);

        if let Some(points) = self.points.get_mut(i) {
            points.push(circle);
            return Ok(i);
        }

        return Err(circle);
    }

    /**
    * An easier way is using a vector of vectors of Circles.
    * We can easily see if a point is out of bounds by just doing (contains)
    * To get the correct box we do div count % remainder
    *
    *  -------------------------
    *  | 0 | 1 | 2 | 3 | 4 | 5 |
    *  | 6 | 7 | 8 | 9 | . | . |
    *  | . |   |   |   |   |   |
    *  -------------------------

    * So something that is x = 80% and y = 80% in the case above
    * would yield
    * The idea here is that we get all the points for this cell
    * and all the surrounding cells to avoid collisions at nodes
    * close to the one where we pop over to a neigboring grid cell.
    *
    * this makes the search space larger, but yields a more accurate
    * result.
    *
    *  ----------------------
    *  |  |  |  |  |  |  |  |
    *  |xx|xx|xx|  |  |  |  |
    *  |xx|oo|xx|  |  |  |  |
    *  |xx|xx|xx|  |  |  |  |
    *  ----------------------
    */
    pub fn get_neighbors(&self, circle: Circle) -> Option<Vec<Circle>> {
        if !self.bounds.contains(circle.center()) {
            return None;
        }

        let i = self.get_index(circle);

        return Some(self.points.get(i).unwrap().to_vec());
    }

    fn get_index(&self, circle: Circle) -> usize {
        let x = ((circle.center().x / (self.bounds.x + self.bounds.width))
            * self.grid_resolution as f64)
            .floor();

        let y = ((circle.center().y / (self.bounds.y + self.bounds.height))
            * self.grid_resolution as f64)
            .floor();

        return (y * (self.grid_resolution as f64) + x - 1.0) as usize;
    }
}

#[cfg(test)]
mod test {

    use crate::{circle::Circle, pointmap::PointMap, rectangle::Rectangle};

    #[test]
    fn get_index() {
        let bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            color: Rectangle::default().color,
        };
        let point_map: PointMap<Circle> = PointMap::new(&bounds, 10);
        assert_eq!(point_map.get_index(Circle::new(9.0, 0.0, 0.0)), 0);
        assert_eq!(point_map.get_index(Circle::new(11.0, 0.0, 0.0)), 1);
        assert_eq!(point_map.get_index(Circle::new(20.0, 0.0, 0.0)), 2);
        assert_eq!(point_map.get_index(Circle::new(34.0, 0.0, 0.0)), 3);
        assert_eq!(point_map.get_index(Circle::new(99.999, 0.0, 0.0)), 9);
    }

    #[test]
    fn insert_point() {
        let bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            color: Rectangle::default().color,
        };
        let mut point_map: PointMap<Circle> = PointMap::new(&bounds, 10);
        let circle = Circle::new(11.0, 11.0, 10.0);
        let result = point_map.insert(circle);
        let points = point_map.points.get_mut(1).unwrap();
        assert_eq!(points.len(), 1);
        assert_eq!(result, Ok(1));
    }

    #[test]
    fn insert_point_fail() {
        let bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            color: Rectangle::default().color,
        };
        let mut point_map: PointMap<Circle> = PointMap::new(&bounds, 10);
        let circle = Circle::new(1000.0, 100.0, 10.0);
        let result = point_map.insert(circle);
        assert_eq!(result, Err(circle));
    }

    #[test]
    fn get_neighbors() {
        let bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            color: Rectangle::default().color,
        };
        let mut point_map: PointMap<Circle> = PointMap::new(&bounds, 10);
        let circle = Circle::new(11.0, 11.0, 5.0);
        let non_neighbor = Circle::new(30.3, 50.4, 10.0);

        let _ = point_map.insert(circle);
        let __ = point_map.insert(non_neighbor);

        let neighbors = point_map.get_neighbors(circle).unwrap();

        assert_eq!(neighbors.len(), 1);
        assert_eq!(neighbors.first().unwrap().to_owned(), circle);
    }

    #[test]
    fn get_neighbors_edgecase() {
        let bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            color: Rectangle::default().color,
        };
        let mut point_map: PointMap<Circle> = PointMap::new(&bounds, 10);
        let circle = Circle::new(99.0, 11.0, 5.0);
        let non_neighbor = Circle::new(101.1, 50.4, 10.0);

        let _ = point_map.insert(circle);
        let __ = point_map.insert(non_neighbor);

        let neighbors = point_map.get_neighbors(circle).unwrap();

        assert_eq!(neighbors.len(), 2);
        assert_eq!(neighbors.first().unwrap().to_owned(), circle);
    }
}
