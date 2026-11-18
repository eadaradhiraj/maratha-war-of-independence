use macroquad::prelude::*;
use crate::game_state::{ActiveTab, GameState};
use crate::models::{
    BuildingType, CampaignStance, CovertAction, DiploAction, FactionId, GovernmentType, MinisterRole, TaxLevel, UnitType,
};
use std::sync::OnceLock;

pub struct GameFonts {
    pub regular: Font,
    pub bold: Font,
}

pub static FONTS: OnceLock<GameFonts> = OnceLock::new();

pub fn init_fonts() {
    let reg_bytes = include_bytes!("../assets/fonts/regular.ttf");
    let bold_bytes = include_bytes!("../assets/fonts/bold.ttf");
    if let (Ok(reg), Ok(bld)) = (
        load_ttf_font_from_bytes(reg_bytes),
        load_ttf_font_from_bytes(bold_bytes),
    ) {
        let _ = FONTS.set(GameFonts { regular: reg, bold: bld });
    }
}

pub fn draw_text(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    if let Some(fonts) = FONTS.get() {
        macroquad::text::draw_text_ex(
            text,
            x,
            y,
            TextParams {
                font: Some(&fonts.regular),
                font_size: font_size.round().max(10.0) as u16,
                color,
                ..Default::default()
            },
        );
    } else {
        macroquad::text::draw_text(text, x, y, font_size, color);
    }
}

pub fn draw_text_bold(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    if let Some(fonts) = FONTS.get() {
        macroquad::text::draw_text_ex(
            text,
            x,
            y,
            TextParams {
                font: Some(&fonts.bold),
                font_size: font_size.round().max(10.0) as u16,
                color,
                ..Default::default()
            },
        );
    } else {
        macroquad::text::draw_text(text, x, y, font_size, color);
    }
}

pub struct UiState {
    pub hovered_province: Option<usize>,
    pub camera_pos: Vec2,
    pub camera_zoom: f32,
    pub is_dragging: bool,
    pub drag_start_mouse: Vec2,
    pub drag_start_cam: Vec2,
    pub show_recruit_modal: bool,
    pub show_build_modal: bool,
    pub show_pause_menu: bool,
    pub selected_save_slot: Option<String>,
    pub save_name_input: String,
    pub delete_confirm_slot: Option<String>,
    pub status_toast: Option<(String, f64)>,
    pub save_list_cache: Vec<crate::storage::SaveSlotMetadata>,
    pub return_tab_after_menu: ActiveTab,
}

impl UiState {
    pub fn new() -> Self {
        let saves = crate::storage::list_save_slots();
        let first_id = saves.first().map(|s| s.id.clone());
        Self {
            hovered_province: None,
            camera_pos: Vec2::new(100.0, 50.0),
            camera_zoom: 0.85,
            is_dragging: false,
            drag_start_mouse: Vec2::ZERO,
            drag_start_cam: Vec2::ZERO,
            show_recruit_modal: false,
            show_build_modal: false,
            show_pause_menu: false,
            selected_save_slot: first_id,
            save_name_input: "Campaign Save".to_string(),
            delete_confirm_slot: None,
            status_toast: None,
            save_list_cache: saves,
            return_tab_after_menu: ActiveTab::Map,
        }
    }

    pub fn set_toast(&mut self, msg: &str) {
        self.status_toast = Some((msg.to_string(), get_time()));
    }

    pub fn refresh_saves(&mut self) {
        self.save_list_cache = crate::storage::list_save_slots();
        if self.selected_save_slot.is_none()
            || !self
                .save_list_cache
                .iter()
                .any(|s| Some(&s.id) == self.selected_save_slot.as_ref())
        {
            self.selected_save_slot = self.save_list_cache.first().map(|s| s.id.clone());
        }
    }
}

pub fn render_top_bar(game: &mut GameState, ui: &mut UiState) {
    let screen_w = screen_width();
    let bar_h = 48.0;

    // Header Background: Dark mahogany wood / bronze frame
    draw_rectangle(0.0, 0.0, screen_w, bar_h, Color::from_rgba(24, 20, 16, 250));
    draw_line(0.0, bar_h, screen_w, bar_h, 2.0, Color::from_rgba(200, 160, 80, 200));

    // Faction Crest / Shield
    let (c1, c2) = game.player_faction.flag_colors();
    draw_rectangle(12.0, 8.0, 32.0, 32.0, c1);
    draw_rectangle_lines(12.0, 8.0, 32.0, 32.0, 2.0, c2);
    draw_text("MWI", 14.0, 28.0, 13.0, c2);

    // Faction Name & Ruler
    let (fac_name, leader_name, treasury) = {
        let f = game.factions.get(&game.player_faction).unwrap();
        (f.name.clone(), f.leader_name.clone(), f.treasury)
    };
    let title_text = if screen_w < 1220.0 {
        fac_name
    } else {
        format!("{} | {}", fac_name, leader_name)
    };
    draw_text_bold(
        &title_text,
        55.0,
        22.0,
        16.0,
        Color::from_rgba(245, 235, 215, 255),
    );

    // Date & Season
    let season_str = match game.season {
        crate::models::Season::Summer => "Summer",
        crate::models::Season::Winter => "Winter",
    };
    draw_text(
        &format!("Turn {} | {} {} A.D.", game.turn, season_str, game.year),
        55.0,
        40.0,
        13.5,
        Color::from_rgba(210, 190, 140, 255),
    );

    let is_bankrupt = game.factions.get(&game.player_faction).map(|f| f.is_bankrupt).unwrap_or(false);
    if is_bankrupt {
        draw_text_bold(
            "⚠ BANKRUPTCY: MUTINIES & DESERTION ACTIVE",
            340.0,
            29.0,
            14.0,
            Color::from_rgba(255, 65, 65, 255),
        );
    }

    // End Turn Button
    let end_btn_w = 95.0;
    let end_btn_x = screen_w - end_btn_w - 10.0;
    draw_rectangle(
        end_btn_x,
        6.0,
        end_btn_w,
        36.0,
        Color::from_rgba(150, 40, 30, 240),
    );
    draw_rectangle_lines(
        end_btn_x,
        6.0,
        end_btn_w,
        36.0,
        2.0,
        Color::from_rgba(245, 200, 90, 255),
    );
    draw_text(
        "END TURN",
        end_btn_x + 10.0,
        29.0,
        15.0,
        Color::from_rgba(255, 235, 170, 255),
    );

    if is_mouse_button_pressed(MouseButton::Left) {
        let m = mouse_position();
        if m.0 >= end_btn_x && m.0 <= end_btn_x + end_btn_w && m.1 >= 6.0 && m.1 <= 42.0 {
            game.end_turn();
            ui.set_toast("Turn Complete | Campaign AutoSaved");
        }
    }

    // Menu Button (ESC Pause Menu)
    let menu_btn_w = 70.0;
    let menu_btn_x = end_btn_x - menu_btn_w - 8.0;
    draw_rectangle(
        menu_btn_x,
        6.0,
        menu_btn_w,
        36.0,
        Color::from_rgba(50, 42, 32, 235),
    );
    draw_rectangle_lines(
        menu_btn_x,
        6.0,
        menu_btn_w,
        36.0,
        1.5,
        Color::from_rgba(220, 180, 90, 200),
    );
    draw_text(
        "MENU",
        menu_btn_x + 14.0,
        29.0,
        15.0,
        Color::from_rgba(240, 220, 170, 255),
    );

    if is_mouse_button_pressed(MouseButton::Left) {
        let m = mouse_position();
        if m.0 >= menu_btn_x && m.0 <= menu_btn_x + menu_btn_w && m.1 >= 6.0 && m.1 <= 42.0 {
            ui.show_pause_menu = !ui.show_pause_menu;
        }
    }

    // Navigation Tabs
    let tab_w = 86.0;
    let tab_gap = 4.0;
    let tab_total_w = (tab_w + tab_gap) * 6.0;
    let tab_start_x = menu_btn_x - tab_total_w - 10.0;
    let tabs = [
        ("Campaign", ActiveTab::Map),
        ("Government", ActiveTab::Government),
        ("Technology", ActiveTab::Technology),
        ("Diplomacy", ActiveTab::Diplomacy),
        ("Ledger", ActiveTab::Ledger),
        ("Archives", ActiveTab::LoadGameMenu),
    ];

    for (i, (label, tab)) in tabs.iter().enumerate() {
        let x = tab_start_x + (i as f32 * (tab_w + tab_gap));
        let is_active = game.active_tab == *tab;
        let btn_bg = if is_active {
            Color::from_rgba(180, 130, 40, 220)
        } else {
            Color::from_rgba(45, 38, 30, 200)
        };
        draw_rectangle(x, 8.0, tab_w, 32.0, btn_bg);
        draw_rectangle_lines(x, 8.0, tab_w, 32.0, 1.5, Color::from_rgba(210, 175, 90, 180));
        let text_color = if is_active { WHITE } else { Color::from_rgba(220, 210, 190, 255) };
        draw_text(label, x + 8.0, 28.0, 12.0, text_color);

        if is_mouse_button_pressed(MouseButton::Left) {
            let m = mouse_position();
            if m.0 >= x && m.0 <= x + tab_w && m.1 >= 8.0 && m.1 <= 40.0 {
                if *tab == ActiveTab::LoadGameMenu {
                    ui.refresh_saves();
                    ui.return_tab_after_menu = game.active_tab;
                }
                game.active_tab = *tab;
                ui.show_recruit_modal = false;
                ui.show_build_modal = false;
            }
        }
    }

    // Treasury & Economy (Positioned safely between faction text and tabs)
    let (total_income, upkeep, net_profit) = game.calculate_faction_economy(game.player_faction);
    let net_color = if net_profit >= 0 {
        Color::from_rgba(80, 220, 120, 255)
    } else {
        Color::from_rgba(240, 80, 80, 255)
    };
    let profit_prefix = if net_profit >= 0 { "+" } else { "" };
    
    let treas_x = 350.0_f32.min(tab_start_x - 170.0);
    if treas_x > 240.0 {
        let treasury_text = format!("Treasury: {}g", treasury);
        draw_text_bold(&treasury_text, treas_x, 22.0, 14.5, Color::from_rgba(240, 210, 100, 255));
        let net_text = format!("Net: {}{} (Inc: {} | Up: {})", profit_prefix, net_profit, total_income, upkeep);
        draw_text(&net_text, treas_x, 39.0, 12.0, net_color);
    }

    // Status Toast Banner (e.g. "Campaign Saved!")
    if let Some((msg, timestamp)) = &ui.status_toast {
        let age = get_time() - timestamp;
        if age < 3.5 {
            let toast_w = 340.0;
            let toast_x = (screen_w - toast_w) / 2.0;
            let alpha = if age > 2.8 { ((3.5 - age) / 0.7) as f32 } else { 1.0 };
            draw_rectangle(
                toast_x,
                bar_h + 8.0,
                toast_w,
                28.0,
                Color::from_rgba(20, 24, 20, (235.0 * alpha) as u8),
            );
            draw_rectangle_lines(
                toast_x,
                bar_h + 8.0,
                toast_w,
                28.0,
                1.5,
                Color::from_rgba(100, 220, 140, (240.0 * alpha) as u8),
            );
            draw_text(
                msg,
                toast_x + 12.0,
                bar_h + 27.0,
                13.5,
                Color::from_rgba(230, 255, 230, (255.0 * alpha) as u8),
            );
        }
    }
}

pub fn render_bottom_panel(game: &mut GameState, ui: &mut UiState) {
    let screen_w = screen_width();
    let screen_h = screen_height();
    let panel_h = 185.0;
    let panel_y = screen_h - panel_h;

    // Selected Army Ribbon (Always visible across all tabs / province views when an army is active)
    let army_snap = game.selected_army.and_then(|idx| {
        game.armies.get(idx).map(|a| {
            let has_cav = a.units.iter().any(|u| u.unit_type.stats().is_cavalry);
            (
                idx,
                a.owner,
                a.province_id,
                a.general_name.clone(),
                a.general_stars,
                a.has_moved,
                a.total_soldiers(),
                a.stance,
                has_cav,
                a.retinue.len(),
            )
        })
    });

    if let Some((army_idx, army_owner, army_prov_id, gen_name, gen_stars, has_moved, total_soldiers, army_stance, has_cavalry, retinue_count)) = army_snap {
        let prov_name = &game.provinces[army_prov_id].name;
        let ribbon_h = 32.0;
        let ribbon_y = panel_y - ribbon_h - 2.0;
        let ribbon_w = screen_w;

        draw_rectangle(0.0, ribbon_y, ribbon_w, ribbon_h, Color::from_rgba(20, 16, 12, 245));
        draw_line(0.0, ribbon_y, ribbon_w, ribbon_y, 1.5, Color::from_rgba(230, 185, 80, 255));

        let status_msg = if has_moved {
            "Expended"
        } else {
            "Ready to March"
        };

        let stars = "*".repeat(gen_stars as usize);
        let ret_str = if retinue_count > 0 { format!(" | Staff: {} ancillaries", retinue_count) } else { "".to_string() };
        draw_text(
            &format!(
                "ARMY: {} [{}] in {} | {} men | {}{}",
                gen_name, stars, prov_name, total_soldiers, status_msg, ret_str
            ),
            18.0,
            ribbon_y + 21.0,
            13.5,
            Color::from_rgba(255, 230, 140, 255),
        );

        // [Deselect Unit (Esc)] Button
        let deselect_btn = Rect::new(screen_w - 150.0, ribbon_y + 4.0, 135.0, 24.0);
        let m = mouse_position();
        let m_vec = Vec2::new(m.0, m.1);
        let hov = deselect_btn.contains(m_vec);

        draw_rectangle(
            deselect_btn.x,
            deselect_btn.y,
            deselect_btn.w,
            deselect_btn.h,
            if hov { Color::from_rgba(160, 45, 30, 240) } else { Color::from_rgba(85, 35, 25, 220) },
        );
        draw_rectangle_lines(
            deselect_btn.x,
            deselect_btn.y,
            deselect_btn.w,
            deselect_btn.h,
            1.2,
            Color::from_rgba(240, 190, 80, 255),
        );
        draw_text("Deselect [Esc]", deselect_btn.x + 12.0, deselect_btn.y + 16.5, 12.5, WHITE);

        if is_mouse_button_pressed(MouseButton::Left) && hov {
            game.selected_army = None;
        }

        let is_player_army = army_owner == game.player_faction;
        if is_player_army {
            // Stance Toggle Button (March vs Fortified vs Ambush)
            let stance_label = match army_stance {
                CampaignStance::Normal => "Stance: [March]",
                CampaignStance::Fortified => "Stance: [Fortified +25%]",
                CampaignStance::Ambush => "Stance: [Ambush +35%]",
                CampaignStance::Raiding => "Stance: [Raiding]",
            };
            let stance_btn = Rect::new(screen_w - 330.0, ribbon_y + 4.0, 170.0, 24.0);
            let stance_hov = stance_btn.contains(m_vec);
            let st_bg = match army_stance {
                CampaignStance::Fortified => Color::from_rgba(60, 95, 130, 240),
                CampaignStance::Ambush => Color::from_rgba(45, 115, 65, 240),
                _ => Color::from_rgba(45, 50, 55, 220),
            };
            draw_rectangle(stance_btn.x, stance_btn.y, stance_btn.w, stance_btn.h, st_bg);
            draw_rectangle_lines(
                stance_btn.x,
                stance_btn.y,
                stance_btn.w,
                stance_btn.h,
                1.2,
                Color::from_rgba(180, 205, 235, 220),
            );
            draw_text(
                stance_label,
                stance_btn.x + 8.0,
                stance_btn.y + 16.5,
                12.0,
                WHITE,
            );
            if is_mouse_button_pressed(MouseButton::Left) && stance_hov {
                let next_st = match army_stance {
                    CampaignStance::Normal => CampaignStance::Fortified,
                    CampaignStance::Fortified => CampaignStance::Ambush,
                    CampaignStance::Ambush => CampaignStance::Normal,
                    CampaignStance::Raiding => CampaignStance::Normal,
                };
                match game.set_army_stance(army_idx, next_st) {
                    Ok(msg) => ui.set_toast(&msg),
                    Err(err) => ui.set_toast(&err),
                }
            }

            // Consolidate Regiments (Merge) Button
            let merge_btn = Rect::new(screen_w - 475.0, ribbon_y + 4.0, 138.0, 24.0);
            let merge_hov = merge_btn.contains(m_vec);
            draw_rectangle(merge_btn.x, merge_btn.y, merge_btn.w, merge_btn.h, if merge_hov { Color::from_rgba(60, 120, 70, 240) } else { Color::from_rgba(40, 85, 50, 220) });
            draw_rectangle_lines(merge_btn.x, merge_btn.y, merge_btn.w, merge_btn.h, 1.0, Color::from_rgba(150, 220, 160, 200));
            draw_text("Merge Regts", merge_btn.x + 14.0, merge_btn.y + 16.5, 12.0, WHITE);
            if is_mouse_button_pressed(MouseButton::Left) && merge_hov {
                match game.merge_army_units(army_idx) {
                    Ok(msg) => ui.set_toast(&msg),
                    Err(err) => ui.set_toast(&err),
                }
            }
        }

        if is_player_army && !has_moved {
            let adjacent_foreign: Vec<usize> = game.provinces[army_prov_id]
                .neighbors
                .iter()
                .filter(|&&n| game.provinces[n].owner != army_owner)
                .copied()
                .collect();

            // Besiege Border Province button
            if !adjacent_foreign.is_empty() {
                let siege_target_idx = game.selected_province
                    .filter(|p| adjacent_foreign.contains(p))
                    .unwrap_or(adjacent_foreign[0]);
                let target_p = &game.provinces[siege_target_idx];
                if !target_p.is_under_siege {
                    let siege_btn = Rect::new(screen_w - 655.0, ribbon_y + 4.0, 172.0, 24.0);
                    let siege_hov = siege_btn.contains(m_vec);
                    draw_rectangle(
                        siege_btn.x,
                        siege_btn.y,
                        siege_btn.w,
                        siege_btn.h,
                        if siege_hov { Color::from_rgba(145, 45, 30, 240) } else { Color::from_rgba(95, 32, 22, 220) },
                    );
                    draw_rectangle_lines(siege_btn.x, siege_btn.y, siege_btn.w, siege_btn.h, 1.2, Color::from_rgba(240, 160, 60, 220));
                    draw_text(&format!("⚔ Besiege {}", target_p.name), siege_btn.x + 8.0, siege_btn.y + 16.5, 12.0, WHITE);
                    if is_mouse_button_pressed(MouseButton::Left) && siege_hov {
                        match game.initiate_siege(army_idx, siege_target_idx) {
                            Ok(msg) => ui.set_toast(&msg),
                            Err(err) => ui.set_toast(&err),
                        }
                    }
                } else if target_p.besieger_army_idx == Some(army_idx) {
                    let lift_btn = Rect::new(screen_w - 655.0, ribbon_y + 4.0, 130.0, 24.0);
                    let lift_hov = lift_btn.contains(m_vec);
                    draw_rectangle(
                        lift_btn.x,
                        lift_btn.y,
                        lift_btn.w,
                        lift_btn.h,
                        if lift_hov { Color::from_rgba(50, 90, 130, 240) } else { Color::from_rgba(35, 60, 95, 220) },
                    );
                    draw_rectangle_lines(lift_btn.x, lift_btn.y, lift_btn.w, lift_btn.h, 1.2, Color::from_rgba(160, 200, 240, 220));
                    draw_text(&format!("Lift Siege ({})", target_p.name), lift_btn.x + 8.0, lift_btn.y + 16.5, 12.0, WHITE);
                    if is_mouse_button_pressed(MouseButton::Left) && lift_hov {
                        match game.lift_siege(siege_target_idx) {
                            Ok(msg) => ui.set_toast(&msg),
                            Err(err) => ui.set_toast(&err),
                        }
                    }

                    // Sapper Wall Breach Button
                    let sapper_btn = Rect::new(screen_w - 800.0, ribbon_y + 4.0, 138.0, 24.0);
                    let sapper_hov = sapper_btn.contains(m_vec);
                    draw_rectangle(
                        sapper_btn.x,
                        sapper_btn.y,
                        sapper_btn.w,
                        sapper_btn.h,
                        if sapper_hov { Color::from_rgba(180, 70, 20, 240) } else { Color::from_rgba(125, 45, 15, 220) },
                    );
                    draw_rectangle_lines(sapper_btn.x, sapper_btn.y, sapper_btn.w, sapper_btn.h, 1.2, Color::from_rgba(255, 180, 60, 220));
                    draw_text("💥 Sapper Breach", sapper_btn.x + 8.0, sapper_btn.y + 16.5, 12.0, WHITE);
                    if is_mouse_button_pressed(MouseButton::Left) && sapper_hov {
                        match game.sapper_assault(siege_target_idx) {
                            Ok(msg) => ui.set_toast(&msg),
                            Err(err) => ui.set_toast(&err),
                        }
                    }
                }
            }

            if has_cavalry && !adjacent_foreign.is_empty() {
                let raid_target_idx = game.selected_province
                    .filter(|p| adjacent_foreign.contains(p))
                    .unwrap_or(adjacent_foreign[0]);
                let target_p_name = &game.provinces[raid_target_idx].name;

                let raid_btn = Rect::new(screen_w - 875.0, ribbon_y + 4.0, 210.0, 24.0);
                let raid_hov = raid_btn.contains(m_vec);

                draw_rectangle(
                    raid_btn.x,
                    raid_btn.y,
                    raid_btn.w,
                    raid_btn.h,
                    if raid_hov { Color::from_rgba(180, 90, 25, 245) } else { Color::from_rgba(110, 55, 18, 225) },
                );
                draw_rectangle_lines(
                    raid_btn.x,
                    raid_btn.y,
                    raid_btn.w,
                    raid_btn.h,
                    1.2,
                    Color::from_rgba(255, 205, 80, 255),
                );
                draw_text(&format!("⚔ Ganimi Kava ({})", target_p_name), raid_btn.x + 8.0, raid_btn.y + 16.5, 12.0, WHITE);

                if is_mouse_button_pressed(MouseButton::Left) && raid_hov {
                    match game.raid_province(army_idx, raid_target_idx) {
                        Ok(msg) => ui.set_toast(&msg),
                        Err(err) => ui.set_toast(&err),
                    }
                }
            }
        }
    }

    // Background
    draw_rectangle(0.0, panel_y, screen_w, panel_h, Color::from_rgba(20, 16, 12, 245));
    draw_line(0.0, panel_y, screen_w, panel_y, 2.0, Color::from_rgba(190, 150, 70, 200));

    // Left Column: Selected Province Info
    if let Some(prov_idx) = game.selected_province {
        let is_visible = game.has_recon_visibility(prov_idx);
        let (prov_name, capital_name, owner, population, base_wealth, public_order, buildings, garrison, is_under_siege, siege_turns, is_blockaded, has_port, commodity, upper_tax, lower_tax) = {
            let p = &game.provinces[prov_idx];
            (
                p.name.clone(),
                p.capital_name.clone(),
                p.owner,
                p.population,
                p.base_wealth,
                p.public_order,
                p.buildings.clone(),
                p.garrison.clone(),
                p.is_under_siege,
                p.siege_turns_left,
                p.is_port_blockaded,
                p.port_name.is_some(),
                p.commodity,
                p.upper_tax,
                p.lower_tax,
            )
        };
        let (f_color, f_accent) = owner.flag_colors();

        // Flag icon
        draw_rectangle(15.0, panel_y + 15.0, 34.0, 34.0, f_color);
        draw_rectangle_lines(15.0, panel_y + 15.0, 34.0, 34.0, 2.0, f_accent);

        // Province name & capital
        draw_text(
            &format!("Province: {}", prov_name),
            60.0,
            panel_y + 28.0,
            23.0,
            Color::from_rgba(245, 225, 170, 255),
        );
        draw_text(
            &format!("Seat: {} | Controlled by: {}", capital_name, owner.name()),
            60.0,
            panel_y + 48.0,
            16.0,
            Color::from_rgba(215, 200, 175, 255),
        );

        if is_under_siege {
            draw_text(
                &format!("[UNDER SIEGE: {} Turns To Capitulation | Starvation Active]", siege_turns),
                350.0,
                panel_y + 28.0,
                14.5,
                Color::from_rgba(255, 75, 75, 255),
            );
        }

        // Stats line (masked under Fog of War if invisible)
        if is_visible {
            let comm_str = commodity.map(|c| c.name()).unwrap_or("Agrarian Staples");
            draw_text(
                &format!(
                    "Pop: {}k | Wealth: {}g | Order: {}{} | Commodity: {}",
                    population / 1000,
                    base_wealth,
                    if public_order >= 0 { "+" } else { "" },
                    public_order,
                    comm_str
                ),
                15.0,
                panel_y + 74.0,
                14.0,
                Color::from_rgba(225, 215, 195, 255),
            );
        } else {
            draw_text(
                "Population: ??? | Base Wealth: ???g | Public Order: ??? | Commodity: ???",
                15.0,
                panel_y + 74.0,
                14.0,
                Color::from_rgba(180, 170, 155, 255),
            );
            draw_text("[Fog of War Active: Infiltrate Subah to Scout]", 15.0, panel_y + 92.0, 13.0, Color::from_rgba(240, 185, 90, 255));
        }

        let m = mouse_position();
        let m_pos = Vec2::new(m.0, m.1);

        // Buttons row
        if owner == game.player_faction {
            // Upper Tax button (affects Town Wealth growth)
            let upper_tax_btn = Rect::new(15.0, panel_y + 96.0, 120.0, 26.0);
            draw_rectangle(upper_tax_btn.x, upper_tax_btn.y, upper_tax_btn.w, upper_tax_btn.h, Color::from_rgba(40, 35, 28, 220));
            draw_rectangle_lines(upper_tax_btn.x, upper_tax_btn.y, upper_tax_btn.w, upper_tax_btn.h, 1.0, Color::from_rgba(180, 145, 75, 200));
            draw_text(&format!("Upper: {}", upper_tax.name()), upper_tax_btn.x + 6.0, upper_tax_btn.y + 17.0, 11.5, Color::from_rgba(245, 220, 150, 255));

            // Lower Tax button (affects Public Order & Pop)
            let lower_tax_btn = Rect::new(140.0, panel_y + 96.0, 120.0, 26.0);
            draw_rectangle(lower_tax_btn.x, lower_tax_btn.y, lower_tax_btn.w, lower_tax_btn.h, Color::from_rgba(40, 35, 28, 220));
            draw_rectangle_lines(lower_tax_btn.x, lower_tax_btn.y, lower_tax_btn.w, lower_tax_btn.h, 1.0, Color::from_rgba(180, 145, 75, 200));
            draw_text(&format!("Lower: {}", lower_tax.name()), lower_tax_btn.x + 6.0, lower_tax_btn.y + 17.0, 11.5, Color::from_rgba(215, 200, 160, 255));

            // Tax Exemption Toggle
            let is_exempt = game.provinces[prov_idx].lower_tax == TaxLevel::Exempt && game.provinces[prov_idx].upper_tax == TaxLevel::Exempt;
            let exempt_btn = Rect::new(265.0, panel_y + 96.0, 75.0, 26.0);
            let ex_bg = if is_exempt {
                Color::from_rgba(40, 120, 50, 230)
            } else {
                Color::from_rgba(50, 42, 35, 200)
            };
            draw_rectangle(exempt_btn.x, exempt_btn.y, exempt_btn.w, exempt_btn.h, ex_bg);
            draw_rectangle_lines(exempt_btn.x, exempt_btn.y, exempt_btn.w, exempt_btn.h, 1.0, if is_exempt { Color::from_rgba(120, 230, 140, 220) } else { Color::from_rgba(150, 130, 85, 180) });
            draw_text(if is_exempt { "EXEMPT" } else { "TAXABLE" }, exempt_btn.x + 8.0, exempt_btn.y + 17.0, 11.5, WHITE);

            let recruit_btn = Rect::new(345.0, panel_y + 96.0, 88.0, 26.0);
            draw_rectangle(recruit_btn.x, recruit_btn.y, recruit_btn.w, recruit_btn.h, Color::from_rgba(60, 95, 45, 240));
            draw_rectangle_lines(recruit_btn.x, recruit_btn.y, recruit_btn.w, recruit_btn.h, 1.5, Color::from_rgba(200, 230, 150, 200));
            draw_text("+ Recruit", recruit_btn.x + 10.0, recruit_btn.y + 17.0, 12.5, WHITE);

            let build_btn = Rect::new(438.0, panel_y + 96.0, 95.0, 26.0);
            draw_rectangle(build_btn.x, build_btn.y, build_btn.w, build_btn.h, Color::from_rgba(95, 70, 35, 240));
            draw_rectangle_lines(build_btn.x, build_btn.y, build_btn.w, build_btn.h, 1.5, Color::from_rgba(230, 195, 120, 200));
            draw_text("+ Construct", build_btn.x + 8.0, build_btn.y + 17.0, 12.5, WHITE);

            if is_mouse_button_pressed(MouseButton::Left) {
                if upper_tax_btn.contains(m_pos) {
                    game.provinces[prov_idx].upper_tax = game.provinces[prov_idx].upper_tax.next();
                } else if lower_tax_btn.contains(m_pos) {
                    game.provinces[prov_idx].lower_tax = game.provinces[prov_idx].lower_tax.next();
                    game.provinces[prov_idx].tax_level = game.provinces[prov_idx].lower_tax;
                } else if exempt_btn.contains(m_pos) {
                    if is_exempt {
                        game.provinces[prov_idx].tax_level = TaxLevel::Normal;
                        game.provinces[prov_idx].upper_tax = TaxLevel::Normal;
                        game.provinces[prov_idx].lower_tax = TaxLevel::Normal;
                        ui.set_toast("Taxation restored to Normal.");
                    } else {
                        game.provinces[prov_idx].tax_level = TaxLevel::Exempt;
                        game.provinces[prov_idx].upper_tax = TaxLevel::Exempt;
                        game.provinces[prov_idx].lower_tax = TaxLevel::Exempt;
                        ui.set_toast("Province granted Tax Exemption (+4 Public Order).");
                    }
                } else if recruit_btn.contains(m_pos) {
                    ui.show_recruit_modal = !ui.show_recruit_modal;
                    ui.show_build_modal = false;
                } else if build_btn.contains(m_pos) {
                    ui.show_build_modal = !ui.show_build_modal;
                    ui.show_recruit_modal = false;
                }
            }
        } else {
            // Foreign Subah: Covert Operations Buttons
            let inf_btn = Rect::new(15.0, panel_y + 96.0, 115.0, 26.0);
            let sab_btn = Rect::new(135.0, panel_y + 96.0, 115.0, 26.0);
            let ass_btn = Rect::new(255.0, panel_y + 96.0, 130.0, 26.0);
            let steal_btn = Rect::new(390.0, panel_y + 96.0, 130.0, 26.0);

            draw_rectangle(inf_btn.x, inf_btn.y, inf_btn.w, inf_btn.h, Color::from_rgba(50, 70, 95, 230));
            draw_rectangle_lines(inf_btn.x, inf_btn.y, inf_btn.w, inf_btn.h, 1.0, Color::from_rgba(150, 200, 240, 200));
            draw_text("🕵 Infiltrate", inf_btn.x + 8.0, inf_btn.y + 17.5, 12.0, WHITE);

            draw_rectangle(sab_btn.x, sab_btn.y, sab_btn.w, sab_btn.h, Color::from_rgba(95, 55, 30, 230));
            draw_rectangle_lines(sab_btn.x, sab_btn.y, sab_btn.w, sab_btn.h, 1.0, Color::from_rgba(240, 160, 90, 200));
            draw_text("💣 Sabotage", sab_btn.x + 8.0, sab_btn.y + 17.5, 12.0, WHITE);

            draw_rectangle(ass_btn.x, ass_btn.y, ass_btn.w, ass_btn.h, Color::from_rgba(110, 30, 35, 230));
            draw_rectangle_lines(ass_btn.x, ass_btn.y, ass_btn.w, ass_btn.h, 1.0, Color::from_rgba(250, 120, 130, 200));
            draw_text("🎯 Assassinate", ass_btn.x + 8.0, ass_btn.y + 17.5, 12.0, WHITE);

            draw_rectangle(steal_btn.x, steal_btn.y, steal_btn.w, steal_btn.h, Color::from_rgba(65, 45, 95, 230));
            draw_rectangle_lines(steal_btn.x, steal_btn.y, steal_btn.w, steal_btn.h, 1.0, Color::from_rgba(190, 150, 240, 200));
            draw_text("📜 Steal Tech", steal_btn.x + 8.0, steal_btn.y + 17.5, 12.0, WHITE);

            if is_mouse_button_pressed(MouseButton::Left) {
                if inf_btn.contains(m_pos) {
                    match game.dispatch_covert_mission(prov_idx, CovertAction::Infiltrate) {
                        Ok(msg) => ui.set_toast(&msg),
                        Err(e) => ui.set_toast(&e),
                    }
                } else if sab_btn.contains(m_pos) {
                    match game.dispatch_covert_mission(prov_idx, CovertAction::SabotageInfrastructure) {
                        Ok(msg) => ui.set_toast(&msg),
                        Err(e) => ui.set_toast(&e),
                    }
                } else if ass_btn.contains(m_pos) {
                    match game.dispatch_covert_mission(prov_idx, CovertAction::AssassinateGeneral) {
                        Ok(msg) => ui.set_toast(&msg),
                        Err(e) => ui.set_toast(&e),
                    }
                } else if steal_btn.contains(m_pos) {
                    match game.dispatch_covert_mission(prov_idx, CovertAction::StealTechnology) {
                        Ok(msg) => ui.set_toast(&msg),
                        Err(e) => ui.set_toast(&e),
                    }
                }
            }
        }

        // Coastal Harbor Blockade Toggle Button
        if has_port {
            let port_btn = Rect::new(525.0, panel_y + 96.0, 125.0, 26.0);
            let port_bg = if is_blockaded { Color::from_rgba(150, 40, 30, 230) } else { Color::from_rgba(40, 75, 95, 220) };
            draw_rectangle(port_btn.x, port_btn.y, port_btn.w, port_btn.h, port_bg);
            draw_rectangle_lines(port_btn.x, port_btn.y, port_btn.w, port_btn.h, 1.0, Color::from_rgba(180, 210, 230, 200));
            draw_text(if is_blockaded { "⚓ [BLOCKADED]" } else { "⚓ Harbor: [OPEN]" }, port_btn.x + 8.0, port_btn.y + 17.5, 12.0, WHITE);

            if is_mouse_button_pressed(MouseButton::Left) && port_btn.contains(m_pos) {
                match game.toggle_port_blockade(prov_idx) {
                    Ok(msg) => ui.set_toast(&msg),
                    Err(e) => ui.set_toast(&e),
                }
            }
        }

        let right_x = screen_w - 470.0;

        // Active Queues (Muster & Construction with remaining turns countdown)
        draw_text("Queues:", 15.0, panel_y + 141.0, 13.5, Color::from_rgba(180, 165, 140, 255));
        if owner == game.player_faction {
            let mut q_x = 80.0;
            let mut any_queue = false;

            let p_ref = &game.provinces[prov_idx];
            for r_order in &p_ref.recruitment_queue {
                let badge_w = 145.0;
                if q_x + badge_w > right_x - 20.0 {
                    break;
                }
                any_queue = true;
                draw_rectangle(q_x, panel_y + 127.0, badge_w, 20.0, Color::from_rgba(35, 65, 45, 230));
                draw_rectangle_lines(q_x, panel_y + 127.0, badge_w, 20.0, 1.0, Color::from_rgba(140, 210, 150, 220));
                draw_text(&format!("⚔ {} ({}T)", r_order.unit_type.name(), r_order.turns_remaining), q_x + 5.0, panel_y + 141.0, 11.0, WHITE);
                q_x += badge_w + 6.0;
            }

            for c_order in &p_ref.construction_queue {
                let badge_w = 150.0;
                if q_x + badge_w > right_x - 20.0 {
                    break;
                }
                any_queue = true;
                draw_rectangle(q_x, panel_y + 127.0, badge_w, 20.0, Color::from_rgba(65, 45, 25, 230));
                draw_rectangle_lines(q_x, panel_y + 127.0, badge_w, 20.0, 1.0, Color::from_rgba(230, 175, 90, 220));
                draw_text(&format!("🏗 {} ({}T)", c_order.building.name(), c_order.turns_remaining), q_x + 5.0, panel_y + 141.0, 11.0, WHITE);
                q_x += badge_w + 6.0;
            }

            if !any_queue {
                draw_text("Idle - No active mustering or construction in progress", 80.0, panel_y + 141.0, 11.5, Color::from_rgba(145, 135, 120, 200));
            }
        } else {
            draw_text("Foreign Subah - Mustering & construction restricted to sovereign lands", 80.0, panel_y + 141.0, 11.5, Color::from_rgba(180, 150, 120, 200));
        }

        // Buildings icons
        draw_text("Buildings:", 15.0, panel_y + 168.0, 13.5, Color::from_rgba(180, 165, 140, 255));
        let mut b_x = 85.0;
        for b in &buildings {
            let badge_w = 120.0;
            if b_x + badge_w > right_x - 20.0 {
                break;
            }
            draw_rectangle(b_x, panel_y + 154.0, badge_w, 21.0, Color::from_rgba(50, 42, 32, 220));
            draw_rectangle_lines(b_x, panel_y + 154.0, badge_w, 21.0, 1.0, Color::from_rgba(160, 130, 70, 180));
            draw_text(b.name(), b_x + 6.0, panel_y + 169.0, 11.5, Color::from_rgba(220, 205, 175, 255));
            b_x += badge_w + 6.0;
        }
        if buildings.is_empty() {
            draw_text("None erected", 85.0, panel_y + 168.0, 11.5, Color::from_rgba(145, 135, 120, 200));
        }

        // Right Column: Garrison & Armies in Province
        draw_line(right_x - 15.0, panel_y + 10.0, right_x - 15.0, screen_h - 10.0, 1.0, Color::from_rgba(100, 80, 50, 120));

        if is_visible {
            draw_text("Regional Forces & Garrison:", right_x, panel_y + 24.0, 16.5, Color::from_rgba(240, 220, 160, 255));
        } else {
            draw_text("Regional Forces: [FOG OF WAR]", right_x, panel_y + 24.0, 16.5, Color::from_rgba(240, 180, 90, 255));
            draw_text("Forces concealed by reconnaissance fog.", right_x, panel_y + 55.0, 14.0, Color::from_rgba(180, 165, 140, 255));
            draw_text("Deploy Infiltration Operative to scout defense strength.", right_x, panel_y + 78.0, 12.5, Color::from_rgba(150, 135, 115, 255));
        }

        // Armies currently in this province (collected as snapshots to avoid borrow conflicts)
        struct ArmySnap {
            idx: usize,
            owner: FactionId,
            name: String,
            stars: u8,
            soldiers: u32,
            has_moved: bool,
        }
        let armies_here: Vec<ArmySnap> = game
            .armies
            .iter()
            .enumerate()
            .filter(|(_, a)| a.province_id == prov_idx)
            .map(|(i, a)| ArmySnap {
                idx: i,
                owner: a.owner,
                name: a.general_name.clone(),
                stars: a.general_stars,
                soldiers: a.total_soldiers(),
                has_moved: a.has_moved,
            })
            .collect();

        let my_army_present = armies_here.iter().find(|a| a.owner == game.player_faction).map(|a| a.idx);

        // Commission General button if owned by player, has garrison, and no friendly army present
        if owner == game.player_faction && my_army_present.is_none() && !garrison.is_empty() {
            let comm_btn = Rect::new(right_x + 220.0, panel_y + 10.0, 210.0, 22.0);
            let m = mouse_position();
            let comm_hov = comm_btn.contains(Vec2::new(m.0, m.1));
            draw_rectangle(comm_btn.x, comm_btn.y, comm_btn.w, comm_btn.h, if comm_hov { Color::from_rgba(195, 135, 30, 240) } else { Color::from_rgba(150, 95, 20, 230) });
            draw_rectangle_lines(comm_btn.x, comm_btn.y, comm_btn.w, comm_btn.h, 1.2, Color::from_rgba(245, 210, 80, 255));
            draw_text("+ Commission General (400g)", comm_btn.x + 8.0, comm_btn.y + 15.5, 12.0, WHITE);

            if is_mouse_button_pressed(MouseButton::Left) && comm_hov {
                match game.commission_general_from_garrison(prov_idx) {
                    Ok(new_idx) => {
                        let name = game.armies[new_idx].general_name.clone();
                        ui.set_toast(&format!("Commissioned {} to lead new Field Army!", name));
                    }
                    Err(e) => ui.set_toast(&e),
                }
            }
        }

        // Garrison cards
        let mut u_x = right_x;
        for (u_idx, u) in garrison.iter().enumerate() {
            let u_card = Rect::new(u_x, panel_y + 36.0, 102.0, 68.0);
            draw_rectangle(u_card.x, u_card.y, u_card.w, u_card.h, Color::from_rgba(35, 30, 24, 240));
            draw_rectangle_lines(u_card.x, u_card.y, u_card.w, u_card.h, 1.0, Color::from_rgba(180, 150, 80, 180));
            draw_text(u.unit_type.name(), u_card.x + 5.0, u_card.y + 18.0, 11.5, WHITE);
            draw_text(&format!("{} men", u.soldiers), u_card.x + 5.0, u_card.y + 36.0, 13.0, Color::from_rgba(210, 190, 140, 255));

            // Disband button if player province
            if owner == game.player_faction {
                let dis_btn = Rect::new(u_card.x + u_card.w - 18.0, u_card.y + 3.0, 15.0, 15.0);
                draw_rectangle(dis_btn.x, dis_btn.y, dis_btn.w, dis_btn.h, Color::from_rgba(150, 35, 25, 220));
                draw_text("x", dis_btn.x + 4.0, dis_btn.y + 11.0, 11.0, WHITE);
                if is_mouse_button_pressed(MouseButton::Left) && dis_btn.contains(m_pos) {
                    match game.disband_garrison_unit(prov_idx, u_idx) {
                        Ok(msg) => ui.set_toast(&msg),
                        Err(err) => ui.set_toast(&err),
                    }
                    break;
                }
            }

            if let Some(target_army_idx) = my_army_present {
                let attach_btn = Rect::new(u_card.x + 4.0, u_card.y + 44.0, 94.0, 20.0);
                let m = mouse_position();
                let att_hov = attach_btn.contains(Vec2::new(m.0, m.1));
                draw_rectangle(attach_btn.x, attach_btn.y, attach_btn.w, attach_btn.h, if att_hov { Color::from_rgba(50, 140, 60, 240) } else { Color::from_rgba(40, 100, 45, 220) });
                draw_rectangle_lines(attach_btn.x, attach_btn.y, attach_btn.w, attach_btn.h, 1.0, Color::from_rgba(180, 230, 140, 200));
                draw_text("+ Join Army", attach_btn.x + 12.0, attach_btn.y + 14.5, 11.0, WHITE);

                if is_mouse_button_pressed(MouseButton::Left) && att_hov {
                    game.transfer_garrison_to_army(prov_idx, u_idx, target_army_idx);
                    ui.set_toast("Regiment joined field army!");
                    break;
                }
            }
            u_x += 108.0;
        }

        if !armies_here.is_empty() {
            draw_text("Field Armies Present (Click to select & order movement):", right_x, panel_y + 112.0, 14.5, Color::from_rgba(215, 195, 140, 255));
            let mut a_x = right_x;
            for army in &armies_here {
                let a_idx = army.idx;
                let is_selected = game.selected_army == Some(a_idx);
                let bg_color = if is_selected {
                    Color::from_rgba(140, 95, 25, 240)
                } else {
                    Color::from_rgba(45, 38, 30, 220)
                };
                let a_rect = Rect::new(a_x, panel_y + 124.0, 205.0, 44.0);
                draw_rectangle(a_rect.x, a_rect.y, a_rect.w, a_rect.h, bg_color);
                draw_rectangle_lines(a_rect.x, a_rect.y, a_rect.w, a_rect.h, 1.5, Color::from_rgba(220, 180, 80, 200));

                let stars_str = "*".repeat(army.stars as usize);
                draw_text(&format!("{} [{}]", army.name, stars_str), a_rect.x + 8.0, a_rect.y + 18.0, 14.0, WHITE);
                let move_status = if is_selected {
                    "[Selected - Click to Deselect]"
                } else if army.has_moved {
                    "Moved"
                } else {
                    "Ready (Click to select)"
                };
                draw_text(&format!("{} men | {}", army.soldiers, move_status), a_rect.x + 8.0, a_rect.y + 35.0, 12.0, Color::from_rgba(230, 205, 140, 255));

                if is_selected && army.owner == game.player_faction && game.armies[a_idx].units.len() > 1 {
                    let detach_btn = Rect::new(a_rect.x + a_rect.w - 74.0, a_rect.y + 24.0, 70.0, 16.0);
                    let m = mouse_position();
                    let det_hov = detach_btn.contains(Vec2::new(m.0, m.1));
                    draw_rectangle(detach_btn.x, detach_btn.y, detach_btn.w, detach_btn.h, if det_hov { Color::from_rgba(180, 55, 35, 240) } else { Color::from_rgba(135, 40, 25, 220) });
                    draw_text("- Detach", detach_btn.x + 8.0, detach_btn.y + 12.0, 11.0, WHITE);

                    let disband_btn = Rect::new(a_rect.x + a_rect.w - 148.0, a_rect.y + 24.0, 70.0, 16.0);
                    let dis_hov = disband_btn.contains(Vec2::new(m.0, m.1));
                    draw_rectangle(disband_btn.x, disband_btn.y, disband_btn.w, disband_btn.h, if dis_hov { Color::from_rgba(160, 35, 30, 240) } else { Color::from_rgba(110, 30, 25, 220) });
                    draw_text("Disband", disband_btn.x + 8.0, disband_btn.y + 12.0, 11.0, WHITE);

                    if is_mouse_button_pressed(MouseButton::Left) {
                        if det_hov {
                            let last_unit = game.armies[a_idx].units.len() - 1;
                            if game.transfer_army_to_garrison(a_idx, last_unit) {
                                ui.set_toast("Regiment detached to local garrison.");
                                break;
                            }
                        } else if dis_hov {
                            let last_unit = game.armies[a_idx].units.len() - 1;
                            match game.disband_army_unit(a_idx, last_unit) {
                                Ok(msg) => ui.set_toast(&msg),
                                Err(err) => ui.set_toast(&err),
                            }
                            break;
                        }
                    }
                }

                if is_mouse_button_pressed(MouseButton::Left) {
                    let m = mouse_position();
                    if a_rect.contains(Vec2::new(m.0, m.1)) {
                        if game.selected_army == Some(a_idx) {
                            game.selected_army = None;
                        } else {
                            game.selected_army = Some(a_idx);
                        }
                    }
                }
                a_x += 215.0;
            }
        }
    } else {
        // No Province Selected: Guidance / Strategic Overview
        draw_text(
            "Subcontinent Strategic Theater - No Province Selected",
            24.0,
            panel_y + 34.0,
            20.0,
            Color::from_rgba(240, 220, 160, 255),
        );
        draw_text(
            "Left-click any province or army disc on the map to inspect regional garrison, economy, and muster forces.",
            24.0,
            panel_y + 65.0,
            14.5,
            Color::from_rgba(215, 200, 175, 255),
        );
        draw_text(
            "Navigation: Hold Right-Click or Middle-Click and drag (> 8px) to pan the camera smoothly.",
            24.0,
            panel_y + 92.0,
            14.0,
            Color::from_rgba(180, 195, 215, 255),
        );
        draw_text(
            "Orders: Select an army, then left or right-click an adjacent neighbor to march or assault. Press [Esc] to cancel.",
            24.0,
            panel_y + 118.0,
            14.0,
            Color::from_rgba(235, 190, 110, 255),
        );
    }
}

pub fn render_recruitment_modal(game: &mut GameState, ui: &mut UiState) {
    if !ui.show_recruit_modal {
        return;
    }
    let prov_idx = match game.selected_province {
        Some(idx) => idx,
        None => return,
    };

    let w = 530.0;
    let h = 385.0;
    let x = (screen_width() - w) / 2.0;
    let y = (screen_height() - h) / 2.0;

    // Window frame
    draw_rectangle(x, y, w, h, Color::from_rgba(24, 20, 16, 250));
    draw_rectangle_lines(x, y, w, h, 2.5, Color::from_rgba(220, 180, 80, 255));

    // Header
    draw_text("Muster Royal Troops (Recruitment)", x + 20.0, y + 26.0, 18.0, Color::from_rgba(245, 220, 150, 255));
    let treasury = game.factions.get(&game.player_faction).map(|f| f.treasury).unwrap_or(0);
    draw_text(
        &format!("Province: {} | Treasury: {}g", game.provinces[prov_idx].name, treasury),
        x + 20.0,
        y + 42.0,
        12.0,
        Color::from_rgba(210, 195, 160, 220),
    );

    // Close button
    let close_btn = Rect::new(x + w - 35.0, y + 10.0, 25.0, 25.0);
    draw_rectangle(close_btn.x, close_btn.y, close_btn.w, close_btn.h, Color::from_rgba(160, 40, 30, 220));
    draw_text("X", close_btn.x + 7.0, close_btn.y + 18.0, 15.0, WHITE);
    if is_mouse_button_pressed(MouseButton::Left) {
        let m = mouse_position();
        if close_btn.contains(Vec2::new(m.0, m.1)) {
            ui.show_recruit_modal = false;
        }
    }

    let units = [
        UnitType::BargirCavalry,
        UnitType::SilhadarLancers,
        UnitType::HinduMatchlocks,
        UnitType::SepoyLineInfantry,
        UnitType::PindariRaiders,
        UnitType::TopchiCannons,
    ];

    let mut row_y = y + 52.0;
    for u in units {
        let stats = u.stats();
        let recruit_turns = u.recruit_turns();
        let turns_str = if recruit_turns == 1 { "1 Turn" } else { "2 Turns" };
        let card_rect = Rect::new(x + 15.0, row_y, w - 30.0, 42.0);
        draw_rectangle(card_rect.x, card_rect.y, card_rect.w, card_rect.h, Color::from_rgba(40, 32, 24, 220));
        draw_rectangle_lines(card_rect.x, card_rect.y, card_rect.w, card_rect.h, 1.0, Color::from_rgba(180, 140, 70, 180));

        draw_text(u.name(), card_rect.x + 10.0, card_rect.y + 17.0, 13.5, Color::from_rgba(245, 230, 180, 255));
        draw_text(
            &format!("Cost: {}g | Upkeep: {}g | Soldiers: {} | Morale: {} | Muster: {}", stats.cost, stats.upkeep, stats.base_soldiers, stats.morale, turns_str),
            card_rect.x + 10.0,
            card_rect.y + 33.0,
            11.0,
            Color::from_rgba(200, 190, 160, 255),
        );

        let recruit_btn = Rect::new(card_rect.x + card_rect.w - 92.0, card_rect.y + 8.0, 84.0, 26.0);
        draw_rectangle(recruit_btn.x, recruit_btn.y, recruit_btn.w, recruit_btn.h, Color::from_rgba(50, 120, 60, 240));
        draw_rectangle_lines(recruit_btn.x, recruit_btn.y, recruit_btn.w, recruit_btn.h, 1.0, Color::from_rgba(180, 230, 140, 220));
        draw_text(&format!("Enlist ({}T)", recruit_turns), recruit_btn.x + 10.0, recruit_btn.y + 18.0, 12.0, WHITE);

        if is_mouse_button_pressed(MouseButton::Left) {
            let m = mouse_position();
            if recruit_btn.contains(Vec2::new(m.0, m.1)) {
                if game.recruit_unit(prov_idx, u) {
                    ui.show_recruit_modal = false;
                }
            }
        }

        row_y += 46.0;
    }

    // Active Province Muster Queue footer
    let q_bar_y = y + 338.0;
    draw_line(x + 15.0, q_bar_y, x + w - 15.0, q_bar_y, 1.0, Color::from_rgba(180, 140, 70, 120));
    draw_text("Active Province Muster Queue:", x + 15.0, q_bar_y + 22.0, 12.5, Color::from_rgba(230, 205, 140, 255));
    let p = &game.provinces[prov_idx];
    if p.recruitment_queue.is_empty() {
        draw_text("No regiments currently mustering", x + 235.0, q_bar_y + 22.0, 12.0, Color::from_rgba(160, 150, 135, 200));
    } else {
        let mut q_str = String::new();
        for (i, r) in p.recruitment_queue.iter().enumerate() {
            if i > 0 { q_str.push_str(", "); }
            q_str.push_str(&format!("{} ({}T left)", r.unit_type.name(), r.turns_remaining));
        }
        draw_text(&q_str, x + 235.0, q_bar_y + 22.0, 12.0, Color::from_rgba(130, 220, 150, 255));
    }
}

pub fn render_construction_modal(game: &mut GameState, ui: &mut UiState) {
    if !ui.show_build_modal {
        return;
    }
    let prov_idx = match game.selected_province {
        Some(idx) => idx,
        None => return,
    };

    let w = 550.0;
    let h = 510.0;
    let x = (screen_width() - w) / 2.0;
    let y = (screen_height() - h) / 2.0;

    draw_rectangle(x, y, w, h, Color::from_rgba(24, 20, 16, 250));
    draw_rectangle_lines(x, y, w, h, 2.5, Color::from_rgba(220, 180, 80, 255));

    draw_text("Provincial Construction & Infrastructure", x + 20.0, y + 26.0, 18.0, Color::from_rgba(245, 220, 150, 255));
    let treasury = game.factions.get(&game.player_faction).map(|f| f.treasury).unwrap_or(0);
    draw_text(
        &format!("Province: {} | Treasury: {}g", game.provinces[prov_idx].name, treasury),
        x + 20.0,
        y + 42.0,
        12.0,
        Color::from_rgba(210, 195, 160, 220),
    );

    let close_btn = Rect::new(x + w - 35.0, y + 10.0, 25.0, 25.0);
    draw_rectangle(close_btn.x, close_btn.y, close_btn.w, close_btn.h, Color::from_rgba(160, 40, 30, 220));
    draw_text("X", close_btn.x + 7.0, close_btn.y + 18.0, 15.0, WHITE);
    if is_mouse_button_pressed(MouseButton::Left) {
        let m = mouse_position();
        if close_btn.contains(Vec2::new(m.0, m.1)) {
            ui.show_build_modal = false;
        }
    }

    let buildings = [
        BuildingType::Fortress,
        BuildingType::Barracks,
        BuildingType::Marketplace,
        BuildingType::TradingPort,
        BuildingType::TempleMosque,
        BuildingType::CollegeOfLogic,
        BuildingType::TextileLoom,
        BuildingType::RoadNetwork,
    ];

    let mut row_y = y + 50.0;
    for b in buildings {
        let card_rect = Rect::new(x + 15.0, row_y, w - 30.0, 43.0);
        draw_rectangle(card_rect.x, card_rect.y, card_rect.w, card_rect.h, Color::from_rgba(40, 32, 24, 220));
        draw_rectangle_lines(card_rect.x, card_rect.y, card_rect.w, card_rect.h, 1.0, Color::from_rgba(180, 140, 70, 180));

        let b_turns = b.build_turns();
        let turns_str = if b_turns == 1 { "1 Turn" } else { "2 Turns" };

        draw_text(b.name(), card_rect.x + 10.0, card_rect.y + 16.0, 13.5, Color::from_rgba(245, 230, 180, 255));
        draw_text(
            &format!("Cost: {}g | Build Time: {} | {}", b.cost(), turns_str, b.effect_description()),
            card_rect.x + 10.0,
            card_rect.y + 33.0,
            11.0,
            Color::from_rgba(200, 190, 160, 255),
        );

        let already_built = game.provinces[prov_idx].buildings.contains(&b);
        let in_queue = game.provinces[prov_idx].construction_queue.iter().find(|c| c.building == b);
        let build_btn = Rect::new(card_rect.x + card_rect.w - 92.0, card_rect.y + 8.0, 84.0, 26.0);

        if already_built {
            draw_rectangle(build_btn.x, build_btn.y, build_btn.w, build_btn.h, Color::from_rgba(60, 60, 60, 180));
            draw_text("Built", build_btn.x + 28.0, build_btn.y + 18.0, 12.0, Color::from_rgba(170, 170, 170, 255));
        } else if let Some(order) = in_queue {
            draw_rectangle(build_btn.x, build_btn.y, build_btn.w, build_btn.h, Color::from_rgba(35, 75, 115, 220));
            draw_rectangle_lines(build_btn.x, build_btn.y, build_btn.w, build_btn.h, 1.0, Color::from_rgba(120, 180, 235, 200));
            draw_text(&format!("{}T Left", order.turns_remaining), build_btn.x + 18.0, build_btn.y + 18.0, 12.0, Color::from_rgba(220, 235, 255, 255));
        } else {
            draw_rectangle(build_btn.x, build_btn.y, build_btn.w, build_btn.h, Color::from_rgba(130, 85, 30, 240));
            draw_rectangle_lines(build_btn.x, build_btn.y, build_btn.w, build_btn.h, 1.0, Color::from_rgba(230, 190, 100, 220));
            draw_text(&format!("Build ({}T)", b_turns), build_btn.x + 10.0, build_btn.y + 18.0, 12.0, WHITE);

            if is_mouse_button_pressed(MouseButton::Left) {
                let m = mouse_position();
                if build_btn.contains(Vec2::new(m.0, m.1)) {
                    if game.build_structure(prov_idx, b) {
                        ui.show_build_modal = false;
                    }
                }
            }
        }

        row_y += 48.0;
    }

    // Active Province Construction Queue footer
    let q_bar_y = y + 452.0;
    draw_line(x + 15.0, q_bar_y, x + w - 15.0, q_bar_y, 1.0, Color::from_rgba(180, 140, 70, 120));
    draw_text("Active Construction Queue:", x + 15.0, q_bar_y + 24.0, 12.5, Color::from_rgba(230, 205, 140, 255));
    let p = &game.provinces[prov_idx];
    if p.construction_queue.is_empty() {
        draw_text("No structures currently under construction", x + 215.0, q_bar_y + 24.0, 12.0, Color::from_rgba(160, 150, 135, 200));
    } else {
        let mut c_str = String::new();
        for (i, c) in p.construction_queue.iter().enumerate() {
            if i > 0 { c_str.push_str(", "); }
            c_str.push_str(&format!("{} ({}T left)", c.building.name(), c.turns_remaining));
        }
        draw_text(&c_str, x + 215.0, q_bar_y + 24.0, 12.0, Color::from_rgba(240, 190, 90, 255));
    }
}

pub fn render_government_view(game: &mut GameState, ui: &mut UiState) {
    let screen_w = screen_width();
    let screen_h = screen_height();

    draw_rectangle(30.0, 56.0, screen_w - 60.0, screen_h - 70.0, Color::from_rgba(20, 16, 12, 252));
    draw_rectangle_lines(30.0, 56.0, screen_w - 60.0, screen_h - 70.0, 2.0, Color::from_rgba(220, 180, 80, 255));

    let player_fac = game.factions.get(&game.player_faction).unwrap().clone();

    // Title & Government Structure
    draw_text_bold(
        "Grand Council of Ministers | Imperial Cabinet & Constitution",
        50.0,
        84.0,
        21.0,
        Color::from_rgba(245, 225, 160, 255),
    );

    let gov_str = format!("Structure: {} [{}]", player_fac.government.name(), player_fac.government.bonus_desc());
    draw_text(
        &gov_str,
        50.0,
        104.0,
        12.0,
        Color::from_rgba(220, 205, 165, 255),
    );

    // Reform Government Button
    let reform_btn = Rect::new(screen_w - 230.0, 66.0, 175.0, 26.0);
    let m = mouse_position();
    let m_vec = Vec2::new(m.0, m.1);
    let ref_hov = reform_btn.contains(m_vec);
    draw_rectangle(reform_btn.x, reform_btn.y, reform_btn.w, reform_btn.h, if ref_hov { Color::from_rgba(140, 80, 30, 240) } else { Color::from_rgba(85, 45, 20, 220) });
    draw_rectangle_lines(reform_btn.x, reform_btn.y, reform_btn.w, reform_btn.h, 1.2, Color::from_rgba(245, 200, 90, 255));
    draw_text("⚖ Enact Reform (600g)", reform_btn.x + 10.0, reform_btn.y + 17.5, 12.0, WHITE);

    if is_mouse_button_pressed(MouseButton::Left) && ref_hov {
        let next_gov = match player_fac.government {
            GovernmentType::AbsoluteMonarchy => GovernmentType::ConstitutionalMonarchy,
            GovernmentType::ConstitutionalMonarchy => GovernmentType::Republic,
            GovernmentType::Republic => GovernmentType::AbsoluteMonarchy,
        };
        match game.reform_government(next_gov) {
            Ok(msg) => ui.set_toast(&msg),
            Err(err) => ui.set_toast(&err),
        }
    }

    // Campaign Objectives Progress Ribbon
    let (owned, holds_delhi, holds_satara, rank) = game.campaign_progress();
    let obj_y = 114.0;
    let obj_rect = Rect::new(50.0, obj_y, screen_w - 100.0, 22.0);
    draw_rectangle(obj_rect.x, obj_rect.y, obj_rect.w, obj_rect.h, Color::from_rgba(35, 30, 22, 220));
    draw_rectangle_lines(obj_rect.x, obj_rect.y, obj_rect.w, obj_rect.h, 1.0, Color::from_rgba(180, 145, 75, 180));
    let delhi_str = if holds_delhi { "[Del: HELD]" } else { "[Del: HOSTILE]" };
    let satara_str = if holds_satara { "[Satara: SECURED]" } else { "[Satara: LOST]" };
    draw_text(
        &format!("Objectives: Subahs Held: {} / 14 | {} | {} | Sovereign Prestige: Rank #{}", owned, delhi_str, satara_str, rank),
        obj_rect.x + 10.0,
        obj_rect.y + 15.0,
        11.5,
        Color::from_rgba(240, 220, 140, 255),
    );

    let mut card_y = 142.0;
    let card_h = 44.0;

    for m_inst in &player_fac.ministers {
        let card_rect = Rect::new(50.0, card_y, screen_w - 100.0, card_h);
        draw_rectangle(card_rect.x, card_rect.y, card_rect.w, card_rect.h, Color::from_rgba(35, 28, 22, 230));
        draw_rectangle_lines(card_rect.x, card_rect.y, card_rect.w, card_rect.h, 1.0, Color::from_rgba(180, 140, 70, 160));

        let stars_str = "*".repeat(m_inst.stars as usize);
        draw_text_bold(
            &format!("{}: {}", m_inst.role.title(), m_inst.name),
            card_rect.x + 10.0,
            card_rect.y + 18.0,
            13.5,
            Color::from_rgba(245, 230, 175, 255),
        );
        draw_text(
            &format!("Ability: [{}] | Loyalty: {}/10 | Trait: {}", stars_str, m_inst.loyalty, m_inst.trait_desc),
            card_rect.x + 10.0,
            card_rect.y + 34.0,
            11.0,
            Color::from_rgba(210, 195, 150, 255),
        );

        if m_inst.role != MinisterRole::HeadOfState {
            let dismiss_btn = Rect::new(card_rect.x + card_rect.w - 165.0, card_rect.y + 8.0, 155.0, 26.0);
            let dis_hov = dismiss_btn.contains(m_vec);
            draw_rectangle(dismiss_btn.x, dismiss_btn.y, dismiss_btn.w, dismiss_btn.h, if dis_hov { Color::from_rgba(140, 45, 30, 230) } else { Color::from_rgba(85, 30, 20, 200) });
            draw_rectangle_lines(dismiss_btn.x, dismiss_btn.y, dismiss_btn.w, dismiss_btn.h, 1.0, Color::from_rgba(230, 160, 80, 200));
            draw_text("Dismiss & Appoint (150g)", dismiss_btn.x + 8.0, dismiss_btn.y + 17.0, 11.0, WHITE);

            if is_mouse_button_pressed(MouseButton::Left) && dis_hov {
                match game.replace_minister(m_inst.role, 0) {
                    Ok(msg) => ui.set_toast(&msg),
                    Err(err) => ui.set_toast(&err),
                }
            }
        }

        card_y += card_h + 5.0;
    }

    // Candidate Statesmen Pool
    draw_text_bold("Statesmen Candidate Pool (Opposition / Available Statesmen):", 50.0, card_y + 15.0, 13.5, Color::from_rgba(240, 215, 130, 255));
    card_y += 20.0;

    let pool_w = (screen_w - 120.0) / 3.0;
    for (p_idx, cand) in player_fac.minister_pool.iter().take(3).enumerate() {
        let p_rect = Rect::new(50.0 + (p_idx as f32 * (pool_w + 10.0)), card_y, pool_w, 48.0);
        draw_rectangle(p_rect.x, p_rect.y, p_rect.w, p_rect.h, Color::from_rgba(28, 22, 18, 220));
        draw_rectangle_lines(p_rect.x, p_rect.y, p_rect.w, p_rect.h, 1.0, Color::from_rgba(160, 130, 70, 180));
        draw_text_bold(&cand.name, p_rect.x + 8.0, p_rect.y + 16.0, 12.5, Color::from_rgba(240, 225, 180, 255));
        let stars_str = "*".repeat(cand.stars as usize);
        draw_text(&format!("Ability: [{}] | Loyalty: {}/10", stars_str, cand.loyalty), p_rect.x + 8.0, p_rect.y + 30.0, 10.5, Color::from_rgba(220, 200, 140, 255));
        draw_text(&cand.trait_desc, p_rect.x + 8.0, p_rect.y + 42.0, 9.5, Color::from_rgba(180, 210, 160, 255));
    }
}

pub fn render_technology_view(game: &mut GameState) {
    let screen_w = screen_width();
    let screen_h = screen_height();

    draw_rectangle(30.0, 60.0, screen_w - 60.0, screen_h - 90.0, Color::from_rgba(20, 16, 12, 248));
    draw_rectangle_lines(30.0, 60.0, screen_w - 60.0, screen_h - 90.0, 2.0, Color::from_rgba(220, 180, 80, 255));

    draw_text(
        "Royal Academy & Technology Tree (18th Century Innovations)",
        50.0,
        95.0,
        22.0,
        Color::from_rgba(245, 225, 160, 255),
    );

    let player_fac = game.factions.get(&game.player_faction).unwrap();
    let current_res = player_fac.current_research.clone();

    if let Some((tech_id, turns)) = &current_res {
        draw_text(
            &format!("Currently Researching: {} ({} turns remaining)", tech_id, turns),
            50.0,
            122.0,
            15.0,
            Color::from_rgba(100, 220, 140, 255),
        );
    } else {
        draw_text(
            "Scholars idle! Select a breakthrough below to begin research.",
            50.0,
            122.0,
            15.0,
            Color::from_rgba(240, 180, 80, 255),
        );
    }

    let mut tech_to_start = None;
    let mut row_y = 145.0;
    for tech in &game.available_techs {
        let is_done = player_fac.researched_techs.contains(&tech.id.to_string());
        let is_in_progress = current_res.as_ref().map(|(id, _)| id == tech.id).unwrap_or(false);

        let card_rect = Rect::new(50.0, row_y, screen_w - 100.0, 58.0);
        let border_color = if is_done {
            Color::from_rgba(70, 180, 90, 200)
        } else if is_in_progress {
            Color::from_rgba(220, 180, 60, 255)
        } else {
            Color::from_rgba(150, 120, 70, 150)
        };

        draw_rectangle(card_rect.x, card_rect.y, card_rect.w, card_rect.h, Color::from_rgba(35, 28, 22, 230));
        draw_rectangle_lines(card_rect.x, card_rect.y, card_rect.w, card_rect.h, 1.5, border_color);

        draw_text(tech.name, card_rect.x + 15.0, card_rect.y + 22.0, 16.0, Color::from_rgba(245, 225, 170, 255));
        draw_text(
            &format!("Category: {:?} | Turns to Research: {} turns", tech.category, tech.cost_turns),
            card_rect.x + 15.0,
            card_rect.y + 40.0,
            12.0,
            Color::from_rgba(200, 185, 155, 255),
        );
        draw_text(tech.description, card_rect.x + 320.0, card_rect.y + 32.0, 13.0, Color::from_rgba(215, 205, 180, 255));

        let action_btn = Rect::new(card_rect.x + card_rect.w - 120.0, card_rect.y + 14.0, 105.0, 30.0);
        if is_done {
            draw_rectangle(action_btn.x, action_btn.y, action_btn.w, action_btn.h, Color::from_rgba(40, 120, 50, 180));
            draw_text("Researched", action_btn.x + 12.0, action_btn.y + 20.0, 12.5, WHITE);
        } else if is_in_progress {
            draw_rectangle(action_btn.x, action_btn.y, action_btn.w, action_btn.h, Color::from_rgba(150, 110, 25, 220));
            draw_text("In Progress", action_btn.x + 12.0, action_btn.y + 20.0, 12.5, WHITE);
        } else {
            draw_rectangle(action_btn.x, action_btn.y, action_btn.w, action_btn.h, Color::from_rgba(120, 75, 25, 240));
            draw_rectangle_lines(action_btn.x, action_btn.y, action_btn.w, action_btn.h, 1.0, Color::from_rgba(220, 180, 80, 220));
            draw_text("Research", action_btn.x + 22.0, action_btn.y + 20.0, 13.0, WHITE);

            if is_mouse_button_pressed(MouseButton::Left) {
                let m = mouse_position();
                if action_btn.contains(Vec2::new(m.0, m.1)) {
                    tech_to_start = Some((tech.id.to_string(), tech.cost_turns));
                }
            }
        }

        row_y += 68.0;
    }

    if let Some((tech_id, cost_turns)) = tech_to_start {
        if let Some(fac) = game.factions.get_mut(&game.player_faction) {
            fac.current_research = Some((tech_id, cost_turns));
        }
    }
}

pub fn render_battle_report_modal(game: &mut GameState) {
    let report = match &game.latest_battle {
        Some(r) => r.clone(),
        None => return,
    };

    let w = 580.0;
    let h = 420.0;
    let x = (screen_width() - w) / 2.0;
    let y = (screen_height() - h) / 2.0;

    draw_rectangle(x, y, w, h, Color::from_rgba(20, 15, 10, 252));
    draw_rectangle_lines(x, y, w, h, 2.5, Color::from_rgba(225, 185, 75, 255));

    // Result banner
    let player_is_attacker = report.attacker_faction == game.player_faction;
    let player_won = (player_is_attacker && report.attacker_won) || (!player_is_attacker && !report.attacker_won);

    let (outcome_title, outcome_color) = if player_won {
        if player_is_attacker {
            (
                format!("GLORIOUS VICTORY AT {}! (PROVINCE CONQUERED)", report.province_name.to_uppercase()),
                Color::from_rgba(90, 225, 110, 255),
            )
        } else {
            (
                format!("HEROIC DEFENSE AT {}! (ENEMY REPULSED)", report.province_name.to_uppercase()),
                Color::from_rgba(90, 225, 110, 255),
            )
        }
    } else {
        if player_is_attacker {
            (
                format!("DEFEAT AT {}! (OFFENSIVE REPULSED)", report.province_name.to_uppercase()),
                Color::from_rgba(240, 85, 85, 255),
            )
        } else {
            (
                format!("DEFEAT AT {}! (PROVINCE FALLEN TO {})", report.province_name.to_uppercase(), report.attacker_faction.name().to_uppercase()),
                Color::from_rgba(240, 85, 85, 255),
            )
        }
    };
    draw_text_bold(&outcome_title, x + 25.0, y + 36.0, 21.0, outcome_color);

    // Forces summary
    draw_text(
        &format!("Attacker: {} ({} initial | {} casualties)", report.attacker_faction.name(), report.attacker_initial, report.attacker_losses),
        x + 25.0,
        y + 68.0,
        14.0,
        Color::from_rgba(245, 225, 160, 255),
    );
    draw_text(
        &format!("Defender: {} ({} initial | {} casualties)", report.defender_faction.name(), report.defender_initial, report.defender_losses),
        x + 25.0,
        y + 88.0,
        14.0,
        Color::from_rgba(225, 205, 180, 255),
    );

    // Phase logs
    let mut log_y = y + 115.0;
    for phase in &report.phases {
        let p_rect = Rect::new(x + 20.0, log_y, w - 40.0, 72.0);
        draw_rectangle(p_rect.x, p_rect.y, p_rect.w, p_rect.h, Color::from_rgba(35, 28, 20, 230));
        draw_rectangle_lines(p_rect.x, p_rect.y, p_rect.w, p_rect.h, 1.0, Color::from_rgba(150, 120, 60, 150));

        draw_text_bold(&phase.title, p_rect.x + 10.0, p_rect.y + 18.0, 14.0, Color::from_rgba(240, 215, 140, 255));
        draw_text(&phase.attacker_action, p_rect.x + 10.0, p_rect.y + 36.0, 12.0, Color::from_rgba(220, 210, 190, 255));
        draw_text(&phase.defender_action, p_rect.x + 10.0, p_rect.y + 52.0, 12.0, Color::from_rgba(195, 185, 165, 255));
        draw_text_bold(
            &format!("Losses: -{} / -{}", phase.attacker_losses, phase.defender_losses),
            p_rect.x + p_rect.w - 130.0,
            p_rect.y + 18.0,
            12.5,
            Color::from_rgba(235, 140, 90, 255),
        );

        log_y += 78.0;
    }

    // Dismiss button
    let dismiss_btn = Rect::new(x + (w - 140.0) / 2.0, y + h - 44.0, 140.0, 32.0);
    draw_rectangle(dismiss_btn.x, dismiss_btn.y, dismiss_btn.w, dismiss_btn.h, Color::from_rgba(140, 45, 30, 240));
    draw_rectangle_lines(dismiss_btn.x, dismiss_btn.y, dismiss_btn.w, dismiss_btn.h, 1.5, Color::from_rgba(235, 200, 90, 255));
    draw_text_bold("DISMISS [X]", dismiss_btn.x + 28.0, dismiss_btn.y + 21.0, 14.5, WHITE);

    let mut dismiss = false;
    if is_mouse_button_pressed(MouseButton::Left) || is_mouse_button_released(MouseButton::Left) {
        let m = mouse_position();
        if dismiss_btn.contains(Vec2::new(m.0, m.1)) {
            dismiss = true;
        }
    }
    if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Escape) {
        dismiss = true;
    }
    if dismiss {
        game.latest_battle = None;
        game.active_tab = ActiveTab::Map;
    }
}

pub fn render_faction_select_view(game: &mut GameState, ui: &mut UiState) {
    let screen_w = screen_width();
    let screen_h = screen_height();

    draw_rectangle(15.0, 52.0, screen_w - 30.0, screen_h - 68.0, Color::from_rgba(18, 14, 10, 252));
    draw_rectangle_lines(15.0, 52.0, screen_w - 30.0, screen_h - 68.0, 2.5, Color::from_rgba(220, 180, 80, 255));

    // Return to Main Menu button at top right
    let ret_btn = Rect::new(screen_w - 180.0, 68.0, 150.0, 36.0);
    draw_rectangle(ret_btn.x, ret_btn.y, ret_btn.w, ret_btn.h, Color::from_rgba(50, 40, 30, 220));
    draw_rectangle_lines(ret_btn.x, ret_btn.y, ret_btn.w, ret_btn.h, 1.5, Color::from_rgba(180, 140, 70, 180));
    draw_text("< Main Menu", ret_btn.x + 28.0, ret_btn.y + 23.0, 14.5, Color::from_rgba(240, 225, 185, 255));

    if is_mouse_button_pressed(MouseButton::Left) {
        let m = mouse_position();
        if ret_btn.contains(Vec2::new(m.0, m.1)) {
            ui.refresh_saves();
            game.active_tab = ActiveTab::MainMenu;
        }
    }

    draw_text(
        "MARATHA: WAR OF INDEPENDENCE -- SELECT YOUR FACTION (1700 A.D.)",
        35.0,
        86.0,
        24.0,
        Color::from_rgba(245, 220, 140, 255),
    );
    draw_text(
        "Choose a major sovereign power to command across the Indian Subcontinent grand campaign",
        35.0,
        110.0,
        15.0,
        Color::from_rgba(210, 195, 160, 255),
    );

    let factions = [
        (
            crate::models::FactionId::Maratha,
            "Maratha Confederacy",
            "Maharani Tarabai I (Regent)",
            "Satara (Bijapur) & Arcot",
            2,
            "Bargir & Silhadar Cavalry, Hindu Matchlockmen",
            "Moderate",
            "Rebel powerhouse of the Deccan. Wield rapid cavalry raids to shatter Mughal hegemony.",
        ),
        (
            crate::models::FactionId::Mughal,
            "Mughal Empire",
            "Emperor Aurangzeb (Alamgir I)",
            "Delhi / Akbarabad",
            11,
            "Mansabdari Guards, Heavy Topchi Cannons, Sepoys",
            "Hard",
            "Dominant gunpowder empire spanning northern India. Immense treasury, but burdened by rebellions.",
        ),
        (
            crate::models::FactionId::Mysore,
            "Sultanate of Mysore",
            "Chikka Devaraja Wodeyar",
            "Mysore (Karnataka)",
            1,
            "Mysorean Rocket Troops, Disciplined Sepoy Battalions",
            "Challenging",
            "Rich sovereign kingdom of the south. Flourishing spice routes and fortified bastions.",
        ),
        (
            crate::models::FactionId::Portugal,
            "Kingdom of Portugal",
            "Viceroy of the Indies",
            "Goa Velha",
            1,
            "Colonial Line Regiments, Heavy Harbor Batteries",
            "Expert",
            "European maritime trading enclave. Dominant sea trade, coastal guns, and trading galleons.",
        ),
    ];

    let card_w = (screen_w - 60.0) / 2.0;
    let card_h = 160.0;
    let mut chosen_faction = None;

    for (i, (f_id, name, ruler, capital, prov_count, roster, diff, desc)) in factions.iter().enumerate() {
        let col = i % 2;
        let row = i / 2;
        let card_x = 25.0 + (col as f32 * (card_w + 10.0));
        let card_y = 130.0 + (row as f32 * (card_h + 12.0));

        let is_current = game.player_faction == *f_id;
        let (f_c1, f_c2) = f_id.flag_colors();

        let border_c = if is_current {
            Color::from_rgba(255, 220, 80, 255)
        } else {
            Color::from_rgba(150, 120, 70, 180)
        };
        let bg_c = if is_current {
            Color::from_rgba(45, 34, 22, 245)
        } else {
            Color::from_rgba(28, 22, 16, 235)
        };

        let card_rect = Rect::new(card_x, card_y, card_w, card_h);
        draw_rectangle(card_rect.x, card_rect.y, card_rect.w, card_rect.h, bg_c);
        draw_rectangle_lines(card_rect.x, card_rect.y, card_rect.w, card_rect.h, if is_current { 2.5 } else { 1.2 }, border_c);

        // Flag banner swatch
        draw_rectangle(card_x + 12.0, card_y + 14.0, 36.0, 36.0, f_c1);
        draw_rectangle_lines(card_x + 12.0, card_y + 14.0, 36.0, 36.0, 2.0, f_c2);

        // Titles
        draw_text(name, card_x + 56.0, card_y + 28.0, 18.0, Color::from_rgba(250, 230, 170, 255));
        draw_text(&format!("Ruler: {} | Capital: {}", ruler, capital), card_x + 56.0, card_y + 45.0, 13.5, Color::from_rgba(215, 200, 165, 255));

        // Details
        draw_text(
            &format!("Starting Provinces: {} | Campaign Difficulty: {}", prov_count, diff),
            card_x + 15.0,
            card_y + 74.0,
            14.0,
            Color::from_rgba(235, 205, 110, 255),
        );
        draw_text(&format!("Special Units: {}", roster), card_x + 15.0, card_y + 95.0, 13.0, Color::from_rgba(180, 215, 160, 255));
        draw_text(desc, card_x + 15.0, card_y + 120.0, 12.0, Color::from_rgba(205, 195, 175, 255));

        // Select button
        let btn_w = 115.0;
        let btn_h = 32.0;
        let btn_x = card_x + card_w - btn_w - 12.0;
        let btn_y = card_y + 12.0;
        let select_btn = Rect::new(btn_x, btn_y, btn_w, btn_h);

        if is_current {
            draw_rectangle(select_btn.x, select_btn.y, select_btn.w, select_btn.h, Color::from_rgba(160, 110, 30, 240));
            draw_rectangle_lines(select_btn.x, select_btn.y, select_btn.w, select_btn.h, 1.5, Color::from_rgba(255, 230, 120, 255));
            draw_text("SELECTED", select_btn.x + 16.0, select_btn.y + 21.0, 14.0, WHITE);
        } else {
            draw_rectangle(select_btn.x, select_btn.y, select_btn.w, select_btn.h, Color::from_rgba(55, 45, 35, 220));
            draw_rectangle_lines(select_btn.x, select_btn.y, select_btn.w, select_btn.h, 1.0, Color::from_rgba(180, 150, 80, 180));
            draw_text("Select", select_btn.x + 35.0, select_btn.y + 21.0, 14.0, Color::from_rgba(230, 215, 180, 255));

            if is_mouse_button_pressed(MouseButton::Left) {
                let m = mouse_position();
                if select_btn.contains(Vec2::new(m.0, m.1)) || card_rect.contains(Vec2::new(m.0, m.1)) {
                    chosen_faction = Some(*f_id);
                }
            }
        }
    }

    if let Some(f_id) = chosen_faction {
        game.player_faction = f_id;
    }

    // Launch Campaign button
    let launch_w = 320.0;
    let launch_h = 48.0;
    let launch_x = (screen_w - launch_w) / 2.0;
    let launch_y = screen_h - 75.0;
    let launch_btn = Rect::new(launch_x, launch_y, launch_w, launch_h);

    draw_rectangle(launch_btn.x, launch_btn.y, launch_btn.w, launch_btn.h, Color::from_rgba(150, 45, 25, 245));
    draw_rectangle_lines(launch_btn.x, launch_btn.y, launch_btn.w, launch_btn.h, 2.5, Color::from_rgba(255, 225, 90, 255));
    draw_text("COMMENCE GRAND CAMPAIGN", launch_btn.x + 24.0, launch_btn.y + 30.0, 17.5, Color::from_rgba(255, 240, 180, 255));

    let mut start_game = false;
    if is_mouse_button_pressed(MouseButton::Left) {
        let m = mouse_position();
        if launch_btn.contains(Vec2::new(m.0, m.1)) {
            start_game = true;
        }
    }
    if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
        start_game = true;
    }

    if start_game {
        let faction = game.player_faction;
        game.start_as_faction(faction);
    }
}

// -------------------------------------------------------------
// Empire: Total War Save / Load & Front-End Menu Views
// -------------------------------------------------------------

pub fn render_main_menu(game: &mut GameState, ui: &mut UiState) {
    let screen_w = screen_width();
    let screen_h = screen_height();

    // Dark Regal Canvas Background with radial warmth
    draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::from_rgba(15, 12, 10, 255));

    // Outer and Inner Gold Ornamental Frames
    draw_rectangle_lines(16.0, 16.0, screen_w - 32.0, screen_h - 32.0, 2.5, Color::from_rgba(180, 140, 60, 220));
    draw_rectangle_lines(22.0, 22.0, screen_w - 44.0, screen_h - 44.0, 1.0, Color::from_rgba(120, 95, 45, 140));

    // Header Title
    let title_text = "MARATHA: WAR OF INDEPENDENCE";
    let sub_text = "THE GRAND CAMPAIGN FOR INDIA (1700 - 1750 A.D.)";
    let title_fs = 36.0;
    let title_w = if let Some(fonts) = FONTS.get() {
        macroquad::text::measure_text(title_text, Some(&fonts.bold), title_fs as u16, 1.0).width
    } else {
        macroquad::text::measure_text(title_text, None, title_fs as u16, 1.0).width
    };
    let title_x = (screen_w - title_w) / 2.0;

    let sub_w = if let Some(fonts) = FONTS.get() {
        macroquad::text::measure_text(sub_text, Some(&fonts.regular), 16, 1.0).width
    } else {
        macroquad::text::measure_text(sub_text, None, 16, 1.0).width
    };
    let sub_x = (screen_w - sub_w) / 2.0;

    draw_text_bold(
        title_text,
        title_x,
        95.0,
        title_fs,
        Color::from_rgba(245, 215, 115, 255),
    );
    draw_text(
        sub_text,
        sub_x,
        125.0,
        16.0,
        Color::from_rgba(210, 190, 150, 255),
    );

    // Decorative Crests on Left and Right of Title
    // Maratha Saffron Bhagwa crest
    let crest_y = 68.0;
    let left_crest_x = (title_x - 64.0).max(28.0);
    let right_crest_x = (title_x + title_w + 16.0).min(screen_w - 76.0);

    draw_rectangle(left_crest_x, crest_y, 48.0, 48.0, Color::from_rgba(235, 115, 20, 255));
    draw_rectangle_lines(left_crest_x, crest_y, 48.0, 48.0, 2.0, Color::from_rgba(255, 200, 40, 255));
    draw_text_bold("MWI", left_crest_x + 6.0, crest_y + 30.0, 15.0, Color::from_rgba(255, 240, 180, 255));

    // Mughal Emerald & Gold Sun crest
    draw_rectangle(right_crest_x, crest_y, 48.0, 48.0, Color::from_rgba(25, 95, 45, 255));
    draw_rectangle_lines(right_crest_x, crest_y, 48.0, 48.0, 2.0, Color::from_rgba(240, 200, 60, 255));
    draw_text_bold("1700", right_crest_x + 6.0, crest_y + 30.0, 15.0, Color::from_rgba(255, 240, 180, 255));

    // Center Menu Options Box
    let box_w = 480.0;
    let box_h = 360.0;
    let box_x = (screen_w - box_w) / 2.0;
    let box_y = 170.0;

    draw_rectangle(box_x, box_y, box_w, box_h, Color::from_rgba(24, 18, 14, 240));
    draw_rectangle_lines(box_x, box_y, box_w, box_h, 2.0, Color::from_rgba(200, 160, 70, 200));

    let btn_w = 400.0;
    let btn_h = 52.0;
    let btn_x = box_x + 40.0;

    let has_saves = !ui.save_list_cache.is_empty();
    let latest_save = ui.save_list_cache.first();

    // 1. CONTINUE CAMPAIGN BUTTON
    let b1_y = box_y + 35.0;
    let b1_rect = Rect::new(btn_x, b1_y, btn_w, btn_h);
    let (b1_bg, b1_border, b1_txt_col) = if has_saves {
        (
            Color::from_rgba(150, 45, 30, 240),
            Color::from_rgba(255, 220, 90, 255),
            Color::from_rgba(255, 245, 200, 255),
        )
    } else {
        (
            Color::from_rgba(40, 32, 26, 200),
            Color::from_rgba(100, 85, 70, 150),
            Color::from_rgba(130, 120, 110, 255),
        )
    };
    draw_rectangle(b1_rect.x, b1_rect.y, b1_rect.w, b1_rect.h, b1_bg);
    draw_rectangle_lines(b1_rect.x, b1_rect.y, b1_rect.w, b1_rect.h, 2.0, b1_border);
    draw_text("CONTINUE CAMPAIGN", b1_rect.x + 95.0, b1_rect.y + 26.0, 17.5, b1_txt_col);
    if let Some(s) = latest_save {
        let sub = format!("{} (Turn {}, {} A.D.)", s.display_name, s.turn, s.year);
        draw_text(&sub, b1_rect.x + 30.0, b1_rect.y + 44.0, 12.0, Color::from_rgba(240, 215, 150, 255));
    } else {
        draw_text("(No Previous Campaign Saves Found)", b1_rect.x + 85.0, b1_rect.y + 42.0, 12.0, Color::from_rgba(140, 130, 120, 255));
    }

    // 2. START NEW CAMPAIGN BUTTON
    let b2_y = b1_y + 68.0;
    let b2_rect = Rect::new(btn_x, b2_y, btn_w, btn_h);
    draw_rectangle(b2_rect.x, b2_rect.y, b2_rect.w, b2_rect.h, Color::from_rgba(65, 45, 25, 230));
    draw_rectangle_lines(b2_rect.x, b2_rect.y, b2_rect.w, b2_rect.h, 2.0, Color::from_rgba(235, 185, 80, 230));
    draw_text("START NEW CAMPAIGN", b2_rect.x + 90.0, b2_rect.y + 32.0, 17.5, Color::from_rgba(255, 240, 190, 255));

    // 3. LOAD CAMPAIGN BUTTON
    let b3_y = b2_y + 68.0;
    let b3_rect = Rect::new(btn_x, b3_y, btn_w, btn_h);
    draw_rectangle(b3_rect.x, b3_rect.y, b3_rect.w, b3_rect.h, Color::from_rgba(45, 38, 30, 230));
    draw_rectangle_lines(b3_rect.x, b3_rect.y, b3_rect.w, b3_rect.h, 1.8, Color::from_rgba(210, 170, 75, 200));
    draw_text("LOAD CAMPAIGN / SAVES", b3_rect.x + 85.0, b3_rect.y + 32.0, 17.5, Color::from_rgba(245, 230, 180, 255));

    // 4. HOW TO PLAY / CONTROLS & MANUAL BUTTON
    let b4_y = b3_y + 68.0;
    let b4_rect = Rect::new(btn_x, b4_y, btn_w, btn_h);
    draw_rectangle(b4_rect.x, b4_rect.y, b4_rect.w, b4_rect.h, Color::from_rgba(35, 30, 25, 230));
    draw_rectangle_lines(b4_rect.x, b4_rect.y, b4_rect.w, b4_rect.h, 1.5, Color::from_rgba(180, 145, 70, 180));
    draw_text("HOW TO PLAY & MANUAL", b4_rect.x + 92.0, b4_rect.y + 32.0, 17.5, Color::from_rgba(230, 215, 175, 255));

    // Keyboard navigation
    if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
        if has_saves {
            if let Some(s) = latest_save {
                let id = s.id.clone();
                if let Ok(meta) = crate::storage::load_campaign_state(&id, game) {
                    ui.set_toast(&format!("Resumed: {}", meta.display_name));
                    game.active_tab = ActiveTab::Map;
                    return;
                }
            }
        }
        game.active_tab = ActiveTab::FactionSelect;
        return;
    }

    // Mouse Interaction
    if is_mouse_button_pressed(MouseButton::Left) {
        let m = mouse_position();
        let m_vec = Vec2::new(m.0, m.1);
        if b1_rect.contains(m_vec) && has_saves {
            if let Some(s) = latest_save {
                let id = s.id.clone();
                match crate::storage::load_campaign_state(&id, game) {
                    Ok(meta) => {
                        ui.set_toast(&format!("Resumed: {}", meta.display_name));
                        game.active_tab = ActiveTab::Map;
                    }
                    Err(e) => {
                        ui.set_toast(&format!("Load error: {}", e));
                    }
                }
            }
        } else if b2_rect.contains(m_vec) {
            game.active_tab = ActiveTab::FactionSelect;
        } else if b3_rect.contains(m_vec) {
            ui.refresh_saves();
            ui.return_tab_after_menu = ActiveTab::MainMenu;
            game.active_tab = ActiveTab::LoadGameMenu;
        } else if b4_rect.contains(m_vec) {
            ui.return_tab_after_menu = ActiveTab::MainMenu;
            game.active_tab = ActiveTab::HowToPlay;
        }
    }

    // Footer Info
    let footer_y = screen_h - 55.0;
    draw_text(
        "Grand Strategy Campaign Engine | Browser Persistent Local Storage Enabled",
        screen_w / 2.0 - 240.0,
        footer_y,
        13.5,
        Color::from_rgba(180, 160, 130, 200),
    );
    draw_text(
        "Keyboard: F5 QuickSave | F9 QuickLoad | ESC Pause Menu | Space End Turn",
        screen_w / 2.0 - 230.0,
        footer_y + 20.0,
        12.0,
        Color::from_rgba(145, 130, 105, 180),
    );
}

pub fn render_load_game_view(game: &mut GameState, ui: &mut UiState) {
    let screen_w = screen_width();
    let screen_h = screen_height();

    // Background Container
    draw_rectangle(15.0, 15.0, screen_w - 30.0, screen_h - 30.0, Color::from_rgba(20, 16, 12, 252));
    draw_rectangle_lines(15.0, 15.0, screen_w - 30.0, screen_h - 30.0, 2.5, Color::from_rgba(215, 175, 75, 255));

    // Header Title
    draw_text(
        "CAMPAIGN ARCHIVES -- LOAD & MANAGE SAVES",
        35.0,
        50.0,
        24.0,
        Color::from_rgba(245, 220, 130, 255),
    );
    draw_text(
        "Select a saved campaign slot from your browser storage to resume, or save your active campaign.",
        35.0,
        72.0,
        14.0,
        Color::from_rgba(205, 190, 155, 255),
    );

    // Top Right Navigation Tabs
    let tab_w = 175.0;
    let tab_h = 32.0;
    let tab_y = 38.0;

    let load_tab_rect = Rect::new(screen_w - 35.0 - tab_w * 2.0 - 10.0, tab_y, tab_w, tab_h);
    draw_rectangle(load_tab_rect.x, load_tab_rect.y, load_tab_rect.w, load_tab_rect.h, Color::from_rgba(75, 52, 28, 245));
    draw_rectangle_lines(load_tab_rect.x, load_tab_rect.y, load_tab_rect.w, load_tab_rect.h, 2.0, Color::from_rgba(255, 220, 85, 255));
    draw_text("LOAD ARCHIVES", load_tab_rect.x + 24.0, load_tab_rect.y + 21.0, 14.0, Color::from_rgba(255, 240, 190, 255));

    let save_tab_rect = Rect::new(screen_w - 35.0 - tab_w, tab_y, tab_w, tab_h);
    let m = mouse_position();
    let m_vec = Vec2::new(m.0, m.1);
    let save_tab_hov = save_tab_rect.contains(m_vec);
    draw_rectangle(
        save_tab_rect.x,
        save_tab_rect.y,
        save_tab_rect.w,
        save_tab_rect.h,
        if save_tab_hov { Color::from_rgba(150, 45, 25, 240) } else { Color::from_rgba(45, 35, 26, 220) },
    );
    draw_rectangle_lines(save_tab_rect.x, save_tab_rect.y, save_tab_rect.w, save_tab_rect.h, 1.5, Color::from_rgba(220, 180, 80, 200));
    draw_text("+ SAVE PROGRESS", save_tab_rect.x + 18.0, save_tab_rect.y + 21.0, 14.0, Color::from_rgba(235, 220, 185, 255));

    let mut go_save = false;
    if (is_mouse_button_pressed(MouseButton::Left) || is_mouse_button_released(MouseButton::Left)) && save_tab_rect.contains(m_vec) {
        go_save = true;
    }
    if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::F5) {
        go_save = true;
    }

    let left_w = 460.0;
    let content_y = 90.0;
    let content_h = screen_h - content_y - 80.0;

    // LEFT PANE: Saves List
    draw_rectangle(35.0, content_y, left_w, content_h, Color::from_rgba(28, 22, 16, 230));
    draw_rectangle_lines(35.0, content_y, left_w, content_h, 1.5, Color::from_rgba(160, 125, 65, 180));

    if ui.save_list_cache.is_empty() {
        draw_text("No saved campaigns found in browser storage.", 55.0, content_y + 60.0, 15.0, Color::from_rgba(180, 160, 130, 255));
        draw_text("Start a new campaign or import a save file (.json) below.", 55.0, content_y + 85.0, 13.0, Color::from_rgba(150, 135, 115, 255));
    } else {
        let item_h = 58.0;
        let mut clicked_slot = None;

        for (idx, slot) in ui.save_list_cache.iter().enumerate() {
            let item_y = content_y + 10.0 + (idx as f32 * (item_h + 8.0));
            if item_y + item_h > content_y + content_h {
                break; // Limit to viewport height
            }
            let item_rect = Rect::new(45.0, item_y, left_w - 20.0, item_h);
            let is_sel = ui.selected_save_slot.as_ref() == Some(&slot.id);

            let (bg, border) = if is_sel {
                (Color::from_rgba(75, 52, 28, 255), Color::from_rgba(255, 220, 85, 255))
            } else {
                (Color::from_rgba(38, 30, 22, 220), Color::from_rgba(130, 105, 55, 160))
            };
            draw_rectangle(item_rect.x, item_rect.y, item_rect.w, item_rect.h, bg);
            draw_rectangle_lines(item_rect.x, item_rect.y, item_rect.w, item_rect.h, if is_sel { 2.0 } else { 1.0 }, border);

            // Flag swatch
            let (f_c1, f_c2) = slot.player_faction.flag_colors();
            draw_rectangle(item_rect.x + 8.0, item_rect.y + 8.0, 36.0, 42.0, f_c1);
            draw_rectangle_lines(item_rect.x + 8.0, item_rect.y + 8.0, 36.0, 42.0, 1.5, f_c2);

            // Title & Info
            draw_text(&slot.display_name, item_rect.x + 52.0, item_rect.y + 24.0, 16.0, Color::from_rgba(245, 230, 185, 255));
            let season_str = match slot.season {
                crate::models::Season::Summer => "Summer",
                crate::models::Season::Winter => "Winter",
            };
            let info = format!("{} | Turn {} ({} {}) | {} Provs", slot.player_faction.name(), slot.turn, season_str, slot.year, slot.provinces_owned);
            draw_text(&info, item_rect.x + 52.0, item_rect.y + 44.0, 12.0, Color::from_rgba(200, 180, 140, 255));

            if is_mouse_button_pressed(MouseButton::Left) {
                let m = mouse_position();
                if item_rect.contains(Vec2::new(m.0, m.1)) {
                    clicked_slot = Some(slot.id.clone());
                }
            }
        }

        if let Some(s_id) = clicked_slot {
            ui.selected_save_slot = Some(s_id);
        }
    }

    // RIGHT PANE: Selected Save Details & Actions
    let right_x = 35.0 + left_w + 20.0;
    let right_w = screen_w - right_x - 35.0;
    draw_rectangle(right_x, content_y, right_w, content_h, Color::from_rgba(28, 22, 16, 230));
    draw_rectangle_lines(right_x, content_y, right_w, content_h, 1.5, Color::from_rgba(160, 125, 65, 180));

    let selected_meta = ui
        .selected_save_slot
        .as_ref()
        .and_then(|id| ui.save_list_cache.iter().find(|s| &s.id == id));

    if let Some(slot) = selected_meta {
        // Detailed Card
        let (f_c1, f_c2) = slot.player_faction.flag_colors();
        draw_rectangle(right_x + 25.0, content_y + 25.0, 56.0, 56.0, f_c1);
        draw_rectangle_lines(right_x + 25.0, content_y + 25.0, 56.0, 56.0, 2.0, f_c2);

        draw_text(&slot.display_name, right_x + 95.0, content_y + 48.0, 24.0, Color::from_rgba(250, 225, 130, 255));
        draw_text(slot.player_faction.name(), right_x + 95.0, content_y + 72.0, 16.0, Color::from_rgba(220, 200, 160, 255));

        let season_str = match slot.season {
            crate::models::Season::Summer => "Summer",
            crate::models::Season::Winter => "Winter",
        };

        // Stats grid
        let stats_y = content_y + 115.0;
        draw_rectangle(right_x + 25.0, stats_y, right_w - 50.0, 140.0, Color::from_rgba(20, 16, 12, 200));
        draw_rectangle_lines(right_x + 25.0, stats_y, right_w - 50.0, 140.0, 1.0, Color::from_rgba(140, 110, 55, 150));

        draw_text(&format!("Turn Number: {}", slot.turn), right_x + 45.0, stats_y + 35.0, 15.0, Color::from_rgba(235, 215, 165, 255));
        draw_text(&format!("Calendar Era: {} {} A.D.", season_str, slot.year), right_x + 45.0, stats_y + 65.0, 15.0, Color::from_rgba(235, 215, 165, 255));
        draw_text(&format!("Territory Held: {} / 18 Indian Provinces", slot.provinces_owned), right_x + 45.0, stats_y + 95.0, 15.0, Color::from_rgba(235, 215, 165, 255));
        draw_text(&format!("Slot Identifier: {}", slot.id), right_x + 45.0, stats_y + 125.0, 13.0, Color::from_rgba(180, 165, 140, 255));

        // Action Buttons inside Right Pane
        let btn_y = content_y + content_h - 75.0;
        let act_btn_w = (right_w - 70.0) / 3.0;

        // 1. LOAD BUTTON
        let load_rect = Rect::new(right_x + 25.0, btn_y, act_btn_w, 48.0);
        draw_rectangle(load_rect.x, load_rect.y, load_rect.w, load_rect.h, Color::from_rgba(150, 45, 25, 245));
        draw_rectangle_lines(load_rect.x, load_rect.y, load_rect.w, load_rect.h, 2.0, Color::from_rgba(255, 220, 85, 255));
        draw_text("LOAD GAME", load_rect.x + (act_btn_w - 95.0) / 2.0, load_rect.y + 30.0, 16.5, Color::from_rgba(255, 240, 190, 255));

        // 2. DELETE BUTTON
        let del_rect = Rect::new(right_x + 35.0 + act_btn_w, btn_y, act_btn_w, 48.0);
        draw_rectangle(del_rect.x, del_rect.y, del_rect.w, del_rect.h, Color::from_rgba(75, 25, 25, 220));
        draw_rectangle_lines(del_rect.x, del_rect.y, del_rect.w, del_rect.h, 1.5, Color::from_rgba(220, 80, 80, 200));
        draw_text("DELETE SAVE", del_rect.x + (act_btn_w - 105.0) / 2.0, del_rect.y + 30.0, 15.0, Color::from_rgba(255, 200, 200, 255));

        // 3. EXPORT BUTTON
        let exp_rect = Rect::new(right_x + 45.0 + act_btn_w * 2.0, btn_y, act_btn_w, 48.0);
        draw_rectangle(exp_rect.x, exp_rect.y, exp_rect.w, exp_rect.h, Color::from_rgba(40, 45, 55, 220));
        draw_rectangle_lines(exp_rect.x, exp_rect.y, exp_rect.w, exp_rect.h, 1.5, Color::from_rgba(100, 160, 220, 200));
        draw_text("EXPORT FILE", exp_rect.x + (act_btn_w - 95.0) / 2.0, exp_rect.y + 30.0, 15.0, Color::from_rgba(210, 235, 255, 255));

        if is_mouse_button_pressed(MouseButton::Left) {
            let m = mouse_position();
            let m_vec = Vec2::new(m.0, m.1);
            if load_rect.contains(m_vec) {
                let id = slot.id.clone();
                match crate::storage::load_campaign_state(&id, game) {
                    Ok(m) => {
                        ui.set_toast(&format!("Loaded: {}", m.display_name));
                        game.active_tab = ActiveTab::Map;
                    }
                    Err(e) => {
                        ui.set_toast(&format!("Load error: {}", e));
                    }
                }
            } else if del_rect.contains(m_vec) {
                ui.delete_confirm_slot = Some(slot.id.clone());
            } else if exp_rect.contains(m_vec) {
                if let Some(json) = crate::storage::export_save_to_json(&slot.id) {
                    let fname = format!("mwi_{}_{}.json", slot.id, slot.turn);
                    crate::storage::trigger_browser_download(&fname, &json);
                    ui.set_toast("Save file exported to downloads!");
                }
            }
        }
    } else {
        draw_text("Select a save file on the left to inspect or load.", right_x + 40.0, content_y + 80.0, 16.0, Color::from_rgba(180, 165, 140, 255));

        // Save current progress button right here in the right pane
        let save_right_btn = Rect::new(right_x + 40.0, content_y + 120.0, 270.0, 46.0);
        let m = mouse_position();
        let m_vec = Vec2::new(m.0, m.1);
        let s_hov = save_right_btn.contains(m_vec);
        draw_rectangle(
            save_right_btn.x,
            save_right_btn.y,
            save_right_btn.w,
            save_right_btn.h,
            if s_hov { Color::from_rgba(175, 45, 25, 245) } else { Color::from_rgba(135, 38, 20, 230) },
        );
        draw_rectangle_lines(save_right_btn.x, save_right_btn.y, save_right_btn.w, save_right_btn.h, 2.0, Color::from_rgba(255, 220, 85, 255));
        draw_text("+ SAVE CURRENT PROGRESS", save_right_btn.x + 24.0, save_right_btn.y + 28.0, 14.5, Color::from_rgba(255, 240, 190, 255));
        if (is_mouse_button_pressed(MouseButton::Left) || is_mouse_button_released(MouseButton::Left)) && s_hov {
            go_save = true;
        }
    }

    // BOTTOM GLOBAL TOOLBAR
    let bot_y = screen_h - 60.0;

    // Import Button
    let imp_rect = Rect::new(35.0, bot_y, 220.0, 38.0);
    draw_rectangle(imp_rect.x, imp_rect.y, imp_rect.w, imp_rect.h, Color::from_rgba(45, 40, 32, 230));
    draw_rectangle_lines(imp_rect.x, imp_rect.y, imp_rect.w, imp_rect.h, 1.5, Color::from_rgba(190, 150, 70, 200));
    draw_text("IMPORT SAVE FILE (.JSON)", imp_rect.x + 14.0, imp_rect.y + 24.0, 13.5, Color::from_rgba(240, 225, 180, 255));

    // Save Current Progress Button in bottom toolbar
    let save_rect = Rect::new(275.0, bot_y, 245.0, 38.0);
    let m = mouse_position();
    let m_vec = Vec2::new(m.0, m.1);
    let s_bot_hov = save_rect.contains(m_vec);
    draw_rectangle(
        save_rect.x,
        save_rect.y,
        save_rect.w,
        save_rect.h,
        if s_bot_hov { Color::from_rgba(165, 45, 25, 245) } else { Color::from_rgba(125, 35, 20, 230) },
    );
    draw_rectangle_lines(save_rect.x, save_rect.y, save_rect.w, save_rect.h, 2.0, Color::from_rgba(255, 220, 85, 255));
    draw_text("+ SAVE CURRENT PROGRESS", save_rect.x + 20.0, save_rect.y + 24.0, 14.0, Color::from_rgba(255, 240, 190, 255));

    // Back Button
    let back_rect = Rect::new(screen_w - 185.0, bot_y, 150.0, 38.0);
    draw_rectangle(back_rect.x, back_rect.y, back_rect.w, back_rect.h, Color::from_rgba(55, 42, 30, 230));
    draw_rectangle_lines(back_rect.x, back_rect.y, back_rect.w, back_rect.h, 1.5, Color::from_rgba(200, 160, 80, 200));
    draw_text("< BACK", back_rect.x + 45.0, back_rect.y + 24.0, 15.0, Color::from_rgba(245, 230, 190, 255));

    if is_mouse_button_pressed(MouseButton::Left) || is_mouse_button_released(MouseButton::Left) {
        if save_rect.contains(m_vec) {
            go_save = true;
        } else if imp_rect.contains(m_vec) {
            crate::storage::trigger_browser_file_import();
        } else if back_rect.contains(m_vec) {
            game.active_tab = ui.return_tab_after_menu;
        }
    }

    if go_save {
        ui.save_name_input = format!("{} Campaign - Turn {}", game.player_faction.name(), game.turn);
        game.active_tab = ActiveTab::SaveGameMenu;
    }
}

pub fn render_save_game_view(game: &mut GameState, ui: &mut UiState) {
    let screen_w = screen_width();
    let screen_h = screen_height();

    // Background Container
    draw_rectangle(20.0, 20.0, screen_w - 40.0, screen_h - 40.0, Color::from_rgba(20, 16, 12, 252));
    draw_rectangle_lines(20.0, 20.0, screen_w - 40.0, screen_h - 40.0, 2.5, Color::from_rgba(220, 180, 80, 255));

    // Title
    draw_text(
        "SAVE CAMPAIGN PROGRESS",
        45.0,
        60.0,
        24.0,
        Color::from_rgba(250, 225, 130, 255),
    );
    draw_text(
        "Enter a title for this campaign save or select an existing slot to overwrite.",
        45.0,
        82.0,
        14.0,
        Color::from_rgba(210, 195, 160, 255),
    );

    // Top Right Navigation Tabs
    let tab_w = 175.0;
    let tab_h = 32.0;
    let tab_y = 38.0;

    let load_tab_rect = Rect::new(screen_w - 40.0 - tab_w * 2.0 - 10.0, tab_y, tab_w, tab_h);
    let m = mouse_position();
    let m_vec = Vec2::new(m.0, m.1);
    let load_tab_hov = load_tab_rect.contains(m_vec);
    draw_rectangle(
        load_tab_rect.x,
        load_tab_rect.y,
        load_tab_rect.w,
        load_tab_rect.h,
        if load_tab_hov { Color::from_rgba(75, 52, 28, 240) } else { Color::from_rgba(40, 32, 24, 220) },
    );
    draw_rectangle_lines(load_tab_rect.x, load_tab_rect.y, load_tab_rect.w, load_tab_rect.h, 1.5, Color::from_rgba(180, 140, 70, 180));
    draw_text("LOAD ARCHIVES", load_tab_rect.x + 24.0, load_tab_rect.y + 21.0, 14.0, Color::from_rgba(220, 205, 175, 255));

    let save_tab_rect = Rect::new(screen_w - 40.0 - tab_w, tab_y, tab_w, tab_h);
    draw_rectangle(save_tab_rect.x, save_tab_rect.y, save_tab_rect.w, save_tab_rect.h, Color::from_rgba(75, 52, 28, 245));
    draw_rectangle_lines(save_tab_rect.x, save_tab_rect.y, save_tab_rect.w, save_tab_rect.h, 2.0, Color::from_rgba(255, 220, 85, 255));
    draw_text("+ SAVE PROGRESS", save_tab_rect.x + 18.0, save_tab_rect.y + 21.0, 14.0, Color::from_rgba(255, 240, 190, 255));

    if is_mouse_button_pressed(MouseButton::Left) && load_tab_rect.contains(m_vec) {
        ui.refresh_saves();
        game.active_tab = ActiveTab::LoadGameMenu;
        return;
    }

    // Save Name Input Field
    let input_y = 115.0;
    draw_text("Save Slot Title:", 45.0, input_y + 24.0, 16.0, Color::from_rgba(240, 225, 180, 255));

    let input_rect = Rect::new(180.0, input_y, 450.0, 38.0);
    draw_rectangle(input_rect.x, input_rect.y, input_rect.w, input_rect.h, Color::from_rgba(35, 28, 20, 255));
    draw_rectangle_lines(input_rect.x, input_rect.y, input_rect.w, input_rect.h, 2.0, Color::from_rgba(225, 180, 85, 255));

    // Handle typing into input field
    while let Some(c) = get_char_pressed() {
        if !c.is_control() && ui.save_name_input.len() < 35 {
            ui.save_name_input.push(c);
        }
    }
    if is_key_pressed(KeyCode::Backspace) {
        ui.save_name_input.pop();
    }

    let blink = ((get_time() * 2.0) as i32) % 2 == 0;
    let cursor = if blink { "_" } else { "" };
    draw_text(
        &format!("{}{}", ui.save_name_input, cursor),
        input_rect.x + 12.0,
        input_rect.y + 25.0,
        16.0,
        WHITE,
    );

    // Quick presets
    let p1_rect = Rect::new(645.0, input_y, 110.0, 38.0);
    draw_rectangle(p1_rect.x, p1_rect.y, p1_rect.w, p1_rect.h, Color::from_rgba(50, 40, 30, 220));
    draw_rectangle_lines(p1_rect.x, p1_rect.y, p1_rect.w, p1_rect.h, 1.2, Color::from_rgba(180, 140, 70, 180));
    draw_text("QuickSave", p1_rect.x + 18.0, p1_rect.y + 24.0, 13.5, Color::from_rgba(230, 215, 175, 255));

    let p2_rect = Rect::new(765.0, input_y, 140.0, 38.0);
    draw_rectangle(p2_rect.x, p2_rect.y, p2_rect.w, p2_rect.h, Color::from_rgba(50, 40, 30, 220));
    draw_rectangle_lines(p2_rect.x, p2_rect.y, p2_rect.w, p2_rect.h, 1.2, Color::from_rgba(180, 140, 70, 180));
    draw_text("AutoSave Slot", p2_rect.x + 18.0, p2_rect.y + 24.0, 13.5, Color::from_rgba(230, 215, 175, 255));

    if is_mouse_button_pressed(MouseButton::Left) {
        let m = mouse_position();
        let m_vec = Vec2::new(m.0, m.1);
        if p1_rect.contains(m_vec) {
            ui.save_name_input = format!("QuickSave - Turn {} ({})", game.turn, game.player_faction.name());
        } else if p2_rect.contains(m_vec) {
            ui.save_name_input = format!("AutoSave ({})", game.player_faction.name());
        }
    }

    // Existing saves list below for overwrite selection
    draw_text("Or select an existing slot to overwrite:", 45.0, 185.0, 15.0, Color::from_rgba(215, 195, 155, 255));

    let list_y = 200.0;
    let list_h = screen_h - list_y - 85.0;
    draw_rectangle(45.0, list_y, screen_w - 90.0, list_h, Color::from_rgba(26, 20, 15, 230));
    draw_rectangle_lines(45.0, list_y, screen_w - 90.0, list_h, 1.5, Color::from_rgba(150, 120, 60, 180));

    for (idx, slot) in ui.save_list_cache.iter().enumerate() {
        let sy = list_y + 10.0 + (idx as f32 * 50.0);
        if sy + 44.0 > list_y + list_h {
            break;
        }
        let s_rect = Rect::new(55.0, sy, screen_w - 110.0, 44.0);
        draw_rectangle(s_rect.x, s_rect.y, s_rect.w, s_rect.h, Color::from_rgba(36, 28, 20, 200));
        draw_rectangle_lines(s_rect.x, s_rect.y, s_rect.w, s_rect.h, 1.0, Color::from_rgba(120, 95, 50, 140));

        let (c1, c2) = slot.player_faction.flag_colors();
        draw_rectangle(s_rect.x + 8.0, s_rect.y + 6.0, 30.0, 32.0, c1);
        draw_rectangle_lines(s_rect.x + 8.0, s_rect.y + 6.0, 30.0, 32.0, 1.0, c2);

        draw_text(&slot.display_name, s_rect.x + 48.0, s_rect.y + 27.0, 15.0, Color::from_rgba(240, 225, 180, 255));
        draw_text(
            &format!("Turn {} | {} | ID: {}", slot.turn, slot.player_faction.name(), slot.id),
            s_rect.x + 350.0,
            s_rect.y + 27.0,
            13.0,
            Color::from_rgba(185, 170, 140, 255),
        );

        if is_mouse_button_pressed(MouseButton::Left) {
            let m = mouse_position();
            if s_rect.contains(Vec2::new(m.0, m.1)) {
                ui.save_name_input = slot.display_name.clone();
            }
        }
    }

    // Save & Cancel Actions
    let bot_y = screen_h - 60.0;
    let save_btn = Rect::new(screen_w / 2.0 - 180.0, bot_y, 160.0, 42.0);
    draw_rectangle(save_btn.x, save_btn.y, save_btn.w, save_btn.h, Color::from_rgba(145, 45, 25, 240));
    draw_rectangle_lines(save_btn.x, save_btn.y, save_btn.w, save_btn.h, 2.0, Color::from_rgba(255, 220, 85, 255));
    draw_text("SAVE GAME", save_btn.x + 35.0, save_btn.y + 26.0, 16.0, Color::from_rgba(255, 240, 190, 255));

    let cancel_btn = Rect::new(screen_w / 2.0 + 20.0, bot_y, 140.0, 42.0);
    draw_rectangle(cancel_btn.x, cancel_btn.y, cancel_btn.w, cancel_btn.h, Color::from_rgba(50, 40, 30, 220));
    draw_rectangle_lines(cancel_btn.x, cancel_btn.y, cancel_btn.w, cancel_btn.h, 1.5, Color::from_rgba(180, 140, 70, 180));
    draw_text("CANCEL", cancel_btn.x + 40.0, cancel_btn.y + 26.0, 15.0, Color::from_rgba(235, 215, 175, 255));

    let mut do_save = false;
    if is_mouse_button_pressed(MouseButton::Left) {
        let m = mouse_position();
        let m_vec = Vec2::new(m.0, m.1);
        if save_btn.contains(m_vec) {
            do_save = true;
        } else if cancel_btn.contains(m_vec) {
            game.active_tab = ui.return_tab_after_menu;
        }
    }
    if is_key_pressed(KeyCode::Enter) {
        do_save = true;
    }

    if do_save {
        let name = if ui.save_name_input.trim().is_empty() {
            format!("Campaign Save Turn {}", game.turn)
        } else {
            ui.save_name_input.trim().to_string()
        };

        // If name matches an existing slot's display_name, overwrite that slot id!
        let slot_id = ui
            .save_list_cache
            .iter()
            .find(|s| s.display_name.to_lowercase() == name.to_lowercase())
            .map(|s| s.id.clone())
            .unwrap_or_else(|| {
                if name.to_lowercase().contains("quick") {
                    "quicksave".to_string()
                } else if name.to_lowercase().contains("auto") {
                    "autosave".to_string()
                } else {
                    format!("save_{}", get_time() as u64)
                }
            });

        match crate::storage::save_campaign_state(&slot_id, &name, game) {
            Ok(m) => {
                ui.refresh_saves();
                ui.selected_save_slot = Some(m.id.clone());
                ui.set_toast(&format!("Campaign Saved: {}", m.display_name));
                game.active_tab = ActiveTab::LoadGameMenu;
            }
            Err(e) => {
                ui.set_toast(&format!("Save error: {}", e));
            }
        }
    }
}

pub fn render_pause_menu_modal(game: &mut GameState, ui: &mut UiState) {
    let screen_w = screen_width();
    let screen_h = screen_height();

    // Dim Background Overlay
    draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::from_rgba(0, 0, 0, 190));

    // Modal Window
    let modal_w = 420.0;
    let modal_h = 390.0;
    let modal_x = (screen_w - modal_w) / 2.0;
    let modal_y = (screen_h - modal_h) / 2.0;

    draw_rectangle(modal_x, modal_y, modal_w, modal_h, Color::from_rgba(24, 18, 14, 252));
    draw_rectangle_lines(modal_x, modal_y, modal_w, modal_h, 2.5, Color::from_rgba(215, 175, 75, 255));

    // Title
    draw_text("CAMPAIGN PAUSED", modal_x + 105.0, modal_y + 40.0, 22.0, Color::from_rgba(250, 225, 130, 255));
    let season_str = match game.season {
        crate::models::Season::Summer => "Summer",
        crate::models::Season::Winter => "Winter",
    };
    let sub = format!("{} | Turn {} ({} {})", game.player_faction.name(), game.turn, season_str, game.year);
    draw_text(&sub, modal_x + 50.0, modal_y + 65.0, 13.5, Color::from_rgba(210, 190, 150, 255));

    let btn_w = 320.0;
    let btn_h = 42.0;
    let btn_x = modal_x + 50.0;

    let b_resume = Rect::new(btn_x, modal_y + 90.0, btn_w, btn_h);
    let b_qsave = Rect::new(btn_x, modal_y + 140.0, btn_w, btn_h);
    let b_save = Rect::new(btn_x, modal_y + 190.0, btn_w, btn_h);
    let b_load = Rect::new(btn_x, modal_y + 240.0, btn_w, btn_h);
    let b_restart = Rect::new(btn_x, modal_y + 290.0, btn_w, btn_h);
    let b_exit = Rect::new(btn_x, modal_y + 340.0, btn_w, btn_h);

    let render_btn = |rect: Rect, label: &str, is_highlight: bool| {
        let (bg, border, tc) = if is_highlight {
            (Color::from_rgba(150, 45, 25, 240), Color::from_rgba(255, 220, 85, 255), Color::from_rgba(255, 240, 190, 255))
        } else {
            (Color::from_rgba(42, 34, 26, 230), Color::from_rgba(180, 140, 65, 180), Color::from_rgba(235, 220, 185, 255))
        };
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, bg);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, if is_highlight { 2.0 } else { 1.2 }, border);
        draw_text(label, rect.x + 30.0, rect.y + 26.0, 15.0, tc);
    };

    render_btn(b_resume, "RESUME CAMPAIGN", true);
    render_btn(b_qsave, "QUICK SAVE (F5)", false);
    render_btn(b_save, "SAVE CAMPAIGN...", false);
    render_btn(b_load, "LOAD CAMPAIGN / ARCHIVE...", false);
    render_btn(b_restart, "RESTART CAMPAIGN", false);
    render_btn(b_exit, "EXIT TO MAIN MENU", false);

    if is_mouse_button_pressed(MouseButton::Left) {
        let m = mouse_position();
        let m_vec = Vec2::new(m.0, m.1);
        if b_resume.contains(m_vec) {
            ui.show_pause_menu = false;
        } else if b_qsave.contains(m_vec) {
            match crate::storage::quick_save_campaign(game) {
                Ok(_) => {
                    ui.refresh_saves();
                    ui.set_toast("QuickSave Completed (F5)");
                    ui.show_pause_menu = false;
                }
                Err(e) => ui.set_toast(&format!("Save error: {}", e)),
            }
        } else if b_save.contains(m_vec) {
            ui.show_pause_menu = false;
            ui.save_name_input = format!("{} Turn {}", game.player_faction.name(), game.turn);
            ui.refresh_saves();
            game.active_tab = ActiveTab::SaveGameMenu;
        } else if b_load.contains(m_vec) {
            ui.show_pause_menu = false;
            ui.refresh_saves();
            ui.return_tab_after_menu = ActiveTab::Map;
            game.active_tab = ActiveTab::LoadGameMenu;
        } else if b_restart.contains(m_vec) {
            let faction = game.player_faction;
            *game = GameState::new();
            game.start_as_faction(faction);
            ui.show_pause_menu = false;
            ui.set_toast("Campaign Restarted");
        } else if b_exit.contains(m_vec) {
            ui.show_pause_menu = false;
            game.active_tab = ActiveTab::MainMenu;
        }
    }
}

pub fn render_delete_confirm_modal(ui: &mut UiState) {
    let slot_id = match &ui.delete_confirm_slot {
        Some(s) => s.clone(),
        None => return,
    };

    let screen_w = screen_width();
    let screen_h = screen_height();

    // Dark backdrop
    draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::from_rgba(0, 0, 0, 210));

    let dlg_w = 420.0;
    let dlg_h = 210.0;
    let dlg_x = (screen_w - dlg_w) / 2.0;
    let dlg_y = (screen_h - dlg_h) / 2.0;

    draw_rectangle(dlg_x, dlg_y, dlg_w, dlg_h, Color::from_rgba(28, 18, 14, 255));
    draw_rectangle_lines(dlg_x, dlg_y, dlg_w, dlg_h, 2.5, Color::from_rgba(225, 75, 75, 255));

    draw_text("DELETE CAMPAIGN SAVE?", dlg_x + 95.0, dlg_y + 40.0, 20.0, Color::from_rgba(255, 180, 180, 255));
    draw_text(
        &format!("Permanently delete save slot '{}'?", slot_id),
        dlg_x + 35.0,
        dlg_y + 75.0,
        14.5,
        Color::from_rgba(240, 220, 200, 255),
    );
    draw_text(
        "This action cannot be undone from your browser storage.",
        dlg_x + 35.0,
        dlg_y + 100.0,
        13.0,
        Color::from_rgba(190, 160, 140, 255),
    );

    let del_btn = Rect::new(dlg_x + 35.0, dlg_y + 140.0, 160.0, 42.0);
    draw_rectangle(del_btn.x, del_btn.y, del_btn.w, del_btn.h, Color::from_rgba(150, 30, 30, 240));
    draw_rectangle_lines(del_btn.x, del_btn.y, del_btn.w, del_btn.h, 1.8, Color::from_rgba(255, 120, 120, 255));
    draw_text("CONFIRM DELETE", del_btn.x + 18.0, del_btn.y + 26.0, 14.0, WHITE);

    let can_btn = Rect::new(dlg_x + 225.0, dlg_y + 140.0, 160.0, 42.0);
    draw_rectangle(can_btn.x, can_btn.y, can_btn.w, can_btn.h, Color::from_rgba(45, 38, 30, 220));
    draw_rectangle_lines(can_btn.x, can_btn.y, can_btn.w, can_btn.h, 1.5, Color::from_rgba(180, 150, 75, 180));
    draw_text("CANCEL", can_btn.x + 52.0, can_btn.y + 26.0, 14.5, Color::from_rgba(240, 225, 180, 255));

    if is_mouse_button_pressed(MouseButton::Left) {
        let m = mouse_position();
        let m_vec = Vec2::new(m.0, m.1);
        if del_btn.contains(m_vec) {
            let _ = crate::storage::delete_campaign_save(&slot_id);
            ui.refresh_saves();
            ui.delete_confirm_slot = None;
            ui.set_toast("Save deleted from browser storage");
        } else if can_btn.contains(m_vec) {
            ui.delete_confirm_slot = None;
        }
    }
}

pub fn render_how_to_play_view(game: &mut GameState, ui: &mut UiState) {
    let screen_w = screen_width();
    let screen_h = screen_height();

    draw_rectangle(20.0, 20.0, screen_w - 40.0, screen_h - 40.0, Color::from_rgba(20, 16, 12, 252));
    draw_rectangle_lines(20.0, 20.0, screen_w - 40.0, screen_h - 40.0, 2.5, Color::from_rgba(215, 175, 75, 255));

    draw_text("MARATHA: WAR OF INDEPENDENCE -- CAMPAIGN MANUAL & GUIDE", 45.0, 60.0, 24.0, Color::from_rgba(250, 225, 130, 255));
    draw_text("Master the Grand Strategy of the Indian Subcontinent (1700 A.D.)", 45.0, 82.0, 14.0, Color::from_rgba(210, 190, 155, 255));

    let col_w = (screen_w - 110.0) / 2.0;
    let col1_x = 45.0;
    let col2_x = 45.0 + col_w + 20.0;
    let text_y = 120.0;

    // Column 1: Controls & Campaign Map
    draw_text("1. VICTORY OBJECTIVE", col1_x, text_y, 17.0, Color::from_rgba(240, 210, 100, 255));
    draw_text("Unify the subcontinent! Seize Delhi (Hindustan) and control 10 of the 18", col1_x, text_y + 22.0, 13.5, Color::from_rgba(220, 205, 170, 255));
    draw_text("historical Indian provinces to establish Hindavi Swarajya supremacy.", col1_x, text_y + 40.0, 13.5, Color::from_rgba(220, 205, 170, 255));

    draw_text("2. CONTROLS & NAVIGATION", col1_x, text_y + 75.0, 17.0, Color::from_rgba(240, 210, 100, 255));
    draw_text("| Pan Map: Right-click drag, Middle-click drag, or Arrow / WASD keys.", col1_x, text_y + 97.0, 13.5, Color::from_rgba(220, 205, 170, 255));
    draw_text("| Zoom: Mouse wheel, +/- keys, E/Q, or on-screen [+] [-] [RST] buttons.", col1_x, text_y + 117.0, 13.5, Color::from_rgba(220, 205, 170, 255));
    draw_text("| Select Province: Left-click on any province on the campaign map.", col1_x, text_y + 137.0, 13.5, Color::from_rgba(220, 205, 170, 255));
    draw_text("| March / Attack: Right-click on an adjacent province to issue attack order.", col1_x, text_y + 157.0, 13.5, Color::from_rgba(220, 205, 170, 255));

    draw_text("3. PROVINCE ECONOMY & RECRUITMENT", col1_x, text_y + 195.0, 17.0, Color::from_rgba(240, 210, 100, 255));
    draw_text("| Tax Level: Adjust between Exempt, Low, Normal, High, and Extortionate.", col1_x, text_y + 217.0, 13.5, Color::from_rgba(220, 205, 170, 255));
    draw_text("| Construction: Build Hill Forts (+Order), Bazaars (+Income), and Ports.", col1_x, text_y + 237.0, 13.5, Color::from_rgba(220, 205, 170, 255));
    draw_text("| Recruitment: Enlist Levies, Sepoys, Matchlocks, Cavalry, and Bronze Cannons.", col1_x, text_y + 257.0, 13.5, Color::from_rgba(220, 205, 170, 255));

    // Column 2: Tactical Auto-Resolve, Technology & Storage
    draw_text("4. 3-PHASE TACTICAL BATTLE RESOLUTION", col2_x, text_y, 17.0, Color::from_rgba(240, 210, 100, 255));
    draw_text("Battles simulate Empire: Total War's real-time engagement in 3 phases:", col2_x, text_y + 22.0, 13.5, Color::from_rgba(220, 205, 170, 255));
    draw_text("Phase I: Artillery Duel & Skirmishers harass enemy battle lines.", col2_x, text_y + 42.0, 13.5, Color::from_rgba(220, 205, 170, 255));
    draw_text("Phase II: Ranked Musket Volleys trade devastating gunfire.", col2_x, text_y + 62.0, 13.5, Color::from_rgba(220, 205, 170, 255));
    draw_text("Phase III: Silhadar Lancers & Bargir Cavalry charges clash in melee.", col2_x, text_y + 82.0, 13.5, Color::from_rgba(220, 205, 170, 255));

    draw_text("5. TECHNOLOGY & GOVERNMENT", col2_x, text_y + 117.0, 17.0, Color::from_rgba(240, 210, 100, 255));
    draw_text("| Research Ring Bayonets, Canister Shot, and Swarajya Charters.", col2_x, text_y + 139.0, 13.5, Color::from_rgba(220, 205, 170, 255));
    draw_text("| Your royal cabinet ministers boost income, public order, and morale.", col2_x, text_y + 159.0, 13.5, Color::from_rgba(220, 205, 170, 255));

    draw_text("6. SAVING & RELOADING LOCALLY", col2_x, text_y + 195.0, 17.0, Color::from_rgba(240, 210, 100, 255));
    draw_text("| AutoSave: The game automatically saves every turn upon End Turn.", col2_x, text_y + 217.0, 13.5, Color::from_rgba(220, 205, 170, 255));
    draw_text("| QuickSave / QuickLoad: Press F5 to QuickSave, F9 to QuickLoad.", col2_x, text_y + 237.0, 13.5, Color::from_rgba(220, 205, 170, 255));
    draw_text("| Export / Import: Download your campaign as a JSON file or restore anytime.", col2_x, text_y + 257.0, 13.5, Color::from_rgba(220, 205, 170, 255));

    // Back Button
    let back_btn = Rect::new((screen_w - 200.0) / 2.0, screen_h - 75.0, 200.0, 42.0);
    draw_rectangle(back_btn.x, back_btn.y, back_btn.w, back_btn.h, Color::from_rgba(150, 45, 25, 240));
    draw_rectangle_lines(back_btn.x, back_btn.y, back_btn.w, back_btn.h, 2.0, Color::from_rgba(255, 220, 85, 255));
    draw_text("< RETURN TO MENU", back_btn.x + 28.0, back_btn.y + 26.0, 15.0, Color::from_rgba(255, 240, 190, 255));

    if is_mouse_button_pressed(MouseButton::Left) {
        let m = mouse_position();
        if back_btn.contains(Vec2::new(m.0, m.1)) {
            game.active_tab = ui.return_tab_after_menu;
        }
    }
}

pub fn render_diplomacy_view(game: &mut GameState, ui: &mut UiState) {
    let screen_w = screen_width();
    let screen_h = screen_height();

    // Dark Regal Background
    draw_rectangle(20.0, 56.0, screen_w - 40.0, screen_h - 70.0, Color::from_rgba(20, 16, 12, 252));
    draw_rectangle_lines(20.0, 56.0, screen_w - 40.0, screen_h - 70.0, 2.0, Color::from_rgba(215, 175, 75, 255));

    // Header Title
    draw_text_bold(
        "Imperial Diplomatic Ledger | Foreign Treaties & Statecraft",
        40.0,
        90.0,
        22.0,
        Color::from_rgba(245, 220, 140, 255),
    );
    draw_text(
        "Negotiate bilateral trade agreements, alliances, peace treaties, or demand regional tribute across India.",
        40.0,
        112.0,
        14.0,
        Color::from_rgba(210, 195, 160, 255),
    );

    let content_y = 126.0;
    let left_w = 340.0;
    let right_x = 40.0 + left_w + 20.0;
    let right_w = screen_w - right_x - 40.0;

    // Left Pane: Foreign Faction List
    draw_text("Known Sovereign Courts:", 40.0, content_y + 18.0, 16.0, Color::from_rgba(235, 215, 160, 255));

    let factions_to_show = [
        FactionId::Mughal,
        FactionId::Mysore,
        FactionId::Portugal,
        FactionId::Independent,
        FactionId::Sikh,
        FactionId::Hyderabad,
        FactionId::Bengal,
    ];

    let mut card_y = content_y + 24.0;
    for &f_id in &factions_to_show {
        let fac = match game.factions.get(&f_id) {
            Some(f) => f,
            None => continue,
        };

        let is_sel = game.selected_diplo_faction == Some(f_id);
        let f_card = Rect::new(40.0, card_y, left_w, 60.0);
        let bg_color = if is_sel {
            Color::from_rgba(140, 95, 30, 240)
        } else {
            Color::from_rgba(35, 28, 22, 220)
        };
        draw_rectangle(f_card.x, f_card.y, f_card.w, f_card.h, bg_color);
        draw_rectangle_lines(f_card.x, f_card.y, f_card.w, f_card.h, 1.5, if is_sel { Color::from_rgba(245, 210, 80, 255) } else { Color::from_rgba(140, 110, 60, 180) });

        let (c1, c2) = f_id.flag_colors();
        draw_rectangle(f_card.x + 8.0, f_card.y + 8.0, 24.0, 24.0, c1);
        draw_rectangle_lines(f_card.x + 8.0, f_card.y + 8.0, 24.0, 24.0, 1.5, c2);

        draw_text_bold(&fac.name, f_card.x + 38.0, f_card.y + 19.0, 13.5, WHITE);
        draw_text(&format!("Ruler: {}", fac.leader_name), f_card.x + 38.0, f_card.y + 35.0, 11.5, Color::from_rgba(215, 200, 170, 255));

        let p_fac = game.factions.get(&game.player_faction).unwrap();
        let rel = *p_fac.relations.get(&f_id).unwrap_or(&0);
        let rel_str = if rel >= 25 {
            ("Friendly / Allied", Color::from_rgba(80, 220, 100, 255))
        } else if rel >= 0 {
            ("Neutral / Cordial", Color::from_rgba(220, 210, 120, 255))
        } else if rel >= -30 {
            ("Unfriendly / Tense", Color::from_rgba(230, 140, 60, 255))
        } else {
            ("Hostile / At War", Color::from_rgba(240, 60, 60, 255))
        };
        draw_text(&format!("Attitude: {} ({})", rel_str.0, rel), f_card.x + 38.0, f_card.y + 51.0, 11.0, rel_str.1);

        if is_mouse_button_pressed(MouseButton::Left) {
            let m = mouse_position();
            if f_card.contains(Vec2::new(m.0, m.1)) {
                game.selected_diplo_faction = Some(f_id);
            }
        }

        card_y += 66.0;
    }

    // Right Pane: Selected Faction Dossier & Diplomatic Actions
    if let Some(target_id) = game.selected_diplo_faction {
        let target_fac = game.factions.get(&target_id).unwrap().clone();
        let p_fac = game.factions.get(&game.player_faction).unwrap().clone();

        draw_rectangle(right_x, content_y + 10.0, right_w, screen_h - content_y - 95.0, Color::from_rgba(28, 22, 18, 240));
        draw_rectangle_lines(right_x, content_y + 10.0, right_w, screen_h - content_y - 95.0, 1.5, Color::from_rgba(180, 140, 70, 200));

        // Header info
        let (tc1, tc2) = target_id.flag_colors();
        draw_rectangle(right_x + 18.0, content_y + 20.0, 38.0, 38.0, tc1);
        draw_rectangle_lines(right_x + 18.0, content_y + 20.0, 38.0, 38.0, 2.0, tc2);

        draw_text_bold(&target_fac.name, right_x + 68.0, content_y + 36.0, 20.0, Color::from_rgba(245, 225, 150, 255));
        draw_text(&format!("Sovereign Leader: {}", target_fac.leader_name), right_x + 68.0, content_y + 54.0, 14.0, Color::from_rgba(210, 195, 160, 255));

        // Treaties status line
        let is_at_war = p_fac.wars.contains(&target_id);
        let is_trading = p_fac.trade_partners.contains(&target_id);
        let is_allied = p_fac.alliances.contains(&target_id);
        let is_vassal = p_fac.vassals.contains(&target_id);
        let is_overlord = target_fac.vassals.contains(&game.player_faction);
        let has_mil_access = p_fac.military_access_with.contains(&target_id);

        let status_text = if is_at_war {
            "CURRENT STATUS: AT OPEN WAR"
        } else if is_vassal {
            "CURRENT STATUS: SWORN PROTECTORATE VASSAL"
        } else if is_overlord {
            "CURRENT STATUS: OVERLORD SOVEREIGN"
        } else if is_allied {
            "CURRENT STATUS: MILITARY ALLIANCE"
        } else {
            "CURRENT STATUS: PEACEFUL COEXISTENCE"
        };
        let status_color = if is_at_war { Color::from_rgba(240, 60, 60, 255) } else if is_allied || is_vassal { Color::from_rgba(80, 220, 100, 255) } else { Color::from_rgba(220, 200, 120, 255) };
        draw_text_bold(status_text, right_x + 20.0, content_y + 84.0, 14.5, status_color);

        let trade_str = if is_trading { "Active (+250g/turn)" } else { "None" };
        let transit_str = if has_mil_access { "Permitted (Military Access Active)" } else { "Closed Border" };
        let fealty_str = if is_vassal { "Sworn Vassal (50% Tribute)" } else if is_overlord { "Imperial Overlord" } else { "Independent Realm" };

        draw_text(&format!("Bilateral Trade: {} | Border Transit: {}", trade_str, transit_str), right_x + 20.0, content_y + 106.0, 12.5, Color::from_rgba(200, 190, 170, 255));
        let rel = *p_fac.relations.get(&target_id).unwrap_or(&0);
        draw_text(&format!("Protectorate Status: {} | Court Trust: {} / 100", fealty_str, rel), right_x + 20.0, content_y + 126.0, 12.5, Color::from_rgba(220, 205, 175, 255));

        // Actions Grid
        draw_text("Available Diplomatic Proposals:", right_x + 20.0, content_y + 155.0, 15.0, Color::from_rgba(240, 215, 130, 255));

        let actions = [
            (
                DiploAction::TradeAgreement,
                "Establish Trade Agreement",
                "Opens overland and sea trade (+250g/turn to both realms)",
                !is_trading && !is_at_war,
            ),
            (
                DiploAction::GrantMilitaryAccess,
                "Grant Military Access",
                "Permits armies to traverse borders peacefully without war",
                !has_mil_access && !is_at_war,
            ),
            (
                DiploAction::SubjugateVassal,
                "Demand Protectorate Vassalage",
                "Subjugates realm as sworn vassal (pays 50% surplus profit)",
                !is_vassal && target_fac.overlord.is_none() && !is_at_war,
            ),
            (
                DiploAction::SmallGift,
                "Dispatch Small Gift (400g)",
                "Sends fine textiles and spices (+15 Diplomatic Trust)",
                p_fac.treasury >= 400,
            ),
            (
                DiploAction::LavishGift,
                "Dispatch Royal Tribute (1000g)",
                "Sends war elephants, jewels, and gold (+40 Diplomatic Trust)",
                p_fac.treasury >= 1000,
            ),
            (
                DiploAction::Alliance,
                "Form Military Alliance",
                "Bilateral defensive & offensive military pact (requires +20 trust)",
                !is_allied && !is_at_war && rel >= 20,
            ),
            (
                DiploAction::Ceasefire,
                "Ratify Peace & Ceasefire (600g)",
                "Cease all hostilities and restore mutual borders",
                is_at_war && p_fac.treasury >= 600,
            ),
            (
                DiploAction::DemandTribute,
                "Demand Regional Tribute (300g)",
                "Coercive ultimatum demanding tribute (-30 Relations)",
                !is_at_war,
            ),
            (
                DiploAction::ShareTechnology,
                "Transfer Military Technology",
                "Shares technical innovations for 600g honorarium (+30 Trust)",
                !is_at_war,
            ),
            (
                DiploAction::DeclareWar,
                "Declare War / Mobilize",
                "Sever all treaties and initiate open warfare (-80 Relations)",
                !is_at_war,
            ),
        ];

        let mut btn_y = content_y + 172.0;
        let btn_w = right_w - 40.0;
        let btn_h = 28.0;

        for (action, title, desc, enabled) in actions {
            let b_rect = Rect::new(right_x + 20.0, btn_y, btn_w, btn_h);
            let m = mouse_position();
            let hov = b_rect.contains(Vec2::new(m.0, m.1));

            let bg = if !enabled {
                Color::from_rgba(35, 30, 25, 180)
            } else if hov {
                Color::from_rgba(150, 100, 30, 240)
            } else {
                Color::from_rgba(65, 48, 32, 230)
            };

            draw_rectangle(b_rect.x, b_rect.y, b_rect.w, b_rect.h, bg);
            draw_rectangle_lines(b_rect.x, b_rect.y, b_rect.w, b_rect.h, 1.2, if enabled { Color::from_rgba(210, 175, 90, 200) } else { Color::from_rgba(90, 80, 70, 140) });

            let title_color = if enabled { Color::from_rgba(245, 230, 160, 255) } else { Color::from_rgba(130, 120, 105, 255) };
            draw_text_bold(title, b_rect.x + 12.0, b_rect.y + 18.0, 12.5, title_color);
            draw_text(desc, b_rect.x + 280.0, b_rect.y + 18.0, 11.0, Color::from_rgba(195, 185, 165, 255));

            if enabled && is_mouse_button_pressed(MouseButton::Left) && hov {
                match game.conduct_diplomacy(target_id, action) {
                    Ok(msg) => ui.set_toast(&msg),
                    Err(err) => ui.set_toast(&err),
                }
            }

            btn_y += 33.0;
        }
    }
}

pub fn render_ledger_view(game: &mut GameState, ui: &mut UiState) {
    let screen_w = screen_width();
    let screen_h = screen_height();

    // Dark Regal Canvas
    draw_rectangle(20.0, 56.0, screen_w - 40.0, screen_h - 70.0, Color::from_rgba(20, 16, 12, 252));
    draw_rectangle_lines(20.0, 56.0, screen_w - 40.0, screen_h - 70.0, 2.0, Color::from_rgba(215, 175, 75, 255));

    draw_text_bold(
        "Subcontinent Strategic Grand Ledger | Provincial Audit",
        40.0,
        90.0,
        22.0,
        Color::from_rgba(245, 220, 140, 255),
    );
    draw_text(
        "Master registry of all 18 Indian provinces. Click any row to center the strategic campaign map on that capital.",
        40.0,
        112.0,
        14.0,
        Color::from_rgba(210, 195, 160, 255),
    );

    // Sovereign Prestige Leaderboard (Military, Economic, Enlightenment)
    let pres_x = screen_w - 550.0;
    draw_text_bold("SOVEREIGN PRESTIGE MATRIX:", pres_x, 82.0, 12.5, Color::from_rgba(255, 215, 100, 255));
    let mut p_x = pres_x;
    for (f_id, short_name) in [
        (FactionId::Maratha, "Maratha"),
        (FactionId::Mughal, "Mughal"),
        (FactionId::Mysore, "Mysore"),
        (FactionId::Portugal, "Portugal"),
    ] {
        if let Some(f) = game.factions.get(&f_id) {
            let total = f.prestige.0 + f.prestige.1 + f.prestige.2;
            let (c1, _) = f_id.flag_colors();
            draw_rectangle(p_x, 95.0, 8.0, 8.0, c1);
            draw_text(
                &format!("{}: {} (M:{} E:{} T:{})", short_name, total, f.prestige.0, f.prestige.1, f.prestige.2),
                p_x + 12.0,
                103.0,
                11.0,
                Color::from_rgba(220, 210, 185, 255),
            );
            p_x += 134.0;
        }
    }

    let table_x = 40.0;
    let table_y = 125.0;
    let table_w = screen_w - 80.0;
    let header_h = 28.0;

    // Header Row
    draw_rectangle(table_x, table_y, table_w, header_h, Color::from_rgba(45, 36, 26, 255));
    draw_rectangle_lines(table_x, table_y, table_w, header_h, 1.0, Color::from_rgba(180, 145, 75, 200));

    let cols = [
        ("Province", 15.0),
        ("Seat of Power", 155.0),
        ("Controlled By", 305.0),
        ("Commodity", 445.0),
        ("Pop", 575.0),
        ("Wealth", 645.0),
        ("Tax (U/L)", 725.0),
        ("Order", 815.0),
        ("Garrison", 885.0),
    ];

    for (name, offset) in cols {
        draw_text_bold(name, table_x + offset, table_y + 19.0, 13.0, Color::from_rgba(240, 215, 140, 255));
    }

    let mut row_y = table_y + header_h + 2.0;
    let row_h = 24.0;

    for (p_idx, p) in game.provinces.iter().enumerate() {
        let is_selected = game.selected_province == Some(p_idx);
        let m = mouse_position();
        let r_rect = Rect::new(table_x, row_y, table_w, row_h);
        let hov = r_rect.contains(Vec2::new(m.0, m.1));

        let bg = if is_selected {
            Color::from_rgba(120, 80, 25, 220)
        } else if hov {
            Color::from_rgba(55, 44, 32, 200)
        } else if p_idx % 2 == 0 {
            Color::from_rgba(26, 20, 16, 200)
        } else {
            Color::from_rgba(32, 25, 20, 200)
        };

        draw_rectangle(r_rect.x, r_rect.y, r_rect.w, r_rect.h, bg);
        if is_selected || hov {
            draw_rectangle_lines(r_rect.x, r_rect.y, r_rect.w, r_rect.h, 1.0, Color::from_rgba(215, 175, 80, 180));
        }

        // Draw col data
        draw_text_bold(&p.name, table_x + 15.0, row_y + 16.0, 12.0, WHITE);
        draw_text(&p.capital_name, table_x + 155.0, row_y + 16.0, 11.5, Color::from_rgba(215, 200, 175, 255));

        let (c1, _) = p.owner.flag_colors();
        draw_rectangle(table_x + 305.0, row_y + 4.0, 12.0, 12.0, c1);
        draw_text(p.owner.name(), table_x + 322.0, row_y + 16.0, 11.5, Color::from_rgba(230, 215, 180, 255));

        let comm_str = match p.commodity {
            Some(crate::models::CommodityType::Spices) => "Spices",
            Some(crate::models::CommodityType::Cotton) => "Cotton",
            Some(crate::models::CommodityType::Silk) => "Silk",
            Some(crate::models::CommodityType::Saltpetre) => "Saltpetre",
            None => "Agrarian",
        };
        draw_text(comm_str, table_x + 445.0, row_y + 16.0, 11.5, Color::from_rgba(240, 210, 115, 255));

        draw_text(&format!("{}k", p.population / 1000), table_x + 575.0, row_y + 16.0, 11.5, Color::from_rgba(210, 195, 165, 255));
        draw_text(&format!("{}g", p.base_wealth), table_x + 645.0, row_y + 16.0, 11.5, Color::from_rgba(240, 210, 110, 255));

        let tax_code = |t: TaxLevel| match t {
            TaxLevel::Exempt => "Ex",
            TaxLevel::Low => "Lo",
            TaxLevel::Normal => "Norm",
            TaxLevel::High => "Hi",
            TaxLevel::Extortionate => "Ext",
        };
        let tax_str = format!("{}/{}", tax_code(p.upper_tax), tax_code(p.lower_tax));
        draw_text(&tax_str, table_x + 725.0, row_y + 16.0, 11.5, Color::from_rgba(200, 190, 170, 255));

        let order_color = if p.public_order >= 0 { Color::from_rgba(80, 220, 120, 255) } else { Color::from_rgba(240, 70, 70, 255) };
        draw_text(&format!("{}{}", if p.public_order >= 0 { "+" } else { "" }, p.public_order), table_x + 815.0, row_y + 16.0, 12.0, order_color);

        let garrison_men: u32 = p.garrison.iter().map(|u| u.soldiers).sum();
        draw_text(&format!("{} men ({} rgts)", garrison_men, p.garrison.len()), table_x + 885.0, row_y + 16.0, 11.5, Color::from_rgba(220, 205, 170, 255));

        if is_mouse_button_pressed(MouseButton::Left) && hov {
            game.selected_province = Some(p_idx);
            ui.camera_pos = p.capital_pos;
            game.active_tab = ActiveTab::Map;
        }

        row_y += row_h + 2.0;
    }
}
