mod maze;

fn main() {
    let _maze = maze::Maze::generate(4, 20, 20);
    for s in _maze.render() {
        println!("{}", s);
    }
}
