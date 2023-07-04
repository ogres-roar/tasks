/// This module is maze node
// 开口方向
use super::dice::{generate_seed, get_uniform};

type Position = (i16, i16);

// 上, 右, 下, 左
type Connect = (bool, bool, bool, bool);

// 节点位置以及开口方向
#[derive(PartialEq, Eq)]
#[allow(dead_code)]
pub struct Node {
    column: i16,
    row: i16,
    connect: Connect,
    steps: u32,
}

impl Node {
    pub fn new(column: i16, row: i16) -> Node {
        Node {
            column: column,
            row: row,
            connect: (false, false, false, false),
            steps: 0,
        }
    }

    pub fn generate_map(seed: u64, row_num: i16, line_num: i16) -> Vec<Vec<Node>> {
        if row_num <= 0 || line_num <= 0 {
            panic!(
                "maze size input negative, row: {}, column: {}",
                row_num, line_num
            )
        }

        // generate maze nodes
        let mut maze: Vec<Vec<Node>> = vec![];
        for row in 0..row_num {
            let mut nodes: Vec<Node> = vec![];
            for column in 0..line_num {
                nodes.push(Node::new(column, row))
            }
            maze.push(nodes);
        }

        Self::connect(seed, &mut maze);
        return maze;
    }

    fn set_connect(node: &mut Node, direction: (i16, i16)) {
        if direction.0 == 1 {
            node.connect.2 = true;
        }

        if direction.1 == 1 {
            node.connect.1 = true;
        }

        if direction.0 == -1 {
            node.connect.0 = true;
        }

        if direction.1 == -1 {
            node.connect.3 = true;
        }
    }

    fn connect(seed: u64, nodes: &mut Vec<Vec<Node>>) {
        let mut seed = seed;
        let mut stack: Vec<(usize, usize)> = vec![];
        let neighbers = Self::get_neighbers(0, 0, nodes);
        nodes[0][0].connect = (true, false, false, true);
        nodes[0][0].steps = 0;

        let choice = get_uniform(seed, neighbers.len() as u64);
        seed = generate_seed(seed);
        let neighber = neighbers[choice as usize];
        Self::set_connect(&mut nodes[0][0], neighber);
        Self::set_connect(
            &mut nodes[neighber.0 as usize][neighber.1 as usize],
            (0 - neighber.0, 0 - neighber.1),
        );
        nodes[neighber.0 as usize][neighber.1 as usize].steps = 1;

        stack.push((0, 0));
        stack.push((neighber.0 as usize, neighber.1 as usize));
        while stack.len() != 0 {
            let position = stack.pop().unwrap();
            let neighbers = Self::get_neighbers(position.0 as i16, position.1 as i16, nodes);
            if neighbers.len() == 0 {
                continue;
            }
            seed = generate_seed(seed);
            let choice = get_uniform(seed, neighbers.len() as u64);
            let neighber = neighbers[choice as usize];
            Self::set_connect(
                &mut nodes[position.0 as usize][position.1 as usize],
                neighber,
            );
            Self::set_connect(
                &mut nodes[(position.0 as i16 + neighber.0) as usize]
                    [(position.1 as i16 + neighber.1) as usize],
                (-neighber.0, -neighber.1),
            );

            nodes[(position.0 as i16 + neighber.0) as usize]
                [(position.1 as i16 + neighber.1) as usize]
                .steps = nodes[position.0][position.1].steps + 1;
            stack.push(position);
            stack.push((
                (position.0 as i16 + neighber.0) as usize,
                (position.1 as i16 + neighber.1) as usize,
            ));
        }
        let row = nodes.len() as i32;
        let column = nodes[0].len() as i32;
        nodes[(row - 1) as usize][(column - 1) as usize].connect.1 = true;
        nodes[(row - 1) as usize][(column - 1) as usize].connect.2 = true;
    }

    fn get_neighbers(row: i16, column: i16, nodes: &Vec<Vec<Node>>) -> Vec<Position> {
        let directions: Vec<(i16, i16)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        let mut neighbers: Vec<Position> = vec![];

        for direction in directions {
            let nrow = row + direction.0;
            let ncolumn = column + direction.1;
            if Self::in_maze(nrow, ncolumn, nodes.len() as i16, nodes[0].len() as i16) {
                if nodes[nrow as usize][ncolumn as usize].connect == (false, false, false, false) {
                    neighbers.push((direction.0, direction.1));
                }
            }
        }

        return neighbers;
    }

    fn in_maze(row: i16, column: i16, row_num: i16, column_num: i16) -> bool {
        if row < 0 || column < 0 {
            return false;
        }

        if row >= row_num || column >= column_num {
            return false;
        }
        return true;
    }

    #[allow(dead_code)]
    pub fn get_char(self) -> String {
        if self.connect.0 == false
            && self.connect.1 == false
            && self.connect.2 == false
            && self.connect.3 == false
        {
            return String::from("");
        }

        if self.connect.0 == true
            && self.connect.1 == false
            && self.connect.2 == false
            && self.connect.3 == false
        {
            return String::from("\u{0020}\u{2575}\u{0020}");
        }

        if self.connect.0 == false
            && self.connect.1 == true
            && self.connect.2 == false
            && self.connect.3 == false
        {
            return String::from("\u{0020}\u{0020}\u{2576}");
        }

        if self.connect.0 == false
            && self.connect.1 == false
            && self.connect.2 == true
            && self.connect.3 == false
        {
            return String::from("\u{0020}\u{2577}\u{0020}");
        }

        if self.connect.0 == false
            && self.connect.1 == false
            && self.connect.2 == false
            && self.connect.3 == true
        {
            return String::from("\u{2574}\u{0020}\u{0020}");
        }

        if self.connect.0 == true
            && self.connect.1 == true
            && self.connect.2 == false
            && self.connect.3 == false
        {
            return String::from("\u{0020}\u{2514}\u{2500}");
        }

        if self.connect.0 == true
            && self.connect.1 == false
            && self.connect.2 == true
            && self.connect.3 == false
        {
            return String::from("\u{0020}\u{2502}\u{0020}");
        }

        if self.connect.0 == true
            && self.connect.1 == false
            && self.connect.2 == false
            && self.connect.3 == true
        {
            return String::from("\u{2500}\u{2518}\u{0020}");
        }

        if self.connect.0 == false
            && self.connect.1 == true
            && self.connect.2 == true
            && self.connect.3 == false
        {
            return String::from("\u{0020}\u{250C}\u{2500}");
        }

        if self.connect.0 == false
            && self.connect.1 == true
            && self.connect.2 == false
            && self.connect.3 == true
        {
            return String::from("\u{2500}\u{2500}\u{2500}");
        }

        if self.connect.0 == false
            && self.connect.1 == false
            && self.connect.2 == true
            && self.connect.3 == true
        {
            return String::from("\u{2500}\u{2510}\u{0020}");
        }

        if self.connect.0 == true
            && self.connect.1 == true
            && self.connect.2 == true
            && self.connect.3 == false
        {
            return String::from("\u{0020}\u{251C}\u{2500}");
        }

        if self.connect.0 == true
            && self.connect.1 == true
            && self.connect.2 == false
            && self.connect.3 == true
        {
            return String::from("\u{2500}\u{2534}\u{2500}");
        }

        if self.connect.0 == true
            && self.connect.1 == false
            && self.connect.2 == true
            && self.connect.3 == true
        {
            return String::from("\u{2500}\u{2524}\u{0020}");
        }

        if self.connect.0 == false
            && self.connect.1 == true
            && self.connect.2 == true
            && self.connect.3 == true
        {
            return String::from("\u{2500}\u{252C}\u{2500}");
        }

        if self.connect.0 == true
            && self.connect.1 == true
            && self.connect.2 == true
            && self.connect.3 == true
        {
            return String::from("\u{2500}\u{253C}\u{2500}");
        }

        panic!("unknow connection type!");
    }
}

#[cfg(test)]
mod node_test {
    use super::Node;
    #[test]
    fn test_in_maze() {
        let expects = vec![
            (0, 0, 4, 4, true),
            (0, 4, 4, 4, false),
            (-1, 0, 4, 4, false),
            (4, 0, 4, 4, false),
            (0, -1, 4, 4, false),
        ];
        for expect in expects {
            assert_eq!(
                Node::in_maze(expect.0, expect.1, expect.2, expect.3),
                expect.4
            );
        }
    }
}
