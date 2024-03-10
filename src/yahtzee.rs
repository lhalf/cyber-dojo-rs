#![allow(dead_code)]

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

struct Yahtzee {
    d1: u8,
    d2: u8,
    d3: u8,
    d4: u8,
    d5: u8
}

impl Yahtzee {
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
}

#[cfg(test)]
mod tests {
    use super::chance;
    use super::yahtzee;
    use super::ones;
    use super::twos;
    use super::threes;
    use super::Yahtzee;

    #[test]
    fn chance_scores_sum_of_all_dice() {
        let expected = 15;
        let actual = chance(2,3,4,5,1);
        assert_eq!(expected, actual);
        assert_eq!(16, chance(3,3,4,5,1));
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
        let actual = yahtzee(&ints(4,4,4,4,4));
        assert_eq!(expected, actual);
        assert_eq!(50, yahtzee(&ints(6,6,6,6,6)));
        assert_eq!(0, yahtzee(&ints(6,6,6,6,3)));
    }

    #[test]
    fn test_1s() {
        assert_eq!(ones(1,2,3,4,5), 1);
        assert_eq!(2, ones(1,2,1,4,5));
        assert_eq!(0, ones(6,2,2,4,5));
        assert_eq!(4, ones(1,2,1,1,1));
    }

    #[test]
    fn test_2s() {
        assert_eq!(4, twos(1,2,3,2,6));
        assert_eq!(10, twos(2,2,2,2,2));
    }

    #[test]
    fn test_threes() {
        assert_eq!(6, threes(1,2,3,2,3));
        assert_eq!(12, threes(2,3,3,3,3));
    }

    #[test]
    fn fours_test() {
        assert_eq!(12, Yahtzee{d1: 4, d2: 4, d3: 4, d4: 5, d5: 5}.fours());
        assert_eq!(8, Yahtzee{d1: 4, d2: 4, d3: 5, d4: 5, d5: 5}.fours());
        assert_eq!(4, Yahtzee{d1: 4, d2: 5, d3: 5, d4: 5, d5: 5}.fours());
    }
}
