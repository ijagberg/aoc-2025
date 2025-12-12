use std::str::FromStr;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Rotation(pub Direction, pub u64);

impl FromStr for Rotation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = &s[0..1];
        let dist = &s[1..];
        let dir = Direction::from_str(dir)?;
        let dist = u64::from_str(dist).map_err(|_| ())?;
        Ok(Self(dir, dist))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dial(u64);

impl Dial {
    pub fn new(pos: u64) -> Result<Self, &'static str> {
        if pos >= 100 {
            Err("pos is too large")
        } else {
            Ok(Self(pos))
        }
    }

    pub fn pos(&self) -> u64 {
        self.0
    }

    pub fn turn_right(&mut self, mut steps: u64) {
        steps %= 100;
        let mut new_pos = (self.0 + steps) % 100;
        self.0 = new_pos;
    }

    pub fn turn_left(&mut self, mut steps: u64) {
        steps %= 100;
        if steps > self.0 {
            // Gonna wrap around 0
            steps -= self.0;
            self.0 = 100 - steps;
        } else {
            self.0 -= steps;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dial_turn() {
        let mut dial = Dial::new(50).unwrap();
        dial.turn_left(51);
        assert_eq!(dial.0, 99);
        dial.turn_right(295);
        assert_eq!(dial.0, 94);
    }
}
