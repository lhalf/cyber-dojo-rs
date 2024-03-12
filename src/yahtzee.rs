#![allow(dead_code)]

struct Yahtzee {
    d1: u8,
    d2: u8,
    d3: u8,
    d4: u8,
    d5: u8
}

impl Yahtzee {
    fn chance(d1: u8, d2: u8, d3: u8, d4: u8, d5:u8) -> u8 {
        let mut total = 0;
        total += d1;
        total += d2;
        total += d3;
        total += d4;
        total += d5;
        total
    }

    fn yahtzee(dice: &[u8;5]) -> u8 {
        let mut counts = [0,0,0,0,0,0];
        for roll in dice {
            counts[(roll-1) as usize] += 1;
        }
        for count in counts {
            if count == 5 {
                return 50;
            }
        }
        return 0;
    }

    fn ones(d1: u8, d2: u8, d3: u8, d4: u8, d5:u8) -> u8 {
        let mut sum = 0;
        if d1 == 1 { sum += 1 };
        if d2 == 1 { sum += 1 };
        if d3 == 1 { sum += 1 };
        if d4 == 1 { sum += 1 };
        if d5 == 1 {
            sum += 1;
        }
        return sum;
    }

    fn twos(d1: u8, d2: u8, d3: u8, d4: u8, d5:u8) -> u8 {
        let mut sum = 0;
        if d1 == 2{ sum += 2 };
        if d2 == 2 { sum += 2 };
        if d3 == 2 { sum += 2 };
        if d4 == 2 { sum += 2 };
        if d5 == 2 {
            sum += 2;
        }
        return sum;
    }

    fn threes(d1: u8, d2: u8, d3: u8, d4: u8, d5:u8) -> u8 {
        let mut s;
        s = 0;
        if d1 == 3 { s += 3 };
        if d2 == 3 { s += 3 };
        if d3 == 3 { s += 3 };
        if d4 == 3 { s += 3 };
        if d5 == 3 {
            s += 3;
        }
        return s;
    }

    fn fours(&self) -> u8 {
        let mut sum;
        sum = 0;
        for dice in [self.d1, self.d2, self.d3, self.d4, self.d5] {
            if dice == 4 {
                sum += 4;
            }
        }
        sum
    }

    fn fives(&self) -> u8 {
        let mut s: u8 = 0;
        let i = 0;
        for d in i..6 {
            match d {
                1 => if self.d1 == 5 { s = s + 5},
                2 => if self.d2 == 5 { s = s + 5},
                3 => if self.d3 == 5 { s = s + 5},
                4 => if self.d4 == 5 { s = s + 5},
                5 => if self.d5 == 5 { s = s + 5},
                _ => (),
            }
        }
        s
    }

    fn sixes(&self) -> u8 {
        let mut s= 0;
        for d in [self.d1, self.d2, self.d3, self.d4, self.d5] {
            if d == 6 {
                s += 6;
            }
        }
        s
    }

    fn scorepair(d1: u8, d2: u8, d3: u8, d4: u8, d5:u8) -> u8 {
        let mut counts = [0,0,0,0,0,0];
        counts[(d1-1) as usize] += 1;
        counts[(d2-1) as usize] += 1;
        counts[(d3-1) as usize] += 1;
        counts[(d4-1) as usize] += 1;
        counts[(d5-1) as usize] += 1;
        for i in 0..6 {
            if counts[6-i-1] == 2 {
                return ((6-i) * 2) as u8;
            }
        }
        0
    }

    fn twopair(d1: u8, d2: u8, d3: u8, d4: u8, d5:u8) -> u8 {
        let mut counts = [0;6];
        counts[(d1-1) as usize] += 1;
        counts[(d2-1) as usize] += 1;
        counts[(d3-1) as usize] += 1;
        counts[(d4-1) as usize] += 1;
        counts[(d5-1) as usize] += 1;
        let mut n = 0;
        let mut score: u8 = 0;
        for i in 0..6 {
            if counts[6-i-1] == 2 {
                n += 1;
                score += (6-i) as u8;
            }
        }
        if n == 2 {
            score*2
        } else {
            0
        }
    }

    fn threeofakind(d1: u8, d2: u8, d3: u8, d4: u8, d5:u8) -> u8 {
        let mut t;
        t = Box::new([0;6]);
        t[(d1-1) as usize] += 1;
        t[(d2-1) as usize] += 1;
        t[(d3-1) as usize] += 1;
        t[(d4-1) as usize] += 1;
        t[(d5-1) as usize] += 1;
        for i in 0..6 {
            if t[i] == 3 {
                return ((i+1)*3) as u8;
            }
        }
        return 0;
    }

    fn fourofakind(_1: u8, _2: u8, d3: u8, d4: u8, d5:u8) -> u8 {
        let mut tallies;
        tallies = Box::new([0;6]);
        tallies[(_1-1) as usize] += 1;
        tallies[(_2-1) as usize] += 1;
        tallies[(d3-1) as usize] += 1;
        tallies[(d4-1) as usize] += 1;
        tallies[(d5-1) as usize] += 1;
        for i in 0..6 {
            if tallies[i] == 4 {
                return ((i+1)*4) as u8;
            }
        }
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::Yahtzee;

    #[test]
    fn chance_scores_sum_of_all_dice() {
        let expected = 15;
        let actual = Yahtzee::chance(2,3,4,5,1);
        assert_eq!(expected, actual);
        assert_eq!(16, Yahtzee::chance(3,3,4,5,1));
    }

    fn ints(a: u8, b: u8, c: u8, d: u8, e: u8) -> Box<[u8;5]> {
        let mut r: Box<[u8;5]> = Box::new([0,0,0,0,0]);
        r[0] = a;
        r[1] = b;
        r[2] = c;
        r[3] = d;
        r[4] = e;
        return r;
    }

    #[test]
    fn yahtzee_scores_50() {
        let expected = 50;
        let actual = Yahtzee::yahtzee(&ints(4,4,4,4,4));
        assert_eq!(expected, actual);
        assert_eq!(50, Yahtzee::yahtzee(&ints(6,6,6,6,6)));
        assert_eq!(0, Yahtzee::yahtzee(&ints(6,6,6,6,3)));
    }

    #[test]
    fn test_1s() {
        assert_eq!(Yahtzee::ones(1,2,3,4,5), 1);
        assert_eq!(2, Yahtzee::ones(1,2,1,4,5));
        assert_eq!(0, Yahtzee::ones(6,2,2,4,5));
        assert_eq!(4, Yahtzee::ones(1,2,1,1,1));
    }

    #[test]
    fn test_2s() {
        assert_eq!(4, Yahtzee::twos(1,2,3,2,6));
        assert_eq!(10, Yahtzee::twos(2,2,2,2,2));
    }

    #[test]
    fn test_threes() {
        assert_eq!(6, Yahtzee::threes(1,2,3,2,3));
        assert_eq!(12, Yahtzee::threes(2,3,3,3,3));
    }

    #[test]
    fn fours_test() {
        assert_eq!(12, Yahtzee{d1: 4, d2: 4, d3: 4, d4: 5, d5: 5}.fours());
        assert_eq!(8, Yahtzee{d1: 4, d2: 4, d3: 5, d4: 5, d5: 5}.fours());
        assert_eq!(4, Yahtzee{d1: 4, d2: 5, d3: 5, d4: 5, d5: 5}.fours());
    }

    #[test]
    fn fives() {
        assert_eq!(10, Yahtzee { d1: 4, d2: 4, d3: 4, d4: 5, d5: 5 }.fives());
        assert_eq!(15, Yahtzee { d1: 4, d2: 4, d3: 5, d4: 5, d5: 5 }.fives());
        assert_eq!(20, Yahtzee { d1: 4, d2: 5, d3: 5, d4: 5, d5: 5 }.fives());
    }

    #[test]
    fn sixes_test() {
        assert_eq!(0, Yahtzee { d1: 4, d2: 4, d3: 4, d4: 5, d5: 5 }.sixes());
        assert_eq!(6, Yahtzee { d1: 4, d2: 4, d3: 6, d4: 5, d5: 5 }.sixes());
        assert_eq!(18, Yahtzee { d1: 6, d2: 5, d3: 6, d4: 6, d5: 5 }.sixes());
    }

    #[test]
    fn one_pair() {
        assert_eq!(6, Yahtzee::scorepair(3,4,3,5,6));
        assert_eq!(10, Yahtzee::scorepair(5,3,3,3,5));
        assert_eq!(12, Yahtzee::scorepair(5,3,6,6,5));
    }

    #[test]
    fn two_pair() {
        assert_eq!(16, Yahtzee::twopair(3,3,5,4,5));
        assert_eq!(0, Yahtzee::twopair(3,3,5,5,5));
    }

    #[test]
    fn three_of_a_kind() {
        assert_eq!(9, Yahtzee::threeofakind(3,3,3,4,5));
        assert_eq!(15, Yahtzee::threeofakind(5,3,5,4,5));
        assert_eq!(0, Yahtzee::threeofakind(3,3,3,3,5));
    }

    #[test]
    fn four_of_a_kind() {
        assert_eq!(12, Yahtzee::fourofakind(3,3,3,3,5));
        assert_eq!(20, Yahtzee::fourofakind(5,5,5,4,5));
        assert_eq!(0, Yahtzee::fourofakind(3,3,3,3,3));
    }
}
