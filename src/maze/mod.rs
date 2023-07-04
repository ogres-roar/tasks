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
    pub fn generate(seed: u64, row: i16, column: i16) -> Maze {
        return Maze {
            nodes: Node::generate_map(seed, row, column),
            row: row,
            column: column,
        };
    }

    #[allow(dead_code)]
    pub fn render(self) -> Vec<String> {
        let mut rds: Vec<String> = vec![];
        for rows in self.nodes {
            let mut line: String = String::from("");
            for columns in rows {
                line += columns.get_char().as_str();
            }
            rds.push(line);
        }
        return rds;
    }
}
