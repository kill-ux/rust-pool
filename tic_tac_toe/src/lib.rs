pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    let mut p1 = false ;
    let mut p2 = false ;
    if diagonals('X',table) || horizontal('X',table) || vertical('X',table) {
        p1 = true
    }
    if diagonals('O',table) || horizontal('O',table) || vertical('O',table) {
        p2 = true
    }
    if !p1 && !p2 {
        String::from("tie")
    } else if p1 {
        String::from("player X won")
    } else {
        String::from("player O won")
    }
}

pub fn diagonals(player: char, tabl: [[char; 3]; 3]) -> bool {
    let mut table = tabl.clone();
    for _i in 0..2 {
        let mut count = 0;
        let mut index = 0;
        for tab in table {
            if player == tab[index] {
                count += 1;
                index += 1;
                if count == 3 {
                    return true;
                }
            }
        }
        table.reverse();
    }
    false
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for tab in table {
        let mut count = 0;
        for c in tab {
            if player == c {
                count += 1;
                if count == 3 {
                    return true;
                }
            }
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for i in 0..3 {
        let mut count = 0;
        for tab in table {
            if player == tab[i] {
                count += 1;
                if count == 3 {
                    return true;
                }
            }
        }
    }
    false
}
