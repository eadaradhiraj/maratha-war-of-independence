use crate::models::{Army, BattlePhaseLog, BattleReport, FactionId, UnitInstance};

pub fn resolve_battle(
    attacker_faction: FactionId,
    defender_faction: FactionId,
    province_id: usize,
    province_name: &str,
    attacker_army: &mut Army,
    defender_units: &mut Vec<UnitInstance>,
    defender_general_stars: u8,
) -> BattleReport {
    let attacker_initial = attacker_army.total_soldiers();
    let defender_initial: u32 = defender_units.iter().map(|u| u.soldiers).sum();

    let mut phases = Vec::new();

    // 1. Skirmish & Artillery
    let attacker_artillery: u32 = attacker_army
        .units
        .iter()
        .filter(|u| u.unit_type.stats().is_artillery)
        .map(|u| u.soldiers)
        .sum();
    let defender_artillery: u32 = defender_units
        .iter()
        .filter(|u| u.unit_type.stats().is_artillery)
        .map(|u| u.soldiers)
        .sum();

    let def_skirmish_loss = (attacker_artillery as f32 * 2.2 + 25.0 * (1.0 + attacker_army.general_stars as f32 * 0.2)) as u32;
    let atk_skirmish_loss = (defender_artillery as f32 * 2.2 + 20.0 * (1.0 + defender_general_stars as f32 * 0.2)) as u32;

    phases.push(BattlePhaseLog {
        title: "Phase 1: Artillery & Long-Range Skirmish".into(),
        attacker_action: if attacker_artillery > 0 {
            "Topchi bronze cannons unleash round shot into the defending battle lines.".into()
        } else {
            "Skirmishers advance through brush, testing the enemy outposts.".into()
        },
        defender_action: if defender_artillery > 0 {
            "Fortress batteries and defending guns respond with heavy canister fire.".into()
        } else {
            "Defenders dig in behind palisades and return sporadic musket fire.".into()
        },
        attacker_losses: atk_skirmish_loss.min(attacker_initial / 4),
        defender_losses: def_skirmish_loss.min(defender_initial / 4),
    });

    // 2. Ranged Musket Volley
    let atk_firepower: f32 = attacker_army
        .units
        .iter()
        .map(|u| u.unit_type.stats().firepower as f32 * (u.soldiers as f32 / 10.0))
        .sum();
    let def_firepower: f32 = defender_units
        .iter()
        .map(|u| u.unit_type.stats().firepower as f32 * (u.soldiers as f32 / 10.0))
        .sum();

    let atk_volley_loss = (def_firepower * 0.45 * (1.0 + defender_general_stars as f32 * 0.05)) as u32;
    let def_volley_loss = (atk_firepower * 0.48 * (1.0 + attacker_army.general_stars as f32 * 0.08)) as u32;

    phases.push(BattlePhaseLog {
        title: "Phase 2: Ranged Musket Volley Clash".into(),
        attacker_action: "Line infantry form disciplined ranks and deliver rolling volleys of lead.".into(),
        defender_action: "Defenders fire concentrated platoon volleys into the advancing smoke.".into(),
        attacker_losses: atk_volley_loss.min(attacker_initial / 3),
        defender_losses: def_volley_loss.min(defender_initial / 3),
    });

    // 3. Cavalry Charge & Melee
    let atk_cav: f32 = attacker_army
        .units
        .iter()
        .filter(|u| u.unit_type.stats().is_cavalry)
        .map(|u| u.soldiers as f32 * 1.8)
        .sum();
    let def_cav: f32 = defender_units
        .iter()
        .filter(|u| u.unit_type.stats().is_cavalry)
        .map(|u| u.soldiers as f32 * 1.5)
        .sum();

    let atk_charge_loss = (def_cav * 0.6 + 40.0) as u32;
    let def_charge_loss = (atk_cav * 0.75 + 50.0) as u32;

    phases.push(BattlePhaseLog {
        title: "Phase 3: Cavalry Flank Charge & Hand-to-Hand Melee".into(),
        attacker_action: if atk_cav > 0.0 {
            "Maratha Bargir and Silhadar horsemen surge around the flanks with drawn talwars!".into()
        } else {
            "Infantry fix bayonets and storm the breaches with spirited shouts!".into()
        },
        defender_action: if def_cav > 0.0 {
            "Defending sowars counter-charge across the plain in a desperate clash of steel.".into()
        } else {
            "Garrison falls back to the inner redoubt, fighting stubbornly hand-to-hand.".into()
        },
        attacker_losses: atk_charge_loss.min(attacker_initial / 3),
        defender_losses: def_charge_loss.min(defender_initial / 3),
    });

    // Total power comparison
    let atk_power = attacker_army.total_combat_power();
    // Defender receives home ground and defensive fortification advantage (+20%)
    let def_power: f32 = defender_units.iter().map(|u| u.combat_power()).sum::<f32>()
        * 1.20
        * (1.0 + defender_general_stars as f32 * 0.10);

    let attacker_won = atk_power > def_power;

    let total_atk_losses: u32 = phases.iter().map(|p| p.attacker_losses).sum();
    let total_def_losses: u32 = phases.iter().map(|p| p.defender_losses).sum();

    let final_atk_loss = total_atk_losses.min(attacker_initial.saturating_sub(10));
    let final_def_loss = if attacker_won {
        // Defender routed: takes phase casualties + rout loss (25%), but retains survivors if not surrounded
        let rout_loss = total_def_losses + (defender_initial as f32 * 0.25) as u32;
        rout_loss.min(defender_initial)
    } else {
        total_def_losses.min(defender_initial.saturating_sub(15))
    };

    // Apply losses to attacker army
    distribute_losses(&mut attacker_army.units, final_atk_loss);

    // Apply losses to defender
    distribute_losses(defender_units, final_def_loss);

    BattleReport {
        attacker_faction,
        defender_faction,
        province_id,
        province_name: province_name.to_string(),
        attacker_initial,
        defender_initial,
        attacker_losses: final_atk_loss,
        defender_losses: final_def_loss,
        attacker_won,
        phases,
    }
}

fn distribute_losses(units: &mut Vec<UnitInstance>, mut total_loss: u32) {
    let count = units.len();
    if count == 0 {
        return;
    }

    let loss_per_unit = total_loss / count as u32;
    for u in units.iter_mut() {
        let applied = loss_per_unit.min(u.soldiers.saturating_sub(5));
        u.soldiers = u.soldiers.saturating_sub(applied);
        total_loss = total_loss.saturating_sub(applied);
    }

    // Residual losses
    for u in units.iter_mut() {
        if total_loss == 0 {
            break;
        }
        let applied = total_loss.min(u.soldiers.saturating_sub(1));
        u.soldiers = u.soldiers.saturating_sub(applied);
        total_loss = total_loss.saturating_sub(applied);
    }

    // Retain only surviving units
    units.retain(|u| u.soldiers > 0);
}
