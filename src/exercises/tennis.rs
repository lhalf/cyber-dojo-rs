#![allow(dead_code)]

fn tennis_score(p1score: i8, p2score: i8) -> String {
    let mut score = String::new();
    let mut p1res = String::new();
    let mut p2res = String::new();

    if p1score == p2score && p1score < 4 {
        if p1score == 0 {
            score = "Love".to_string();
        }
        if p1score == 1 {
            score = "Fifteen".to_string();
        }
        if p1score == 2 {
            score = "Thirty".to_string();
        }
        score += "-All";
    }

    if p1score == p2score && p1score > 2 {
        score = "Deuce".to_string();
    }

    if p1score > 0 && p2score == 0 {
        if p1score==1 {
            p1res = "Fifteen".to_string();
        }
        if p1score==2 {
            p1res = "Thirty".to_string();
        }
        if p1score==3 {
            p1res = "Forty".to_string();
        }
        p2res = "Love".to_string();
        score = p1res.clone() + "-" + &p2res;
    }

    if p2score > 0 && p1score == 0 {
        if p2score==1 {
            p2res = "Fifteen".to_string();
        }
        if p2score==2 {
            p2res = "Thirty".to_string();
        }
        if p2score==3 {
            p2res = "Forty".to_string();
        }
        p1res = "Love".to_string();
        score = p1res.clone() + "-" + &p2res;
    }

    if p1score>p2score && p1score < 4 {
        if p1score==2 {
            p1res = "Thirty".to_string();
        }
        if p1score==3 {
            p1res = "Forty".to_string();
        }
        if p2score==1 {
            p2res = "Fifteen".to_string();
        }
        if p2score==2 {
            p2res = "Thirty".to_string();
        }
        score = p1res.clone() + "-" + &p2res;
    }

    if p2score>p1score && p2score < 4 {
        if p2score==2 {
            p2res = "Thirty".to_string();
        }
        if p2score==3 {
            p2res = "Forty".to_string();
        }
        if p1score==1 {
            p1res = "Fifteen".to_string();
        }
        if p1score==2 {
            p1res = "Thirty".to_string();
        }
        score = p1res.clone() + "-" + &p2res;
    }

    if p1score > p2score && p2score >= 3 {
        score = "Advantage player1".to_string();
    }

    if p2score > p1score && p1score >= 3 {
        score = "Advantage player2".to_string();
    }

    if p1score >= 4 && p2score >= 0 && (p1score-p2score)>=2 {
        score = "Win for player1".to_string();
    }

    if p2score >= 4 && p1score >= 0 && (p2score-p1score)>=2 {
        score = "Win for player2".to_string();
    }

    return score;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn love_all_0_0() {
        assert_eq!("Love-All", tennis_score(0, 0));
    }
    #[test]
    fn fifteen_all_1_1() {
        assert_eq!("Fifteen-All", tennis_score(1, 1));
    }
    #[test]
    fn thirty_all_2_2() {
        assert_eq!("Thirty-All", tennis_score(2, 2));
    }
    #[test]
    fn deuce_3_3() {
        assert_eq!("Deuce", tennis_score(3, 3));
    }
    #[test]
    fn deuce_4_4() {
        assert_eq!("Deuce", tennis_score(4, 4));
    }
    #[test]
    fn fifteen_love_1_0() {
        assert_eq!("Fifteen-Love", tennis_score(1, 0));
    }
    #[test]
    fn love_fifteen_0_1() {
        assert_eq!("Love-Fifteen", tennis_score(0, 1));
    }
    #[test]
    fn thirty_love_2_0() {
        assert_eq!("Thirty-Love", tennis_score(2, 0));
    }
    #[test]
    fn love_thirty_0_2() {
        assert_eq!("Love-Thirty", tennis_score(0, 2));
    }
    #[test]
    fn forty_love_3_0() {
        assert_eq!("Forty-Love", tennis_score(3, 0));
    }
    #[test]
    fn love_forty_0_3() {
        assert_eq!("Love-Forty", tennis_score(0, 3));
    }
    #[test]
    fn win_for_player1_4_0() {
        assert_eq!("Win for player1", tennis_score(4, 0));
    }
    #[test]
    fn win_for_player2_0_4() {
        assert_eq!("Win for player2", tennis_score(0, 4));
    }
    #[test]
    fn thirty_fifteen_2_1() {
        assert_eq!("Thirty-Fifteen", tennis_score(2, 1));
    }
    #[test]
    fn fifteen_thirty_1_2() {
        assert_eq!("Fifteen-Thirty", tennis_score(1, 2));
    }
    #[test]
    fn forty_fifteen_3_1() {
        assert_eq!("Forty-Fifteen", tennis_score(3, 1));
    }
    #[test]
    fn fifteen_forty_1_3() {
        assert_eq!("Fifteen-Forty", tennis_score(1, 3));
    }
    #[test]
    fn win_for_player1_4_1() {
        assert_eq!("Win for player1", tennis_score(4, 1));
    }
    #[test]
    fn win_for_player2_1_4() {
        assert_eq!("Win for player2", tennis_score(1, 4));
    }
    #[test]
    fn forty_thirty_3_2() {
        assert_eq!("Forty-Thirty", tennis_score(3, 2));
    }
    #[test]
    fn thirty_forty_2_3() {
        assert_eq!("Thirty-Forty", tennis_score(2, 3));
    }
    #[test]
    fn win_for_player1_4_2() {
        assert_eq!("Win for player1", tennis_score(4, 2));
    }
    #[test]
    fn win_for_player2_2_4() {
        assert_eq!("Win for player2", tennis_score(2, 4));
    }
    #[test]
    fn advantage_player1_4_3() {
        assert_eq!("Advantage player1", tennis_score(4, 3));
    }
    #[test]
    fn advantage_player2_3_4() {
        assert_eq!("Advantage player2", tennis_score(3, 4));
    }
    #[test]
    fn advantage_player1_5_4() {
        assert_eq!("Advantage player1", tennis_score(5, 4));
    }
    #[test]
    fn advantage_player2_4_5() {
        assert_eq!("Advantage player2", tennis_score(4, 5));
    }
    #[test]
    fn advantage_player1_15_14() {
        assert_eq!("Advantage player1", tennis_score(15, 14));
    }
    #[test]
    fn advantage_player2_14_15() {
        assert_eq!("Advantage player2", tennis_score(14, 15));
    }
    #[test]
    fn win_for_player1_6_4() {
        assert_eq!("Win for player1", tennis_score(6, 4));
    }
    #[test]
    fn win_for_player2_4_6() {
        assert_eq!("Win for player2", tennis_score(4, 6));
    }
    #[test]
    fn win_for_player1_16_14() {
        assert_eq!("Win for player1", tennis_score(16, 14));
    }
    #[test]
    fn win_for_player2_14_16() {
        assert_eq!("Win for player2", tennis_score(14, 16));
    }
}
