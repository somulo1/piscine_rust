pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if diagonals('X', table) || horizontal('X', table) || vertical('X', table) {
        return "player X won".to_string();
    }
    if diagonals('O', table) || horizontal('O', table) || vertical('O', table) {
        return "player O won".to_string();
    }
    "tie".to_string()
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
<<<<<<< HEAD
    (table[0][0] == player && table[1][1] == player && table[2][2] == player) ||
=======
    // Check main diagonal
    (table[0][0] == player && table[1][1] == player && table[2][2] == player) ||
    // Check anti-diagonal
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
    (table[0][2] == player && table[1][1] == player && table[2][0] == player)
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
<<<<<<< HEAD
    for row in table.iter() {
        if row[0] == player && row[1] == player && row[2] == player {
            return true;
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for col in 0..3 {
        if table[0][col] == player && table[1][col] == player && table[2][col] == player {
            return true;
        }
    }
    false
}
=======
    table.iter().any(|&row| row.iter().all(|&c| c == player))
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    (0..3).any(|col| (0..3).all(|row| table[row][col] == player))
}

>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
