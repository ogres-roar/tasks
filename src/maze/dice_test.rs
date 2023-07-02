#[cfg(test)]
use super::dice;
#[test]
fn test_uniform() {
    let expects = vec![(0, 6, 4), (2, 6, 1)];
    for expect in expects {
        assert_eq!(dice::get_uniform(expect.0, expect.1), expect.2);
    }
}

#[test]
fn test_norm() {
    let expects = vec![
        (0, 0.0, 10.0, 7.12813),
        (10, 0.0, 10.0, -6.2340534),
        (15, 3.0, 10.0, 5.47559),
    ];
    for expect in expects {
        assert_eq!(dice::get_norm(expect.0, expect.1, expect.2), expect.3);
    }
}
