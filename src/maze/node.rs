/// This module is maze node
pub struct Node {
    column: i16,
    row: i16,
    connects: Vec<Box<Node>>,
    visit: bool,
    steps: u32,
}

impl Node {
    pub fn new(column: i16, row: i16) -> Node {
        Node {
            column: column,
            row: row,
            connects: Vec<Box<Node>>{},
            visit: false,
        }
    }

    pub fn visit(self) {
        self.visit = true
    }

    pub fn hasVisited(self) -> bool {
        return self.visit;
    }

    pub fn connects(self, cell: Box<Node>) {
        self.connects.append(cell);
    }
}
