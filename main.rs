use queue::*;

enum NodeIndex {
    Middle,
    Node1,
    Node2,
    Node3,
    Node4,
    Node5,
    Node6,
    Room1,
    Room2,
    Room3,
    Room4,
    Last,
}

struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        return Point { x: x, y: y };
    }
}

struct Node {
    pub node_point: Point,
    pub bound_upp_left: Point,
    pub bound_low_right: Point,
    pub neighbours: Vec<usize>,
    pub name: String,
    pub previous: usize,
    pub index: usize,
    pub visited: bool,
}

impl Node {
    pub fn new(
        name: String,
        node_index: usize,
        node_x: i32,
        node_y: i32,
        upp_left_x: i32,
        upp_left_y: i32,
        low_right_x: i32,
        low_right_y: i32,
        neighbours: Vec<usize>,
    ) -> Node {
        return Node {
            node_point: Point::new(node_x, node_y),
            bound_upp_left: Point::new(upp_left_x, upp_left_y),
            bound_low_right: Point::new(low_right_x, low_right_y),
            index: node_index,
            neighbours: neighbours,
            name: name,
            previous: usize::MAX,
            visited: false,
        };
    }

    pub fn get_neighbours(&self) -> Vec<usize> {
        return self.neighbours.clone();
    }

    pub fn add_neighbours(&mut self, index: usize) {
        self.neighbours.push(index);
    }

    pub fn set_neighbours(&mut self, neighbours: Vec<usize>) {
        self.neighbours = neighbours;
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn is_visited(&self) -> bool {
        return self.visited;
    }

    pub fn set_visited(&mut self, val: bool) {
        self.visited = val;
    }

    pub fn set_previous(&mut self, previous: usize) {
        self.previous = previous;
    }

    pub fn get_previous(&self) -> usize {
        return self.previous;
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        return self.bound_upp_left.x <= x
            && x <= self.bound_low_right.x
            && self.bound_upp_left.y <= y
            && y <= self.bound_upp_left.y;
    }
}

struct NodeMap {
    nodes: Vec<Node>,
    rooms: Vec<usize>,
}

impl NodeMap {
    fn new() -> NodeMap {
        return NodeMap {
            nodes: Vec::new(),
            rooms: Vec::new(),
        };
    }

    fn init(&mut self) {
        self.nodes = vec![
            Node::new(
                "Middle".into(),
                0,
                950,
                1180,
                -1,
                -1,
                -1,
                -1,
                vec![
                    NodeIndex::Node1 as usize,
                    NodeIndex::Node3 as usize,
                    NodeIndex::Node4 as usize,
                    NodeIndex::Node5 as usize,
                ],
            ),
            Node::new(
                "Node 1".into(),
                1,
                935,
                2165,
                700,
                2400,
                2400,
                1930,
                vec![NodeIndex::Middle as usize],
            ),
            Node::new(
                "Node 2".into(),
                2,
                2160,
                1140,
                1920,
                1930,
                2400,
                900,
                vec![NodeIndex::Node3 as usize],
            ),
            Node::new(
                "Node 3".into(),
                3,
                1405,
                1140,
                1170,
                1380,
                1920,
                900,
                vec![
                    NodeIndex::Middle as usize,
                    NodeIndex::Node2 as usize,
                    NodeIndex::Room1 as usize,
                ],
            ),
            Node::new(
                "Node 4".into(),
                4,
                235,
                1255,
                0,
                1490,
                710,
                1020,
                vec![NodeIndex::Middle as usize, NodeIndex::Room2 as usize],
            ),
            Node::new(
                "Node 5".into(),
                5,
                945,
                670,
                710,
                900,
                1180,
                440,
                vec![
                    NodeIndex::Middle as usize,
                    NodeIndex::Node6 as usize,
                    NodeIndex::Room4 as usize,
                ],
            ),
            Node::new(
                "Node 6".into(),
                6,
                945,
                220,
                710,
                440,
                1180,
                0,
                vec![NodeIndex::Room3 as usize, NodeIndex::Node5 as usize],
            ),
            Node::new(
                "Room 1".into(),
                7,
                1405,
                1500,
                1170,
                1930,
                1920,
                1380,
                vec![NodeIndex::Node3 as usize],
            ),
            Node::new(
                "Room 2".into(),
                8, 
                235,
                1800,
                0,
                2400,
                700,
                1490,
                vec![NodeIndex::Node4 as usize],
            ),
            Node::new(
                "Room 3".into(),
                9,
                500,
                220,
                0,
                1020,
                700,
                0,
                vec![NodeIndex::Node6 as usize],
            ),
            Node::new(
                "Room 4".into(),
                10,
                1400,
                670,
                1180,
                900,
                2400,
                0,
                vec![NodeIndex::Node5 as usize],
            ),
        ];
    }

    pub fn get_current_node(self, x: i32, y: i32) -> i32 {
        for i in 0..NodeIndex::Last as usize {
            if (self.nodes[i].in_bounds(x, y)) {
                return i as i32;
            }
        }
        return NodeIndex::Last as i32;
    }

    pub fn get_path(&mut self, start_node : usize, end_node : usize) -> Vec<Point> {

        let mut processing_queue: Queue<usize> = Queue::new();
        let mut path: Vec<Point> = vec![];

        self.reset_visited();

        processing_queue.queue(start_node);
        self.nodes[start_node].visited = true;

        let mut current_node: usize;

        while !processing_queue.is_empty() {
            current_node = processing_queue.dequeue().unwrap();
            if current_node == end_node {
                recreate_path(start_node, end_node, path);
                return path;
            }
            else {
                for i in 0..neighbours.len() {
                    if (!self.nodes[i].visited) {
                        self.nodes[i].visited = true;
                        self.nodes[i].previous = current_node;
                        processing_queue.queue(i);
                    }
                }
            }

        }
        path.append(Point(self.nodes[start_node].node_point.x, self.nodes[start_node].node_point.y);
        return path;
    }

    pub fn reset_visited(&mut self) 
    {
        for i in 0..NodeIndex::Last as usize {
            self.nodes[i].visited = false;
        }
    }
}

fn main() {}
