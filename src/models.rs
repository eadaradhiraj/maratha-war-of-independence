use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FactionId {
    Maratha,
    Mughal,
    Mysore,
    Portugal,
    Independent,
    Rebels,
    Sikh,
    Hyderabad,
    Bengal,
}

impl FactionId {
    pub fn name(&self) -> &'static str {
        match self {
            FactionId::Maratha => "Maratha Confederacy",
            FactionId::Mughal => "Mughal Empire",
            FactionId::Mysore => "Sultanate of Mysore",
            FactionId::Portugal => "Kingdom of Portugal",
            FactionId::Independent => "Independent Nawabs",
            FactionId::Rebels => "Deccan Insurgents & Rebels",
            FactionId::Sikh => "Sikh Empire (Khalsa Fauj)",
            FactionId::Hyderabad => "Nizamate of Hyderabad",
            FactionId::Bengal => "Nawab of Bengal",
        }
    }

    pub fn flag_colors(&self) -> (Color, Color) {
        match self {
            // Saffron & Gold / Red
            FactionId::Maratha => (Color::from_rgba(235, 115, 20, 255), Color::from_rgba(255, 195, 0, 255)),
            // Mughal Green & Gold Sun
            FactionId::Mughal => (Color::from_rgba(25, 95, 45, 255), Color::from_rgba(240, 200, 60, 255)),
            // Mysore Crimson & Tiger Gold
            FactionId::Mysore => (Color::from_rgba(160, 30, 45, 255), Color::from_rgba(255, 175, 40, 255)),
            // Portuguese White & Navy Blue
            FactionId::Portugal => (Color::from_rgba(240, 240, 240, 255), Color::from_rgba(25, 60, 160, 255)),
            // Independent Slate & Silver
            FactionId::Independent => (Color::from_rgba(110, 110, 120, 255), Color::from_rgba(200, 200, 210, 255)),
            // Rebels Black & Crimson
            FactionId::Rebels => (Color::from_rgba(45, 15, 15, 255), Color::from_rgba(200, 50, 50, 255)),
            // Sikh Royal Blue & Saffron
            FactionId::Sikh => (Color::from_rgba(20, 60, 160, 255), Color::from_rgba(255, 185, 30, 255)),
            // Hyderabad Cyan & Gold
            FactionId::Hyderabad => (Color::from_rgba(20, 150, 160, 255), Color::from_rgba(255, 215, 70, 255)),
            // Bengal Emerald & Silver
            FactionId::Bengal => (Color::from_rgba(30, 120, 70, 255), Color::from_rgba(220, 220, 230, 255)),
        }
    }

    pub fn primary_color(&self) -> Color {
        self.flag_colors().0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiploAction {
    TradeAgreement,
    SmallGift,
    LavishGift,
    Alliance,
    Ceasefire,
    DemandTribute,
    DeclareWar,
    DemandTerritory(usize),
    OfferTerritory(usize),
    SubjugateVassal,
    GrantMilitaryAccess,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CampaignStance {
    #[default]
    Normal,
    Fortified,
    Raiding,
    Ambush,
}

impl CampaignStance {
    pub fn name(&self) -> &'static str {
        match self {
            CampaignStance::Normal => "March / Standard",
            CampaignStance::Fortified => "Fortified Camp (+25% Def)",
            CampaignStance::Raiding => "Ganimi Kava (Raiding)",
            CampaignStance::Ambush => "Concealed Ambush (75% Surprise)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommodityType {
    Spices,
    Cotton,
    Silk,
    Saltpetre,
}

impl CommodityType {
    pub fn name(&self) -> &'static str {
        match self {
            CommodityType::Spices => "Malabar Spices & Pepper",
            CommodityType::Cotton => "Gujarat Fine Cotton",
            CommodityType::Silk => "Bengal Raw Silk",
            CommodityType::Saltpetre => "Malwa Gunpowder Saltpetre",
        }
    }

    pub fn base_price(&self) -> i32 {
        match self {
            CommodityType::Spices => 50,
            CommodityType::Cotton => 35,
            CommodityType::Silk => 60,
            CommodityType::Saltpetre => 45,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeneralAncillary {
    FrenchArtilleryMaster,
    BhaktiChaplain,
    WarElephantMaster,
    SwissDrillmaster,
}

impl GeneralAncillary {
    pub fn name(&self) -> &'static str {
        match self {
            GeneralAncillary::FrenchArtilleryMaster => "French Artillery Master (+2 Firepower)",
            GeneralAncillary::BhaktiChaplain => "Bhakti Chaplain (+2 Morale)",
            GeneralAncillary::WarElephantMaster => "War Elephant Master (+2 Melee)",
            GeneralAncillary::SwissDrillmaster => "European Drillmaster (+2 Defense)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CovertAction {
    Infiltrate,
    SabotageInfrastructure,
    AssassinateGeneral,
}

impl CovertAction {
    pub fn name(&self) -> &'static str {
        match self {
            CovertAction::Infiltrate => "Infiltrate Subah (Lift Fog 4 Turns)",
            CovertAction::SabotageInfrastructure => "Sabotage Infrastructure (Disable 3 Turns)",
            CovertAction::AssassinateGeneral => "Assassinate Enemy Commander",
        }
    }

    pub fn cost(&self) -> i32 {
        match self {
            CovertAction::Infiltrate => 250,
            CovertAction::SabotageInfrastructure => 400,
            CovertAction::AssassinateGeneral => 650,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OccupationChoice {
    Peaceful,
    SackAndPillage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Season {
    Summer,
    Winter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TaxLevel {
    Exempt,
    Low,
    #[default]
    Normal,
    High,
    Extortionate,
}

impl TaxLevel {
    pub fn rate_multiplier(&self) -> f32 {
        match self {
            TaxLevel::Exempt => 0.0,
            TaxLevel::Low => 0.65,
            TaxLevel::Normal => 1.0,
            TaxLevel::High => 1.45,
            TaxLevel::Extortionate => 2.0,
        }
    }

    pub fn public_order_penalty(&self) -> i32 {
        match self {
            TaxLevel::Exempt => 4,
            TaxLevel::Low => 2,
            TaxLevel::Normal => 0,
            TaxLevel::High => -3,
            TaxLevel::Extortionate => -7,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            TaxLevel::Exempt => TaxLevel::Low,
            TaxLevel::Low => TaxLevel::Normal,
            TaxLevel::Normal => TaxLevel::High,
            TaxLevel::High => TaxLevel::Extortionate,
            TaxLevel::Extortionate => TaxLevel::Exempt,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            TaxLevel::Exempt => "Exempt",
            TaxLevel::Low => "Low",
            TaxLevel::Normal => "Normal",
            TaxLevel::High => "High",
            TaxLevel::Extortionate => "Extortionate",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnitType {
    // Infantry
    ZamindariLevies,
    HinduMatchlocks,
    SepoyLineInfantry,
    MansabdarGuards,
    JezailSkirmishers,
    // Cavalry
    BargirCavalry,
    SilhadarLancers,
    PindariRaiders,
    // Artillery
    TopchiCannons,
}

impl UnitType {
    pub fn name(&self) -> &'static str {
        match self {
            UnitType::ZamindariLevies => "Zamindari Levies",
            UnitType::HinduMatchlocks => "Hindu Matchlockmen",
            UnitType::SepoyLineInfantry => "Sepoy Line Infantry",
            UnitType::MansabdarGuards => "Mansabdari Royal Guards",
            UnitType::JezailSkirmishers => "Jezail Skirmishers",
            UnitType::BargirCavalry => "Bargir Light Cavalry",
            UnitType::SilhadarLancers => "Silhadar Heavy Lancers",
            UnitType::PindariRaiders => "Pindari Raiding Horse",
            UnitType::TopchiCannons => "Topchi Bronze Cannons",
        }
    }

    pub fn stats(&self) -> UnitStats {
        match self {
            UnitType::ZamindariLevies => UnitStats {
                cost: 250,
                upkeep: 60,
                base_soldiers: 120,
                melee_attack: 4,
                defense: 3,
                firepower: 2,
                range: 60,
                morale: 4,
                is_cavalry: false,
                is_artillery: false,
            },
            UnitType::HinduMatchlocks => UnitStats {
                cost: 450,
                upkeep: 110,
                base_soldiers: 100,
                melee_attack: 6,
                defense: 5,
                firepower: 7,
                range: 90,
                morale: 6,
                is_cavalry: false,
                is_artillery: false,
            },
            UnitType::SepoyLineInfantry => UnitStats {
                cost: 650,
                upkeep: 150,
                base_soldiers: 120,
                melee_attack: 8,
                defense: 8,
                firepower: 9,
                range: 100,
                morale: 8,
                is_cavalry: false,
                is_artillery: false,
            },
            UnitType::MansabdarGuards => UnitStats {
                cost: 850,
                upkeep: 210,
                base_soldiers: 90,
                melee_attack: 11,
                defense: 10,
                firepower: 8,
                range: 90,
                morale: 11,
                is_cavalry: false,
                is_artillery: false,
            },
            UnitType::JezailSkirmishers => UnitStats {
                cost: 480,
                upkeep: 120,
                base_soldiers: 60,
                melee_attack: 4,
                defense: 4,
                firepower: 10,
                range: 130,
                morale: 5,
                is_cavalry: false,
                is_artillery: false,
            },
            UnitType::BargirCavalry => UnitStats {
                cost: 550,
                upkeep: 130,
                base_soldiers: 60,
                melee_attack: 8,
                defense: 5,
                firepower: 0,
                range: 0,
                morale: 7,
                is_cavalry: true,
                is_artillery: false,
            },
            UnitType::SilhadarLancers => UnitStats {
                cost: 800,
                upkeep: 190,
                base_soldiers: 60,
                melee_attack: 13,
                defense: 8,
                firepower: 0,
                range: 0,
                morale: 9,
                is_cavalry: true,
                is_artillery: false,
            },
            UnitType::PindariRaiders => UnitStats {
                cost: 400,
                upkeep: 90,
                base_soldiers: 60,
                melee_attack: 7,
                defense: 4,
                firepower: 3,
                range: 50,
                morale: 5,
                is_cavalry: true,
                is_artillery: false,
            },
            UnitType::TopchiCannons => UnitStats {
                cost: 900,
                upkeep: 220,
                base_soldiers: 30,
                melee_attack: 2,
                defense: 4,
                firepower: 15,
                range: 200,
                morale: 6,
                is_cavalry: false,
                is_artillery: true,
            },
        }
    }

    pub fn faction_allowed(&self, faction: FactionId) -> bool {
        match self {
            UnitType::BargirCavalry | UnitType::SilhadarLancers | UnitType::PindariRaiders => {
                faction == FactionId::Maratha
            }
            UnitType::MansabdarGuards | UnitType::JezailSkirmishers => {
                faction == FactionId::Mughal
            }
            _ => true,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct UnitStats {
    pub cost: u32,
    pub upkeep: u32,
    pub base_soldiers: u32,
    pub melee_attack: u8,
    pub defense: u8,
    pub firepower: u8,
    pub range: u16,
    pub morale: u8,
    pub is_cavalry: bool,
    pub is_artillery: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitInstance {
    pub id: u64,
    pub unit_type: UnitType,
    pub soldiers: u32,
    pub max_soldiers: u32,
    pub experience: u8,
    pub morale: f32,
}

impl UnitInstance {
    pub fn new(id: u64, unit_type: UnitType) -> Self {
        let stats = unit_type.stats();
        Self {
            id,
            unit_type,
            soldiers: stats.base_soldiers,
            max_soldiers: stats.base_soldiers,
            experience: 0,
            morale: stats.morale as f32,
        }
    }

    pub fn combat_power(&self) -> f32 {
        let stats = self.unit_type.stats();
        let ratio = self.soldiers as f32 / stats.base_soldiers.max(1) as f32;
        let base_power = (stats.melee_attack as f32 * 1.5 + stats.defense as f32 + stats.firepower as f32 * 1.8) * ratio;
        base_power * (1.0 + self.experience as f32 * 0.1)
    }

    pub fn replenish(&mut self, amount: u32) {
        if self.soldiers >= self.max_soldiers {
            return;
        }
        let healed = amount.min(self.max_soldiers - self.soldiers);
        let old_vets = self.soldiers as f32 * self.experience as f32;
        self.soldiers += healed;
        if self.soldiers > 0 {
            self.experience = (old_vets / self.soldiers as f32).round() as u8;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildingType {
    Fortress,
    Barracks,
    Marketplace,
    TradingPort,
    TempleMosque,
    CollegeOfLogic,
    TextileLoom,
    RoadNetwork,
}

impl BuildingType {
    pub fn name(&self) -> &'static str {
        match self {
            BuildingType::Fortress => "Hill Fortress (Gadh)",
            BuildingType::Barracks => "Regimental Encampment",
            BuildingType::Marketplace => "Bazaar & Trade Guild",
            BuildingType::TradingPort => "Deepwater Trading Port",
            BuildingType::TempleMosque => "Grand Sacred Temple / Mosque",
            BuildingType::CollegeOfLogic => "Madrasa / Gurukul Academy",
            BuildingType::TextileLoom => "Textile Mill & Loom",
            BuildingType::RoadNetwork => "Grand Royal Highway",
        }
    }

    pub fn cost(&self) -> u32 {
        match self {
            BuildingType::Fortress => 1200,
            BuildingType::Barracks => 800,
            BuildingType::Marketplace => 700,
            BuildingType::TradingPort => 1000,
            BuildingType::TempleMosque => 600,
            BuildingType::CollegeOfLogic => 950,
            BuildingType::TextileLoom => 650,
            BuildingType::RoadNetwork => 500,
        }
    }

    pub fn effect_description(&self) -> &'static str {
        match self {
            BuildingType::Fortress => "+3 Public Order, +25% Defense garrison",
            BuildingType::Barracks => "Enables veteran line infantry & cavalry recruitment",
            BuildingType::Marketplace => "+220 Tax Revenue per turn, +10% regional growth",
            BuildingType::TradingPort => "+350 Sea Trade Revenue per turn",
            BuildingType::TempleMosque => "+4 Public Order, converts province culture",
            BuildingType::CollegeOfLogic => "+15% Technology Research Speed",
            BuildingType::TextileLoom => "+180g Commodity Export Revenue",
            BuildingType::RoadNetwork => "+35% Army Movement, +10% Trade Efficiency",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Minister {
    pub role: MinisterRole,
    pub name: String,
    pub stars: u8,
    pub loyalty: u8,
    pub trait_desc: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MinisterRole {
    HeadOfState,
    PeshwaGrandVizier,
    Treasury,
    War,
    Justice,
}

impl MinisterRole {
    pub fn title(&self) -> &'static str {
        match self {
            MinisterRole::HeadOfState => "Chhatrapati / Padishah (Ruler)",
            MinisterRole::PeshwaGrandVizier => "Peshwa / Grand Vizier (Prime Minister)",
            MinisterRole::Treasury => "Diwan / Pant Amatya (Finance)",
            MinisterRole::War => "Senapati / Mir Bakhshi (Commander-in-Chief)",
            MinisterRole::Justice => "Nyayadhish / Qadi (Justice & Order)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TechCategory {
    Military,
    Industry,
    Philosophy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechNode {
    pub id: &'static str,
    pub name: &'static str,
    pub category: TechCategory,
    pub cost_turns: u32,
    pub description: &'static str,
    pub prerequisite: Option<&'static str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Army {
    pub id: u64,
    pub owner: FactionId,
    pub province_id: usize,
    pub units: Vec<UnitInstance>,
    pub general_name: String,
    pub general_stars: u8,
    pub has_moved: bool,
    #[serde(default)]
    pub stance: CampaignStance,
    #[serde(default)]
    pub retinue: Vec<GeneralAncillary>,
}

impl Army {
    pub fn total_soldiers(&self) -> u32 {
        self.units.iter().map(|u| u.soldiers).sum()
    }

    pub fn total_combat_power(&self) -> f32 {
        let raw: f32 = self.units.iter().map(|u| u.combat_power()).sum();
        let stance_mult = match self.stance {
            CampaignStance::Fortified => 1.25,
            CampaignStance::Ambush => 1.35,
            _ => 1.0,
        };
        let retinue_bonus = self.retinue.len() as f32 * 0.05;
        raw * (1.0 + self.general_stars as f32 * 0.08 + retinue_bonus) * stance_mult
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecruitOrder {
    pub unit_type: UnitType,
    pub turns_remaining: u32,
}

#[derive(Debug, Clone)]
pub struct Province {
    pub id: usize,
    pub name: String,
    pub capital_name: String,
    pub port_name: Option<String>,
    pub owner: FactionId,
    pub original_owner: FactionId,
    pub polygon: Vec<Vec2>,
    pub center: Vec2,
    pub capital_pos: Vec2,
    pub port_pos: Option<Vec2>,
    pub neighbors: Vec<usize>,
    pub population: u32,
    pub base_wealth: u32,
    pub tax_level: TaxLevel,
    pub public_order: i32,
    pub hindu_pct: f32,
    pub muslim_pct: f32,
    pub buildings: Vec<BuildingType>,
    pub garrison: Vec<UnitInstance>,
    pub recruitment_queue: Vec<RecruitOrder>,
    pub is_under_siege: bool,
    pub siege_turns_left: u32,
    pub besieger_army_idx: Option<usize>,
    pub is_port_blockaded: bool,
    pub resistance_to_occupation: i32,
    pub revealed_turns: u32,
    pub sabotaged_turns: u32,
    pub commodity: Option<CommodityType>,
    pub upper_tax: TaxLevel,
    pub lower_tax: TaxLevel,
    pub road_level: u8,
}

#[derive(Debug, Clone)]
pub struct BattlePhaseLog {
    pub title: String,
    pub attacker_action: String,
    pub defender_action: String,
    pub attacker_losses: u32,
    pub defender_losses: u32,
}

#[derive(Debug, Clone)]
pub struct BattleReport {
    pub attacker_faction: FactionId,
    pub defender_faction: FactionId,
    pub province_id: usize,
    pub province_name: String,
    pub attacker_initial: u32,
    pub defender_initial: u32,
    pub attacker_losses: u32,
    pub defender_losses: u32,
    pub attacker_won: bool,
    pub phases: Vec<BattlePhaseLog>,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub turn: u32,
    pub year: u32,
    pub title: String,
    pub message: String,
    pub is_urgent: bool,
}
