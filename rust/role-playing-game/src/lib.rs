// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match (self.health, self.level) {
            (0, 0..=9) => Some(Player {
                health: 100,
                mana: None,
                level: self.level,
            }),
            (0, _) => Some(Player {
                health: 100,
                mana: Some(100),
                level: self.level,
            }),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(x) = &mut self.mana {
            if *x >= mana_cost {
                *x -= mana_cost;
                return mana_cost * 2;
            } else {
                return 0;
            }
        } else {
            self.health = self.health.saturating_sub(mana_cost);
            return 0;
        }
    }
}
