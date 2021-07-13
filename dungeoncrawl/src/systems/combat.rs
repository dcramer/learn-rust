use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Player)]
#[write_component(Health)]
#[read_component(Damage)]
#[read_component(Carried)]
#[read_component(ScoreValue)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut total_score_change: u32 = 0;
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity, Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect();
    victims.iter().for_each(|(message, attacker, victim)| {
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();
        let base_damage = if let Ok(v) = ecs.entry_ref(*attacker) {
            if let Ok(dmg) = v.get_component::<Damage>() {
                dmg.0
            } else {
                0
            }
        } else {
            0
        };
        let weapon_damage: i32 = <(&Carried, &Damage)>::query()
            .iter(ecs)
            .filter(|(carried, _)| carried.0 == *attacker)
            .map(|(_, dmg)| dmg.0)
            .sum();
        let final_damage = base_damage + weapon_damage;

        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= final_damage;
            if health.current < 1 && !is_player {
                if let Ok(score_value) = ecs
                    .entry_ref(*victim)
                    .unwrap()
                    .get_component::<ScoreValue>()
                {
                    total_score_change += score_value.0;
                }

                commands.remove(*victim);
            }
        }

        commands.remove(*message);
    });

    if total_score_change != 0 {
        let mut player = <&mut Player>::query().iter_mut(ecs).nth(0).unwrap();
        player.score += total_score_change
    }
}
