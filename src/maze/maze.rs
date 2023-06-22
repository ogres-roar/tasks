/// A maze always start from (0,0);
pub type maze = Node;

impl maze {
    pub fn generate(randSeed: u128) -> maze {
        Node::new(0, 0);
    }

    pub fn render() {
        let directions = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
    }

    fn in_maze(column: i16, row: i16) -> bool {
        if column >= 0 && row >= 0 {
            return true;
        }
        return false;
    }
}
