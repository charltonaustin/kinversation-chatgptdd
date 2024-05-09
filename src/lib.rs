mod ai;

pub fn empty_board() -> String {
    "
    a     b     c
      |     |
1  -  |  -  |  -
 _____|_____|_____
      |     |
2  -  |  -  |  -
 _____|_____|_____
      |     |
3  -  |  -  |  -
      |     |     ".to_string()
}

pub fn add_x(mut board: String, position: &str) -> String {
    add_symbol(&mut board, position, "x")
}

pub fn add_y(mut board: String, position: &str) -> String {
    add_symbol(&mut board, position, "y")
}
fn add_symbol(board: &mut String, position: &str, symbol: &str) -> String {
    let index = match position {
        "a1" => 36,  // the index of the first '-' in "1  -  |  -  |  -"
        "b1" => 42,
        "c1" => 48,
        "a2" => 86,  // the index of the first '-' in "2  -  |  -  |  -"
        "b2" => 92,
        "c2" => 98,
        "a3" => 136, // the index of the first '-' in "3  -  |  -  |  -"
        "b3" => 142,
        "c3" => 148,
        _ => return board.to_string(),  // return the board unchanged if position is invalid
    };

    board.replace_range(index..index + 1, symbol);  // replace the character at the calculated index
    board.to_string()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_board() {
        let board = "
    a     b     c
      |     |
1  -  |  -  |  -
 _____|_____|_____
      |     |
2  -  |  -  |  -
 _____|_____|_____
      |     |
3  -  |  -  |  -
      |     |     ";
        assert_eq!(board, &empty_board());
    }
    #[test]
    fn test_add_x_a1() {
        let board = "
    a     b     c
      |     |
1  x  |  -  |  -
 _____|_____|_____
      |     |
2  -  |  -  |  -
 _____|_____|_____
      |     |
3  -  |  -  |  -
      |     |     ";
        assert_eq!(board, add_x(empty_board(), "a1"));
    }
    #[test]
    fn test_add_x_b1() {
        let board = "
    a     b     c
      |     |
1  -  |  x  |  -
 _____|_____|_____
      |     |
2  -  |  -  |  -
 _____|_____|_____
      |     |
3  -  |  -  |  -
      |     |     ";
        assert_eq!(board, add_x(empty_board(), "b1"));
    }

    #[test]
    fn test_add_x_c1() {
        let board = "
    a     b     c
      |     |
1  -  |  -  |  x
 _____|_____|_____
      |     |
2  -  |  -  |  -
 _____|_____|_____
      |     |
3  -  |  -  |  -
      |     |     ";
        assert_eq!(board, add_x(empty_board(), "c1"));
    }

    #[test]
    fn test_add_x_a2() {
        let board = "
    a     b     c
      |     |
1  -  |  -  |  -
 _____|_____|_____
      |     |
2  x  |  -  |  -
 _____|_____|_____
      |     |
3  -  |  -  |  -
      |     |     ";
        assert_eq!(board, add_x(empty_board(), "a2"));
    }
    #[test]
    fn test_add_x_a3() {
        let board = "
    a     b     c
      |     |
1  -  |  -  |  -
 _____|_____|_____
      |     |
2  -  |  -  |  -
 _____|_____|_____
      |     |
3  x  |  -  |  -
      |     |     ";
        assert_eq!(board, add_x(empty_board(), "a3"));
    }
    #[test]
    fn test_add_y_a1() {
        let board = "
    a     b     c
      |     |
1  y  |  -  |  -
 _____|_____|_____
      |     |
2  -  |  -  |  -
 _____|_____|_____
      |     |
3  -  |  -  |  -
      |     |     ";
        assert_eq!(board, add_y(empty_board(), "a1"));
    }
    #[test]
    fn test_add_y_b1() {
        let board = "
    a     b     c
      |     |
1  -  |  y  |  -
 _____|_____|_____
      |     |
2  -  |  -  |  -
 _____|_____|_____
      |     |
3  -  |  -  |  -
      |     |     ";
        assert_eq!(board, add_y(empty_board(), "b1"));
    }

    #[test]
    fn test_add_y_c1() {
        let board = "
    a     b     c
      |     |
1  -  |  -  |  y
 _____|_____|_____
      |     |
2  -  |  -  |  -
 _____|_____|_____
      |     |
3  -  |  -  |  -
      |     |     ";
        assert_eq!(board, add_y(empty_board(), "c1"));
    }

    #[test]
    fn test_add_y_a2() {
        let board = "
    a     b     c
      |     |
1  -  |  -  |  -
 _____|_____|_____
      |     |
2  y  |  -  |  -
 _____|_____|_____
      |     |
3  -  |  -  |  -
      |     |     ";
        assert_eq!(board, add_y(empty_board(), "a2"));
    }
    #[test]
    fn test_add_y_a3() {
        let board = "
    a     b     c
      |     |
1  -  |  -  |  -
 _____|_____|_____
      |     |
2  -  |  -  |  -
 _____|_____|_____
      |     |
3  y  |  -  |  -
      |     |     ";
        assert_eq!(board, add_y(empty_board(), "a3"));
    }
}