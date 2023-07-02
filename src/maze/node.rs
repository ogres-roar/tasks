/// This module is maze node
// 开口方向

#[allow(dead_code)]
pub enum Connect {
    Null,
    Up,
    Down,
    Right,
    Left,
    UpRight,
    UpDown,
    UpLeft,
    RightDown,
    RightLeft,
    DownLeft,
    UpRightDown,
    UpRightLeft,
    UpDownLeft,
    RightDownLeft,
    UpRightDownLeft,
}

// 节点位置以及开口方向

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
            connect: Connect::Null,
            steps: 0,
        }
    }

    pub fn generate_map(row_num: i16, line_num: i16) -> Vec<Vec<Node>> {
        let mut maze: Vec<Vec<Node>> = vec![];
        for row in 0..row_num {
            let mut nodes: Vec<Node> = vec![];
            for column in 0..line_num {
                nodes.push(Node::new(column, row))
            }
            maze.push(nodes);
        }
        return maze;
    }

    #[allow(dead_code)]
    pub fn get_char(self) -> String {
        match self.connect {
            Connect::Null => return String::from(""),
            Connect::Up => return String::from("\u{2575}"),
            Connect::Right => return String::from("\u{2576}"),
            Connect::Down => return String::from("\u{2577}"),
            Connect::Left => return String::from("\u{2574}"),
            Connect::UpRight => return String::from("\u{2514}"),
            Connect::UpDown => return String::from("\u{2502}"),
            Connect::UpLeft => return String::from("\u{2518}"),
            Connect::RightDown => return String::from("\u{250C}"),
            Connect::RightLeft => return String::from("\u{2500}"),
            Connect::DownLeft => return String::from("\u{2510}"),
            Connect::UpRightDown => return String::from("\u{251C}"),
            Connect::UpRightLeft => return String::from("\u{2534}"),
            Connect::UpDownLeft => return String::from("\u{2524}"),
            Connect::RightDownLeft => return String::from("\u{252C}"),
            Connect::UpRightDownLeft => return String::from("\u{253C}"),
        }
    }
}
