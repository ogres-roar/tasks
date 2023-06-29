/// This module is maze node
// 开口方向
enum Connect {
    null,
    up,
    down,
    right,
    left,
    up_right,
    up_down,
    up_left,
    right_down,
    right_left,
    down_left,
    up_right_down,
    up_right_left,
    up_down_left,
    right_down_left,
    up_right_down_left,
}

// 节点位置以及开口方向
pub struct Node {
    column: i16,
    row: i16,
    connect: Connect,
    visit: bool,
    steps: u32,
}

impl Node {
    pub fn new(column: i16, row: i16) -> Node {
        Node {
            column: column,
            row: row,
            connect: null,
            visit: false,
            steps: 0,
        }
    }

    pub fn visit(self) {
        self.visit = true
    }

    pub fn hasVisited(self) -> bool {
        return self.visit;
    }
}
