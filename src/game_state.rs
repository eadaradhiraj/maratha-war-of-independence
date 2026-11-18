use std::collections::HashMap;
use crate::geo_data::create_default_provinces;
use crate::models::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    MainMenu,
    FactionSelect,
    Map,
    Government,
    Technology,
    Diplomacy,
    BattleReport,
    LoadGameMenu,
    SaveGameMenu,
    HowToPlay,
}

pub struct GameState {
    pub turn: u32,
    pub year: u32,
    pub season: Season,
    pub player_faction: FactionId,
    pub factions: HashMap<FactionId, FactionData>,
    pub provinces: Vec<Province>,
    pub armies: Vec<Army>,
    pub selected_province: Option<usize>,
    pub selected_army: Option<usize>,
    pub active_tab: ActiveTab,
    pub latest_battle: Option<BattleReport>,
    pub notifications: Vec<Notification>,
    pub available_techs: Vec<TechNode>,
    pub victory_claimed: bool,
}

#[derive(Debug, Clone)]
pub struct FactionData {
    pub id: FactionId,
    pub name: String,
    pub leader_name: String,
    pub treasury: i32,
    pub ministers: Vec<Minister>,
    pub researched_techs: Vec<String>,
    pub current_research: Option<(String, u32)>, // (tech_id, turns_left)
    pub wars: Vec<FactionId>,
    pub trade_partners: Vec<FactionId>,
}

impl GameState {
    pub fn new() -> Self {
        let mut factions = HashMap::new();

        // 1. Maratha Confederacy (Player)
        let maratha_ministers = vec![
            Minister {
                role: MinisterRole::HeadOfState,
                name: "Maharani Tarabai I".into(),
                stars: 5,
                loyalty: 10,
                trait_desc: "Iron Regent: Inspires troops & +15% Morale in all battles".into(),
            },
            Minister {
                role: MinisterRole::PeshwaGrandVizier,
                name: "Ramchandra Pant Amatya".into(),
                stars: 5,
                loyalty: 9,
                trait_desc: "Master Administrator: +10% Province Wealth Growth".into(),
            },
            Minister {
                role: MinisterRole::Treasury,
                name: "Shankraji Narayan".into(),
                stars: 4,
                loyalty: 8,
                trait_desc: "Frugal Steward: -10% Army Upkeep Costs".into(),
            },
            Minister {
                role: MinisterRole::War,
                name: "Dhanaji Jadhav".into(),
                stars: 5,
                loyalty: 9,
                trait_desc: "Master of Ganimi Kava (Guerrilla Warfare): +2 Army Speed".into(),
            },
            Minister {
                role: MinisterRole::Justice,
                name: "Honaji Pant".into(),
                stars: 4,
                loyalty: 8,
                trait_desc: "Righteous Judge: +2 Public Order across all provinces".into(),
            },
        ];

        factions.insert(
            FactionId::Maratha,
            FactionData {
                id: FactionId::Maratha,
                name: "Maratha Confederacy".into(),
                leader_name: "Maharani Tarabai I".into(),
                treasury: 6500, // Exact historical ETW 1700 start!
                ministers: maratha_ministers,
                researched_techs: vec!["flintlock_musket".into()],
                current_research: Some(("ring_bayonet".into(), 3)),
                wars: vec![FactionId::Mughal],
                trade_partners: vec![FactionId::Portugal],
            },
        );

        // 2. Mughal Empire
        let mughal_ministers = vec![
            Minister {
                role: MinisterRole::HeadOfState,
                name: "Emperor Aurangzeb (Alamgir I)".into(),
                stars: 4,
                loyalty: 10,
                trait_desc: "Aging Conqueror: Immense treasury, but high religious friction".into(),
            },
            Minister {
                role: MinisterRole::PeshwaGrandVizier,
                name: "Asad Khan".into(),
                stars: 4,
                loyalty: 8,
                trait_desc: "Imperial Grand Vizier: Strong tax collection".into(),
            },
            Minister {
                role: MinisterRole::War,
                name: "Zulfiqar Khan Nusrat Jung".into(),
                stars: 4,
                loyalty: 7,
                trait_desc: "Deccan Veteran: Experienced commander of heavy siege guns".into(),
            },
        ];

        factions.insert(
            FactionId::Mughal,
            FactionData {
                id: FactionId::Mughal,
                name: "Mughal Empire".into(),
                leader_name: "Emperor Aurangzeb".into(),
                treasury: 12000,
                ministers: mughal_ministers,
                researched_techs: vec!["flintlock_musket".into(), "heavy_foundry".into()],
                current_research: Some(("carbine_drills".into(), 4)),
                wars: vec![FactionId::Maratha],
                trade_partners: vec![],
            },
        );

        // 3. Mysore
        factions.insert(
            FactionId::Mysore,
            FactionData {
                id: FactionId::Mysore,
                name: "Sultanate of Mysore".into(),
                leader_name: "Chikka Devaraja Wodeyar".into(),
                treasury: 3500,
                ministers: vec![],
                researched_techs: vec!["flintlock_musket".into()],
                current_research: None,
                wars: vec![],
                trade_partners: vec![FactionId::Maratha],
            },
        );

        // 4. Portugal
        factions.insert(
            FactionId::Portugal,
            FactionData {
                id: FactionId::Portugal,
                name: "Kingdom of Portugal".into(),
                leader_name: "Viceroy of the Indies".into(),
                treasury: 4000,
                ministers: vec![],
                researched_techs: vec!["ring_bayonet".into()],
                current_research: None,
                wars: vec![],
                trade_partners: vec![FactionId::Maratha],
            },
        );

        // 5. Independent
        factions.insert(
            FactionId::Independent,
            FactionData {
                id: FactionId::Independent,
                name: "Independent States".into(),
                leader_name: "Regional Nawabs".into(),
                treasury: 2000,
                ministers: vec![],
                researched_techs: vec![],
                current_research: None,
                wars: vec![],
                trade_partners: vec![],
            },
        );

        let provinces = create_default_provinces();

        // Initial Armies
        let mut armies = Vec::new();
        // Maratha Main Field Army in Bijapur (Satara)
        armies.push(Army {
            id: 1,
            owner: FactionId::Maratha,
            province_id: 4, // Bijapur
            units: vec![
                UnitInstance::new(1001, UnitType::BargirCavalry),
                UnitInstance::new(1002, UnitType::SilhadarLancers),
                UnitInstance::new(1003, UnitType::HinduMatchlocks),
                UnitInstance::new(1004, UnitType::SepoyLineInfantry),
                UnitInstance::new(1005, UnitType::TopchiCannons),
            ],
            general_name: "Senapati Dhanaji Jadhav".into(),
            general_stars: 4,
            has_moved: false,
        });

        // Maratha Carnatica Army in Arcot
        armies.push(Army {
            id: 2,
            owner: FactionId::Maratha,
            province_id: 5, // Carnatica
            units: vec![
                UnitInstance::new(1006, UnitType::BargirCavalry),
                UnitInstance::new(1007, UnitType::HinduMatchlocks),
                UnitInstance::new(1008, UnitType::PindariRaiders),
            ],
            general_name: "Santaji Ghorpade".into(),
            general_stars: 3,
            has_moved: false,
        });

        // Mughal Southern Army in Ahmadnagar
        armies.push(Army {
            id: 3,
            owner: FactionId::Mughal,
            province_id: 1, // Ahmadnagar
            units: vec![
                UnitInstance::new(2001, UnitType::MansabdarGuards),
                UnitInstance::new(2002, UnitType::SepoyLineInfantry),
                UnitInstance::new(2003, UnitType::ZamindariLevies),
                UnitInstance::new(2004, UnitType::TopchiCannons),
            ],
            general_name: "Zulfiqar Khan".into(),
            general_stars: 3,
            has_moved: false,
        });

        // Mughal Imperial Army in Hindustan (Delhi)
        armies.push(Army {
            id: 4,
            owner: FactionId::Mughal,
            province_id: 9, // Hindustan
            units: vec![
                UnitInstance::new(2005, UnitType::MansabdarGuards),
                UnitInstance::new(2006, UnitType::SepoyLineInfantry),
                UnitInstance::new(2007, UnitType::SepoyLineInfantry),
                UnitInstance::new(2008, UnitType::TopchiCannons),
            ],
            general_name: "Prince Azam Shah".into(),
            general_stars: 4,
            has_moved: false,
        });

        let available_techs = vec![
            TechNode {
                id: "ring_bayonet",
                name: "Ring Bayonet",
                category: TechCategory::Military,
                cost_turns: 3,
                description: "+2 Melee Attack & +2 Defense for all Line Infantry",
                prerequisite: None,
            },
            TechNode {
                id: "canister_shot",
                name: "Canister Shot",
                category: TechCategory::Military,
                cost_turns: 4,
                description: "+30% Artillery Firepower at close range",
                prerequisite: Some("ring_bayonet"),
            },
            TechNode {
                id: "carbine_drills",
                name: "Carbine Drills",
                category: TechCategory::Military,
                cost_turns: 4,
                description: "Light cavalry fire while moving; +2 Bargir Cavalry Attack",
                prerequisite: None,
            },
            TechNode {
                id: "land_survey",
                name: "Cadastral Land Survey",
                category: TechCategory::Industry,
                cost_turns: 3,
                description: "+15% Tax Revenue across all controlled provinces",
                prerequisite: None,
            },
            TechNode {
                id: "weaving_mills",
                name: "Fly Shuttle Weaving Mills",
                category: TechCategory::Industry,
                cost_turns: 5,
                description: "+250 Wealth per turn from trading ports and towns",
                prerequisite: Some("land_survey"),
            },
            TechNode {
                id: "swarajya_charter",
                name: "Hindavi Swarajya Charter",
                category: TechCategory::Philosophy,
                cost_turns: 4,
                description: "+3 Public Order in all Hindu provinces & eliminates religious unrest",
                prerequisite: None,
            },
        ];

        let mut notifications = Vec::new();
        notifications.push(Notification {
            turn: 1,
            year: 1700,
            title: "Summer 1700: The Campaign for India Begins".into(),
            message: "Under Regent Maharani Tarabai I, the Maratha Confederacy rises from Satara. The aging Mughal Emperor Aurangzeb holds Delhi and the Deccan. Liberate the Subcontinent!".into(),
            is_urgent: true,
        });

        Self {
            turn: 1,
            year: 1700,
            season: Season::Summer,
            player_faction: FactionId::Maratha,
            factions,
            provinces,
            armies,
            selected_province: Some(4), // Start with Bijapur selected
            selected_army: Some(0),
            active_tab: ActiveTab::MainMenu,
            latest_battle: None,
            notifications,
            available_techs,
            victory_claimed: false,
        }
    }

    pub fn apply_save_data(&mut self, data: crate::storage::CampaignSaveData) {
        self.turn = data.turn;
        self.year = data.year;
        self.season = data.season;
        self.player_faction = data.player_faction;
        self.victory_claimed = data.victory_claimed;
        self.armies = data.armies;

        // Restore factions
        for fac_save in data.factions {
            if let Some(fac) = self.factions.get_mut(&fac_save.id) {
                fac.treasury = fac_save.treasury;
                fac.researched_techs = fac_save.researched_techs;
                fac.current_research = fac_save.current_research;
                fac.ministers = fac_save.ministers;
                fac.wars = fac_save.wars;
                fac.trade_partners = fac_save.trade_partners;
            }
        }

        // Restore provinces
        for p_save in data.provinces {
            if let Some(p) = self.provinces.get_mut(p_save.id) {
                p.owner = p_save.owner;
                p.tax_level = p_save.tax_level;
                p.public_order = p_save.public_order;
                p.buildings = p_save.buildings;
                p.garrison = p_save.garrison;
                p.recruitment_queue = p_save.recruitment_queue;
                p.is_under_siege = p_save.is_under_siege;
            }
        }

        let capital_prov_idx = match self.player_faction {
            FactionId::Maratha => 4,
            FactionId::Mughal => 9,
            FactionId::Mysore => 13,
            FactionId::Portugal => 7,
            FactionId::Independent => 0,
        };
        self.selected_province = Some(capital_prov_idx);
        self.selected_army = self.armies.iter().position(|a| a.owner == self.player_faction);
        self.active_tab = ActiveTab::Map;
        self.notifications.clear();
        let season_str = match self.season {
            Season::Summer => "Summer",
            Season::Winter => "Winter",
        };
        self.notifications.push(Notification {
            turn: self.turn,
            year: self.year,
            title: format!("Campaign Resumed: {}", data.meta.display_name),
            message: format!(
                "Turn {}, {} {} A.D. Forces of the {} stand ready at your order!",
                self.turn,
                season_str,
                self.year,
                self.player_faction.name()
            ),
            is_urgent: true,
        });
    }

    pub fn start_as_faction(&mut self, faction: FactionId) {
        self.player_faction = faction;
        self.active_tab = ActiveTab::Map;

        let capital_prov_idx = match faction {
            FactionId::Maratha => 4, // Bijapur / Satara
            FactionId::Mughal => 9,  // Hindustan / Delhi
            FactionId::Mysore => 13, // Mysore
            FactionId::Portugal => 7, // Goa
            FactionId::Independent => 0, // Afghanistan / Kabul
        };
        self.selected_province = Some(capital_prov_idx);

        let army_idx = self.armies.iter().position(|a| a.owner == faction);
        self.selected_army = army_idx;

        let leader_name = self.factions.get(&faction).map(|f| f.leader_name.clone()).unwrap_or_default();
        self.notifications.push(Notification {
            turn: self.turn,
            year: self.year,
            title: format!("Summer 1700: The Campaign of the {}", faction.name()),
            message: format!(
                "Under {}, the {} takes the field. Command your realm to dominance across India!",
                leader_name, faction.name()
            ),
            is_urgent: true,
        });
    }

    pub fn calculate_faction_economy(&self, faction_id: FactionId) -> (i32, i32, i32) {
        let mut tax_income = 0;
        let mut trade_income = 0;

        for p in self.provinces.iter().filter(|p| p.owner == faction_id) {
            let mult = p.tax_level.rate_multiplier();
            let base_rev = (p.base_wealth as f32 * 0.4 * mult) as i32;
            tax_income += base_rev;

            for b in &p.buildings {
                match b {
                    BuildingType::Marketplace => tax_income += 220,
                    BuildingType::TradingPort => trade_income += 350,
                    _ => {}
                }
            }
        }

        // Army upkeep
        let mut upkeep = 0;
        for army in self.armies.iter().filter(|a| a.owner == faction_id) {
            for unit in &army.units {
                upkeep += unit.unit_type.stats().upkeep as i32;
            }
        }
        for p in self.provinces.iter().filter(|p| p.owner == faction_id) {
            for unit in &p.garrison {
                upkeep += (unit.unit_type.stats().upkeep as f32 * 0.75) as i32;
            }
        }

        let total_income = tax_income + trade_income;
        let net_profit = total_income - upkeep;
        (total_income, upkeep, net_profit)
    }

    pub fn can_move_army_to(&self, army_idx: usize, target_prov_idx: usize) -> bool {
        if let Some(army) = self.armies.get(army_idx) {
            if army.has_moved {
                return false;
            }
            if let Some(cur_prov) = self.provinces.get(army.province_id) {
                return cur_prov.neighbors.contains(&target_prov_idx);
            }
        }
        false
    }

    pub fn move_army(&mut self, army_idx: usize, target_prov_idx: usize) {
        if !self.can_move_army_to(army_idx, target_prov_idx) {
            return;
        }

        let army_owner = self.armies[army_idx].owner;
        let target_owner = self.provinces[target_prov_idx].owner;

        // If target is owned by self or ally, move peacefully
        if target_owner == army_owner {
            self.armies[army_idx].province_id = target_prov_idx;
            self.armies[army_idx].has_moved = true;
            self.selected_province = Some(target_prov_idx);
            return;
        }

        // Target is hostile or unowned: trigger battle!
        let target_name = self.provinces[target_prov_idx].name.clone();
        let mut def_garrison = self.provinces[target_prov_idx].garrison.clone();

        // Also check if there's an enemy field army in target province
        let enemy_army_idx = self
            .armies
            .iter()
            .position(|a| a.province_id == target_prov_idx && a.owner != army_owner);

        let def_stars = if let Some(e_idx) = enemy_army_idx {
            for u in &self.armies[e_idx].units {
                def_garrison.push(u.clone());
            }
            self.armies[e_idx].general_stars
        } else {
            1
        };

        let report = crate::combat::resolve_battle(
            army_owner,
            target_owner,
            target_prov_idx,
            &target_name,
            &mut self.armies[army_idx],
            &mut def_garrison,
            def_stars,
        );

        if report.attacker_won {
            // Province conquered!
            self.provinces[target_prov_idx].owner = army_owner;
            self.provinces[target_prov_idx].garrison = vec![
                UnitInstance::new(9990 + self.turn as u64, UnitType::ZamindariLevies),
            ];
            self.armies[army_idx].province_id = target_prov_idx;
            self.armies[army_idx].has_moved = true;

            if let Some(e_idx) = enemy_army_idx {
                // Update surviving units of defender army
                self.armies[e_idx].units = def_garrison.clone();
                // Check if any friendly neighbor province exists to retreat to
                let retreat_prov = self.provinces[target_prov_idx]
                    .neighbors
                    .iter()
                    .copied()
                    .find(|&n| self.provinces[n].owner == target_owner);

                if let Some(ret_id) = retreat_prov {
                    if !self.armies[e_idx].units.is_empty() {
                        self.armies[e_idx].province_id = ret_id;
                        self.armies[e_idx].has_moved = true;
                    } else {
                        self.armies.remove(e_idx);
                        if army_idx > e_idx {
                            self.selected_army = Some(army_idx - 1);
                        }
                    }
                } else {
                    // Trapped without friendly neighbor to retreat to -> shattered
                    self.armies.remove(e_idx);
                    if army_idx > e_idx {
                        self.selected_army = Some(army_idx - 1);
                    }
                }
            }

            if army_owner == self.player_faction {
                self.notifications.push(Notification {
                    turn: self.turn,
                    year: self.year,
                    title: format!("Victory in {}!", target_name),
                    message: format!(
                        "Our troops stormed and liberated {} from {} control!",
                        target_name, target_owner.name()
                    ),
                    is_urgent: true,
                });
            } else if target_owner == self.player_faction {
                self.notifications.push(Notification {
                    turn: self.turn,
                    year: self.year,
                    title: format!("Defeat in {}!", target_name),
                    message: format!(
                        "Hostile {} troops breached our fortifications in {}.",
                        army_owner.name(), target_name
                    ),
                    is_urgent: true,
                });
            }
        } else {
            // Repelled
            self.provinces[target_prov_idx].garrison = def_garrison;
            self.armies[army_idx].has_moved = true;

            if army_owner == self.player_faction {
                self.notifications.push(Notification {
                    turn: self.turn,
                    year: self.year,
                    title: format!("Repelled at {}!", target_name),
                    message: "Our vanguard suffered heavy resistance and fell back to defensible lines.".into(),
                    is_urgent: false,
                });
            } else if target_owner == self.player_faction {
                self.notifications.push(Notification {
                    turn: self.turn,
                    year: self.year,
                    title: format!("Heroic Defense in {}!", target_name),
                    message: format!(
                        "Our garrison repelled the hostile {} assault!",
                        army_owner.name()
                    ),
                    is_urgent: true,
                });
            }
        }

        self.latest_battle = Some(report);
        self.active_tab = ActiveTab::BattleReport;
    }

    pub fn recruit_unit(&mut self, prov_idx: usize, unit_type: UnitType) -> bool {
        let stats = unit_type.stats();
        let player_faction = self.player_faction;

        if let Some(fac) = self.factions.get_mut(&player_faction) {
            if fac.treasury < stats.cost as i32 {
                return false;
            }
            fac.treasury -= stats.cost as i32;
        }

        if let Some(p) = self.provinces.get_mut(prov_idx) {
            p.recruitment_queue.push(RecruitOrder {
                unit_type,
                turns_remaining: 1,
            });
            return true;
        }
        false
    }

    pub fn build_structure(&mut self, prov_idx: usize, b_type: BuildingType) -> bool {
        let cost = b_type.cost() as i32;
        let player_faction = self.player_faction;

        if let Some(fac) = self.factions.get_mut(&player_faction) {
            if fac.treasury < cost {
                return false;
            }
            fac.treasury -= cost;
        }

        if let Some(p) = self.provinces.get_mut(prov_idx) {
            if !p.buildings.contains(&b_type) {
                p.buildings.push(b_type);
                return true;
            }
        }
        false
    }

    pub fn end_turn(&mut self) {
        self.turn += 1;
        if self.season == Season::Summer {
            self.season = Season::Winter;
        } else {
            self.season = Season::Summer;
            self.year += 1;
        }

        // 1. Process player & AI economy
        for faction_id in [
            FactionId::Maratha,
            FactionId::Mughal,
            FactionId::Mysore,
            FactionId::Portugal,
        ] {
            let (_, _, net_profit) = self.calculate_faction_economy(faction_id);
            if let Some(fac) = self.factions.get_mut(&faction_id) {
                fac.treasury += net_profit;
            }
        }

        // 2. Process recruitment queues
        for prov in self.provinces.iter_mut() {
            let mut ready_units = Vec::new();
            prov.recruitment_queue.retain_mut(|order| {
                if order.turns_remaining <= 1 {
                    ready_units.push(order.unit_type);
                    false
                } else {
                    order.turns_remaining -= 1;
                    true
                }
            });

            for u_type in ready_units {
                let id = 10000 + (prov.garrison.len() as u64) * 100 + prov.id as u64;
                prov.garrison.push(UnitInstance::new(id, u_type));
            }
        }

        // 3. Reset army movement
        for army in self.armies.iter_mut() {
            army.has_moved = false;
        }

        // 4. Research progress
        if let Some(fac) = self.factions.get_mut(&self.player_faction) {
            if let Some((tech_id, turns)) = &mut fac.current_research {
                if *turns <= 1 {
                    let completed = tech_id.clone();
                    fac.researched_techs.push(completed.clone());
                    fac.current_research = None;
                    self.notifications.push(Notification {
                        turn: self.turn,
                        year: self.year,
                        title: "Technological Breakthrough!".into(),
                        message: format!("Our savants and engineers mastered '{}'!", completed),
                        is_urgent: true,
                    });
                } else {
                    *turns -= 1;
                }
            }
        }

        // 5. AI Turn Logic (Mughal & Independent aggression/reinforcement)
        crate::ai::execute_ai_turn(self);

        // 6. Check Victory Condition: Maratha controls Hindustan (Delhi) and 10+ provinces
        let maratha_count = self
            .provinces
            .iter()
            .filter(|p| p.owner == FactionId::Maratha)
            .count();
        let holds_delhi = self.provinces[9].owner == FactionId::Maratha;

        if holds_delhi && maratha_count >= 10 && !self.victory_claimed {
            self.victory_claimed = true;
            self.notifications.push(Notification {
                turn: self.turn,
                year: self.year,
                title: "ULTIMATE VICTORY: Hindavi Swarajya Established!".into(),
                message: "The saffron Bhagwa Dhwaj flies high over the Red Fort in Delhi! Maharani Tarabai's armies have united the Indian Subcontinent into an enduring Maratha Empire!".into(),
                is_urgent: true,
            });
        }

        // 7. AutoSave Campaign State to Local Storage!
        let _ = crate::storage::auto_save_campaign(self);
    }
}
