/// A maze always start from (0,0);
pub mod dice;
pub mod dice_test;
pub mod node;

use node::Node;

#[allow(dead_code)]
pub struct Maze {
    nodes: Vec<Vec<Node>>,
    row: i16,
    column: i16,
}

impl Maze {
    pub fn generate(row: i16, column: i16) -> Maze {
        if row <= 0 || column <= 0 {
            panic!("maze size input negative, row: {}, column: {}", row, column)
        }
        return Maze {
            nodes: Node::generate_map(row, column),
            row: row,
            column: column,
        };
    }

    #[allow(dead_code)]
    pub fn render() {
        // let directions = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
    }

    #[allow(dead_code)]
    fn in_maze(column: i16, row: i16) -> bool {
        if column >= 0 && row >= 0 {
            return true;
        }
        return false;
    }
}
