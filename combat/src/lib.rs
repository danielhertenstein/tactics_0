use game_state::CombatStatistics;

pub fn attack(combat_statistics: &mut Vec<CombatStatistics>, attacker: usize, defender: usize) {
    let damage = combat_statistics[attacker].strength - combat_statistics[defender].constitution;
    if combat_statistics[defender].health - damage < 0 {
        combat_statistics[defender].health = 0;
    } else {
        combat_statistics[defender].health -= damage;
    }
    println!("You dealt {} damage.", damage);
}

pub fn check_if_anyone_died(combat_statistics: &Vec<CombatStatistics>) -> Vec<usize>{
    combat_statistics
        .iter()
        .enumerate()
        .filter(|(_index, stats)| stats.health == 0)
        .map(|(index, _stats)| index)
        .collect()

}
