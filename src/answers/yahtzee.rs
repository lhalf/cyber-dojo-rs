trait Yahtzee {
    fn chance(&self) -> u8;
    fn yahtzee(&self) -> u8;
    fn ones(&self) -> u8;
    fn twos(&self) -> u8;
    fn threes(&self) -> u8;
    fn fours(&self) -> u8;
    fn fives(&self) -> u8;
    fn sixes(&self) -> u8;
}

impl Yahtzee for [u8; 5] {
    fn chance(&self) -> u8 {
        self.iter().sum()
    }

    fn yahtzee(&self) -> u8 {
        if self.iter().all(|roll| *roll == self[0]) {
            50
        } else {
            0
        }
    }

    fn ones(&self) -> u8 {
        self.iter().filter(|roll| **roll == 1).sum()
    }

    fn twos(&self) -> u8 {
        self.iter().filter(|roll| **roll == 2).sum()
    }

    fn threes(&self) -> u8 {
        self.iter().filter(|roll| **roll == 3).sum()
    }

    fn fours(&self) -> u8 {
        self.iter().filter(|roll| **roll == 4).sum()
    }

    fn fives(&self) -> u8 {
        self.iter().filter(|roll| **roll == 5).sum()
    }

    fn sixes(&self) -> u8 {
        self.iter().filter(|roll| **roll == 6).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Yahtzee;

    #[test]
    fn chance() {
        assert_eq!(15, [2, 3, 4, 5, 1].chance());
        assert_eq!(16, [3, 3, 4, 5, 1].chance());
        assert_eq!(18, [3, 3, 6, 5, 1].chance());
    }

    #[test]
    fn yahtzee() {
        assert_eq!(50, [6, 6, 6, 6, 6].yahtzee());
        assert_eq!(0, [6, 6, 6, 6, 3].yahtzee());
        assert_eq!(50, [2, 2, 2, 2, 2].yahtzee());
    }

    #[test]
    fn ones() {
        assert_eq!(3, [1, 1, 1, 2, 2].ones());
        assert_eq!(4, [1, 1, 1, 1, 2].ones());
        assert_eq!(0, [0, 0, 0, 0, 2].ones());
    }

    #[test]
    fn twos() {
        assert_eq!(4, [1, 1, 1, 2, 2].twos());
        assert_eq!(10, [2, 2, 2, 2, 2].twos());
        assert_eq!(8, [0, 2, 2, 2, 2].twos());
    }

    #[test]
    fn threes() {
        assert_eq!(6, [3, 3, 1, 2, 2].threes());
        assert_eq!(12, [2, 3, 3, 3, 3].threes());
        assert_eq!(3, [0, 2, 2, 2, 3].threes());
    }

    #[test]
    fn fours() {
        assert_eq!(4, [3, 3, 4, 2, 2].fours());
        assert_eq!(12, [2, 4, 4, 4, 3].fours());
        assert_eq!(0, [0, 2, 2, 2, 3].fours());
    }

    #[test]
    fn fives() {
        assert_eq!(5, [3, 3, 4, 5, 2].fives());
        assert_eq!(15, [2, 4, 5, 5, 5].fives());
        assert_eq!(0, [0, 2, 2, 2, 3].fives());
    }

    #[test]
    fn sixes() {
        assert_eq!(6, [3, 3, 4, 5, 6].sixes());
        assert_eq!(30, [6, 6, 6, 6, 6].sixes());
        assert_eq!(0, [0, 2, 2, 2, 3].sixes());
    }
}
