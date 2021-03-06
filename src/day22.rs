use std::collections:: VecDeque;

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
#[derive(Copy, Clone)]
struct State {
    hp: i32,
    mana: i32,
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

    /// Returns mana spent this turn, if any
    fn tick_player(&mut self, spell: Spell, p2: bool) -> i32 {
        if p2 {
            self.hp -= 1;
            if self.is_over() {
                return 0;
            }
        }

        self.tick_effects();

        if self.is_over() {
            return 0;
        }

        self.mana -= spell.cost();
        
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

        spell.cost()
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

    fn can_cast(&self, spell: Spell) -> bool {
        // Apply effects that would happen before we cast `spell`, since these affect available mana
        let mut tmp = *self;
        tmp.tick_effects();

        // Check mana
        if tmp.mana < spell.cost() {
            return false;
        }

        // Check timers
        match spell {
            Spell::Shield => tmp.shield_timer <= 0,
            Spell::Poison => tmp.poison_timer <= 0,
            Spell::Recharge => tmp.recharge_timer <= 0,
            _ => true,
        }
    }
}

fn step((mana_used, state): (i32, State), spell: Spell, p2: bool) -> (i32, State) {
    let mut new_state = state;
    let mut new_mana_used = mana_used;
    new_mana_used += new_state.tick_player(spell, p2);

    if !new_state.is_over() {
        new_state.tick_boss();
    }

    (new_mana_used, new_state)
}

fn compute(p2: bool) -> i32 {
    let init = State::new(50, 500, 58, 9);

    let mut best_winning_cost = i32::MAX;
    let mut q = VecDeque::new();
    q.push_back((0, init));

    while !q.is_empty() {
        let (mana_used, state) = q.pop_front().unwrap();

        // Invariant: the queue should only contain unfinished games
        assert!(!state.is_over());

        // If our current running mana usage is greater than a best case we know already, stop. We won't be able to do better no matter what.
        if mana_used >= best_winning_cost {
            continue;
        }

        // Try next move
        for &spell in ALL_SPELLS {
            if state.can_cast(spell) {
                let (new_mana_used, new_state) = step((mana_used, state), spell, p2);
                if new_state.is_over() {
                    // If player won, record the best mana cost seen so far
                    if new_state.hp > 0 {
                        best_winning_cost = best_winning_cost.min(new_mana_used);
                    }
                } else {
                    // Otherwise, keep exploring
                    q.push_back((new_mana_used, new_state));
                }
            }
        }
    }

    best_winning_cost
}

pub fn run() {
    println!("Part 1: {}", compute(false));
    println!("Part 2: {}", compute(true));
}

#[cfg(test)]
mod test {
    use super::*;

    fn execute_all(state: State, actions: &[Spell]) -> (i32, State) {
        let mut cur = (0, state);

        for (i, &spell) in actions.iter().enumerate() {
            cur = step(cur, spell, false);

            if i < actions.len() - 1 {
                assert!(!cur.1.is_over())
            } else {
                assert!(cur.1.is_over())
            }
        }

        cur
    }
    
    #[test]
    fn example_one() {
        let state = State::new(10, 250, 13, 8);
        let actions = [Spell::Poison, Spell::MagicMissile];
        let (mana_used, final_state) = execute_all(state, &actions);

        assert_eq!(mana_used, actions.iter().map(Spell::cost).sum::<i32>());
        assert_eq!(final_state.hp, 2);
        assert_eq!(final_state.mana, 24);
        assert!(final_state.boss_hp <= 0);
    }

    #[test]
    fn example_two() {
        let state = State::new(10, 250, 14, 8);
        let actions = [Spell::Recharge, Spell::Shield, Spell::Drain, Spell::Poison, Spell::MagicMissile];
        let (mana_used, final_state) = execute_all(state, &actions);

        assert_eq!(mana_used, actions.iter().map(Spell::cost).sum::<i32>());
        assert_eq!(final_state.hp, 1);
        assert_eq!(final_state.mana, 114);
        assert!(final_state.boss_hp <= 0);
    }
}
