mod maze;

use maze::dice;

fn main() {
    let length = 1;

    let mut row = String::from("\u{2575}\u{2576}\u{2577}\u{2574}");
    println!("{}", row);

    row = String::from("\u{250C}");
    println!("{}", row);
    for _ in 0..length {
        row += "\u{002500}\u{002500}\u{002500}\u{00252C}";
    }
    row += "\u{002500}\u{002500}\u{002500}\u{002510}";
    println!("{}", row);

    for _ in 0..length {
        let mut row1 = String::from("\u{002502}");
        let mut row2 = String::from("\u{00251C}");
        for _ in 0..length {
            row1 += "\u{000020}\u{000020}\u{000020}\u{002502}";
            row2 += "\u{002500}\u{002500}\u{002500}\u{00253C}"
        }
        row1 += "\u{000020}\u{000020}\u{000020}\u{002502}";
        row2 += "\u{002500}\u{002500}\u{002500}\u{002524}";
        println!("{}", row1);
        println!("{}", row2);
    }

    let mut row1 = String::from("\u{002502}");
    let mut row2 = String::from("\u{002514}");
    for _ in 0..length {
        row1 += "\u{000020}\u{000020}\u{000020}\u{002502}";
        row2 += "\u{002500}\u{002500}\u{002500}\u{002534}";
    }
    row1 += "\u{000020}\u{000020}\u{000020}\u{002502}";
    row2 += "\u{002500}\u{002500}\u{002500}\u{002518}";
    println!("{}", row1);
    println!("{}", row2);

    println!("{}", dice::get_uniform(2, 6));
    println!("{}", dice::get_norm(15, 3.0, 10.0));

    let _maze = maze::Maze::generate(20, 20);
}
