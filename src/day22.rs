use std::collections::{HashMap, VecDeque};

#[derive(Copy, Clone)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge
}

const ALL_SPELLS: &[Spell] = &[
    Spell::MagicMissile,
    Spell::Drain,
    Spell::Shield,
    Spell::Poison,
    Spell::Recharge,
];

impl Spell {
    fn cost(&self) -> i32 {
        match *self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
}

/// Stores the total state of the game
#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    hp: i32,
    mana: i32,
    mana_spent: i32,
    boss_hp: i32,
    boss_damage: i32,
    shield_timer: i32,
    poison_timer: i32,
    recharge_timer: i32,
}

impl State {
    fn new(hp: i32, mana: i32, boss_hp: i32, boss_damage: i32) -> Self {
        Self {
            hp,
            mana,
            mana_spent: 0,
            boss_hp,
            boss_damage,
            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,
        }
    }

    fn is_over(&self) -> bool {
        self.boss_hp <= 0 || self.hp <= 0
    }

    fn tick_effects(&mut self) {
        if self.shield_timer > 0 {
            self.shield_timer -= 1;
        }

        if self.poison_timer > 0 {
            self.boss_hp -= 3;
            self.poison_timer -= 1;
        }

        if self.recharge_timer > 0 {
            self.mana += 101;
            self.recharge_timer -= 1;
        }
    }

    fn tick_player(&mut self, spell: Spell) {
        self.tick_effects();

        if self.is_over() {
            return;
        }

        self.mana -= spell.cost();
        self.mana_spent += spell.cost();
        
        match spell {
            Spell::MagicMissile => {
                self.boss_hp -= 4;
            },
            Spell::Drain => {
                self.boss_hp -= 2;
                self.hp += 2;
            },
            Spell::Shield => {
                self.shield_timer = 6;
            },
            Spell::Poison => {
                self.poison_timer = 6;
            },
            Spell::Recharge => {
                self.recharge_timer = 5;
            }
        }
    }

    fn tick_boss(&mut self) {
        self.tick_effects();

        if self.is_over() {
            return;
        }

        let armor = if self.shield_timer > 0 { 7 } else { 0 };
        let boss_eff_dmg = (self.boss_damage - armor).max(1);
        self.hp -= boss_eff_dmg;
    }

    fn step(&self, spell: Spell) -> Self {
        let mut ret = self.clone();
        ret.tick_player(spell);

        if ret.is_over() {
            return ret;
        }

        ret.tick_boss();
        ret
    }

    fn can_cast(&self, spell: Spell) -> bool {
        // Apply effects that would happen before we cast `spell`
        let mut tmp = self.clone();
        tmp.tick_effects();

        if tmp.mana < spell.cost() {
            return false;
        }

        match spell {
            Spell::Shield => tmp.shield_timer <= 0,
            Spell::Poison => tmp.poison_timer <= 0,
            Spell::Recharge => tmp.recharge_timer <= 0,
            _ => true,
        }
    }
}

pub fn run() {
    let init = State::new(50, 500, 58, 9);

    let mut dists = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back((0, init));

    while !q.is_empty() {
        let (dist, cur) = q.pop_front().unwrap();
        let old_dist = *dists.entry(cur.clone()).or_insert(dist);
        if dist < old_dist {
            dists.insert(cur.clone(), dist);
        }

        for &spell in ALL_SPELLS {
            if cur.can_cast(spell) {
                let next = cur.step(spell);
            }
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn execute_all(state: State, actions: &[Spell]) -> State {
        let mut my_state = state;

        for (i, &spell) in actions.iter().enumerate() {
            my_state = my_state.step(spell);

            if i < actions.len() - 1 {
                assert!(!my_state.is_over())
            } else {
                assert!(my_state.is_over())
            }
        }

        my_state
    }
    
    #[test]
    fn example_one() {
        let state = State::new(10, 250, 13, 8);
        let actions = [Spell::Poison, Spell::MagicMissile];
        let final_state = execute_all(state, &actions);

        assert_eq!(final_state.hp, 2);
        assert_eq!(final_state.mana, 24);
        assert!(final_state.boss_hp <= 0);
    }

    #[test]
    fn example_two() {
        let state = State::new(10, 250, 14, 8);
        let actions = [Spell::Recharge, Spell::Shield, Spell::Drain, Spell::Poison, Spell::MagicMissile];
        let final_state = execute_all(state, &actions);
        assert_eq!(final_state.hp, 1);
        assert_eq!(final_state.mana, 114);
        assert!(final_state.boss_hp <= 0);
    }
}
