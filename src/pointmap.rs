use crate::circle::Circle;

pub struct PointMap<T> {
    width: f64,
    height: f64,
    pub points: Vec<Vec<Vec<T>>>,
}

impl PointMap<Circle> {
    pub fn new(width: f64, height: f64) -> PointMap<Circle> {
        let mut map: Vec<Vec<Vec<Circle>>> = vec![Vec::new(); 10];

        for y in 0..9 {
            map[y] = vec![Vec::new(); 10];
            for x in 0..9 {
                map[y][x] = vec![];
            }
        }

        PointMap {
            width,
            height,
            points: map,
        }
    }

    pub fn insert(&mut self, circle: Circle) -> () {
        let x = self.get_x(circle);
        let y = self.get_y(circle);

        if !(0..=9).contains(&x) || !(0..=9).contains(&y) {
            panic!("{}:{} is out of bounds ({})", x, y, circle);
        }

        if let Some(g_y) = self.points.get_mut(y) {
            if let Some(g_x) = g_y.get_mut(x) {
                g_x.push(circle);
            }
        }

        // self.points
        //     .get_mut(y)
        //     .unwrap()
        //     .get_mut(x)
        //     .unwrap()
        //     .push(circle);
    }

    pub fn get_neighbors(&self, circle: Circle) -> Option<&Vec<Circle>> {
        let x = self.get_x(circle);
        let y = self.get_y(circle);

        if !(0..=9).contains(&x) || !(0..=9).contains(&y) {
            panic!("{}:{} is out of bounds", x, y);
        }

        return self.points.get(y).unwrap().get(x);
    }

    fn get_x(&self, circle: Circle) -> usize {
        return (circle.x / self.width * 10.0).floor() as usize;
    }

    fn get_y(&self, circle: Circle) -> usize {
        return (circle.y / self.height * 10.0).floor() as usize;
    }
}

#[cfg(test)]
mod test {
    use super::PointMap;
    use crate::circle::Circle;

    #[test]
    fn get_x() {
        let point_map: PointMap<Circle> = PointMap::new(100.0, 100.0);
        assert_eq!(point_map.get_x(Circle::new(9.0, 0.0, 0.0)), 0);
        assert_eq!(point_map.get_x(Circle::new(11.0, 0.0, 0.0)), 1);
        assert_eq!(point_map.get_x(Circle::new(20.0, 0.0, 0.0)), 2);
        assert_eq!(point_map.get_x(Circle::new(34.0, 0.0, 0.0)), 3);
        assert_eq!(point_map.get_x(Circle::new(99.999, 0.0, 0.0)), 9);
    }

    #[test]
    fn get_y() {
        let point_map: PointMap<Circle> = PointMap::new(100.0, 100.0);
        assert_eq!(point_map.get_y(Circle::new(11.0, 5.5, 0.0)), 0);
        assert_eq!(point_map.get_y(Circle::new(20.0, 15.0, 0.0)), 1);
        assert_eq!(point_map.get_y(Circle::new(34.0, 73.0, 0.0)), 7);
        assert_eq!(point_map.get_y(Circle::new(99.999, 90.0, 0.0)), 9);
    }

    #[test]
    fn insert_point() {
        let mut point_map: PointMap<Circle> = PointMap::new(100.0, 100.0);
        let circle = Circle::new(11.0, 11.0, 10.0);
        point_map.insert(circle);
        let points = point_map.points.get_mut(1).unwrap().get_mut(1).unwrap();
        assert_eq!(points.len(), 1);
    }

    #[test]
    fn get_neighbors() {
        let mut point_map: PointMap<Circle> = PointMap::new(100.0, 100.0);
        let circle = Circle::new(11.0, 11.0, 5.0);
        let non_neighbor = Circle::new(30.3, 50.4, 10.0);
        point_map.insert(circle);
        point_map.insert(non_neighbor);

        let neighbors = point_map.get_neighbors(circle);

        assert_eq!(neighbors.unwrap().len(), 1);
        assert_eq!(neighbors.unwrap().first().unwrap().to_owned(), circle);
    }
}
