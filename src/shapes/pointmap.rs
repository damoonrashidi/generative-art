use std::vec;

use super::{point::Point, rectangle::Rectangle, shape::Shape};

/**
A PointMap is a grid subdivision of a Rectangle where each cell holds
zero or more shapes. It speeds up the search for close proximity shapes (neighbors) by
limiting the search to only look in the same cell, or same cell as well as adjacent cells.
*/
#[derive(Debug, Clone)]
pub struct PointMap<'a, T: Shape> {
    bounds: &'a Rectangle,
    cells: Vec<Vec<T>>,
    grid_resolution: usize,
}

impl<'a, T: Shape + Clone + PartialEq> PointMap<'a, T> {
    /**
    Create a new map for a given bounds with a list of cells to aid in operations
    like finding the nearest neighbors or doing collision detections.

    Example:
    ```
    use generative_art::shapes::{pointmap::PointMap, point::Point, circle::Circle, rectangle::Rectangle};

    let bounds = Rectangle::new(Point{x: 0.0, y: 0.}, 500.0, 500.0);
    let mut point_map: PointMap<'_, Circle> = PointMap::new(&bounds, 20);
    let first = Circle::new(Point{x: 200., y: 200.0}, 10.);
    let second = Circle::new(Point{x: 205., y: 205.0}, 10.);
    point_map.insert(first);
    point_map.insert(second);
    point_map.get_neighbors(&first, Some(100.)); // => vec[first];
    ```
    */
    pub fn new(bounds: &'a Rectangle, resolution: usize) -> PointMap<'a, T> {
        PointMap {
            bounds,
            cells: vec![Vec::with_capacity(resolution); resolution.pow(2)],
            grid_resolution: resolution,
        }
    }

    /// Insert a new shape into the grid
    pub fn insert(&mut self, shape: T) -> Result<usize, T> {
        let center = shape.center();
        let i = self.get_index(&center);

        match self.cells.get_mut(i) {
            Some(points) => {
                points.push(shape);
                Ok(i)
            }
            None => Err(shape),
        }
    }

    /// Remove a shape from the grid
    pub fn remove(&mut self, shape: T) {
        let center = shape.center();
        let i = self.get_index(&center);

        if let Some(cell) = self.cells.get_mut(i) {
            let result = cell.iter().enumerate().find(|(_, s)| **s == shape);

            if let Some((index, _)) = result {
                cell.remove(index);
            }
        }
    }

    /// Get all items from the grid
    pub fn get_items(&self) -> Vec<&T> {
        self.cells.iter().flatten().collect::<Vec<&T>>()
    }

    /**
    Get all neighboring items that are close to the given shape. An optional distance
    parameter can be supplied to limit the results to all items to be at most the given
    distance away.
    */
    pub fn get_neighbors(&self, shape: &T, distance: Option<f64>) -> Result<Vec<T>, &str> {
        /*
         * The general idea here is to do some simple math to be able
         * to overlay the bounds with cell after cell until we are
         * out of bounds, when the bounds have been hit, we loop back
         * to a new row using modulo and restart the overlaying.
         *
         *  -------------------------
         *  | 0 | 1 | 2 | 3 | 4 | 5 |
         *  | 6 | 7 | 8 | 9 | 10| . |
         *  | . |   |   |   |   |   |
         *  -------------------------
         *
         * This allows us to use <Vec<Vec<T>>> instead of Vec<Vec<Vec<T>>>
         * meaning we don't have to think of the bounds as a grid,
         * but rather a list of cells.
         *
         * We also get all the surrounding cells to avoid collisions at nodes
         * close to the one where we pop over to a neighboring grid cell.
         *
         * this makes the search space larger, but yields a more accurate
         * result, so no shapes overlap at the edges of cells.
         *
         *  ----------------------
         *  |  |  |  |  |  |  |  |
         *  |xx|xx|xx|  |  |  |  |
         *  |xx|oo|xx|  |  |  |  |
         *  |xx|xx|xx|  |  |  |  |
         *  ----------------------
         */

        let center = shape.center();
        if !self.bounds.contains(&center) {
            return Err("out of bounds call for this pointmap");
        }
        let i = self.get_index(&center);

        let items = self
            .get_neighboring_cells(i)
            .iter()
            .fold(vec![], |mut list, index| match self.cells.get(*index) {
                Some(cell_items) => {
                    cell_items
                        .iter()
                        .filter(|item| {
                            if let Some(distance) = distance {
                                return shape.center().distance_to(&item.center()) < distance;
                            }
                            true
                        })
                        .for_each(|item| list.push(item.to_owned()));
                    list
                }
                None => list,
            });

        Ok(items)
    }

    /// Find out what cell a given point is located in.
    fn get_index(&self, point: &Point) -> usize {
        let resolution = self.grid_resolution as f64;

        let x = ((point.x / (self.bounds.position.x + self.bounds.width)) * resolution).floor();
        let y = ((point.y / (self.bounds.position.y + self.bounds.height)) * resolution).floor();

        (y * resolution + x - 1.0) as usize
    }

    /// Get all the surrounding cells for a given cell index. This is useful when considering
    /// that shapes can spill over the edges of a cell, i.e, if a large circle is close the edge
    /// of a cell, the origo might be inside but other points of the circle might actually be
    /// in neighboring cells.
    fn get_neighboring_cells(&self, index: usize) -> Vec<usize> {
        let i = index as i32;
        let step = (self.cells.len() as f64).sqrt() as i32;
        let over = i - step;
        let under = i + step;

        vec![
            over - 1,
            over,
            over + 1,
            i - 1,
            i,
            i + 1,
            under - 1,
            under,
            under + 1,
        ]
        .into_iter()
        .filter(|cell| *cell > 0 || (*cell as usize) < self.cells.len())
        .map(|cell| cell as usize)
        .collect::<Vec<usize>>()
    }
}

#[cfg(test)]
mod test {
    use crate::shapes::{circle::Circle, point::Point, pointmap::PointMap, rectangle::Rectangle};

    #[test]
    fn get_index() {
        let bounds = Rectangle {
            position: Point { x: 0.0, y: 0.0 },
            width: 100.0,
            height: 100.0,
            color: Rectangle::default().color,
        };
        let point_map: PointMap<'_, Circle> = PointMap::new(&bounds, 10);
        assert_eq!(point_map.get_index(&Point { x: 9.0, y: 0.0 }), 0);
        assert_eq!(point_map.get_index(&Point { x: 11.0, y: 0.0 }), 0);
        assert_eq!(point_map.get_index(&Point { x: 20.0, y: 0.0 }), 1);
        assert_eq!(point_map.get_index(&Point { x: 34.0, y: 0.0 }), 2);
        assert_eq!(point_map.get_index(&Point { x: 99.999, y: 0.0 }), 8);
    }

    #[test]
    fn insert_point() {
        let bounds = Rectangle {
            position: Point { x: 0.0, y: 0.0 },
            width: 100.0,
            height: 100.0,
            color: Rectangle::default().color,
        };
        let mut point_map: PointMap<'_, Circle> = PointMap::new(&bounds, 10);
        let circle = Circle::new(Point { x: 11.0, y: 11.0 }, 10.0);
        if let Ok(result) = point_map.insert(circle) {
            assert_eq!(result, 10);
        }
    }

    #[test]
    fn insert_point_fail() {
        let bounds = Rectangle {
            position: Point { x: 0.0, y: 0.0 },
            width: 100.0,
            height: 100.0,
            color: Rectangle::default().color,
        };
        let mut point_map: PointMap<'_, Circle> = PointMap::new(&bounds, 10);
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
            position: Point { x: 0.0, y: 0.0 },
            width: 100.0,
            height: 100.0,
            color: Rectangle::default().color,
        };
        let mut point_map: PointMap<'_, Circle> = PointMap::new(&bounds, 10);
        let circle = Circle::new(Point { x: 11.0, y: 11.0 }, 5.0);
        let non_neighbor = Circle::new(Point { x: 30.3, y: 50.4 }, 10.0);

        let _insertion = point_map.insert(circle);
        let _neighbor_insertion = point_map.insert(non_neighbor);

        if let Ok(neighbors) = point_map.get_neighbors(&circle, None) {
            assert_eq!(neighbors.len(), 1);
            assert_eq!(neighbors.first().unwrap(), &circle);
        }
    }

    #[test]
    fn get_neighbors_edgecase() {
        let bounds = Rectangle {
            position: Point { x: 0.0, y: 0.0 },
            width: 100.0,
            height: 100.0,
            color: Rectangle::default().color,
        };
        let mut point_map: PointMap<'_, Circle> = PointMap::new(&bounds, 10);
        let circle = Circle::new(Point { x: 99.0, y: 11.0 }, 5.0);
        let non_neighbor = Circle::new(Point { x: 101.1, y: 50.4 }, 10.0);

        let _insertion = point_map.insert(circle);
        let _neighbor_insertion = point_map.insert(non_neighbor);

        let neighbors = point_map.get_neighbors(&circle, None).unwrap();

        assert_eq!(neighbors.len(), 1);
        assert_eq!(neighbors.first().unwrap(), &circle);
    }

    #[test]
    fn get_all_items() {
        let bounds = Rectangle {
            position: Point { x: 0.0, y: 0.0 },
            width: 100.0,
            height: 100.0,
            color: Rectangle::default().color,
        };

        let mut point_map: PointMap<'_, Point> = PointMap::new(&bounds, 10);
        let point = Point { x: 0., y: 0. };
        let _ = point_map.insert(point);
        let points = point_map.get_items();

        assert_eq!(points, vec![&Point { x: 0., y: 0. }]);
    }

    #[test]
    fn get_surrounding_cells() {
        let bounds = Rectangle {
            position: Point { x: 0.0, y: 0.0 },
            width: 100.0,
            height: 100.0,
            color: Rectangle::default().color,
        };

        let map: PointMap<'_, Point> = PointMap::new(&bounds, 10);

        let indicies = map.get_neighboring_cells(25);
        println!("{}", map.cells.len());

        assert_eq!(indicies, vec![14, 15, 16, 24, 25, 26, 34, 35, 36]);
    }
}
