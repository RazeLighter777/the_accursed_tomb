use rand::{self, Rng};

pub struct Dice {
    sides : u8,
    roll : u32
}

impl Dice {
    pub fn new(sides : u8) -> Dice {
        Dice {
            sides : sides,
            roll : rand::thread_rng().gen_range(1..sides as u32)
        }
    }
    pub fn get_roll(&self) -> u32 {
        self.roll
    }
    pub fn get_side_count(&self) -> u8 {
        self.sides
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dice_test() {
        let dice = Dice::new(6);
        assert!(dice.get_roll() > 0);
        assert!(dice.get_roll() < 7);
    }
}