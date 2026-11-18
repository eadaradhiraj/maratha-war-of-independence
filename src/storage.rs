#[cfg(not(target_arch = "wasm32"))]
use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::game_state::GameState;
use crate::models::{
    Army, BuildingType, CampaignVictoryCondition, CommodityType, FactionId, GovernmentType, Minister, RecruitOrder, Season, TaxLevel, UnitInstance,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvinceSaveData {
    pub id: usize,
    pub owner: FactionId,
    pub tax_level: TaxLevel,
    pub public_order: i32,
    pub buildings: Vec<BuildingType>,
    pub garrison: Vec<UnitInstance>,
    pub recruitment_queue: Vec<RecruitOrder>,
    pub is_under_siege: bool,
    #[serde(default)]
    pub siege_turns_left: u32,
    #[serde(default)]
    pub is_port_blockaded: bool,
    #[serde(default)]
    pub resistance_to_occupation: i32,
    #[serde(default)]
    pub revealed_turns: u32,
    #[serde(default)]
    pub sabotaged_turns: u32,
    #[serde(default)]
    pub commodity: Option<CommodityType>,
    #[serde(default)]
    pub upper_tax: TaxLevel,
    #[serde(default)]
    pub lower_tax: TaxLevel,
    #[serde(default)]
    pub road_level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionSaveData {
    pub id: FactionId,
    pub treasury: i32,
    pub researched_techs: Vec<String>,
    pub current_research: Option<(String, u32)>,
    pub ministers: Vec<Minister>,
    pub wars: Vec<FactionId>,
    pub trade_partners: Vec<FactionId>,
    #[serde(default)]
    pub alliances: Vec<FactionId>,
    #[serde(default)]
    pub relations: std::collections::HashMap<FactionId, i32>,
    #[serde(default)]
    pub is_bankrupt: bool,
    #[serde(default)]
    pub prestige: (u32, u32, u32),
    #[serde(default)]
    pub vassals: Vec<FactionId>,
    #[serde(default)]
    pub overlord: Option<FactionId>,
    #[serde(default)]
    pub military_access_with: Vec<FactionId>,
    #[serde(default)]
    pub government: GovernmentType,
    #[serde(default)]
    pub minister_pool: Vec<Minister>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SaveSlotMetadata {
    pub id: String,
    pub display_name: String,
    pub player_faction: FactionId,
    pub turn: u32,
    pub year: u32,
    pub season: Season,
    pub provinces_owned: usize,
    pub timestamp_str: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignSaveData {
    pub meta: SaveSlotMetadata,
    pub turn: u32,
    pub year: u32,
    pub season: Season,
    pub player_faction: FactionId,
    pub factions: Vec<FactionSaveData>,
    pub provinces: Vec<ProvinceSaveData>,
    pub armies: Vec<Army>,
    pub victory_claimed: bool,
    #[serde(default)]
    pub victory_mode: CampaignVictoryCondition,
    #[serde(default)]
    pub capital_unrest_turns: u32,
}

// -------------------------------------------------------------
// Low-level Storage Bridge
// -------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
mod ffi {
    #[link(wasm_import_module = "env")]
    unsafe extern "C" {
        pub fn js_storage_set(key_ptr: *const u8, key_len: usize, val_ptr: *const u8, val_len: usize) -> i32;
        pub fn js_storage_get_len(key_ptr: *const u8, key_len: usize) -> i32;
        pub fn js_storage_get(key_ptr: *const u8, key_len: usize, dst_ptr: *mut u8, dst_max_len: usize) -> i32;
        pub fn js_storage_delete(key_ptr: *const u8, key_len: usize) -> i32;
        pub fn js_storage_download_file(fname_ptr: *const u8, fname_len: usize, data_ptr: *const u8, data_len: usize) -> i32;
        pub fn js_storage_trigger_file_import() -> i32;
        pub fn js_storage_get_imported_save_len() -> i32;
        pub fn js_storage_get_imported_save(dst_ptr: *mut u8, dst_max_len: usize) -> i32;
    }
}

#[cfg(not(target_arch = "wasm32"))]
static NATIVE_STORE: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);

pub fn storage_set(key: &str, val: &str) -> bool {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        ffi::js_storage_set(key.as_ptr(), key.len(), val.as_ptr(), val.len()) == 1
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut lock = NATIVE_STORE.lock().unwrap();
        if lock.is_none() {
            *lock = Some(HashMap::new());
        }
        lock.as_mut().unwrap().insert(key.to_string(), val.to_string());
        true
    }
}

pub fn storage_get(key: &str) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        let len = ffi::js_storage_get_len(key.as_ptr(), key.len());
        if len <= 0 {
            return None;
        }
        let mut buf = vec![0u8; len as usize];
        let bytes_read = ffi::js_storage_get(key.as_ptr(), key.len(), buf.as_mut_ptr(), buf.len());
        if bytes_read <= 0 {
            return None;
        }
        String::from_utf8(buf).ok()
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let lock = NATIVE_STORE.lock().unwrap();
        lock.as_ref().and_then(|m| m.get(key).cloned())
    }
}

pub fn storage_delete(key: &str) -> bool {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        ffi::js_storage_delete(key.as_ptr(), key.len()) == 1
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut lock = NATIVE_STORE.lock().unwrap();
        if let Some(m) = lock.as_mut() {
            m.remove(key).is_some()
        } else {
            false
        }
    }
}

pub fn trigger_browser_download(filename: &str, content: &str) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        ffi::js_storage_download_file(
            filename.as_ptr(),
            filename.len(),
            content.as_ptr(),
            content.len(),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (filename, content);
    }
}

pub fn trigger_browser_file_import() {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        ffi::js_storage_trigger_file_import();
    }
}

pub fn check_pending_imported_save() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        let len = ffi::js_storage_get_imported_save_len();
        if len <= 0 {
            return None;
        }
        let mut buf = vec![0u8; len as usize];
        let bytes_read = ffi::js_storage_get_imported_save(buf.as_mut_ptr(), buf.len());
        if bytes_read <= 0 {
            return None;
        }
        String::from_utf8(buf).ok()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}

// -------------------------------------------------------------
// Campaign Catalog & High-Level Save / Load API
// -------------------------------------------------------------

const CATALOG_KEY: &str = "etw_catalog_v1";

pub fn list_save_slots() -> Vec<SaveSlotMetadata> {
    if let Some(raw) = storage_get(CATALOG_KEY) {
        if let Ok(list) = serde_json::from_str::<Vec<SaveSlotMetadata>>(&raw) {
            return list;
        }
    }
    Vec::new()
}

pub fn get_latest_save_slot() -> Option<SaveSlotMetadata> {
    let slots = list_save_slots();
    slots.first().cloned()
}

pub fn save_campaign_state(
    slot_id: &str,
    display_name: &str,
    game: &GameState,
) -> Result<SaveSlotMetadata, String> {
    let owned = game
        .provinces
        .iter()
        .filter(|p| p.owner == game.player_faction)
        .count();

    let season_str = match game.season {
        Season::Summer => "Summer",
        Season::Winter => "Winter",
    };

    let meta = SaveSlotMetadata {
        id: slot_id.to_string(),
        display_name: display_name.to_string(),
        player_faction: game.player_faction,
        turn: game.turn,
        year: game.year,
        season: game.season,
        provinces_owned: owned,
        timestamp_str: format!("Turn {} - {} {} A.D.", game.turn, season_str, game.year),
    };

    let factions: Vec<FactionSaveData> = game
        .factions
        .values()
        .map(|f| FactionSaveData {
            id: f.id,
            treasury: f.treasury,
            researched_techs: f.researched_techs.clone(),
            current_research: f.current_research.clone(),
            ministers: f.ministers.clone(),
            wars: f.wars.clone(),
            trade_partners: f.trade_partners.clone(),
            alliances: f.alliances.clone(),
            relations: f.relations.clone(),
            is_bankrupt: f.is_bankrupt,
            prestige: f.prestige,
            vassals: f.vassals.clone(),
            overlord: f.overlord,
            military_access_with: f.military_access_with.clone(),
            government: f.government,
            minister_pool: f.minister_pool.clone(),
        })
        .collect();

    let provinces: Vec<ProvinceSaveData> = game
        .provinces
        .iter()
        .map(|p| ProvinceSaveData {
            id: p.id,
            owner: p.owner,
            tax_level: p.tax_level,
            public_order: p.public_order,
            buildings: p.buildings.clone(),
            garrison: p.garrison.clone(),
            recruitment_queue: p.recruitment_queue.clone(),
            is_under_siege: p.is_under_siege,
            siege_turns_left: p.siege_turns_left,
            is_port_blockaded: p.is_port_blockaded,
            resistance_to_occupation: p.resistance_to_occupation,
            revealed_turns: p.revealed_turns,
            sabotaged_turns: p.sabotaged_turns,
            commodity: p.commodity,
            upper_tax: p.upper_tax,
            lower_tax: p.lower_tax,
            road_level: p.road_level,
        })
        .collect();

    let save_data = CampaignSaveData {
        meta: meta.clone(),
        turn: game.turn,
        year: game.year,
        season: game.season,
        player_faction: game.player_faction,
        factions,
        provinces,
        armies: game.armies.clone(),
        victory_claimed: game.victory_claimed,
        victory_mode: game.victory_mode,
        capital_unrest_turns: game.capital_unrest_turns,
    };

    let json_str = serde_json::to_string(&save_data)
        .map_err(|e| format!("Serialization error: {}", e))?;

    let slot_key = format!("etw_save_{}", slot_id);
    if !storage_set(&slot_key, &json_str) {
        return Err("Failed to write save data to local storage".to_string());
    }

    // Update catalog (move updated slot to front)
    let mut catalog = list_save_slots();
    catalog.retain(|item| item.id != slot_id);
    catalog.insert(0, meta.clone());

    if let Ok(cat_json) = serde_json::to_string(&catalog) {
        let _ = storage_set(CATALOG_KEY, &cat_json);
    }

    Ok(meta)
}

pub fn load_campaign_state(slot_id: &str, game: &mut GameState) -> Result<SaveSlotMetadata, String> {
    let slot_key = format!("etw_save_{}", slot_id);
    let raw = storage_get(&slot_key)
        .ok_or_else(|| format!("Save slot '{}' not found in storage", slot_id))?;

    let save_data: CampaignSaveData = serde_json::from_str(&raw)
        .map_err(|e| format!("Corrupt or incompatible save data: {}", e))?;

    let meta = save_data.meta.clone();
    game.apply_save_data(save_data);
    Ok(meta)
}

pub fn delete_campaign_save(slot_id: &str) -> Result<(), String> {
    let slot_key = format!("etw_save_{}", slot_id);
    storage_delete(&slot_key);

    let mut catalog = list_save_slots();
    catalog.retain(|item| item.id != slot_id);

    if let Ok(cat_json) = serde_json::to_string(&catalog) {
        let _ = storage_set(CATALOG_KEY, &cat_json);
    }

    Ok(())
}

pub fn auto_save_campaign(game: &GameState) -> Result<SaveSlotMetadata, String> {
    let name = format!("AutoSave ({})", game.player_faction.name());
    save_campaign_state("autosave", &name, game)
}

pub fn quick_save_campaign(game: &GameState) -> Result<SaveSlotMetadata, String> {
    let name = format!("QuickSave - Turn {} ({})", game.turn, game.player_faction.name());
    save_campaign_state("quicksave", &name, game)
}

pub fn export_save_to_json(slot_id: &str) -> Option<String> {
    let slot_key = format!("etw_save_{}", slot_id);
    storage_get(&slot_key)
}

pub fn import_save_from_json(json_str: &str, game: &mut GameState) -> Result<SaveSlotMetadata, String> {
    let save_data: CampaignSaveData = serde_json::from_str(json_str)
        .map_err(|e| format!("Invalid save JSON: {}", e))?;

    let slot_id = format!("import_{}", save_data.turn);
    let name = format!("Imported - {}", save_data.meta.display_name);

    let mut meta = save_data.meta.clone();
    meta.id = slot_id.clone();
    meta.display_name = name.clone();

    // Write to storage
    let slot_key = format!("etw_save_{}", slot_id);
    storage_set(&slot_key, json_str);

    let mut catalog = list_save_slots();
    catalog.retain(|item| item.id != slot_id);
    catalog.insert(0, meta.clone());
    if let Ok(cat_json) = serde_json::to_string(&catalog) {
        let _ = storage_set(CATALOG_KEY, &cat_json);
    }

    game.apply_save_data(save_data);
    Ok(meta)
}
