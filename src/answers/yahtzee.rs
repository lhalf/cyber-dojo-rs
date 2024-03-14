trait Yahtzee {
    fn chance(&self) -> u8;
    fn yahtzee(&self) -> u8;
    fn ones(&self) -> u8;
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
}
