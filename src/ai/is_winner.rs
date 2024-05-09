pub fn is_winner(board: &String, player: char) -> bool {
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