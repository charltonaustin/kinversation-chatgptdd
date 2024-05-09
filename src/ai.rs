pub fn make_move(mut board: String) -> String{
    "".to_string()
}

#[cfg(test)]
mod tests {
    use crate::ai::make_move;

    #[test]
    fn test_make_move() {
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
        assert_eq!("b2", make_move(board.to_string()));
    }

}
