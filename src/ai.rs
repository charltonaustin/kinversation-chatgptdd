mod score;
use score::is_winner;
use score::is_full;
pub fn make_move(board: String) -> String {
    let mut best_score = i32::MIN;
    let mut best_move = String::new();

    // Simulate placing 'x' in each possible position and evaluate using minimax
    for (position, index) in positions().iter() {
        if board.chars().nth(*index).unwrap() == '-' {
            let mut new_board = board.clone();
            new_board.replace_range(*index..=*index, "x");
            let score = minimax(new_board, false);
            if score > best_score {
                best_score = score;
                best_move = position.to_string();
            }
        }
    }

    best_move
}

fn minimax(board: String, is_maximizing: bool) -> i32 {
    if is_winner(&board, 'x') {
        return 1;
    } else if is_winner(&board, 'o') {
        return -1;
    } else if is_full(&board) {
        return 0;
    }

    if is_maximizing {
        let mut best_score = i32::MIN;
        for (position, index) in positions().iter() {
            if board.chars().nth(*index).unwrap() == '-' {
                let mut new_board = board.clone();
                new_board.replace_range(*index..=*index, "x");
                let score = minimax(new_board, false);
                best_score = i32::max(best_score, score);
            }
        }
        best_score
    } else {
        let mut best_score = i32::MAX;
        for (position, index) in positions().iter() {
            if board.chars().nth(*index).unwrap() == '-' {
                let mut new_board = board.clone();
                new_board.replace_range(*index..=*index, "y");
                let score = minimax(new_board, true);
                best_score = i32::min(best_score, score);
            }
        }
        best_score
    }
}

fn positions() -> Vec<(&'static str, usize)> {
    vec![
        ("a1", 31), ("b1", 37), ("c1", 43),
        ("a2", 79), ("b2", 85), ("c2", 91),
        ("a3", 127), ("b3", 133), ("c3", 139)
    ]
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
