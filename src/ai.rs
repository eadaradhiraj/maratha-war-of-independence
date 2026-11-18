use crate::game_state::GameState;
use crate::models::{FactionId, UnitInstance, UnitType};

pub fn execute_ai_turn(game: &mut GameState) {
    // 1. Mughal recruitment
    let mughal_treasury = game
        .factions
        .get(&FactionId::Mughal)
        .map(|f| f.treasury)
        .unwrap_or(0);

    if mughal_treasury > 800 {
        // Find a Mughal province with low garrison
        let prov_to_reinforce = game
            .provinces
            .iter_mut()
            .find(|p| p.owner == FactionId::Mughal && p.garrison.len() < 3);

        if let Some(p) = prov_to_reinforce {
            let id = 50000 + (p.garrison.len() as u64) * 10 + game.turn as u64;
            p.garrison.push(UnitInstance::new(id, UnitType::SepoyLineInfantry));
            if let Some(fac) = game.factions.get_mut(&FactionId::Mughal) {
                fac.treasury -= 650;
            }
        }
    }

    // 2. Mughal army movement & offensive actions
    let mut pending_moves = Vec::new();
    for (idx, army) in game.armies.iter().enumerate() {
        if army.owner == FactionId::Mughal && !army.has_moved {
            let current_prov_id = army.province_id;
            let current_prov = &game.provinces[current_prov_id];

            // Evaluate adjacent Maratha targets with strategic prudence
            let mut target_maratha = None;
            for &n in &current_prov.neighbors {
                let target_p = &game.provinces[n];
                if target_p.owner == FactionId::Maratha {
                    let target_garrison_power: f32 = target_p.garrison.iter().map(|u| u.combat_power()).sum();
                    let target_army_power: f32 = game
                        .armies
                        .iter()
                        .filter(|a| a.province_id == n && a.owner == FactionId::Maratha)
                        .map(|a| a.total_combat_power())
                        .sum();
                    let total_def_power = target_garrison_power + target_army_power;
                    let my_power = army.total_combat_power();

                    // Only attack if army is in solid condition (> 180 men)
                    // and possesses sufficient tactical strength advantage (> 1.05x total defender power)
                    if army.total_soldiers() >= 180 && my_power >= total_def_power * 1.05 {
                        target_maratha = Some(n);
                        break;
                    }
                }
            }

            if let Some(target_idx) = target_maratha {
                pending_moves.push((idx, target_idx));
            } else {
                // Otherwise move closer to the front line (Ahmadnagar or Berar)
                let front_prov = current_prov
                    .neighbors
                    .iter()
                    .find(|&&n| n == 1 || n == 3 || n == 10);
                if let Some(&f_idx) = front_prov {
                    if game.provinces[f_idx].owner == FactionId::Mughal {
                        pending_moves.push((idx, f_idx));
                    }
                }
            }
        }
    }

    for (army_idx, target_prov_idx) in pending_moves {
        if army_idx < game.armies.len() {
            game.move_army(army_idx, target_prov_idx);
        }
    }

    // 3. Independent & Mysore passive growth
    for p in game.provinces.iter_mut() {
        if (p.owner == FactionId::Mysore || p.owner == FactionId::Independent)
            && p.garrison.len() < 2
            && game.turn % 2 == 0
        {
            let id = 60000 + p.id as u64 * 100 + game.turn as u64;
            p.garrison.push(UnitInstance::new(id, UnitType::ZamindariLevies));
        }
    }
}
