use queue::*;

const ROOM_SEARCH_ORDER : [[usize ; 3] ; 4] = [[3, 2, 1],   //room0
                                               [2, 3, 0],   //room1
                                               [1, 3, 0],   //room2
                                               [0, 2, 1]];  //room3

const AMOUNT_OF_ROOMS : usize = 3;

#[derive(Clone, Copy, PartialEq)]
enum NodeIndex {
    S0,
    S1,
    S2,
    S3,
    F0,
    F1_0,
    F1_1,
    F2,
    F3_0,
    F3_1,
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    Last,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ScanSettings {
    NoScan,
    Scan,
    Done,
}

pub struct PathPoint {
    pub x : i32,
    pub y : i32,
    pub scan_settings : ScanSettings,
    pub angle : f64,
}

impl PathPoint {
    pub fn new(x: i32, y:i32, scan_settings : ScanSettings, angle : f64) -> PathPoint {
        return PathPoint { x: x, y: y, scan_settings : scan_settings, angle : angle};
    }
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        return Point { x: x, y: y };
    }
}

pub struct Node {
    pub node_point: Point,
    pub bound_upp_left: Point,
    pub bound_low_right: Point,
    pub neighbours: Vec<usize>,
    pub name: String,
    pub previous: usize,
    pub index: usize,
    pub visited: bool,
    pub scan_settings: ScanSettings,
    pub scan_angle: f64,
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
        scan_angle: f64,
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
        println!("{}, {} {} {} {}", self.name, self.bound_upp_left.x <= x, x <= self.bound_low_right.x, self.bound_upp_left.y <= y, y <= self.bound_upp_left.y);
        return self.bound_upp_left.x <= x
            && x <= self.bound_low_right.x
            && self.bound_upp_left.y <= y
            && y <= self.bound_low_right.y;
    }
}

pub struct NodeMap {
    nodes: Vec<Node>,
    rooms: Vec<usize>,
    global_path: Vec<PathPoint>,
    start_x: i32,
    start_y: i32,
    cur_path_index: i32,
}

impl NodeMap {
    pub fn new(x: i32, y:i32) -> NodeMap {
        let mut map = NodeMap {
            nodes: Vec::new(),
            rooms: Vec::new(),
            global_path: Vec::new(),
            start_x: x,
            start_y: y,
            cur_path_index: 0,
        };
        map.init();
        return map;
    }

    fn init(&mut self) {
        self.global_path = vec![];
        //Creating the nodes
        self.nodes = vec![
            Node::new( //done
                "s0".into(),
                NodeIndex::S0 as usize,
                180,
                2100,
                0,
                1370,
                720,
                2400,
                ScanSettings::Scan,
                300_f64.to_radians(),
                vec![
                    NodeIndex::F0 as usize,
                ]

            ),
            Node::new(//done
                "s1".into(),
                NodeIndex::S1 as usize,
                180,
                300,
                0,
                0,
                700,
                900,
                ScanSettings::Scan,
                55_f64.to_radians(),
                vec![
                    NodeIndex::F1_1 as usize,
                ]
            ),
            Node::new(//done
                "s2".into(),
                NodeIndex::S2 as usize,
                1300,
                630,
                1170,
                470,
                1900,
                1020,
                ScanSettings::Scan,
                40_f64.to_radians(),
                vec![
                    NodeIndex::F2 as usize,
                ]

            ),
            Node::new(//done
                "s3".into(),
                NodeIndex::S3 as usize,
                2080,
                1700,
                1190,
                1490,
                2400,
                2400,
                ScanSettings::Scan,
                150_f64.to_radians(),
                vec![
                    NodeIndex::F3_1 as usize,
                ]

            ),
            Node::new(//done
                "f0".into(),
                NodeIndex::F0 as usize,
                800,
                2100,
                -1,
                -1,
                -1,
                -1,
                ScanSettings::Scan,
                180_f64.to_radians(),
                vec![
                    NodeIndex::R5 as usize,
                    NodeIndex::S0 as usize,
                ]

            ),
            Node::new(//done
                "f1_0".into(),
                NodeIndex::F1_0 as usize,
                180,
                2100,
                -1,
                -1,
                -1,
                -1,
                ScanSettings::Scan,
                270_f64.to_radians(),
                vec![
                    NodeIndex::R0 as usize,
                    NodeIndex::F1_1 as usize,   
                ]

            ),
            Node::new(
                "f1_1".into(),
                6,
                180,
                2100,
                -1,
                -1,
                -1,
                -1,
                ScanSettings::Scan,
                270_f64.to_radians(),
                vec![
                    NodeIndex::F1_0 as usize,
                    NodeIndex::S1 as usize,
                ]

            ),
            Node::new(//done
                "f2".into(),
                NodeIndex::F2 as usize,
                1300,
                1150,
                -1,
                -1,
                -1,
                -1,
                ScanSettings::Scan,
                270_f64.to_radians(),
                vec![
                    NodeIndex::R3 as usize,
                    NodeIndex::S2 as usize,
                ]

            ),
            Node::new(//done
                "f3_0".into(),
                NodeIndex::F3_1 as usize,
                1050,
                1700,
                -1,
                -1,
                -1,
                -1,
                ScanSettings::Scan,
                0_f64.to_radians(),
                vec![
                    NodeIndex::R4 as usize,
                    NodeIndex::F3_1 as usize,
                ]

            ),
            Node::new(//done
                "f3_1".into(),
                NodeIndex::F3_1 as usize,
                1580,
                1700,
                -1,
                -1,
                -1,
                -1,
                ScanSettings::Scan,
                0_f64.to_radians(),
                vec![
                    NodeIndex::F3_0 as usize,
                    NodeIndex::S3 as usize,
                ]

            ),
            Node::new( //done
                "r0".into(),
                NodeIndex::R0 as usize,
                180,
                1150,
                -1,
                -1,
                -1,
                -1,
                ScanSettings::NoScan,
                300_f64.to_radians(),
                vec![
                    NodeIndex::F1_0 as usize,
                    NodeIndex::R1 as usize,
                ]

            ),
            Node::new( //done
                "r1".into(),
                NodeIndex::R1 as usize,
                960,
                1150,
                -1,
                -1,
                -1,
                -1,
                ScanSettings::NoScan,
                0_f64.to_radians(),
                vec![
                    NodeIndex::R0 as usize,
                    NodeIndex::R2 as usize,
                ]

            ),
            Node::new(//done
                "r2".into(),
                NodeIndex::R2 as usize,
                960,
                1300,
                -1,
                -1,
                -1,
                -1,
                ScanSettings::NoScan,
                0_f64.to_radians(),
                vec![
                    NodeIndex::R1 as usize,
                    NodeIndex::R3 as usize,
                    NodeIndex::R4 as usize,
                ]

            ),
            Node::new( //done
                "r3".into(),
                NodeIndex::R3 as usize,
                1300,
                1300,
                -1,
                -1,
                -1,
                -1,
                ScanSettings::NoScan,
                0_f64.to_radians(),
                vec![
                    NodeIndex::R2 as usize,
                    NodeIndex::F2 as usize,
                ]

            ),
            Node::new( //Done
                "r4".into(),
                NodeIndex::R4 as usize,
                960,
                1700,
                -1,
                -1,
                -1,
                -1,
                ScanSettings::NoScan,
                0_f64.to_radians(),
                vec![
                    NodeIndex::R2 as usize,
                    NodeIndex::R5 as usize,
                    NodeIndex::F3_0 as usize,
                ]

            ),
            Node::new( //
                "r5".into(),
                NodeIndex::R5 as usize,
                960,
                1300,
                -1,
                -1,
                -1,
                -1,
                ScanSettings::NoScan,
                0_f64.to_radians(),
                vec![
                    NodeIndex::R4 as usize,
                    NodeIndex::F0 as usize,
                ]

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

        let _ = processing_queue.queue(start_node);
        self.nodes[start_node].visited = true;

        let mut current_node: usize;
        println!("start_node = {}, end_node = {}", start_node, end_node);
        while !processing_queue.is_empty() {
            //println!("len of the queue = {}", processing_queue.len());
            current_node = processing_queue.dequeue().unwrap();
            println!("current_node = {}", self.nodes[current_node].name);
            if current_node == end_node {
                self.recreate_path(start_node, end_node, &mut path);
                return path;
            }
            else {
                let mut neighbours: usize;
                for i in 0..self.nodes[current_node].neighbours.len() {
                    neighbours = self.nodes[current_node].neighbours[i];
                    if !self.nodes[neighbours].visited {
                        self.nodes[neighbours].visited = true;
                        self.nodes[neighbours].previous = current_node;
                        let _ = processing_queue.queue(neighbours);
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
            path.push(backward_path[i]);
        }
    }

    pub fn go_home(&mut self,) {
        let current_x = self.global_path[self.cur_path_index as usize].x;
        let current_y = self.global_path[self.cur_path_index as usize].y;
        let mut current_node_index: usize = 0;
        let mut end_node_index: usize = 0;

        self.global_path = vec![];
        self.cur_path_index = 0;

        for node_index in 0..self.nodes.len() {
            if self.nodes[node_index].in_bounds(current_x, current_y) {
                current_node_index = node_index;
            }
        }

        for node_index in 0..self.nodes.len() {
            if self.nodes[node_index].in_bounds(self.start_x, self.start_y) {
                end_node_index = node_index;
            }
        }

        let home_path: Vec<usize> = self.get_path(current_node_index, end_node_index);
        println!("the lenght of the path is {} from {} to {}", home_path.len(), current_node_index, end_node_index);
        for current_node in home_path {
            println!("{:?}", self.nodes[current_node].name);
            // If the node should be scanned and it has not been scanned, then it should be scanned.
            self.global_path.push(PathPoint::new(self.nodes[current_node].node_point.x,
                                                    self.nodes[current_node].node_point.y,
                                                    ScanSettings::NoScan,
                                                    self.nodes[current_node].scan_angle))
        }
    }

    pub fn create_global_path(&mut self,) {
        self.global_path = vec![];
        let mut current_path : Vec<usize> = vec![];

        let start_room : usize = self.get_current_node(self.start_x, self.start_y);
        let mut previous_visited_room : usize = start_room;
        let mut scan_settings : ScanSettings;

        //Go throught all the rooms and find the paths to them from the previous visited room.
        for room_index in 0..AMOUNT_OF_ROOMS {
            println!("start_node: {}, end_node:{}", previous_visited_room, ROOM_SEARCH_ORDER[start_room][room_index]);
            current_path = self.get_path(previous_visited_room, 
                                         ROOM_SEARCH_ORDER[start_room][room_index]);

            for i in 0..current_path.len() {
            println!("{:?}", self.nodes[current_path[i]].name);
            }
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
            previous_visited_room = ROOM_SEARCH_ORDER[start_room][room_index]
        }

        let last_index = self.global_path.len() - 1;
        self.global_path[last_index].scan_settings = ScanSettings::Done;

    }

    pub fn get_next_path(&mut self) -> &PathPoint{
        let ret = &self.global_path[self.cur_path_index as usize];
        self.cur_path_index += 1;
        return ret;
    }

}
fn main() {
    let mut node_map : NodeMap = NodeMap::new(1191, 1491);
    node_map.create_global_path();
    let mut path_point: &PathPoint;
    for i in 0..node_map.global_path.len() {
        path_point = node_map.get_next_path();
        println!("{} {}", path_point.x, path_point.y);
        // if i == 10 {
            // println!("===========================");
            // println!("Going home :)");
            // println!("===========================");
            // node_map.go_home();
            // break;
        // }
    }
    // println!("===========================");
    // println!("Going home :)");
    // println!("===========================");
    // for i in 0..node_map.global_path.len() {
        // path_point = node_map.get_next_path();
        // println!("{} {}", path_point.x, path_point.y);
    // }
}







































//oscar was here