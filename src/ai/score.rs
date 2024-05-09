pub fn is_winner(board: &String, player: char) -> bool {
    false
}

pub fn is_full(board: &String) -> bool {
    !board.contains('-')
}

#[cfg(test)]
mod tests {
    use crate::ai::score::is_full;

    #[test]
    fn test_is_full_with_one() {
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
        assert_eq!(false, is_full(&board.to_string()));
    }

    #[test]
    fn test_is_full_is_true() {
        let board = "
    a     b     c
      |     |
1  o  |  o  |  x
 _____|_____|_____
      |     |
2  x  |  o  |  x
 _____|_____|_____
      |     |
3  o  |  x  |  o
      |     |     ";
        assert_eq!(true, is_full(&board.to_string()));
    }

}