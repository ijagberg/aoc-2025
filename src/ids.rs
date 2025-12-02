use std::str::FromStr;

pub struct IdRange {
    pub from: u64,
    pub to: u64,
}

impl IdRange {
    pub fn new(from: u64, to: u64) -> Self {
        Self { from, to }
    }
}

impl FromStr for IdRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once("-").ok_or(())?;
        let from = u64::from_str(&from.trim()).map_err(|_| ())?;
        let to = u64::from_str(&to.trim()).map_err(|_| ())?;
        Ok(Self::new(from, to))
    }
}
