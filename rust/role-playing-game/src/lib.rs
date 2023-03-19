pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            Some(
                Player {
                    health: 100,
                    mana: if self.level >= 10 {Some(100)} else {None},
                    level: self.level,
                }
            )
        }
        else {None}
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana.is_none() {
            if self.health > mana_cost{
            self.health -= mana_cost;}
            else { self.health = 0;}
            0
        }
        else if self.mana.unwrap() < mana_cost {
            0
        }
        else if self.level >= 10 {
            self.mana = Some(self.mana.unwrap() - mana_cost);
            mana_cost * 2
        }
        else {
            0
        }
    }
}
