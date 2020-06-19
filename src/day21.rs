#[derive(Copy, Clone)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

const EMPTY: Item = Item { cost: 0, damage: 0, armor: 0 };

const WEAPONS: [Item; 5] = [
    Item { cost: 8, damage: 4, armor: 0 },
    Item { cost: 10, damage: 5, armor: 0 },
    Item { cost: 25, damage: 6, armor: 0 },
    Item { cost: 40, damage: 7, armor: 0 },
    Item { cost: 74, damage: 8, armor: 0 },
];

const ARMORS: [Item; 6] = [
    EMPTY,
    Item { cost: 13, damage: 0, armor: 1 },
    Item { cost: 31, damage: 0, armor: 2 },
    Item { cost: 53, damage: 0, armor: 3 },
    Item { cost: 75, damage: 0, armor: 4 },
    Item { cost: 102, damage: 0, armor: 5 },
];

const RINGS: [Item; 6] = [
    Item { cost: 25, damage: 1, armor: 0 },
    Item { cost: 50, damage: 2, armor: 0 },
    Item { cost: 100, damage: 3, armor: 0 },
    Item { cost: 20, damage: 0, armor: 1 },
    Item { cost: 40, damage: 0, armor: 2 },
    Item { cost: 80, damage: 0, armor: 3 },
];

fn wins(player_hp: i32, player_damage: i32, player_armor: i32, boss_hp: i32, boss_damage: i32, boss_armor: i32) -> bool {
    let mut player_hp = player_hp;
    let mut boss_hp = boss_hp;

    let player_eff_dmg = (player_damage - boss_armor).max(1);
    let boss_eff_dmg = (boss_damage - player_armor).max(1);
    
    loop {
        boss_hp -= player_eff_dmg;
        if boss_hp <= 0 {
            return true;
        }

        player_hp -= boss_eff_dmg;
        if player_hp <= 0 {
            return false;
        }
    }
}

pub fn run() {
    // Generate the options for rings first
    let ring_opts = {
        let mut tmp = Vec::new();
        // no ring
        tmp.push((EMPTY, EMPTY)); 
        // one ring
        for &i in &RINGS {
            tmp.push((i, EMPTY));
        }
        // pairs of rings
        for (i, &ring_a) in RINGS.iter().enumerate() {
            for &ring_b in RINGS.iter().skip(i) {
                tmp.push((ring_a, ring_b));
            }
        }
        tmp
    };
    let mut min_cost = i32::MAX;
    let mut max_cost = i32::MIN;

    for armor in &ARMORS {
        for weapon in &WEAPONS {
            for (ring_a, ring_b) in &ring_opts {
                let cost = weapon.cost + armor.cost + ring_a.cost + ring_b.cost;
                let damage = weapon.damage + armor.damage + ring_a.damage + ring_b.damage;
                let armor = weapon.armor + armor.armor + ring_a.armor + ring_b.armor;

                if wins(100, damage, armor, 100, 8, 2) {
                    min_cost = min_cost.min(cost);
                } else {
                    max_cost = max_cost.max(cost);
                }
            }
        }
    }
    
    println!("Part 1: {}", min_cost);
    println!("Part 2: {}", max_cost);
}

#[cfg(test)]
mod test {
    use super::wins;
    
    #[test]
    fn test_wins() {
        assert!(wins(8, 5, 5, 12, 7, 2));
    }
}
