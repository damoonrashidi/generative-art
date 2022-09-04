use crate::circle::Circle;

pub struct PointMap<T> {
    width: f64,
    height: f64,
    points: Vec<Vec<Vec<T>>>,
    grid_resolution: usize,
}

#[derive(Debug, PartialEq)]
pub enum PointMapResult {
    Ok,
    Error(String),
}

impl PointMap<Circle> {
    pub fn new(width: f64, height: f64, resolution: usize) -> PointMap<Circle> {
        let mut map = vec![Vec::new(); resolution];

        for y in 0..resolution {
            map[y] = vec![Vec::new(); resolution];
            for x in 0..resolution {
                map[y][x] = vec![];
            }
        }

        PointMap {
            width,
            height,
            points: map,
            grid_resolution: resolution,
        }
    }

    pub fn insert(&mut self, circle: Circle) -> PointMapResult {
        let x = self.get_x(circle);
        let y = self.get_y(circle);

        if !(0..self.grid_resolution).contains(&x) || !(0..self.grid_resolution).contains(&y) {
            return PointMapResult::Error(format!("{}:{} is out of bounds", circle.x, circle.y));
        }

        if let Some(g_y) = self.points.get_mut(y) {
            if let Some(g_x) = g_y.get_mut(x) {
                g_x.push(circle);
                return PointMapResult::Ok;
            }
        }

        return PointMapResult::Error(String::from(format!(
            "Could not insert at {}:{}",
            circle.x, circle.y
        )));
    }

    /**
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
        let x = self.get_x(circle);
        let y = self.get_y(circle);

        if !self.index_is_valid(x) || !self.index_is_valid(y) {
            return None;
        }

        let mut points: Vec<Circle> = Vec::new();

        for a in y - 1..y + 1 {
            for b in x - 1..x + 1 {
                if self.index_is_valid(a) && self.index_is_valid(b) {
                    let mut points_in_grid = self.points.get(y).unwrap().get(x).unwrap().to_owned();
                    points.append(&mut points_in_grid);
                }
            }
        }
        return Some(points);
    }

    fn get_x(&self, circle: Circle) -> usize {
        return (circle.x / self.width * (self.grid_resolution as f64)).floor() as usize;
    }

    fn get_y(&self, circle: Circle) -> usize {
        return (circle.y / self.height * (self.grid_resolution as f64)).floor() as usize;
    }

    fn index_is_valid(&self, index: usize) -> bool {
        return index > 0 && index < self.grid_resolution;
    }
}

#[cfg(test)]
mod test {

    use crate::{
        circle::Circle,
        pointmap::{PointMap, PointMapResult},
    };

    #[test]
    fn get_x() {
        let point_map: PointMap<Circle> = PointMap::new(100.0, 100.0, 10);
        assert_eq!(point_map.get_x(Circle::new(9.0, 0.0, 0.0)), 0);
        assert_eq!(point_map.get_x(Circle::new(11.0, 0.0, 0.0)), 1);
        assert_eq!(point_map.get_x(Circle::new(20.0, 0.0, 0.0)), 2);
        assert_eq!(point_map.get_x(Circle::new(34.0, 0.0, 0.0)), 3);
        assert_eq!(point_map.get_x(Circle::new(99.999, 0.0, 0.0)), 9);
    }

    #[test]
    fn get_y() {
        let point_map: PointMap<Circle> = PointMap::new(100.0, 100.0, 10);
        assert_eq!(point_map.get_y(Circle::new(11.0, 5.5, 0.0)), 0);
        assert_eq!(point_map.get_y(Circle::new(20.0, 15.0, 0.0)), 1);
        assert_eq!(point_map.get_y(Circle::new(34.0, 73.0, 0.0)), 7);
        assert_eq!(point_map.get_y(Circle::new(99.999, 90.0, 0.0)), 9);
    }

    #[test]
    fn insert_point() {
        let mut point_map: PointMap<Circle> = PointMap::new(100.0, 100.0, 10);
        let circle = Circle::new(11.0, 11.0, 10.0);
        let result = point_map.insert(circle);
        let points = point_map.points.get_mut(1).unwrap().get_mut(1).unwrap();
        assert_eq!(points.len(), 1);
        assert_eq!(result, PointMapResult::Ok);
    }

    #[test]
    fn insert_point_fail() {
        let mut point_map: PointMap<Circle> = PointMap::new(100.0, 100.0, 10);
        let circle = Circle::new(1000.0, 100.0, 10.0);
        let result = point_map.insert(circle);
        assert_eq!(
            result,
            PointMapResult::Error(String::from("1000:100 is out of bounds"))
        );
    }

    #[test]
    fn get_neighbors() {
        let mut point_map: PointMap<Circle> = PointMap::new(100.0, 100.0, 10);
        let circle = Circle::new(11.0, 11.0, 5.0);
        let non_neighbor = Circle::new(30.3, 50.4, 10.0);
        point_map.insert(circle);
        point_map.insert(non_neighbor);

        let neighbors = point_map.get_neighbors(circle).unwrap();

        assert_eq!(neighbors.len(), 1);
        assert_eq!(neighbors.first().unwrap().to_owned(), circle);
    }

    #[test]
    fn get_neighbors_edgecase() {
        let mut point_map: PointMap<Circle> = PointMap::new(100.0, 100.0, 10);
        let circle = Circle::new(99.0, 11.0, 5.0);
        let non_neighbor = Circle::new(101.1, 50.4, 10.0);
        point_map.insert(circle);
        point_map.insert(non_neighbor);

        let neighbors = point_map.get_neighbors(circle).unwrap();

        assert_eq!(neighbors.len(), 2);
        assert_eq!(neighbors.first().unwrap().to_owned(), circle);
    }
}
