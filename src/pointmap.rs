use std::vec;

use crate::{point::Point, rectangle::Rectangle, shape::Shape};

pub struct PointMap<'a, T> {
    bounds: &'a Rectangle,
    items: Vec<Vec<T>>,
    grid_resolution: usize,
}

impl<'a, T: Shape + Clone> PointMap<'a, T> {
    pub fn new<S>(bounds: &Rectangle, resolution: usize) -> PointMap<T> {
        let mut map = vec![Vec::new(); resolution.pow(2)];

        for i in 0..map.len() {
            map[i] = vec![];
        }

        return PointMap {
            bounds: &bounds,
            items: map,
            grid_resolution: resolution,
        };
    }

    pub fn insert(&mut self, shape: T) -> Result<usize, T> {
        let i = self.get_index(shape.center());

        if let Some(points) = self.items.get_mut(i) {
            points.push(shape);
            return Ok(i);
        }

        return Err(shape);
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
    pub fn get_neighbors(&self, shape: T) -> Result<Vec<T>, String> {
        if !self.bounds.contains(shape.center()) {
            return Err(format!(
                "{} {} is out of bounds for this pointmap",
                shape.center().x,
                shape.center().y
            ));
        }

        let i = self.get_index(shape.center());

        if let Some(list) = self.items.get(i) {
            Ok(list.to_vec())
        } else {
            Err(format!("{} is out of bounds", i))
        }
    }

    fn get_index(&self, shape: Point) -> usize {
        let x =
            ((shape.x / (self.bounds.x + self.bounds.width)) * self.grid_resolution as f64).floor();

        let y = ((shape.y / (self.bounds.y + self.bounds.height)) * self.grid_resolution as f64)
            .floor();

        return (y * (self.grid_resolution as f64) + x - 1.0) as usize;
    }
}

#[cfg(test)]
mod test {

    use crate::{circle::Circle, point::Point, pointmap::PointMap, rectangle::Rectangle};

    #[test]
    fn get_index() {
        let bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            color: Rectangle::default().color,
        };
        let point_map: PointMap<Circle> = PointMap::new::<Circle>(&bounds, 10);
        assert_eq!(point_map.get_index(Point { x: 9.0, y: 0.0 }), 0);
        assert_eq!(point_map.get_index(Point { x: 11.0, y: 0.0 }), 0);
        assert_eq!(point_map.get_index(Point { x: 20.0, y: 0.0 }), 1);
        assert_eq!(point_map.get_index(Point { x: 34.0, y: 0.0 }), 2);
        assert_eq!(point_map.get_index(Point { x: 99.999, y: 0.0 }), 8);
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
        let mut point_map: PointMap<Circle> = PointMap::new::<Circle>(&bounds, 10);
        let circle = Circle::new(Point { x: 11.0, y: 11.0 }, 10.0);
        let result = point_map.insert(circle);
        if let Some(points) = point_map.items.get_mut(1) {
            assert_eq!(points.len(), 0);
            assert_eq!(result, Ok(10));
        }
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
        let mut point_map: PointMap<Circle> = PointMap::new::<Circle>(&bounds, 10);
        let circle = Circle::new(
            Point {
                x: 1000.0,
                y: 100.0,
            },
            10.0,
        );
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
        let mut point_map: PointMap<Circle> = PointMap::new::<Circle>(&bounds, 10);
        let circle = Circle::new(Point { x: 11.0, y: 11.0 }, 5.0);
        let non_neighbor = Circle::new(Point { x: 30.3, y: 50.4 }, 10.0);

        let _ = point_map.insert(circle);
        let __ = point_map.insert(non_neighbor);

        if let Ok(neighbors) = point_map.get_neighbors(circle) {
            assert_eq!(neighbors.len(), 1);
            assert_eq!(neighbors.first().unwrap().to_owned(), circle);
        }
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
        let mut point_map: PointMap<Circle> = PointMap::new::<Circle>(&bounds, 10);
        let circle = Circle::new(Point { x: 99.0, y: 11.0 }, 5.0);
        let non_neighbor = Circle::new(Point { x: 101.1, y: 50.4 }, 10.0);

        let _ = point_map.insert(circle);
        let __ = point_map.insert(non_neighbor);

        let neighbors = point_map.get_neighbors(circle).unwrap();

        assert_eq!(neighbors.len(), 1);
        assert_eq!(neighbors.first().unwrap().to_owned(), circle);
    }
}
