mod is_full;
mod is_winner;
use is_winner::is_winner;
use is_full::is_full;
pub fn make_move(board: String, to_win: String, to_lose: String) -> String {
    let mut best_score = i32::MIN;
    let mut best_move = String::new();

    if board.chars().nth(92).unwrap() == '-' {
        return "b2".to_string()
    }
    // Simulate placing 'x' in each possible position and evaluate using minimax
    for (_position, index) in positions().iter() {
        if board.chars().nth(*index).unwrap() == '-' {
            let mut new_board = board.clone();
            new_board.replace_range(*index..=*index, &to_win);
            let score = minimax(new_board, to_win.to_string(), to_lose.to_string(),true);
            if score > best_score {
                best_score = score;
                best_move = _position.to_string();
            }
        }
    }

    best_move
}

fn minimax(board: String, to_win: String, to_lose: String, is_maximizing: bool) -> i32 {
    if is_winner(&board, to_win.chars().next().unwrap()) {
        return 1;
    } else if is_winner(&board, 'o') {
        return -1;
    } else if is_full(&board) {
        return 0;
    }

    if is_maximizing {
        let mut best_score = i32::MIN;
        for (_position, index) in positions().iter() {
            if board.chars().nth(*index).unwrap() == '-' {
                let mut new_board = board.clone();
                new_board.replace_range(*index..=*index, &to_win);
                let score = minimax(new_board, to_win.to_string(), to_lose.to_string(), false);
                best_score = i32::max(best_score, score);
            }
        }
        best_score
    } else {
        let mut best_score = i32::MAX;
        for (_position, index) in positions().iter() {
            if board.chars().nth(*index).unwrap() == '-' {
                let mut new_board = board.clone();
                new_board.replace_range(*index..=*index, &to_lose);
                let score = minimax(new_board, to_win.to_string(), to_lose.to_string(), false);
                best_score = i32::min(best_score, score);
            }
        }
        best_score
    }
}

fn positions() -> Vec<(&'static str, usize)> {
    vec![
        ("b2", 92), ("b1", 42), ("c1", 48),
        ("a2", 86), ("a1", 36), ("c2", 98),
        ("a3", 136), ("b3", 142), ("c3", 148)
    ]
}
#[cfg(test)]
mod tests {
    use crate::ai::make_move;

    #[test]
    fn test_always_pick_middle_with_top_right() {
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
        assert_eq!("b2", make_move(board.to_string(), "o".to_string(), "x".to_string()));
    }

    #[test]
    fn test_always_pick_middle_with_top_left() {
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
        assert_eq!("b2", make_move(board.to_string(), "o".to_string(), "x".to_string()));
    }

    #[test]
    fn test_pick_with_empty_board() {
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
        assert_eq!("b2", make_move(board.to_string(), "o".to_string(), "x".to_string()));
    }

    #[test]
    fn test_pick_stop_win() {
        let board = "
    a     b     c
      |     |
1  x  |  x  |  -
 _____|_____|_____
      |     |
2  -  |  o  |  -
 _____|_____|_____
      |     |
3  -  |  -  |  -
      |     |     ";
        assert_eq!("c1", make_move(board.to_string(), "o".to_string(), "x".to_string()));
    }
    #[test]
    fn test_pick_best_move_for_o() {
        let board = "
    a     b     c
      |     |
1  x  |  x  |  -
 _____|_____|_____
      |     |
2  o  |  o  |  -
 _____|_____|_____
      |     |
3  -  |  -  |  -
      |     |     ";
        assert_eq!("c2", make_move(board.to_string(), "o".to_string(), "x".to_string()));
    }

}
