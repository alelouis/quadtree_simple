use crate::utils::{Area, Node};
use partition::partition;

pub struct Quadtree {
    pub nodes: Vec<Node>,
    pub indices: Vec<usize>,
    pub max_points_per_cell: usize,
    pub area: Area,
}

impl Quadtree {
    pub fn new(max_points_per_cell: usize, area: Area) -> Self {
        let nodes: Vec<Node> = vec![];
        let indices: Vec<usize> = vec![0];
        Quadtree {
            nodes,
            indices,
            max_points_per_cell,
            area,
        }
    }
    pub fn build(&mut self, points: &mut [Vec<f64>], area: Area) -> Option<usize> {
        let index = self.nodes.len();
        let n_current_point = points.len();
        if n_current_point == 0 {
            return None;
        } else if n_current_point < self.max_points_per_cell {
            self.indices
                .push(self.indices.last().unwrap() + n_current_point);
            self.nodes.push(Node::from(area));
        } else {
            let (middle_x, middle_y) = (area.middle_x(), area.middle_y());

            let (left, right) = partition(points, |point| point[0] < middle_x);
            let (lower_left, upper_left) = partition(left, |point| point[1] < middle_y);
            let (lower_right, upper_right) = partition(right, |point| point[1] < middle_y);

            self.nodes.push(Node::from(area));

            self.nodes[index].upper_left = self.build(
                upper_left,
                Area::new(area.x_min, middle_x, middle_y, area.y_max),
            );
            self.nodes[index].upper_right = self.build(
                upper_right,
                Area::new(middle_x, area.x_max, middle_y, area.y_max),
            );
            self.nodes[index].lower_left = self.build(
                lower_left,
                Area::new(area.x_min, middle_x, area.y_min, middle_y),
            );
            self.nodes[index].lower_right = self.build(
                lower_right,
                Area::new(middle_x, area.x_max, area.y_min, middle_y),
            );
        }
        Some(index)
    }
}
