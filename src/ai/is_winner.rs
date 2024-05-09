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
    for i in [31, 37, 43] { // indices for columns a, b, c in the first row
        if lines.iter().enumerate().filter(|&(idx, _)| idx == 3 || idx == 6 || idx == 9)
            .map(|(_, line)| line.chars().nth(i).unwrap())
            .filter(|&c| c == player)
            .count() == 3 {
            return true;
        }
    }

    // Diagonal wins
    let main_diagonal = [lines[3].chars().nth(31).unwrap(), lines[6].chars().nth(37).unwrap(), lines[9].chars().nth(43).unwrap()];
    let anti_diagonal = [lines[3].chars().nth(43).unwrap(), lines[6].chars().nth(37).unwrap(), lines[9].chars().nth(31).unwrap()];

    if main_diagonal.iter().all(|&c| c == player) || anti_diagonal.iter().all(|&c| c == player) {
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