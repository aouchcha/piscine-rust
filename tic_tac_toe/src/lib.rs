pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if diagonals('O', table) || vertical('O', table) || horizontal('O', table) {
        return "player O won".to_string();
    }
    if diagonals('X', table) || vertical('X', table) || horizontal('X', table) {
        return "player X won".to_string();
    }
    return "tie".to_string();
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    let mut answer = 0;
    for i in 0..table.len() {
        for j in 0..table[i].len() {
            if i == j {
                if table[i][j] == player {
                    answer += 1;
                }
            }
        }
    }
    if answer == 3 {
        return true;
    }
    answer = 0;
    for i in 0..table.len() {
        for j in 0..table[i].len() {
            if i + j == table.len() - 1 {
                if table[i][j] == player {
                    answer += 1;
                }
            }
        }
    }
    answer == 3
    // table[1][1] == player && (table[0][0] == player || table[0][2] == player) && (table[2][2] == player || table[2][0] == player)
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for i in 0..table.len() {
        if table[i][0] == player && table[i][1] == player && table[i][2] == player {
            return true;
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for i in 0..table.len() {
        if table[0][i] == player && table[1][i] == player && table[2][i] == player {
            return true;
        }
    }
    false
}
