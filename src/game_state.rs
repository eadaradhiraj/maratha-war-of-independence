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
    Ledger,
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
    pub selected_diplo_faction: Option<FactionId>,
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
    pub alliances: Vec<FactionId>,
    pub relations: HashMap<FactionId, i32>,
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

        let mut maratha_rel = HashMap::new();
        maratha_rel.insert(FactionId::Mughal, -80);
        maratha_rel.insert(FactionId::Mysore, 25);
        maratha_rel.insert(FactionId::Portugal, 30);
        maratha_rel.insert(FactionId::Independent, 10);

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
                alliances: vec![],
                relations: maratha_rel,
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

        let mut mughal_rel = HashMap::new();
        mughal_rel.insert(FactionId::Maratha, -80);
        mughal_rel.insert(FactionId::Mysore, -20);
        mughal_rel.insert(FactionId::Portugal, -10);
        mughal_rel.insert(FactionId::Independent, -30);

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
                alliances: vec![],
                relations: mughal_rel,
            },
        );

        // 3. Mysore
        let mut mysore_rel = HashMap::new();
        mysore_rel.insert(FactionId::Maratha, 25);
        mysore_rel.insert(FactionId::Mughal, -20);
        mysore_rel.insert(FactionId::Portugal, 15);
        mysore_rel.insert(FactionId::Independent, 0);

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
                alliances: vec![],
                relations: mysore_rel,
            },
        );

        // 4. Portugal
        let mut port_rel = HashMap::new();
        port_rel.insert(FactionId::Maratha, 30);
        port_rel.insert(FactionId::Mughal, -10);
        port_rel.insert(FactionId::Mysore, 15);
        port_rel.insert(FactionId::Independent, 5);

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
                alliances: vec![],
                relations: port_rel,
            },
        );

        // 5. Independent
        let mut ind_rel = HashMap::new();
        ind_rel.insert(FactionId::Maratha, 10);
        ind_rel.insert(FactionId::Mughal, -30);
        ind_rel.insert(FactionId::Mysore, 0);
        ind_rel.insert(FactionId::Portugal, 5);

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
                alliances: vec![],
                relations: ind_rel,
            },
        );

        // 6. Rebels
        factions.insert(
            FactionId::Rebels,
            FactionData {
                id: FactionId::Rebels,
                name: "Deccan Insurgents & Peasant Rebels".into(),
                leader_name: "Rebel Warlords".into(),
                treasury: 500,
                ministers: vec![],
                researched_techs: vec![],
                current_research: None,
                wars: vec![FactionId::Maratha, FactionId::Mughal, FactionId::Mysore, FactionId::Portugal, FactionId::Independent],
                trade_partners: vec![],
                alliances: vec![],
                relations: HashMap::new(),
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
            selected_diplo_faction: Some(FactionId::Mughal),
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
                fac.alliances = fac_save.alliances;
                fac.relations = fac_save.relations;
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
            FactionId::Rebels => 0,
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
            FactionId::Rebels => 0,
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

        // Trade treaties generate maritime & overland commerce (+250g per partner)
        if let Some(fac) = self.factions.get(&faction_id) {
            trade_income += fac.trade_partners.len() as i32 * 250;
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

    pub fn transfer_garrison_to_army(&mut self, prov_idx: usize, garrison_idx: usize, army_idx: usize) -> bool {
        if prov_idx >= self.provinces.len() || army_idx >= self.armies.len() {
            return false;
        }
        if garrison_idx >= self.provinces[prov_idx].garrison.len() {
            return false;
        }
        if self.armies[army_idx].province_id != prov_idx {
            return false;
        }
        let unit = self.provinces[prov_idx].garrison.remove(garrison_idx);
        self.armies[army_idx].units.push(unit);
        true
    }

    pub fn transfer_army_to_garrison(&mut self, army_idx: usize, unit_idx: usize) -> bool {
        if army_idx >= self.armies.len() {
            return false;
        }
        if self.armies[army_idx].units.len() <= 1 {
            return false;
        }
        if unit_idx >= self.armies[army_idx].units.len() {
            return false;
        }
        let prov_id = self.armies[army_idx].province_id;
        let unit = self.armies[army_idx].units.remove(unit_idx);
        if let Some(prov) = self.provinces.get_mut(prov_id) {
            prov.garrison.push(unit);
            true
        } else {
            false
        }
    }

    pub fn commission_general_from_garrison(&mut self, prov_idx: usize) -> Result<usize, String> {
        let cost = 400;
        let fac = self.factions.get_mut(&self.player_faction).unwrap();
        if fac.treasury < cost {
            return Err("Insufficient treasury to commission a general (requires 400g)".into());
        }
        if self.provinces[prov_idx].garrison.is_empty() {
            return Err("No garrison units available to form a field army".into());
        }

        fac.treasury -= cost;
        let unit = self.provinces[prov_idx].garrison.remove(0);

        let names = [
            "Sardar Yashwantrao Pawar",
            "Kanhoji Angre",
            "Malhar Rao Holkar",
            "Ranoji Shinde",
            "Damaji Gaekwad",
            "Chimaji Appa",
        ];
        let name_idx = (self.armies.len() + self.turn as usize) % names.len();
        let next_id = (self.armies.len() as u64) + 10;

        let new_army = Army {
            id: next_id,
            owner: self.player_faction,
            province_id: prov_idx,
            units: vec![unit],
            general_name: names[name_idx].to_string(),
            general_stars: 3,
            has_moved: false,
        };

        self.armies.push(new_army);
        let new_idx = self.armies.len() - 1;
        self.selected_army = Some(new_idx);
        Ok(new_idx)
    }

    pub fn conduct_diplomacy(&mut self, target: FactionId, action: DiploAction) -> Result<String, String> {
        if target == self.player_faction {
            return Err("Cannot conduct diplomacy with your own realm.".into());
        }

        let p_fac = self.factions.get(&self.player_faction).unwrap().clone();
        let target_fac = self.factions.get(&target).ok_or("Target faction not found")?.clone();
        let current_rel = *p_fac.relations.get(&target).unwrap_or(&0);

        match action {
            DiploAction::TradeAgreement => {
                if p_fac.wars.contains(&target) {
                    return Err("Cannot establish trade while at war!".into());
                }
                if p_fac.trade_partners.contains(&target) {
                    return Err("A trade agreement is already active with this faction.".into());
                }
                if current_rel < -20 {
                    return Err(format!("The court of {} rejects trade due to hostile diplomatic relations ({}).", target_fac.name, current_rel));
                }
                if let Some(f) = self.factions.get_mut(&self.player_faction) {
                    f.trade_partners.push(target);
                    let rel = f.relations.entry(target).or_insert(0);
                    *rel = (*rel + 15).min(100);
                }
                if let Some(f) = self.factions.get_mut(&target) {
                    f.trade_partners.push(self.player_faction);
                    let rel = f.relations.entry(self.player_faction).or_insert(0);
                    *rel = (*rel + 15).min(100);
                }
                Ok(format!("Trade corridor established with {}! (+250g turn revenue)", target_fac.name))
            }
            DiploAction::SmallGift => {
                if p_fac.treasury < 400 {
                    return Err("Insufficient treasury for a gift (requires 400g).".into());
                }
                if let Some(f) = self.factions.get_mut(&self.player_faction) {
                    f.treasury -= 400;
                    let rel = f.relations.entry(target).or_insert(0);
                    *rel = (*rel + 15).min(100);
                }
                if let Some(f) = self.factions.get_mut(&target) {
                    f.treasury += 400;
                    let rel = f.relations.entry(self.player_faction).or_insert(0);
                    *rel = (*rel + 15).min(100);
                }
                Ok(format!("Dispatched fine silks and spices to {}. Relations improved by +15.", target_fac.leader_name))
            }
            DiploAction::LavishGift => {
                if p_fac.treasury < 1000 {
                    return Err("Insufficient treasury for a lavish gift (requires 1000g).".into());
                }
                if let Some(f) = self.factions.get_mut(&self.player_faction) {
                    f.treasury -= 1000;
                    let rel = f.relations.entry(target).or_insert(0);
                    *rel = (*rel + 40).min(100);
                }
                if let Some(f) = self.factions.get_mut(&target) {
                    f.treasury += 1000;
                    let rel = f.relations.entry(self.player_faction).or_insert(0);
                    *rel = (*rel + 40).min(100);
                }
                Ok(format!("Dispatched royal war elephants and jewels to {}. Relations improved by +40.", target_fac.leader_name))
            }
            DiploAction::Alliance => {
                if p_fac.wars.contains(&target) {
                    return Err("Cannot sign military alliance while at war!".into());
                }
                if p_fac.alliances.contains(&target) {
                    return Err("An alliance is already active with this faction.".into());
                }
                if current_rel < 20 {
                    return Err(format!("{} declines alliance: relations must be at least +20 (current: {}).", target_fac.name, current_rel));
                }
                if let Some(f) = self.factions.get_mut(&self.player_faction) {
                    f.alliances.push(target);
                    let rel = f.relations.entry(target).or_insert(0);
                    *rel = (*rel + 25).min(100);
                }
                if let Some(f) = self.factions.get_mut(&target) {
                    f.alliances.push(self.player_faction);
                    let rel = f.relations.entry(self.player_faction).or_insert(0);
                    *rel = (*rel + 25).min(100);
                }
                Ok(format!("Military Alliance forged with {} against common adversaries!", target_fac.name))
            }
            DiploAction::Ceasefire => {
                if !p_fac.wars.contains(&target) {
                    return Err("You are not currently at war with this faction.".into());
                }
                let cost = 600;
                if p_fac.treasury < cost {
                    return Err(format!("The court requires 600g in war indemnities for a ceasefire (treasury: {}g).", p_fac.treasury));
                }
                if let Some(f) = self.factions.get_mut(&self.player_faction) {
                    f.treasury -= cost;
                    f.wars.retain(|w| *w != target);
                    let rel = f.relations.entry(target).or_insert(0);
                    *rel = -10;
                }
                if let Some(f) = self.factions.get_mut(&target) {
                    f.treasury += cost;
                    f.wars.retain(|w| *w != self.player_faction);
                    let rel = f.relations.entry(self.player_faction).or_insert(0);
                    *rel = -10;
                }
                Ok(format!("Peace Treaty ratified with {}! Hostilities ceased.", target_fac.name))
            }
            DiploAction::DemandTribute => {
                if target == FactionId::Mughal && target_fac.treasury > 5000 {
                    return Err("The Mughal Emperor laughs at your ultimatum and refuses tribute!".into());
                }
                if current_rel < -40 {
                    return Err(format!("{} refuses your demands under arms!", target_fac.name));
                }
                let tribute = 300.min(target_fac.treasury.max(0));
                if let Some(f) = self.factions.get_mut(&self.player_faction) {
                    f.treasury += tribute;
                    let rel = f.relations.entry(target).or_insert(0);
                    *rel -= 30;
                }
                if let Some(f) = self.factions.get_mut(&target) {
                    f.treasury += tribute;
                    let rel = f.relations.entry(self.player_faction).or_insert(0);
                    *rel -= 30;
                }
                Ok(format!("Extorted {}g in regional tribute from {}.", tribute, target_fac.name))
            }
            DiploAction::DeclareWar => {
                if p_fac.wars.contains(&target) {
                    return Err("Already at war with this faction!".into());
                }
                if let Some(f) = self.factions.get_mut(&self.player_faction) {
                    f.wars.push(target);
                    f.trade_partners.retain(|t| *t != target);
                    f.alliances.retain(|a| *a != target);
                    let rel = f.relations.entry(target).or_insert(0);
                    *rel = -80;
                }
                if let Some(f) = self.factions.get_mut(&target) {
                    f.wars.push(self.player_faction);
                    f.trade_partners.retain(|t| *t != self.player_faction);
                    f.alliances.retain(|a| *a != self.player_faction);
                    let rel = f.relations.entry(self.player_faction).or_insert(0);
                    *rel = -80;
                }
                Ok(format!("Declared War on {}! Regiments mobilized across frontiers.", target_fac.name))
            }
        }
    }

    pub fn raid_province(&mut self, army_idx: usize, target_prov_idx: usize) -> Result<String, String> {
        if army_idx >= self.armies.len() {
            return Err("Invalid army index.".into());
        }
        let army = &self.armies[army_idx];
        if army.has_moved {
            return Err("Army has already expended its movement this turn.".into());
        }
        let army_prov = army.province_id;
        let army_owner = army.owner;
        let has_cavalry = army.units.iter().any(|u| u.unit_type.stats().is_cavalry);
        if !has_cavalry {
            return Err("Raiding requires mounted cavalry regiments (Bargir, Silhadar, or Pindari).".into());
        }

        if !self.provinces[army_prov].neighbors.contains(&target_prov_idx) {
            return Err("Target province is not adjacent to army position.".into());
        }

        let target_owner = self.provinces[target_prov_idx].owner;
        if target_owner == army_owner {
            return Err("Cannot raid your own sovereign province!".into());
        }

        let target_name = self.provinces[target_prov_idx].name.clone();
        let army_name = army.general_name.clone();

        let cavalry_strength: f32 = army.units.iter()
            .filter(|u| u.unit_type.stats().is_cavalry)
            .map(|u| u.combat_power())
            .sum();

        let base_loot = (self.provinces[target_prov_idx].base_wealth as f32 * 0.20).max(140.0);
        let cav_bonus = (cavalry_strength * 0.20).min(180.0);
        let looted_gold = (base_loot + cav_bonus).round() as u32;

        if let Some(target_p) = self.provinces.get_mut(target_prov_idx) {
            target_p.base_wealth = target_p.base_wealth.saturating_sub(40).max(120);
            target_p.public_order = (target_p.public_order - 3).clamp(-10, 10);
        }

        if let Some(fac) = self.factions.get_mut(&army_owner) {
            fac.treasury += looted_gold as i32;
        }

        if let Some(enemy_fac) = self.factions.get_mut(&target_owner) {
            enemy_fac.treasury = enemy_fac.treasury.saturating_sub(looted_gold as i32);
            let rel = enemy_fac.relations.entry(army_owner).or_insert(0);
            *rel -= 30;
            if !enemy_fac.wars.contains(&army_owner) {
                enemy_fac.wars.push(army_owner);
            }
        }

        if let Some(my_fac) = self.factions.get_mut(&army_owner) {
            if !my_fac.wars.contains(&target_owner) {
                my_fac.wars.push(target_owner);
            }
        }

        self.armies[army_idx].has_moved = true;

        let notif_msg = format!(
            "Ganimi Kava: {}'s swift cavalry swept into {}! Extorted {}g in Chauth tribute and disrupted imperial administration (-3 Public Order).",
            army_name, target_name, looted_gold
        );

        self.notifications.push(Notification {
            turn: self.turn,
            year: self.year,
            title: "Ganimi Kava Raid Successful".into(),
            message: notif_msg.clone(),
            is_urgent: false,
        });

        Ok(notif_msg)
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

        // 2b. Unit Replenishment in friendly territory
        let mut total_repl_cost = 0;
        let mut replenished_units = 0;
        for army in self.armies.iter_mut() {
            let prov_owner = self.provinces.get(army.province_id).map(|p| p.owner);
            if prov_owner == Some(army.owner) {
                for u in army.units.iter_mut() {
                    if u.soldiers < u.max_soldiers {
                        let heal = (u.max_soldiers as f32 * 0.20).ceil() as u32;
                        u.replenish(heal);
                        total_repl_cost += 15;
                        replenished_units += 1;
                    }
                }
            }
        }
        for prov in self.provinces.iter_mut() {
            for u in prov.garrison.iter_mut() {
                if u.soldiers < u.max_soldiers {
                    let heal = (u.max_soldiers as f32 * 0.25).ceil() as u32;
                    u.replenish(heal);
                    total_repl_cost += 10;
                    replenished_units += 1;
                }
            }
        }
        if let Some(fac) = self.factions.get_mut(&self.player_faction) {
            fac.treasury = fac.treasury.saturating_sub(total_repl_cost);
        }
        if replenished_units > 0 {
            self.notifications.push(Notification {
                turn: self.turn,
                year: self.year,
                title: "Regimental Reinforcements Deployed".into(),
                message: format!("{} frontline battalions refilled their ranks in friendly cantonments (-{}g supply costs).", replenished_units, total_repl_cost),
                is_urgent: false,
            });
        }

        // 2c. Public Order, Religious Demographics & Civil Unrest
        for prov in self.provinces.iter_mut() {
            let tax_order = prov.tax_level.public_order_penalty();
            let garrison_repression = ((prov.garrison.iter().map(|u| u.soldiers).sum::<u32>() / 35) as i32).min(6);
            let mut building_order = 0;
            let mut has_temple = false;
            for b in &prov.buildings {
                match b {
                    BuildingType::Fortress => building_order += 3,
                    BuildingType::TempleMosque => {
                        building_order += 2;
                        has_temple = true;
                    }
                    _ => {}
                }
            }

            // Religious friction
            let religious_friction = if !has_temple {
                match prov.owner {
                    FactionId::Maratha if prov.muslim_pct > 0.6 => -2,
                    FactionId::Mughal if prov.hindu_pct > 0.6 => -2,
                    _ => 0,
                }
            } else {
                0
            };

            let net_order_delta = tax_order + garrison_repression + building_order + religious_friction;
            prov.public_order = (prov.public_order + net_order_delta).clamp(-10, 10);
        }

        // 2d. Civil Rebellions & Insurrections (ETW Grand Campaign Provincial Uprisings)
        let mut rebellions = Vec::new();
        for (idx, prov) in self.provinces.iter_mut().enumerate() {
            if prov.public_order <= -8 {
                let garrison_size: u32 = prov.garrison.iter().map(|u| u.soldiers).sum();
                if garrison_size < 120 {
                    // Rebels overthrow imperial/sovereign administration
                    let prev_owner = prov.owner;
                    let prov_name = prov.name.clone();
                    prov.owner = FactionId::Rebels;
                    prov.public_order = 0;
                    prov.garrison.clear();
                    prov.garrison.push(UnitInstance::new(9000 + idx as u64 * 10 + 1, UnitType::ZamindariLevies));
                    prov.garrison.push(UnitInstance::new(9000 + idx as u64 * 10 + 2, UnitType::JezailSkirmishers));
                    prov.garrison.push(UnitInstance::new(9000 + idx as u64 * 10 + 3, UnitType::HinduMatchlocks));
                    rebellions.push((idx, prov_name, prev_owner));
                } else {
                    // Garrison quells riot with minor casualties
                    for u in prov.garrison.iter_mut() {
                        u.soldiers = u.soldiers.saturating_sub(15).max(1);
                    }
                    prov.public_order = -5;
                }
            }
        }
        for (_, p_name, prev_owner) in rebellions {
            self.notifications.push(Notification {
                turn: self.turn,
                year: self.year,
                title: "Provincial Rebellion Erupted!".into(),
                message: format!(
                    "Unbearable public dissent has ignited an armed insurrection in {}! Rebels have driven out {} garrison forces.",
                    p_name, prev_owner.name()
                ),
                is_urgent: true,
            });
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
