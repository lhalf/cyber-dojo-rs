#![allow(dead_code)]

const SCORES: [&'static str; 4] = ["Love", "Fifteen", "Thirty", "Forty"];

fn tennis_score(score1: u8, score2: u8) -> String {
    match (score1, score2) {
        (score1, score2) if score1 >= 4 && (score1.saturating_sub(score2)) >= 2 => "Win for player1".to_string(),
        (score1, score2) if score2 >= 4 && (score2.saturating_sub(score1)) >= 2 => "Win for player2".to_string(),
        (score1, score2) if score1 > score2 && score2 >= 3 => "Advantage player1".to_string(),
        (score1, score2) if score2 > score1 && score1 >= 3 => "Advantage player2".to_string(),
        (score1, score2) if score1 == score2 && score1 >= 3 => "Deuce".to_string(),
        (score1, score2) if score1 == score2 && score1 < 4 => format!("{}-All", SCORES.get(score1 as usize).unwrap_or(&"?")),
        (score1, score2) => format!(
            "{}-{}",
            SCORES.get(score1 as usize).unwrap_or(&"?"),
            SCORES.get(score2 as usize).unwrap_or(&"?")
        ),
    }
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
