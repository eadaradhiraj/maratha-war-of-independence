use std::collections::HashMap;
use macroquad::rand;
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
    pub capital_unrest_turns: u32,
    pub victory_mode: CampaignVictoryCondition,
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
    pub is_bankrupt: bool,
    pub prestige: (u32, u32, u32), // (military, economic, enlightenment)
    pub vassals: Vec<FactionId>,
    pub overlord: Option<FactionId>,
    pub military_access_with: Vec<FactionId>,
    pub government: GovernmentType,
    pub minister_pool: Vec<Minister>,
}

pub fn generate_initial_minister_pool(f_id: FactionId) -> Vec<Minister> {
    match f_id {
        FactionId::Maratha => vec![
            Minister {
                role: MinisterRole::Treasury,
                name: "Ramchandra Pant Amatya".into(),
                stars: 5,
                loyalty: 9,
                trait_desc: "Fiscal Visionary: +15% Tax Efficiency & Mercantilism".into(),
            },
            Minister {
                role: MinisterRole::War,
                name: "Chimaji Appa".into(),
                stars: 4,
                loyalty: 10,
                trait_desc: "Siege Specialist: -15% Upkeep & +10% Cavalry Power".into(),
            },
            Minister {
                role: MinisterRole::Justice,
                name: "Balaji Vishwanath".into(),
                stars: 4,
                loyalty: 8,
                trait_desc: "Master Statesman: +2 Public Order & Diplomatic Drift".into(),
            },
        ],
        FactionId::Mughal => vec![
            Minister {
                role: MinisterRole::War,
                name: "Zulfiqar Khan Nusrat Jung".into(),
                stars: 4,
                loyalty: 8,
                trait_desc: "Grand Deccan Marshal: -12% Imperial Upkeep".into(),
            },
            Minister {
                role: MinisterRole::Treasury,
                name: "Inayatullah Khan Kashmiri".into(),
                stars: 4,
                loyalty: 9,
                trait_desc: "Exchequer Reformer: +12% Imperial Revenue".into(),
            },
            Minister {
                role: MinisterRole::Justice,
                name: "Qazi Abdul Wahhab".into(),
                stars: 3,
                loyalty: 8,
                trait_desc: "Orthodox Jurist: +2 Religious Repression".into(),
            },
        ],
        _ => vec![
            Minister {
                role: MinisterRole::Treasury,
                name: "Diwan Venkanna".into(),
                stars: 3,
                loyalty: 7,
                trait_desc: "Competent Tax Collector: +8% Revenue".into(),
            },
            Minister {
                role: MinisterRole::War,
                name: "Sardar Bakht Singh".into(),
                stars: 3,
                loyalty: 8,
                trait_desc: "Frontier Commander: -8% Upkeep".into(),
            },
            Minister {
                role: MinisterRole::Justice,
                name: "Mirza Fazlullah".into(),
                stars: 3,
                loyalty: 7,
                trait_desc: "Magistrate: +1 Public Order".into(),
            },
        ],
    }
}

pub fn generate_random_statesman(role: MinisterRole) -> Minister {
    let names = [
        "Trimbakrao Dabhade", "Malhar Rao Holkar", "Ranoji Scindia",
        "Santaji Ghorpade", "Dhanaji Jadhav", "Moropant Pingle",
        "Sadashivrao Bhau", "Nana Fadnavis", "Mahadji Shinde",
    ];
    let traits = [
        "Fiscal Genius: +12% Provincial Tax Collection",
        "Iron Marshal: -12% Military Upkeep Costs",
        "Pious Scholar: +2 National Public Order & Stability",
        "Reformer: +15% Enlightenment Research Speed",
        "Disciplinarian: +2 Garrison Repression",
    ];
    let name_idx = (rand::rand() as usize) % names.len();
    let trait_idx = (rand::rand() as usize) % traits.len();
    let stars = ((rand::rand() % 4) + 2) as u8;
    let loyalty = ((rand::rand() % 4) + 7) as u8;
    Minister {
        role,
        name: names[name_idx].into(),
        stars,
        loyalty,
        trait_desc: traits[trait_idx].into(),
    }
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
                is_bankrupt: false,
                prestige: (210, 180, 80),
                vassals: vec![],
                overlord: None,
                military_access_with: vec![],
                government: GovernmentType::AbsoluteMonarchy,
                minister_pool: generate_initial_minister_pool(FactionId::Maratha),
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
                is_bankrupt: false,
                prestige: (450, 600, 150),
                vassals: vec![],
                overlord: None,
                military_access_with: vec![],
                government: GovernmentType::AbsoluteMonarchy,
                minister_pool: generate_initial_minister_pool(FactionId::Mughal),
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
                is_bankrupt: false,
                prestige: (110, 120, 60),
                vassals: vec![],
                overlord: None,
                military_access_with: vec![],
                government: GovernmentType::ConstitutionalMonarchy,
                minister_pool: generate_initial_minister_pool(FactionId::Mysore),
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
                is_bankrupt: false,
                prestige: (90, 160, 100),
                vassals: vec![],
                overlord: None,
                military_access_with: vec![],
                government: GovernmentType::ConstitutionalMonarchy,
                minister_pool: generate_initial_minister_pool(FactionId::Portugal),
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
                is_bankrupt: false,
                prestige: (80, 80, 20),
                vassals: vec![],
                overlord: None,
                military_access_with: vec![],
                government: GovernmentType::Republic,
                minister_pool: generate_initial_minister_pool(FactionId::Independent),
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
                is_bankrupt: false,
                prestige: (30, 20, 0),
                vassals: vec![],
                overlord: None,
                military_access_with: vec![],
                government: GovernmentType::Republic,
                minister_pool: vec![],
            },
        );

        // 7. Sikh Empire (Emergent)
        factions.insert(
            FactionId::Sikh,
            FactionData {
                id: FactionId::Sikh,
                name: "Sikh Empire (Khalsa Fauj)".into(),
                leader_name: "Banda Singh Bahadur".into(),
                treasury: 1800,
                ministers: vec![],
                researched_techs: vec!["flintlock_musket".into()],
                current_research: None,
                wars: vec![FactionId::Mughal],
                trade_partners: vec![],
                alliances: vec![],
                relations: HashMap::new(),
                is_bankrupt: false,
                prestige: (70, 50, 30),
                vassals: vec![],
                overlord: None,
                military_access_with: vec![],
                government: GovernmentType::Republic,
                minister_pool: generate_initial_minister_pool(FactionId::Sikh),
            },
        );

        // 8. Nizamate of Hyderabad (Emergent)
        factions.insert(
            FactionId::Hyderabad,
            FactionData {
                id: FactionId::Hyderabad,
                name: "Nizamate of Hyderabad".into(),
                leader_name: "Nizam-ul-Mulk Asaf Jah I".into(),
                treasury: 3200,
                ministers: vec![],
                researched_techs: vec!["flintlock_musket".into()],
                current_research: None,
                wars: vec![],
                trade_partners: vec![],
                alliances: vec![],
                relations: HashMap::new(),
                is_bankrupt: false,
                prestige: (90, 110, 40),
                vassals: vec![],
                overlord: None,
                military_access_with: vec![],
                government: GovernmentType::AbsoluteMonarchy,
                minister_pool: generate_initial_minister_pool(FactionId::Hyderabad),
            },
        );

        // 9. Nawab of Bengal (Emergent)
        factions.insert(
            FactionId::Bengal,
            FactionData {
                id: FactionId::Bengal,
                name: "Nawab of Bengal".into(),
                leader_name: "Murshid Quli Khan".into(),
                treasury: 4800,
                ministers: vec![],
                researched_techs: vec!["flintlock_musket".into()],
                current_research: None,
                wars: vec![],
                trade_partners: vec![],
                alliances: vec![],
                relations: HashMap::new(),
                is_bankrupt: false,
                prestige: (80, 160, 50),
                vassals: vec![],
                overlord: None,
                military_access_with: vec![],
                government: GovernmentType::AbsoluteMonarchy,
                minister_pool: generate_initial_minister_pool(FactionId::Bengal),
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
            stance: CampaignStance::Normal,
            retinue: vec![GeneralAncillary::WarElephantMaster],
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
            stance: CampaignStance::Normal,
            retinue: vec![GeneralAncillary::BhaktiChaplain],
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
            stance: CampaignStance::Normal,
            retinue: vec![GeneralAncillary::FrenchArtilleryMaster],
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
            stance: CampaignStance::Normal,
            retinue: vec![GeneralAncillary::SwissDrillmaster],
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
            capital_unrest_turns: 0,
            victory_mode: CampaignVictoryCondition::LongCampaign,
        }
    }

    pub fn apply_save_data(&mut self, data: crate::storage::CampaignSaveData) {
        self.turn = data.turn;
        self.year = data.year;
        self.season = data.season;
        self.player_faction = data.player_faction;
        self.victory_claimed = data.victory_claimed;
        self.victory_mode = data.victory_mode;
        self.capital_unrest_turns = data.capital_unrest_turns;
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
                fac.is_bankrupt = fac_save.is_bankrupt;
                fac.prestige = fac_save.prestige;
                fac.vassals = fac_save.vassals;
                fac.overlord = fac_save.overlord;
                fac.military_access_with = fac_save.military_access_with;
                fac.government = fac_save.government;
                fac.minister_pool = fac_save.minister_pool;
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
                p.siege_turns_left = p_save.siege_turns_left;
                p.is_port_blockaded = p_save.is_port_blockaded;
                p.resistance_to_occupation = p_save.resistance_to_occupation;
                p.revealed_turns = p_save.revealed_turns;
                p.sabotaged_turns = p_save.sabotaged_turns;
                p.commodity = p_save.commodity;
                p.upper_tax = p_save.upper_tax;
                p.lower_tax = p_save.lower_tax;
                p.road_level = p_save.road_level;
            }
        }

        let capital_prov_idx = match self.player_faction {
            FactionId::Maratha => 4,
            FactionId::Mughal => 9,
            FactionId::Mysore => 13,
            FactionId::Portugal => 7,
            FactionId::Independent => 0,
            FactionId::Rebels => 0,
            FactionId::Sikh => 15,
            FactionId::Hyderabad => 10,
            FactionId::Bengal => 2,
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
            FactionId::Sikh => 15, // Punjab / Lahore
            FactionId::Hyderabad => 10, // Deccan / Hyderabad
            FactionId::Bengal => 2, // Bengal / Calcutta
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

        let partner_count = self.factions.get(&faction_id).map(|f| f.trade_partners.len()).unwrap_or(0);

        for p in self.provinces.iter().filter(|p| p.owner == faction_id) {
            if !p.is_under_siege {
                let mult = (p.upper_tax.rate_multiplier() + p.lower_tax.rate_multiplier()) * 0.5;
                let base_rev = (p.base_wealth as f32 * 0.4 * mult) as i32;
                tax_income += base_rev;

                if p.sabotaged_turns == 0 {
                    for b in &p.buildings {
                        match b {
                            BuildingType::Marketplace => tax_income += 220,
                            BuildingType::TradingPort if !p.is_port_blockaded => trade_income += 350,
                            BuildingType::TextileLoom if partner_count > 0 => trade_income += 180,
                            BuildingType::RoadNetwork if partner_count > 0 => trade_income += 60,
                            _ => {}
                        }
                    }

                    // Commodity Export Physics
                    if partner_count > 0 {
                        if let Some(comm) = p.commodity {
                            trade_income += (comm.base_price() * partner_count as i32) / 2;
                        }
                    }
                }
            }
        }

        // Trade treaties generate maritime & overland commerce (+250g per partner)
        let blockaded_ports = self.provinces.iter().filter(|p| p.owner == faction_id && p.is_port_blockaded).count();
        if let Some(fac) = self.factions.get(&faction_id) {
            let base_trade = fac.trade_partners.len() as i32 * 250;
            let penalty = (blockaded_ports as i32 * 125).min(base_trade);
            trade_income += base_trade - penalty;

            if fac.government == GovernmentType::ConstitutionalMonarchy {
                trade_income = (trade_income as f32 * 1.10) as i32;
            }
        }

        // Treasury Minister Efficiency Bonus (+3% per ability star)
        let treasury_stars = self.factions.get(&faction_id)
            .and_then(|f| f.ministers.iter().find(|m| m.role == MinisterRole::Treasury).map(|m| m.stars))
            .unwrap_or(3);
        let tax_bonus = (tax_income as f32 * (treasury_stars as f32 * 0.03)) as i32;
        tax_income += tax_bonus;

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

        // War Minister Upkeep Reduction (-3% per ability star)
        let war_stars = self.factions.get(&faction_id)
            .and_then(|f| f.ministers.iter().find(|m| m.role == MinisterRole::War).map(|m| m.stars))
            .unwrap_or(3);
        let upkeep_discount = (upkeep as f32 * (war_stars as f32 * 0.03)) as i32;
        upkeep = upkeep.saturating_sub(upkeep_discount);

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

        let has_military_access = self.factions.get(&army_owner)
            .map(|f| f.military_access_with.contains(&target_owner) && !f.wars.contains(&target_owner))
            .unwrap_or(false);

        // If target is owned by self or has bilateral military access, move peacefully
        if target_owner == army_owner || has_military_access {
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

        // Emergency Citizens-in-Arms Militia: If target settlement has no garrison, populace mounts emergency defense!
        if def_garrison.is_empty() && enemy_army_idx.is_none() {
            def_garrison.push(UnitInstance::new(8880 + target_prov_idx as u64 * 10 + 1, UnitType::ZamindariLevies));
        }

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
            // General Retinue & Ancillary appointment
            if self.armies[army_idx].retinue.len() < 3 {
                let roll = (self.turn * 37 + target_prov_idx as u32 * 13) % 4;
                let anc = match roll {
                    0 => GeneralAncillary::FrenchArtilleryMaster,
                    1 => GeneralAncillary::BhaktiChaplain,
                    2 => GeneralAncillary::WarElephantMaster,
                    _ => GeneralAncillary::SwissDrillmaster,
                };
                if !self.armies[army_idx].retinue.contains(&anc) {
                    self.armies[army_idx].retinue.push(anc);
                    if army_owner == self.player_faction {
                        self.notifications.push(Notification {
                            turn: self.turn,
                            year: self.year,
                            title: "General Retinue Appointed!".into(),
                            message: format!("General {} has appointed a {} to their retinue!", self.armies[army_idx].general_name, anc.name()),
                            is_urgent: false,
                        });
                    }
                }
            }

            // Province conquered!
            self.occupy_province(target_prov_idx, OccupationChoice::Peaceful);
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
            stance: CampaignStance::Normal,
            retinue: vec![],
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
                    f.treasury = f.treasury.saturating_sub(tribute);
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
                    f.military_access_with.retain(|m| *m != target);
                    let rel = f.relations.entry(target).or_insert(0);
                    *rel = -80;
                }
                if let Some(f) = self.factions.get_mut(&target) {
                    f.wars.push(self.player_faction);
                    f.trade_partners.retain(|t| *t != self.player_faction);
                    f.alliances.retain(|a| *a != self.player_faction);
                    f.military_access_with.retain(|m| *m != self.player_faction);
                    let rel = f.relations.entry(self.player_faction).or_insert(0);
                    *rel = -80;
                }
                Ok(format!("Declared War on {}! Regiments mobilized across frontiers.", target_fac.name))
            }
            DiploAction::GrantMilitaryAccess => {
                if p_fac.wars.contains(&target) {
                    return Err("Cannot negotiate military transit while at war!".into());
                }
                if p_fac.military_access_with.contains(&target) {
                    return Err("Military access is already granted between our nations.".into());
                }
                if current_rel < -10 {
                    return Err(format!("The court of {} refuses to permit foreign troops upon their soil (relations: {}).", target_fac.name, current_rel));
                }
                if let Some(f) = self.factions.get_mut(&self.player_faction) {
                    f.military_access_with.push(target);
                    let rel = f.relations.entry(target).or_insert(0);
                    *rel = (*rel + 15).min(100);
                }
                if let Some(f) = self.factions.get_mut(&target) {
                    f.military_access_with.push(self.player_faction);
                    let rel = f.relations.entry(self.player_faction).or_insert(0);
                    *rel = (*rel + 15).min(100);
                }
                Ok(format!("Bilateral Military Access ratified with {}! Field armies may march through borders peacefully.", target_fac.name))
            }
            DiploAction::SubjugateVassal => {
                if target == FactionId::Mughal && target_fac.treasury > 3000 {
                    return Err("The Mughal Emperor considers your court upstarts and refuses vassalage!".into());
                }
                if target_fac.overlord.is_some() {
                    return Err(format!("{} is already the vassal of another sovereign.", target_fac.name));
                }
                let p_prestige = p_fac.prestige.0 + p_fac.prestige.1 + p_fac.prestige.2;
                let t_prestige = target_fac.prestige.0 + target_fac.prestige.1 + target_fac.prestige.2;
                if p_prestige <= t_prestige && current_rel < 30 {
                    return Err(format!("{} possesses prestige ({} pts) comparable to your realm ({} pts) and rejects subservience.", target_fac.name, t_prestige, p_prestige));
                }
                if let Some(f) = self.factions.get_mut(&target) {
                    f.overlord = Some(self.player_faction);
                    f.wars.retain(|w| *w != self.player_faction);
                    if !f.trade_partners.contains(&self.player_faction) {
                        f.trade_partners.push(self.player_faction);
                    }
                    if !f.military_access_with.contains(&self.player_faction) {
                        f.military_access_with.push(self.player_faction);
                    }
                    let rel = f.relations.entry(self.player_faction).or_insert(0);
                    *rel = (*rel + 25).min(100);
                }
                if let Some(f) = self.factions.get_mut(&self.player_faction) {
                    if !f.vassals.contains(&target) {
                        f.vassals.push(target);
                    }
                    f.wars.retain(|w| *w != target);
                    if !f.trade_partners.contains(&target) {
                        f.trade_partners.push(target);
                    }
                    if !f.military_access_with.contains(&target) {
                        f.military_access_with.push(target);
                    }
                    let rel = f.relations.entry(target).or_insert(0);
                    *rel = (*rel + 25).min(100);
                }
                Ok(format!("{} has sworn fealty as a Protectorate Vassal! They remit 50% surplus revenue each season.", target_fac.name))
            }
            DiploAction::DemandTerritory(prov_idx) => {
                if prov_idx >= self.provinces.len() {
                    return Err("Invalid province index.".into());
                }
                if self.provinces[prov_idx].owner != target {
                    return Err(format!("{} does not own {}.", target_fac.name, self.provinces[prov_idx].name));
                }
                let cap_idx = match target {
                    FactionId::Mughal => 9,
                    FactionId::Mysore => 13,
                    FactionId::Portugal => 7,
                    FactionId::Sikh => 15,
                    FactionId::Hyderabad => 10,
                    FactionId::Bengal => 2,
                    _ => 999,
                };
                if prov_idx == cap_idx {
                    return Err(format!("{} will never cede their seat of power ({})!", target_fac.name, self.provinces[prov_idx].capital_name));
                }
                let is_vassal = p_fac.vassals.contains(&target);
                if !is_vassal && current_rel < 35 {
                    return Err(format!("The court of {} rejects your territorial demands (relations: {}).", target_fac.name, current_rel));
                }
                let prov_name = self.provinces[prov_idx].name.clone();
                self.provinces[prov_idx].owner = self.player_faction;
                self.provinces[prov_idx].public_order = 0;
                if let Some(f) = self.factions.get_mut(&target) {
                    let rel = f.relations.entry(self.player_faction).or_insert(0);
                    *rel = (*rel - 25).clamp(-100, 100);
                }
                Ok(format!("{} ceded territorial governance of {} to your realm!", target_fac.name, prov_name))
            }
            DiploAction::OfferTerritory(prov_idx) => {
                if prov_idx >= self.provinces.len() {
                    return Err("Invalid province index.".into());
                }
                if self.provinces[prov_idx].owner != self.player_faction {
                    return Err("You do not hold this province.".into());
                }
                let prov_name = self.provinces[prov_idx].name.clone();
                self.provinces[prov_idx].owner = target;
                self.provinces[prov_idx].public_order = 2;
                if let Some(f) = self.factions.get_mut(&target) {
                    let rel = f.relations.entry(self.player_faction).or_insert(0);
                    *rel = (*rel + 45).min(100);
                }
                if let Some(f) = self.factions.get_mut(&self.player_faction) {
                    let rel = f.relations.entry(target).or_insert(0);
                    *rel = (*rel + 45).min(100);
                }
                Ok(format!("Transferred sovereignty of {} to {}. Bilateral diplomatic trust surged (+45)!", prov_name, target_fac.name))
            }
            DiploAction::ShareTechnology => {
                if p_fac.wars.contains(&target) {
                    return Err("Cannot share military technology while at war!".into());
                }
                let shareable: Vec<String> = p_fac.researched_techs.iter()
                    .filter(|t| !target_fac.researched_techs.contains(t))
                    .cloned()
                    .collect();
                if shareable.is_empty() {
                    return Err(format!("The court of {} already possesses all innovations known to your scholars.", target_fac.name));
                }
                let tech_name = shareable[0].clone();
                if let Some(f) = self.factions.get_mut(&target) {
                    f.researched_techs.push(tech_name.clone());
                    let rel = f.relations.entry(self.player_faction).or_insert(0);
                    *rel = (*rel + 30).min(100);
                }
                if let Some(f) = self.factions.get_mut(&self.player_faction) {
                    f.treasury += 600;
                    let rel = f.relations.entry(target).or_insert(0);
                    *rel = (*rel + 30).min(100);
                }
                Ok(format!("Shared technical breakthrough [{}] with {}! Received 600g royal honorarium and +30 Diplomatic Trust.", tech_name, target_fac.name))
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

    pub fn has_recon_visibility(&self, prov_idx: usize) -> bool {
        if prov_idx >= self.provinces.len() {
            return false;
        }
        let p = &self.provinces[prov_idx];
        if p.owner == self.player_faction || p.revealed_turns > 0 {
            return true;
        }
        if self.armies.iter().any(|a| a.province_id == prov_idx && a.owner == self.player_faction) {
            return true;
        }
        for &n in &p.neighbors {
            if let Some(np) = self.provinces.get(n) {
                if np.owner == self.player_faction {
                    return true;
                }
            }
            if self.armies.iter().any(|a| a.province_id == n && a.owner == self.player_faction) {
                return true;
            }
        }
        false
    }

    pub fn set_army_stance(&mut self, army_idx: usize, stance: CampaignStance) -> Result<String, String> {
        if army_idx >= self.armies.len() {
            return Err("Invalid army index.".into());
        }
        let army = &mut self.armies[army_idx];
        if army.owner != self.player_faction {
            return Err("Cannot command foreign forces.".into());
        }
        if army.has_moved && stance == CampaignStance::Fortified {
            return Err("Army must have unspent movement to establish a Fortified Camp.".into());
        }
        army.stance = stance;
        if stance == CampaignStance::Fortified {
            army.has_moved = true;
        }
        Ok(format!("{} assumed stance: {}.", army.general_name, stance.name()))
    }

    pub fn initiate_siege(&mut self, army_idx: usize, target_prov_idx: usize) -> Result<String, String> {
        if army_idx >= self.armies.len() || target_prov_idx >= self.provinces.len() {
            return Err("Invalid army or province index.".into());
        }
        let army = &self.armies[army_idx];
        if army.has_moved {
            return Err("Army has already moved this turn.".into());
        }
        let cur_prov = army.province_id;
        let army_owner = army.owner;
        if !self.provinces[cur_prov].neighbors.contains(&target_prov_idx) {
            return Err("Target province is not adjacent.".into());
        }
        let target_owner = self.provinces[target_prov_idx].owner;
        if target_owner == army_owner {
            return Err("Cannot besiege your own provincial capital!".into());
        }

        if let Some(f) = self.factions.get_mut(&army_owner) {
            if !f.wars.contains(&target_owner) {
                f.wars.push(target_owner);
            }
        }
        if let Some(f) = self.factions.get_mut(&target_owner) {
            if !f.wars.contains(&army_owner) {
                f.wars.push(army_owner);
            }
        }

        self.armies[army_idx].province_id = target_prov_idx;
        self.armies[army_idx].has_moved = true;

        let target_name = self.provinces[target_prov_idx].name.clone();
        let army_name = self.armies[army_idx].general_name.clone();

        let target_prov = &mut self.provinces[target_prov_idx];
        target_prov.is_under_siege = true;
        target_prov.siege_turns_left = 3;
        target_prov.besieger_army_idx = Some(army_idx);

        let msg = format!("{} laid siege to {}! Provincial supply lines severed, 3-turn capitulation timer initiated.", army_name, target_name);
        self.notifications.push(Notification {
            turn: self.turn,
            year: self.year,
            title: "Siege Established".into(),
            message: msg.clone(),
            is_urgent: true,
        });
        Ok(msg)
    }

    pub fn lift_siege(&mut self, prov_idx: usize) -> Result<String, String> {
        if prov_idx >= self.provinces.len() {
            return Err("Invalid province index.".into());
        }
        let p = &mut self.provinces[prov_idx];
        if !p.is_under_siege {
            return Err("Province is not under siege.".into());
        }
        p.is_under_siege = false;
        p.siege_turns_left = 0;
        p.besieger_army_idx = None;
        Ok(format!("Siege lines withdrawn from {}.", p.name))
    }

    pub fn toggle_port_blockade(&mut self, prov_idx: usize) -> Result<String, String> {
        if prov_idx >= self.provinces.len() {
            return Err("Invalid province index.".into());
        }
        let p = &mut self.provinces[prov_idx];
        if p.port_name.is_none() {
            return Err("This province has no maritime coastline or harbor.".into());
        }
        p.is_port_blockaded = !p.is_port_blockaded;
        let st = if p.is_port_blockaded { "Blockaded (Maritime commerce severed)" } else { "Open (Harbor restored)" };
        Ok(format!("Harbor of {} is now {}.", p.name, st))
    }

    pub fn dispatch_covert_mission(&mut self, target_prov_idx: usize, action: CovertAction) -> Result<String, String> {
        if target_prov_idx >= self.provinces.len() {
            return Err("Invalid province index.".into());
        }
        let cost = action.cost();
        if let Some(fac) = self.factions.get(&self.player_faction) {
            if fac.treasury < cost {
                return Err(format!("Insufficient treasury (requires {}g).", cost));
            }
        }
        if let Some(fac) = self.factions.get_mut(&self.player_faction) {
            fac.treasury -= cost;
        }

        let target_prov = &mut self.provinces[target_prov_idx];
        let prov_name = target_prov.name.clone();

        match action {
            CovertAction::Infiltrate => {
                target_prov.revealed_turns = 4;
                let msg = format!("Operative infiltrated {}! Fog of War lifted across this Subah for 4 seasonal turns.", prov_name);
                self.notifications.push(Notification {
                    turn: self.turn,
                    year: self.year,
                    title: "Espionage: Infiltration Successful".into(),
                    message: msg.clone(),
                    is_urgent: false,
                });
                Ok(msg)
            }
            CovertAction::SabotageInfrastructure => {
                target_prov.sabotaged_turns = 3;
                target_prov.public_order = (target_prov.public_order - 2).clamp(-10, 10);
                let msg = format!("Operative sabotaged key fortifications & granaries in {}! Infrastructure crippled for 3 turns.", prov_name);
                self.notifications.push(Notification {
                    turn: self.turn,
                    year: self.year,
                    title: "Espionage: Sabotage Executed".into(),
                    message: msg.clone(),
                    is_urgent: false,
                });
                Ok(msg)
            }
            CovertAction::AssassinateGeneral => {
                let enemy_army_opt = self.armies.iter_mut().find(|a| a.province_id == target_prov_idx && a.owner != self.player_faction);
                if let Some(army) = enemy_army_opt {
                    let gen_name = army.general_name.clone();
                    let stars = army.general_stars;
                    let roll = (self.turn * 17 + target_prov_idx as u32 * 31) % 100;
                    let chance = (65 - (stars as u32 * 8)).max(25);
                    if roll < chance {
                        army.general_stars = 0;
                        army.general_name = format!("Staff Officers (Ex-{})", gen_name);
                        let msg = format!("Assassination successful! General {} was struck down in {}, shattering enemy command.", gen_name, prov_name);
                        self.notifications.push(Notification {
                            turn: self.turn,
                            year: self.year,
                            title: "Espionage: Commander Assassinated".into(),
                            message: msg.clone(),
                            is_urgent: true,
                        });
                        Ok(msg)
                    } else {
                        let msg = format!("Assassination attempt in {} failed! The operative was captured and executed.", prov_name);
                        self.notifications.push(Notification {
                            turn: self.turn,
                            year: self.year,
                            title: "Espionage: Operative Compromised".into(),
                            message: msg.clone(),
                            is_urgent: false,
                        });
                        Ok(msg)
                    }
                } else {
                    Err(format!("No hostile field general stationed in {}.", prov_name))
                }
            }
            CovertAction::StealTechnology => {
                let target_owner = target_prov.owner;
                let target_fac = match self.factions.get(&target_owner) {
                    Some(f) => f,
                    None => return Err("Invalid foreign sovereign court.".into()),
                };
                let p_fac = self.factions.get(&self.player_faction).unwrap();
                let stealable: Vec<String> = target_fac.researched_techs.iter()
                    .filter(|t| !p_fac.researched_techs.contains(t))
                    .cloned()
                    .collect();
                if stealable.is_empty() {
                    return Err(format!("Foreign scholars in {} possess no technical innovations unknown to our academies.", prov_name));
                }
                let roll = (self.turn * 23 + target_prov_idx as u32 * 41) % 100;
                if roll < 55 {
                    let tech_id = stealable[0].clone();
                    if let Some(f) = self.factions.get_mut(&self.player_faction) {
                        f.researched_techs.push(tech_id.clone());
                    }
                    let msg = format!("Grand Espionage Success! Operatives in {} stole technical schematics for [{}].", prov_name, tech_id);
                    self.notifications.push(Notification {
                        turn: self.turn,
                        year: self.year,
                        title: "Espionage: Technology Blueprints Stolen!".into(),
                        message: msg.clone(),
                        is_urgent: true,
                    });
                    Ok(msg)
                } else {
                    let msg = format!("Theft compromised in {}! Operatives were seized by palace guards while copying military treaties.", prov_name);
                    self.notifications.push(Notification {
                        turn: self.turn,
                        year: self.year,
                        title: "Espionage: Infiltration Compromised".into(),
                        message: msg.clone(),
                        is_urgent: false,
                    });
                    Ok(msg)
                }
            }
        }
    }

    pub fn occupy_province(&mut self, prov_idx: usize, choice: OccupationChoice) {
        if prov_idx >= self.provinces.len() {
            return;
        }
        let p = &mut self.provinces[prov_idx];
        p.owner = self.player_faction;
        p.is_under_siege = false;
        p.siege_turns_left = 0;
        p.besieger_army_idx = None;

        match choice {
            OccupationChoice::Peaceful => {
                p.resistance_to_occupation = 8;
                self.notifications.push(Notification {
                    turn: self.turn,
                    year: self.year,
                    title: "Peaceful Annexation".into(),
                    message: format!("{} has been peacefully incorporated into the realm. Resistance to occupation (-8) will decay over time.", p.name),
                    is_urgent: false,
                });
            }
            OccupationChoice::SackAndPillage => {
                let looted = 750;
                if let Some(fac) = self.factions.get_mut(&self.player_faction) {
                    fac.treasury += looted;
                }
                p.base_wealth = p.base_wealth.saturating_sub(60).max(100);
                p.resistance_to_occupation = 14;
                p.public_order = -8;
                self.notifications.push(Notification {
                    turn: self.turn,
                    year: self.year,
                    title: "Subah Sacked & Pillaged!".into(),
                    message: format!("{} was looted for {}g in wartime plunder! Devastated infrastructure and severe unrest (-14) in the aftermath.", p.name, looted),
                    is_urgent: true,
                });
            }
        }
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
            FactionId::Independent,
            FactionId::Sikh,
            FactionId::Hyderabad,
            FactionId::Bengal,
        ] {
            let (_, _, net_profit) = self.calculate_faction_economy(faction_id);
            if let Some(fac) = self.factions.get_mut(&faction_id) {
                fac.treasury += net_profit;
            }
        }

        // 1b. Check National Bankruptcy, Mutiny & Desertion
        for faction_id in [
            FactionId::Maratha,
            FactionId::Mughal,
            FactionId::Mysore,
            FactionId::Portugal,
            FactionId::Independent,
            FactionId::Sikh,
            FactionId::Hyderabad,
            FactionId::Bengal,
        ] {
            let in_debt = self.factions.get(&faction_id).map(|f| f.treasury < 0).unwrap_or(false);
            if let Some(fac) = self.factions.get_mut(&faction_id) {
                fac.is_bankrupt = in_debt;
            }
            if in_debt {
                for army in self.armies.iter_mut().filter(|a| a.owner == faction_id) {
                    for u in army.units.iter_mut() {
                        let deserted = (u.soldiers as f32 * 0.15).ceil() as u32;
                        u.soldiers = u.soldiers.saturating_sub(deserted).max(1);
                    }
                }
                if faction_id == self.player_faction {
                    self.notifications.push(Notification {
                        turn: self.turn,
                        year: self.year,
                        title: "National Bankruptcy Crisis!".into(),
                        message: "Treasury deficit has triggered regimental desertions (-15% troop strength) and administrative paralysis!".into(),
                        is_urgent: true,
                    });
                }
            }
        }

        // 1c. Vassal Tribute Collection (50% surplus remitted to overlords)
        for faction_id in [
            FactionId::Maratha,
            FactionId::Mughal,
            FactionId::Mysore,
            FactionId::Portugal,
            FactionId::Sikh,
            FactionId::Hyderabad,
            FactionId::Bengal,
        ] {
            let vassals = self.factions.get(&faction_id).map(|f| f.vassals.clone()).unwrap_or_default();
            let mut total_vassal_tribute = 0;
            for v_id in vassals {
                let (_, _, v_profit) = self.calculate_faction_economy(v_id);
                if v_profit > 0 {
                    let tribute = v_profit / 2;
                    total_vassal_tribute += tribute;
                    if let Some(v_fac) = self.factions.get_mut(&v_id) {
                        v_fac.treasury = v_fac.treasury.saturating_sub(tribute);
                    }
                }
            }
            if total_vassal_tribute > 0 {
                if let Some(fac) = self.factions.get_mut(&faction_id) {
                    fac.treasury += total_vassal_tribute;
                }
                if faction_id == self.player_faction {
                    self.notifications.push(Notification {
                        turn: self.turn,
                        year: self.year,
                        title: "Protectorate Tribute Collected".into(),
                        message: format!("Remitted {}g in seasonal imperial tribute from sworn vassal protectorates.", total_vassal_tribute),
                        is_urgent: false,
                    });
                }
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

            let net_order_delta = tax_order + garrison_repression + building_order + religious_friction - prov.resistance_to_occupation - (if prov.is_port_blockaded { 2 } else { 0 }) - (if self.factions.get(&prov.owner).map(|f| f.is_bankrupt).unwrap_or(false) { 2 } else { 0 });
            prov.public_order = (prov.public_order + net_order_delta).clamp(-10, 10);
        }

        // 2d. Civil Rebellions & Insurrections (ETW Emergent Factions & Uprisings)
        let mut rebellions = Vec::new();
        for (idx, prov) in self.provinces.iter_mut().enumerate() {
            if prov.public_order <= -8 {
                let garrison_size: u32 = prov.garrison.iter().map(|u| u.soldiers).sum();
                if garrison_size < 120 {
                    let prev_owner = prov.owner;
                    let prov_name = prov.name.clone();

                    // Historical Emergent Factions:
                    // Punjab (15)    -> Sikh Empire
                    // Hyderabad (10) -> Nizamate of Hyderabad
                    // Bengal (2)     -> Nawab of Bengal
                    let new_owner = match idx {
                        15 => FactionId::Sikh,
                        10 => FactionId::Hyderabad,
                        2  => FactionId::Bengal,
                        _  => FactionId::Rebels,
                    };

                    prov.owner = new_owner;
                    prov.public_order = 0;
                    prov.garrison.clear();

                    match new_owner {
                        FactionId::Sikh => {
                            prov.garrison.push(UnitInstance::new(9000 + idx as u64 * 10 + 1, UnitType::MansabdarGuards));
                            prov.garrison.push(UnitInstance::new(9000 + idx as u64 * 10 + 2, UnitType::ZamindariLevies));
                            prov.garrison.push(UnitInstance::new(9000 + idx as u64 * 10 + 3, UnitType::JezailSkirmishers));
                        }
                        FactionId::Hyderabad => {
                            prov.garrison.push(UnitInstance::new(9000 + idx as u64 * 10 + 1, UnitType::SilhadarLancers));
                            prov.garrison.push(UnitInstance::new(9000 + idx as u64 * 10 + 2, UnitType::SepoyLineInfantry));
                            prov.garrison.push(UnitInstance::new(9000 + idx as u64 * 10 + 3, UnitType::ZamindariLevies));
                        }
                        FactionId::Bengal => {
                            prov.garrison.push(UnitInstance::new(9000 + idx as u64 * 10 + 1, UnitType::SepoyLineInfantry));
                            prov.garrison.push(UnitInstance::new(9000 + idx as u64 * 10 + 2, UnitType::SepoyLineInfantry));
                            prov.garrison.push(UnitInstance::new(9000 + idx as u64 * 10 + 3, UnitType::BargirCavalry));
                        }
                        _ => {
                            prov.garrison.push(UnitInstance::new(9000 + idx as u64 * 10 + 1, UnitType::ZamindariLevies));
                            prov.garrison.push(UnitInstance::new(9000 + idx as u64 * 10 + 2, UnitType::JezailSkirmishers));
                            prov.garrison.push(UnitInstance::new(9000 + idx as u64 * 10 + 3, UnitType::HinduMatchlocks));
                        }
                    }
                    rebellions.push((idx, prov_name, prev_owner, new_owner));
                } else {
                    // Garrison quells riot with minor casualties
                    for u in prov.garrison.iter_mut() {
                        u.soldiers = u.soldiers.saturating_sub(15).max(1);
                    }
                    prov.public_order = -5;
                }
            }
        }
        for (_, p_name, prev_owner, new_owner) in rebellions {
            let is_emergent = new_owner != FactionId::Rebels;
            let title = if is_emergent {
                format!("Sovereign Emergence: {}!", new_owner.name())
            } else {
                "Provincial Rebellion Erupted!".into()
            };
            let desc = if is_emergent {
                format!("The people of {} have declared independence under the banners of the {}!", p_name, new_owner.name())
            } else {
                format!("Unbearable public dissent has ignited an armed insurrection in {}! Rebels have driven out {} garrison forces.", p_name, prev_owner.name())
            };
            self.notifications.push(Notification {
                turn: self.turn,
                year: self.year,
                title,
                message: desc,
                is_urgent: true,
            });
        }

        // 2e. Demographic & Town Wealth Physics (ETW Population Growth & Dual-Class Taxes)
        for prov in self.provinces.iter_mut() {
            let pop_growth_rate = match prov.lower_tax {
                TaxLevel::Exempt => 0.015,
                TaxLevel::Low => 0.012,
                TaxLevel::Normal => 0.008,
                TaxLevel::High => 0.002,
                TaxLevel::Extortionate => -0.005,
            };
            prov.population = (prov.population as f32 * (1.0 + pop_growth_rate)).round() as u32;

            let wealth_delta = match prov.upper_tax {
                TaxLevel::Exempt => 25,
                TaxLevel::Low => 18,
                TaxLevel::Normal => 5,
                TaxLevel::High => -12,
                TaxLevel::Extortionate => -30,
            };
            prov.base_wealth = (prov.base_wealth as i32 + wealth_delta).clamp(100, 5000) as u32;

            let has_religious_building = prov.buildings.contains(&BuildingType::TempleMosque);
            if has_religious_building {
                match prov.owner {
                    FactionId::Maratha => {
                        prov.hindu_pct = (prov.hindu_pct + 0.015).min(0.98);
                        prov.muslim_pct = 1.0 - prov.hindu_pct;
                    }
                    FactionId::Mughal => {
                        prov.muslim_pct = (prov.muslim_pct + 0.015).min(0.98);
                        prov.hindu_pct = 1.0 - prov.muslim_pct;
                    }
                    _ => {}
                }
            }
        }

        // 2f. Winter & Hostile Territory Attrition (ETW Supply Logistics)
        if self.season == Season::Winter {
            for army in self.armies.iter_mut() {
                let prov_owner = self.provinces.get(army.province_id).map(|p| p.owner);
                if prov_owner.is_some() && prov_owner != Some(army.owner) {
                    for u in army.units.iter_mut() {
                        let attrition_loss = (u.soldiers as f32 * 0.06).ceil() as u32;
                        u.soldiers = u.soldiers.saturating_sub(attrition_loss).max(1);
                    }
                }
            }
        }

        // 2g. Fortress Siege Starvation & Capitulation Countdown
        let mut capitulations = Vec::new();
        for (p_idx, prov) in self.provinces.iter_mut().enumerate() {
            if prov.is_under_siege {
                for u in prov.garrison.iter_mut() {
                    let starv = (u.soldiers as f32 * 0.15).ceil() as u32;
                    u.soldiers = u.soldiers.saturating_sub(starv).max(1);
                }
                prov.siege_turns_left = prov.siege_turns_left.saturating_sub(1);
                if prov.siege_turns_left == 0 {
                    let victor = prov.besieger_army_idx
                        .and_then(|idx| if idx < self.armies.len() { Some(self.armies[idx].owner) } else { None })
                        .unwrap_or(FactionId::Maratha);
                    capitulations.push((p_idx, prov.name.clone(), victor));
                }
            }
        }
        for (p_idx, p_name, victor) in capitulations {
            let p = &mut self.provinces[p_idx];
            p.owner = victor;
            p.is_under_siege = false;
            p.siege_turns_left = 0;
            p.besieger_army_idx = None;
            p.resistance_to_occupation = 6;
            p.garrison.clear();
            self.notifications.push(Notification {
                turn: self.turn,
                year: self.year,
                title: "Fortress Capitulation!".into(),
                message: format!("Starvation forced the garrison of {} to capitulate to {} forces without further assault!", p_name, victor.name()),
                is_urgent: true,
            });
        }

        // 2h. Decay Resistance to Occupation, Reconnaissance & Sabotage
        for prov in self.provinces.iter_mut() {
            prov.revealed_turns = prov.revealed_turns.saturating_sub(1);
            prov.sabotaged_turns = prov.sabotaged_turns.saturating_sub(1);
            prov.resistance_to_occupation = prov.resistance_to_occupation.saturating_sub(2);
        }

        // 2i. Historical Crises & Epochal Events (1700-1750)
        if self.year == 1707 && self.season == Season::Summer {
            if let Some(mughal) = self.factions.get_mut(&FactionId::Mughal) {
                mughal.treasury = mughal.treasury.saturating_sub(2500);
            }
            for prov in self.provinces.iter_mut().filter(|p| p.owner == FactionId::Mughal) {
                prov.public_order = (prov.public_order - 3).clamp(-10, 10);
            }
            self.notifications.push(Notification {
                turn: self.turn,
                year: self.year,
                title: "Historical Crisis: Death of Badshah Aurangzeb (1707)".into(),
                message: "The aged Mughal Emperor Aurangzeb has died at Ahmednagar! Succession civil war erupts among imperial princes. Mughal authority crumbles (-2500g treasury, -3 public order in all Subahs)!".into(),
                is_urgent: true,
            });
        }

        if self.year == 1713 && self.season == Season::Summer {
            if let Some(maratha) = self.factions.get_mut(&FactionId::Maratha) {
                maratha.treasury += 1000;
            }
            self.notifications.push(Notification {
                turn: self.turn,
                year: self.year,
                title: "Historical Era: Rise of the Peshwa Council (1713)".into(),
                message: "Under Chhatrapati Shahu, Balaji Vishwanath is appointed Peshwa! The Ashta Pradhan council is formalized into a confederate war machine (+1000g treasury)!".into(),
                is_urgent: true,
            });
        }

        if self.year == 1739 && self.season == Season::Summer {
            if let Some(mughal) = self.factions.get_mut(&FactionId::Mughal) {
                mughal.treasury = mughal.treasury.saturating_sub(4000);
            }
            self.provinces[9].base_wealth = self.provinces[9].base_wealth.saturating_sub(300).max(100);
            self.provinces[9].public_order = -8;
            self.notifications.push(Notification {
                turn: self.turn,
                year: self.year,
                title: "Historical Cataclysm: Sack of Delhi by Nader Shah (1739)".into(),
                message: "The Persian host under Nader Shah has sacked Delhi! The Peacock Throne and Koh-i-Noor are plundered. The Mughal treasury is decimated (-4000g treasury, -300g Delhi wealth)!".into(),
                is_urgent: true,
            });
        }

        // 3. Reset army movement
        for army in self.armies.iter_mut() {
            army.has_moved = false;
        }

        // 4. Research progress (frozen if bankrupt; boosted under Republic)
        if let Some(fac) = self.factions.get_mut(&self.player_faction) {
            if !fac.is_bankrupt {
                let speed_step = if fac.government == GovernmentType::Republic { 2 } else { 1 };
                if let Some((tech_id, turns)) = &mut fac.current_research {
                    if *turns <= speed_step {
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
                        *turns -= speed_step;
                    }
                }
            }
        }

        // 5. AI Turn Logic (Mughal & Independent aggression/reinforcement)
        crate::ai::execute_ai_turn(self);

        // 5b. Update National Prestige Matrix (Military, Economic, Enlightenment)
        for (f_id, fac) in self.factions.iter_mut() {
            let prov_count = self.provinces.iter().filter(|p| p.owner == *f_id).count() as u32;
            let mil_power: u32 = self.armies.iter().filter(|a| a.owner == *f_id).map(|a| a.total_soldiers() / 15).sum();
            let mil_prestige = prov_count * 50 + mil_power;

            let eco_prestige = (fac.treasury.max(0) as u32 / 40) + (fac.trade_partners.len() as u32 * 75);
            let enl_prestige = fac.researched_techs.len() as u32 * 120;
            fac.prestige = (mil_prestige, eco_prestige, enl_prestige);
        }

        // 5c. Revolution in Capital Province Check
        let cap_idx = match self.player_faction {
            FactionId::Maratha => 4,
            FactionId::Mughal => 9,
            FactionId::Mysore => 13,
            FactionId::Portugal => 7,
            FactionId::Independent => 0,
            FactionId::Rebels => 0,
            FactionId::Sikh => 15,
            FactionId::Hyderabad => 10,
            FactionId::Bengal => 2,
        };
        if self.provinces[cap_idx].owner == self.player_faction {
            if self.provinces[cap_idx].public_order <= -4 {
                self.capital_unrest_turns += 1;
                if self.capital_unrest_turns >= 2 {
                    self.capital_unrest_turns = 0;
                    let current_gov = self.factions.get(&self.player_faction).map(|f| f.government).unwrap_or_default();
                    let next_gov = match current_gov {
                        GovernmentType::AbsoluteMonarchy => GovernmentType::Republic,
                        GovernmentType::ConstitutionalMonarchy => GovernmentType::Republic,
                        GovernmentType::Republic => GovernmentType::AbsoluteMonarchy,
                    };
                    if let Some(f) = self.factions.get_mut(&self.player_faction) {
                        f.government = next_gov;
                    }
                    self.provinces[cap_idx].public_order = 4;
                    self.notifications.push(Notification {
                        turn: self.turn,
                        year: self.year,
                        title: "POPULAR REVOLUTION IN THE CAPITAL!".into(),
                        message: format!("Unbearable public discord sparked an uprising in the capital seat! The realm has transitioned to a {}. {}", next_gov.name(), next_gov.bonus_desc()),
                        is_urgent: true,
                    });
                }
            } else {
                self.capital_unrest_turns = 0;
            }
        }

        // 6. Check Victory Condition
        self.check_victory_conditions();

        // 7. AutoSave Campaign State to Local Storage!
        let _ = crate::storage::auto_save_campaign(self);
    }

    pub fn disband_army_unit(&mut self, army_idx: usize, unit_idx: usize) -> Result<String, String> {
        if army_idx >= self.armies.len() {
            return Err("Invalid army index.".into());
        }
        let army = &mut self.armies[army_idx];
        if army.owner != self.player_faction {
            return Err("Cannot order disbandment of foreign troops.".into());
        }
        if unit_idx >= army.units.len() {
            return Err("Invalid unit index.".into());
        }
        if army.units.len() <= 1 {
            return Err("Cannot disband the commander's sole bodyguard battalion! Detach or commission another instead.".into());
        }
        let u = army.units.remove(unit_idx);
        let upkeep_saved = u.unit_type.stats().upkeep;
        Ok(format!("Disbanded {} ({} soldiers). Upkeep reduced by {}g/turn.", u.unit_type.name(), u.soldiers, upkeep_saved))
    }

    pub fn disband_garrison_unit(&mut self, prov_idx: usize, unit_idx: usize) -> Result<String, String> {
        if prov_idx >= self.provinces.len() {
            return Err("Invalid province index.".into());
        }
        let prov = &mut self.provinces[prov_idx];
        if prov.owner != self.player_faction {
            return Err("Cannot disband foreign garrison forces.".into());
        }
        if unit_idx >= prov.garrison.len() {
            return Err("Invalid unit index.".into());
        }
        let u = prov.garrison.remove(unit_idx);
        let upkeep_saved = (u.unit_type.stats().upkeep as f32 * 0.75) as u32;
        Ok(format!("Disbanded garrison {} ({} soldiers). Upkeep reduced by {}g/turn.", u.unit_type.name(), u.soldiers, upkeep_saved))
    }

    pub fn merge_army_units(&mut self, army_idx: usize) -> Result<String, String> {
        if army_idx >= self.armies.len() {
            return Err("Invalid army index.".into());
        }
        let army = &mut self.armies[army_idx];
        if army.owner != self.player_faction {
            return Err("Cannot command foreign forces.".into());
        }
        let mut merged_count = 0;
        let mut absorbed_regiments = 0;
        let n = army.units.len();
        let mut to_remove = Vec::new();

        for i in 0..n {
            if to_remove.contains(&i) {
                continue;
            }
            for j in (i + 1)..n {
                if to_remove.contains(&j) {
                    continue;
                }
                if army.units[i].unit_type == army.units[j].unit_type {
                    let cap = army.units[i].max_soldiers;
                    if army.units[i].soldiers < cap && army.units[j].soldiers > 0 {
                        let space = cap - army.units[i].soldiers;
                        let transfer = space.min(army.units[j].soldiers);
                        army.units[i].soldiers += transfer;
                        army.units[j].soldiers -= transfer;
                        army.units[i].experience = army.units[i].experience.max(army.units[j].experience);
                        merged_count += 1;
                        if army.units[j].soldiers == 0 {
                            to_remove.push(j);
                            absorbed_regiments += 1;
                        }
                    }
                }
            }
        }

        to_remove.sort_by(|a, b| b.cmp(a));
        for idx in to_remove {
            army.units.remove(idx);
        }

        if merged_count > 0 {
            Ok(format!("Consolidated ranks across {} depleted battalions ({} empty regiments dissolved into active line).", merged_count, absorbed_regiments))
        } else {
            Err("No under-strength regiments of identical type found to consolidate.".into())
        }
    }

    pub fn replace_minister(&mut self, role: MinisterRole, pool_idx: usize) -> Result<String, String> {
        let p_fac = self.factions.get_mut(&self.player_faction).ok_or("Player faction not found")?;
        if p_fac.treasury < 150 {
            return Err("Insufficient funds! Political appointment requires 150g court fee.".into());
        }
        if pool_idx >= p_fac.minister_pool.len() {
            return Err("Invalid candidate index.".into());
        }
        p_fac.treasury -= 150;
        let mut new_minister = p_fac.minister_pool.remove(pool_idx);
        new_minister.role = role;

        let res_msg = if let Some(pos) = p_fac.ministers.iter().position(|m| m.role == role) {
            let old_m = std::mem::replace(&mut p_fac.ministers[pos], new_minister.clone());
            let candidate = generate_random_statesman(role);
            p_fac.minister_pool.push(candidate);
            format!("Dismissed {}. Appointed {} (★{}) as {}.", old_m.name, new_minister.name, new_minister.stars, role.title())
        } else {
            p_fac.ministers.push(new_minister.clone());
            let candidate = generate_random_statesman(role);
            p_fac.minister_pool.push(candidate);
            format!("Appointed {} (★{}) as {}.", new_minister.name, new_minister.stars, role.title())
        };
        Ok(res_msg)
    }

    pub fn reform_government(&mut self, new_gov: GovernmentType) -> Result<String, String> {
        let p_fac = self.factions.get_mut(&self.player_faction).ok_or("Player faction not found")?;
        if p_fac.government == new_gov {
            return Err("Government structure is already in effect.".into());
        }
        if p_fac.treasury < 600 {
            return Err("Constitutional reform requires 600g in administrative reorganization fees.".into());
        }
        p_fac.treasury -= 600;
        p_fac.government = new_gov;
        self.capital_unrest_turns = 0;
        Ok(format!("Constitutional Reform Enacted! Government restructured to {}. {}", new_gov.name(), new_gov.bonus_desc()))
    }

    pub fn sapper_assault(&mut self, prov_idx: usize) -> Result<String, String> {
        if prov_idx >= self.provinces.len() {
            return Err("Invalid province index.".into());
        }
        let p = &self.provinces[prov_idx];
        if !p.is_under_siege {
            return Err("Province is not under siege.".into());
        }
        let army_idx = p.besieger_army_idx.ok_or("No besieger registered")?;
        let army = self.armies.get(army_idx).ok_or("Besieging army not found")?;
        let has_art = army.units.iter().any(|u| u.unit_type.stats().is_artillery);
        if !has_art {
            return Err("Sapper wall breach requires Artillery (Topchi Cannons or Howitzers) in the besieging army!".into());
        }
        let p_name = p.name.clone();
        let victor = army.owner;
        let p_mut = &mut self.provinces[prov_idx];
        p_mut.owner = victor;
        p_mut.is_under_siege = false;
        p_mut.siege_turns_left = 0;
        p_mut.besieger_army_idx = None;
        p_mut.resistance_to_occupation = 10;
        p_mut.garrison.clear();
        self.notifications.push(Notification {
            turn: self.turn,
            year: self.year,
            title: "Fortress Carried by Sapper Storm!".into(),
            message: format!("Heavy artillery shattered the stone bastions of {}. Assault columns stormed the breaches and captured the seat!", p_name),
            is_urgent: true,
        });
        Ok(format!("Artillery Sapper Breach! Stone bastions of {} collapsed. Fortress carried by storm!", p_name))
    }

    pub fn campaign_progress(&self) -> (usize, bool, bool, usize) {
        let owned = self.provinces.iter().filter(|p| p.owner == self.player_faction).count();
        let holds_delhi = self.provinces.get(9).map(|p| p.owner == self.player_faction).unwrap_or(false);
        let holds_satara = self.provinces.get(4).map(|p| p.owner == self.player_faction).unwrap_or(false);

        let my_p = self.factions.get(&self.player_faction)
            .map(|f| f.prestige.0 + f.prestige.1 + f.prestige.2)
            .unwrap_or(0);
        let mut rank = 1;
        for (f_id, f) in &self.factions {
            if *f_id != self.player_faction && (f.prestige.0 + f.prestige.1 + f.prestige.2) > my_p {
                rank += 1;
            }
        }
        (owned, holds_delhi, holds_satara, rank)
    }

    pub fn check_victory_conditions(&mut self) -> Option<&'static str> {
        let (owned, holds_delhi, holds_satara, rank) = self.campaign_progress();

        if !self.victory_claimed {
            // Long Campaign Victory (14 provinces including Delhi & Satara)
            if owned >= 14 && holds_delhi && holds_satara {
                self.victory_claimed = true;
                self.notifications.push(Notification {
                    turn: self.turn,
                    year: self.year,
                    title: "IMPERIAL TRIUMPH: GRAND CAMPAIGN VICTORY!".into(),
                    message: format!("Supreme Hegemony Achieved! The {} controls {} Subahs including Delhi and Satara. The Indian Subcontinent unites under your sovereign rule!", self.player_faction.name(), owned),
                    is_urgent: true,
                });
                return Some("Long Campaign Victory Achieved");
            }
            // Prestige Victory (leading prestige by 1750)
            if self.year >= 1750 && rank == 1 {
                self.victory_claimed = true;
                self.notifications.push(Notification {
                    turn: self.turn,
                    year: self.year,
                    title: "PRESTIGE VICTORY: FOREMOST EMPIRE OF ASIA!".into(),
                    message: format!("By 1750, the {} stands as the undisputed foremost power of the Indian Subcontinent in arms, commerce, and enlightenment!", self.player_faction.name()),
                    is_urgent: true,
                });
                return Some("Prestige Victory Achieved");
            }
        }
        None
    }
}
