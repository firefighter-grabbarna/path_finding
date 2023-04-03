use queue::*;

const ROOM_SEARCH_ORDER : [[usize ; 3] ; 4] = [[1, 2, 3],   //room0
                                               [1, 2, 3],   //room1
                                               [1, 2, 3],   //room2
                                               [1, 2, 3]];  //room3

const AMOUNT_OF_ROOMS : usize = 3;

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

#[derive(PartialEq)]
enum ScanSettings {
    NoScan,
    Scan,
    Done,
}

struct PathPoint {
    pub x : i32,
    pub y : i32,
    pub scan_settings : ScanSettings,
    pub angle : f32,
}

impl PathPoint {
    fn new(x: i32, y:i32, scan_settings : ScanSettings, angle : f32) -> PathPoint {
        return PathPoint { x: x, y: y, scan_settings : scan_settings, angle : angle};
    }
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
    pub scan_settings: ScanSettings,
    pub scan_angle: f32,
    pub scanned: bool,
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
        scan_settings: ScanSettings,
        scan_angle: f32,
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
            scan_settings: scan_settings,
            scan_angle: scan_angle,
            scanned: false,
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
    global_path: Vec<PathPoint>,
    start_x: i32,
    start_y: i32,
    cur_path_index: i32,
}

impl NodeMap {
    fn new(x: i32, y:i32) -> NodeMap {
        return NodeMap {
            nodes: Vec::new(),
            rooms: Vec::new(),
            global_path: Vec::new(),
            start_x: x,
            start_y: y,
            cur_path_index: 0,
        };
    }

    fn init(&mut self) {
        self.global_path = vec![];
        //Creating the nodes
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
                ScanSettings::NoScan,
                0.0,
                vec![
                    NodeIndex::Node1 as usize,
                    NodeIndex::Node3 as usize,
                    NodeIndex::Node4 as usize,
                    NodeIndex::Node5 as usize,
                ],
            ),
        ];

        // Creating the nodes we need to visit according to where start.

    }

    pub fn get_current_node(&self, x: i32, y: i32) -> usize {
        for i in 0..NodeIndex::Last as usize {
            if self.nodes[i].in_bounds(x, y) {
                return i;
            }
        }
        return NodeIndex::Last as usize;
    }

    pub fn get_path(&mut self, start_node : usize, end_node : usize) -> Vec<usize> {

        let mut processing_queue: Queue<usize> = Queue::new();
        let mut path: Vec<usize> = vec![];

        // Reset the previous nodes.
        self.reset_visited();

        processing_queue.queue(start_node);
        self.nodes[start_node].visited = true;

        let mut current_node: usize;

        while !processing_queue.is_empty() {
            current_node = processing_queue.dequeue().unwrap();
            if current_node == end_node {
                self.recreate_path(start_node, end_node, &mut path);
                return path;
            }
            else {
                for i in 0..self.nodes[current_node].neighbours.len() {
                    if !self.nodes[i].visited {
                        self.nodes[i].visited = true;
                        self.nodes[i].previous = current_node;
                        processing_queue.queue(i);
                    }
                }
            }

        }
        path.push(start_node);
        return path;
    }

    pub fn reset_visited(&mut self) 
    {
        for i in 0..NodeIndex::Last as usize {
            self.nodes[i].visited = false;
        }
    }

    pub fn recreate_path(&mut self, start_node : usize, end_node : usize, path : &mut Vec<usize>)
    {
        let mut backward_path: Vec<usize> = vec![];
        let mut current_node : usize = end_node;

        while current_node != start_node {
            backward_path.push(current_node);
            current_node = self.nodes[current_node].previous;
        }
        backward_path.push(current_node);

        for i in (0..backward_path.len()).rev() {
            path.push(5);
        }
    }

    pub fn create_global_path(&mut self,) {
        self.global_path = vec![];
        let mut current_path : Vec<usize> = vec![];

        let start_room : usize = self.get_current_node(self.start_x, self.start_y);
        let mut previous_visited_room : usize = start_room;
        let mut scan_settings : ScanSettings;

        for room_index in 0..AMOUNT_OF_ROOMS {
            current_path = self.get_path(previous_visited_room, 
                                         ROOM_SEARCH_ORDER[start_room][room_index]);
            
            for current_node in current_path {
                // If the node should be scanned and it has not been scanned, then it should be scanned.
                if self.nodes[current_node].scan_settings == ScanSettings::Scan &&
                   !self.nodes[current_node].scanned {
                    scan_settings = ScanSettings::Scan;
                }
                else {
                    scan_settings = ScanSettings::NoScan;
                }
                self.nodes[current_node].scanned = true;


                self.global_path.push(PathPoint::new(self.nodes[current_node].node_point.x,
                                                     self.nodes[current_node].node_point.y,
                                                     scan_settings,
                                                     self.nodes[current_node].scan_angle))
            }
        }

        let last_index = self.global_path.len() - 1;
        self.global_path[last_index].scan_settings = ScanSettings::Done;

    }

}

fn main() {}
