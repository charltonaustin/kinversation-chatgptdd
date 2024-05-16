use std::ops::Index;

pub fn is_winner(board: &String, player: char) -> bool {
    let lines: Vec<&str> = board.split('\n').collect();

    // Horizontal wins
    let horizontal_lines = vec![lines[3], lines[6], lines[9]]; // rows are lines 3, 6, and 9 in the input string
    for line in horizontal_lines {
        if line.chars().filter(|&c| c == player).count() == 3 {
            return true;
        }
    }

    // Vertical wins
    for row in [4, 10, 15] { // indices for columns a, b, c in the first row
        let first = lines.index(3);
        let second = lines.index(6);
        let third = lines.index(9);
        if first.chars().nth(row) == second.chars().nth(row) && second.chars().nth(row) == third.chars().nth(row) && third.chars().nth(row) == Some(player) {
           return true;
       }
    }

    // Diagonal wins
    let main_diagonal = [lines[3].chars().nth(3), lines[6].chars().nth(9), lines[9].chars().nth(15)];
    let anti_diagonal = [lines[3].chars().nth(15), lines[6].chars().nth(9), lines[9].chars().nth(3)];
    if main_diagonal.iter().all(|&c| c == Some(player)) || anti_diagonal.iter().all(|&c| c == Some(player)) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::ai::is_winner::is_winner;

    #[test]
    fn test_is_winner_with_one() {
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
        assert_eq!(false, is_winner(&board.to_string(), 'x'));
    }

    #[test]
    fn test_is_winner_with_wrong_char() {
        let board = "
    a     b     c
      |     |
1  x  |  x  |  x
 _____|_____|_____
      |     |
2  -  |  -  |  -
 _____|_____|_____
      |     |
3  -  |  -  |  -
      |     |     ";
        assert_eq!(false, is_winner(&board.to_string(), 'y'));
    }

    #[test]
    fn test_is_winner_with_right_char() {
        let board = "
    a     b     c
      |     |
1  x  |  x  |  x
 _____|_____|_____
      |     |
2  -  |  -  |  -
 _____|_____|_____
      |     |
3  -  |  -  |  -
      |     |     ";
        assert_eq!(true, is_winner(&board.to_string(), 'x'));
    }

    #[test]
    fn test_is_winner_with_right_char_diagonal() {
        let board = "
    a     b     c
      |     |
1  -  |  -  |  x
 _____|_____|_____
      |     |
2  -  |  x  |  -
 _____|_____|_____
      |     |
3  x  |  -  |  -
      |     |     ";
        assert_eq!(true, is_winner(&board.to_string(), 'x'));
    }

    #[test]
    fn test_is_winner_with_right_char_vertical() {
        let board = "
    a     b     c
      |     |
1  -  |  -  |  x
 _____|_____|_____
      |     |
2  -  |  -  |  x
 _____|_____|_____
      |     |
3  -  |  -  |  x
      |     |     ";
        assert_eq!(true, is_winner(&board.to_string(), 'x'));
    }

}