mod ai;
mod combat;
mod game_state;
mod geo_data;
mod models;
mod storage;
mod ui;

use game_state::{ActiveTab, GameState};
use geo_data::point_in_polygon;
use macroquad::prelude::*;
use ui::{draw_text, UiState};

fn window_conf() -> Conf {
    Conf {
        window_title: "Maratha: War of Independence (1700-1750)".to_string(),
        window_width: 1280,
        window_height: 800,
        high_dpi: true,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    ui::init_fonts();
    let mut game = GameState::new();
    let mut ui = UiState::new();

    // Center camera on Bijapur/Maharashtra initially
    let screen_w = screen_width();
    let screen_h = screen_height();
    ui.camera_pos = Vec2::new(screen_w * 0.42 - 380.0 * ui.camera_zoom, screen_h * 0.45 - 600.0 * ui.camera_zoom);

    loop {
        clear_background(Color::from_rgba(18, 24, 30, 255)); // Deep ocean ink

        // Check for pending imported saves from browser file input
        if let Some(imported_json) = crate::storage::check_pending_imported_save() {
            match crate::storage::import_save_from_json(&imported_json, &mut game) {
                Ok(meta) => {
                    ui.refresh_saves();
                    ui.set_toast(&format!("Imported: {}", meta.display_name));
                    game.active_tab = ActiveTab::Map;
                }
                Err(e) => {
                    ui.set_toast(&format!("Import Error: {}", e));
                }
            }
        }

        handle_input(&mut game, &mut ui);

        if game.active_tab == ActiveTab::MainMenu {
            ui::render_main_menu(&mut game, &mut ui);
        } else if game.active_tab == ActiveTab::LoadGameMenu {
            ui::render_load_game_view(&mut game, &mut ui);
            ui::render_delete_confirm_modal(&mut ui);
        } else if game.active_tab == ActiveTab::SaveGameMenu {
            ui::render_save_game_view(&mut game, &mut ui);
        } else if game.active_tab == ActiveTab::HowToPlay {
            ui::render_how_to_play_view(&mut game, &mut ui);
        } else if game.active_tab == ActiveTab::FactionSelect {
            ui::render_faction_select_view(&mut game, &mut ui);
        } else {
            // Render Campaign Map
            render_ocean_details(&ui);
            render_provinces(&game, &ui);
            render_province_labels(&game, &ui);
            render_armies(&game, &ui);

            // Render Active Overlay View
            match game.active_tab {
                ActiveTab::Map => {
                    ui::render_bottom_panel(&mut game, &mut ui);
                    ui::render_recruitment_modal(&mut game, &mut ui);
                    ui::render_construction_modal(&mut game, &mut ui);
                    render_zoom_controls(&mut ui, &game);
                }
                ActiveTab::Government => {
                    ui::render_government_view(&mut game, &mut ui);
                }
                ActiveTab::Technology => {
                    ui::render_technology_view(&mut game);
                }
                ActiveTab::Diplomacy => {
                    ui::render_diplomacy_view(&mut game, &mut ui);
                }
                ActiveTab::Ledger => {
                    ui::render_ledger_view(&mut game, &mut ui);
                }
                ActiveTab::BattleReport => {
                    ui::render_battle_report_modal(&mut game);
                }
                _ => {}
            }

            // Top HUD Banner is visible across campaign views
            ui::render_top_bar(&mut game, &mut ui);

            // Notifications Popups (Dismissible, only shown on Map view)
            if game.active_tab == ActiveTab::Map {
                render_notifications(&mut game);
            }

            // In-Game Pause Menu Modal Overlay
            if ui.show_pause_menu {
                ui::render_pause_menu_modal(&mut game, &mut ui);
            }
        }

        next_frame().await;
    }
}

fn handle_input(game: &mut GameState, ui: &mut UiState) {
    let m_pos = mouse_position();
    let mouse_screen = Vec2::new(m_pos.0, m_pos.1);
    let mouse_world = (mouse_screen - ui.camera_pos) / ui.camera_zoom;

    // Handle ESC across all menu views
    if is_key_pressed(KeyCode::Escape) {
        if ui.delete_confirm_slot.is_some() {
            ui.delete_confirm_slot = None;
            return;
        }
        if ui.show_pause_menu {
            ui.show_pause_menu = false;
            return;
        }
        if ui.show_recruit_modal || ui.show_build_modal {
            ui.show_recruit_modal = false;
            ui.show_build_modal = false;
            return;
        }
        if let Some(notif) = game.notifications.last_mut() {
            if notif.is_urgent {
                notif.is_urgent = false;
                return;
            }
        }
        // Deselect Army before opening pause menu
        if game.selected_army.is_some() {
            game.selected_army = None;
            return;
        }
        // Deselect Province before opening pause menu
        if game.selected_province.is_some() {
            game.selected_province = None;
            return;
        }
        match game.active_tab {
            ActiveTab::LoadGameMenu | ActiveTab::SaveGameMenu | ActiveTab::HowToPlay => {
                game.active_tab = ui.return_tab_after_menu;
                return;
            }
            ActiveTab::FactionSelect => {
                game.active_tab = ActiveTab::MainMenu;
                return;
            }
            ActiveTab::Government | ActiveTab::Technology => {
                game.active_tab = ActiveTab::Map;
                return;
            }
            ActiveTab::Map => {
                ui.show_pause_menu = true;
                return;
            }
            _ => {}
        }
    }

    // Skip map interactions when in front-end menus or when pause menu is open
    if game.active_tab == ActiveTab::MainMenu
        || game.active_tab == ActiveTab::LoadGameMenu
        || game.active_tab == ActiveTab::SaveGameMenu
        || game.active_tab == ActiveTab::HowToPlay
        || game.active_tab == ActiveTab::FactionSelect
        || ui.show_pause_menu
    {
        return;
    }

    // QuickSave (F5)
    if is_key_pressed(KeyCode::F5) {
        match crate::storage::quick_save_campaign(game) {
            Ok(_) => {
                ui.refresh_saves();
                ui.set_toast("QuickSave Completed (F5)");
            }
            Err(e) => ui.set_toast(&format!("Save error: {}", e)),
        }
    }

    // QuickLoad (F9)
    if is_key_pressed(KeyCode::F9) {
        if let Some(s) = ui.save_list_cache.first() {
            let id = s.id.clone();
            match crate::storage::load_campaign_state(&id, game) {
                Ok(m) => {
                    ui.set_toast(&format!("QuickLoaded: {}", m.display_name));
                    game.active_tab = ActiveTab::Map;
                }
                Err(e) => ui.set_toast(&format!("Load error: {}", e)),
            }
        } else {
            ui.set_toast("No save found for QuickLoad");
        }
    }

    let in_modal = ui.show_recruit_modal || ui.show_build_modal;
    let in_ui_header = mouse_screen.y <= 54.0;
    let in_ui_bottom = mouse_screen.y >= screen_height() - 225.0 && game.active_tab == ActiveTab::Map;

    // Mouse Wheel Zooming
    let wheel = mouse_wheel().1;
    if wheel.abs() > 0.0001 && !in_ui_header && !in_ui_bottom && !in_modal {
        let zoom_mult = if wheel > 0.0 { 1.15 } else { 0.87 };
        let new_zoom = (ui.camera_zoom * zoom_mult).clamp(0.35, 4.0);
        ui.camera_pos = mouse_screen - (mouse_screen - ui.camera_pos) * (new_zoom / ui.camera_zoom);
        ui.camera_zoom = new_zoom;
    }

    // Keyboard Zooming (+ / - / E / Q)
    let screen_center = Vec2::new(screen_width() * 0.5, screen_height() * 0.5);
    if is_key_pressed(KeyCode::Equal) || is_key_pressed(KeyCode::KpAdd) || is_key_pressed(KeyCode::E) {
        let new_zoom = (ui.camera_zoom * 1.25).clamp(0.35, 4.0);
        ui.camera_pos = screen_center - (screen_center - ui.camera_pos) * (new_zoom / ui.camera_zoom);
        ui.camera_zoom = new_zoom;
    }
    if is_key_pressed(KeyCode::Minus) || is_key_pressed(KeyCode::KpSubtract) || is_key_pressed(KeyCode::Q) {
        let new_zoom = (ui.camera_zoom * 0.8).clamp(0.35, 4.0);
        ui.camera_pos = screen_center - (screen_center - ui.camera_pos) * (new_zoom / ui.camera_zoom);
        ui.camera_zoom = new_zoom;
    }

    // Reset Camera with 'R'
    if is_key_pressed(KeyCode::R) {
        if let Some(prov_idx) = game.selected_province {
            let target = game.provinces[prov_idx].center;
            ui.camera_zoom = 0.95;
            ui.camera_pos = screen_center - target * ui.camera_zoom;
        }
    }

    // Panning with right click drag or middle drag (Threshold > 8.0px prevents runaway panning / mouse capture)
    if is_mouse_button_pressed(MouseButton::Right) || is_mouse_button_pressed(MouseButton::Middle) {
        ui.drag_start_mouse = mouse_screen;
        ui.drag_start_cam = ui.camera_pos;
        ui.is_dragging = false;
    }

    if is_mouse_button_down(MouseButton::Right) || is_mouse_button_down(MouseButton::Middle) {
        let drag_distance = (mouse_screen - ui.drag_start_mouse).length();
        if drag_distance > 8.0 {
            ui.is_dragging = true;
            ui.camera_pos = ui.drag_start_cam + (mouse_screen - ui.drag_start_mouse);
        }
    }

    if is_mouse_button_released(MouseButton::Right) {
        let drag_distance = (mouse_screen - ui.drag_start_mouse).length();
        // If release happened without dragging (> 8px), it is a dedicated right-click action
        if !ui.is_dragging && drag_distance <= 8.0 && !in_ui_header && !in_ui_bottom && !in_modal && game.active_tab == ActiveTab::Map {
            if let Some(target_idx) = ui.hovered_province {
                if let Some(army_idx) = game.selected_army {
                    if game.armies[army_idx].owner == game.player_faction {
                        if game.provinces[game.armies[army_idx].province_id].neighbors.contains(&target_idx) {
                            game.move_army(army_idx, target_idx);
                        } else if target_idx == game.armies[army_idx].province_id {
                            // Right click on self province deselects army
                            game.selected_army = None;
                        }
                    }
                }
            } else {
                // Right click on open water / empty space deselects army
                game.selected_army = None;
            }
        }
        ui.is_dragging = false;
    }

    if is_mouse_button_released(MouseButton::Middle) {
        ui.is_dragging = false;
    }

    // Keyboard Pan
    let pan_speed = 10.0;
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        ui.camera_pos.x += pan_speed;
    }
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        ui.camera_pos.x -= pan_speed;
    }
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        ui.camera_pos.y += pan_speed;
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        ui.camera_pos.y -= pan_speed;
    }

    // Determine hovered province
    ui.hovered_province = None;
    if !in_ui_header && !in_ui_bottom && !in_modal && game.active_tab == ActiveTab::Map {
        for (idx, p) in game.provinces.iter().enumerate() {
            if point_in_polygon(mouse_world, &p.polygon) {
                ui.hovered_province = Some(idx);
                break;
            }
        }
    }

    // Province Selection & Left-Click Movement
    if is_mouse_button_pressed(MouseButton::Left) && !in_ui_header && !in_ui_bottom && !in_modal && game.active_tab == ActiveTab::Map {
        if let Some(hovered) = ui.hovered_province {
            if let Some(army_idx) = game.selected_army {
                let army = &game.armies[army_idx];
                if army.owner == game.player_faction {
                    if hovered == army.province_id {
                        // User clicked the army's own province again -> toggle/deselect army!
                        game.selected_army = None;
                        return;
                    } else if game.provinces[army.province_id].neighbors.contains(&hovered) {
                        // User clicked an adjacent neighbor -> march/attack order!
                        game.move_army(army_idx, hovered);
                        ui.show_recruit_modal = false;
                        ui.show_build_modal = false;
                        return;
                    }
                }
            }

            // Normal province selection
            if game.selected_province == Some(hovered) && game.selected_army.is_none() {
                // If clicked already-selected province again with no army, deselect province
                game.selected_province = None;
            } else {
                game.selected_province = Some(hovered);
                // Select army in this province if player owns one here; otherwise clear army selection
                let army_here = game
                    .armies
                    .iter()
                    .position(|a| a.province_id == hovered && a.owner == game.player_faction);
                game.selected_army = army_here;
            }
            ui.show_recruit_modal = false;
            ui.show_build_modal = false;
        } else {
            // Clicked empty ocean/water -> deselect all!
            game.selected_army = None;
            game.selected_province = None;
            ui.show_recruit_modal = false;
            ui.show_build_modal = false;
        }
    }

    // Spacebar to end turn
    if is_key_pressed(KeyCode::Space) {
        game.end_turn();
        ui.set_toast("Turn Complete | Campaign AutoSaved");
    }
}

fn render_ocean_details(ui: &UiState) {
    let to_screen = |pos: Vec2| -> Vec2 { pos * ui.camera_zoom + ui.camera_pos };

    // Ocean styling & Water Textures
    let arabian_sea_pos = to_screen(Vec2::new(140.0, 620.0));
    draw_text_ex(
        "ARABIAN SEA (SINDHU SAGAR)",
        arabian_sea_pos.x,
        arabian_sea_pos.y,
        TextParams {
            font_size: (18.0 * ui.camera_zoom).max(10.0) as u16,
            color: Color::from_rgba(65, 85, 110, 140),
            ..Default::default()
        },
    );

    let bengal_bay_pos = to_screen(Vec2::new(710.0, 710.0));
    draw_text_ex(
        "BAY OF BENGAL",
        bengal_bay_pos.x,
        bengal_bay_pos.y,
        TextParams {
            font_size: (18.0 * ui.camera_zoom).max(10.0) as u16,
            color: Color::from_rgba(65, 85, 110, 140),
            ..Default::default()
        },
    );

    let indian_ocean_pos = to_screen(Vec2::new(320.0, 1040.0));
    draw_text_ex(
        "INDIAN OCEAN",
        indian_ocean_pos.x,
        indian_ocean_pos.y,
        TextParams {
            font_size: (20.0 * ui.camera_zoom).max(11.0) as u16,
            color: Color::from_rgba(65, 85, 110, 140),
            ..Default::default()
        },
    );
}

fn render_provinces(game: &GameState, ui: &UiState) {
    let to_screen = |pos: Vec2| -> Vec2 { pos * ui.camera_zoom + ui.camera_pos };

    for (idx, prov) in game.provinces.iter().enumerate() {
        let is_selected = game.selected_province == Some(idx);
        let is_hovered = ui.hovered_province == Some(idx);

        let (base_c, border_accent) = prov.owner.flag_colors();

        // Polygon Fill using Centroid Fan Triangulation
        let center_screen = to_screen(prov.center);
        let poly_screen: Vec<Vec2> = prov.polygon.iter().map(|&p| to_screen(p)).collect();

        let fill_color = if is_selected {
            Color::from_rgba(
                (base_c.r * 255.0) as u8,
                (base_c.g * 255.0) as u8,
                (base_c.b * 255.0) as u8,
                240,
            )
        } else if is_hovered {
            Color::from_rgba(
                (base_c.r * 255.0 * 1.15).min(255.0) as u8,
                (base_c.g * 255.0 * 1.15).min(255.0) as u8,
                (base_c.b * 255.0 * 1.15).min(255.0) as u8,
                210,
            )
        } else {
            Color::from_rgba(
                (base_c.r * 255.0 * 0.85) as u8,
                (base_c.g * 255.0 * 0.85) as u8,
                (base_c.b * 255.0 * 0.85) as u8,
                180,
            )
        };

        let n = poly_screen.len();
        for i in 0..n {
            let next_i = (i + 1) % n;
            draw_triangle(center_screen, poly_screen[i], poly_screen[next_i], fill_color);
        }

        // Polygon Borders
        let border_thick = if is_selected { 3.5 } else if is_hovered { 2.5 } else { 1.5 };
        let border_color = if is_selected {
            Color::from_rgba(255, 220, 80, 255) // Golden glow
        } else if is_hovered {
            border_accent
        } else {
            Color::from_rgba(30, 24, 18, 220) // Antique ink outline
        };

        for i in 0..n {
            let next_i = (i + 1) % n;
            draw_line(
                poly_screen[i].x,
                poly_screen[i].y,
                poly_screen[next_i].x,
                poly_screen[next_i].y,
                border_thick,
                border_color,
            );
        }
    }
}

fn render_province_labels(game: &GameState, ui: &UiState) {
    let to_screen = |pos: Vec2| -> Vec2 { pos * ui.camera_zoom + ui.camera_pos };

    for (idx, prov) in game.provinces.iter().enumerate() {
        let is_selected = game.selected_province == Some(idx);
        let cap_pos = to_screen(prov.capital_pos);

        // Capital Icon: Drawn Fortress Crest
        let icon_sz = (16.0 * ui.camera_zoom).max(10.0);
        let rect = Rect::new(cap_pos.x - icon_sz / 2.0, cap_pos.y - icon_sz / 2.0, icon_sz, icon_sz);
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::from_rgba(25, 20, 15, 230));
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.5, Color::from_rgba(240, 210, 100, 255));
        // Miniature crenellation / battlements
        draw_rectangle(rect.x + 2.0, rect.y + 2.0, rect.w - 4.0, rect.h * 0.4, Color::from_rgba(200, 160, 60, 255));

        // Capital Name & Province Name Plate
        let label_text = format!("{} ({})", prov.name, prov.capital_name);
        let f_size = (16.0 * ui.camera_zoom).clamp(13.0, 24.0);
        let label_w = measure_text(&label_text, None, f_size as u16, 1.0).width + 12.0;
        let plate_rect = Rect::new(cap_pos.x - label_w / 2.0, cap_pos.y + icon_sz * 0.6, label_w, f_size + 6.0);

        let bg_color = if is_selected {
            Color::from_rgba(150, 40, 20, 230)
        } else {
            Color::from_rgba(15, 12, 10, 210)
        };
        draw_rectangle(plate_rect.x, plate_rect.y, plate_rect.w, plate_rect.h, bg_color);
        draw_rectangle_lines(plate_rect.x, plate_rect.y, plate_rect.w, plate_rect.h, 1.0, Color::from_rgba(200, 170, 80, 180));
        draw_text(
            &label_text,
            plate_rect.x + 6.0,
            plate_rect.y + f_size,
            f_size,
            Color::from_rgba(245, 235, 210, 255),
        );

        // Port Icon
        if let Some(port_pos) = prov.port_pos {
            let p_screen = to_screen(port_pos);
            draw_circle(p_screen.x, p_screen.y, 6.0 * ui.camera_zoom, Color::from_rgba(20, 40, 80, 220));
            draw_circle_lines(p_screen.x, p_screen.y, 6.0 * ui.camera_zoom, 1.5, Color::from_rgba(220, 190, 90, 255));
            draw_circle(p_screen.x, p_screen.y, 2.5 * ui.camera_zoom, Color::from_rgba(240, 220, 130, 255));
        }
    }
}

fn render_armies(game: &GameState, ui: &UiState) {
    let to_screen = |pos: Vec2| -> Vec2 { pos * ui.camera_zoom + ui.camera_pos };

    // If an army is selected and belongs to player, draw movement arrows to adjacent neighbors!
    if let Some(army_idx) = game.selected_army {
        if let Some(army) = game.armies.get(army_idx) {
            if army.owner == game.player_faction && !army.has_moved {
                let from_pos = to_screen(game.provinces[army.province_id].center);
                for &neighbor_idx in &game.provinces[army.province_id].neighbors {
                    let to_pos = to_screen(game.provinces[neighbor_idx].center);
                    let is_enemy = game.provinces[neighbor_idx].owner != game.player_faction;
                    let arrow_color = if is_enemy {
                        Color::from_rgba(240, 60, 40, 220) // Attack red
                    } else {
                        Color::from_rgba(80, 220, 120, 220) // Move green
                    };

                    draw_line(from_pos.x, from_pos.y, to_pos.x, to_pos.y, 3.0, arrow_color);
                    // Draw target circular indicator
                    draw_circle_lines(to_pos.x, to_pos.y, 14.0 * ui.camera_zoom, 2.0, arrow_color);
                }
            }
        }
    }

    // Render Army Tokens
    for (a_idx, army) in game.armies.iter().enumerate() {
        let is_selected = game.selected_army == Some(a_idx);
        let prov = &game.provinces[army.province_id];
        let token_pos = to_screen(prov.center + Vec2::new(10.0, 15.0));

        let token_radius = (18.0 * ui.camera_zoom).max(12.0);
        let (f_color, f_accent) = army.owner.flag_colors();

        // Pulsing selection ring
        if is_selected {
            draw_circle_lines(token_pos.x, token_pos.y, token_radius + 6.0, 2.5, Color::from_rgba(255, 230, 80, 255));
        }

        // Army Token Disc
        draw_circle(token_pos.x, token_pos.y, token_radius, f_color);
        draw_circle_lines(token_pos.x, token_pos.y, token_radius, 2.0, f_accent);

        // General Stars & Soldier count banner
        let count_str = format!("{}", army.total_soldiers());
        let count_sz = (10.0 * ui.camera_zoom).clamp(8.0, 13.0);
        draw_text(
            &count_str,
            token_pos.x - token_radius * 0.7,
            token_pos.y + token_radius * 0.35,
            count_sz,
            WHITE,
        );

        // General Stars indicator above token
        let stars = "*".repeat(army.general_stars as usize);
        draw_text(
            &stars,
            token_pos.x - token_radius * 0.8,
            token_pos.y - token_radius * 0.9,
            14.0,
            Color::from_rgba(255, 220, 60, 255),
        );
    }
}

fn render_notifications(game: &mut GameState) {
    let mut dismiss = false;
    if let Some(notif) = game.notifications.last() {
        // Show urgent notifications as a top announcement ribbon
        if notif.is_urgent && game.turn == notif.turn {
            let banner_w = (screen_width() - 40.0).min(780.0);
            let banner_h = 92.0;
            let x = (screen_width() - banner_w) / 2.0;
            let y = 60.0;

            draw_rectangle(x, y, banner_w, banner_h, Color::from_rgba(20, 15, 10, 252));
            draw_rectangle_lines(x, y, banner_w, banner_h, 2.0, Color::from_rgba(225, 185, 75, 255));

            draw_text(&notif.title, x + 18.0, y + 27.0, 18.5, Color::from_rgba(255, 220, 120, 255));
            draw_text(&notif.message, x + 18.0, y + 51.0, 13.5, Color::from_rgba(235, 225, 205, 255));
            draw_text("Click anywhere or press [Esc] to dismiss", x + 18.0, y + 74.0, 12.0, Color::from_rgba(180, 165, 130, 200));

            // [Dismiss X] button
            let close_w = 110.0;
            let close_h = 28.0;
            let close_x = x + banner_w - close_w - 12.0;
            let close_y = y + 10.0;
            let m = mouse_position();
            let m_vec = Vec2::new(m.0, m.1);
            let banner_rect = Rect::new(x, y, banner_w, banner_h);
            let btn_rect = Rect::new(close_x, close_y, close_w, close_h);
            let btn_hov = btn_rect.contains(m_vec);

            draw_rectangle(
                close_x,
                close_y,
                close_w,
                close_h,
                if btn_hov { Color::from_rgba(180, 45, 30, 255) } else { Color::from_rgba(140, 35, 25, 240) },
            );
            draw_rectangle_lines(close_x, close_y, close_w, close_h, 1.5, Color::from_rgba(255, 210, 90, 255));
            draw_text("Dismiss [X]", close_x + 14.0, close_y + 19.0, 14.0, WHITE);

            if is_mouse_button_pressed(MouseButton::Left) && banner_rect.contains(m_vec) {
                dismiss = true;
            }
            if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                dismiss = true;
            }
        }
    }
    if dismiss {
        if let Some(n) = game.notifications.last_mut() {
            n.is_urgent = false;
        }
    }
}

fn render_zoom_controls(ui: &mut UiState, game: &GameState) {
    let screen_w = screen_width();
    let x = screen_w - 52.0;
    let y = 72.0;

    // [+] Zoom In button
    let in_rect = Rect::new(x, y, 42.0, 36.0);
    draw_rectangle(in_rect.x, in_rect.y, in_rect.w, in_rect.h, Color::from_rgba(25, 20, 15, 235));
    draw_rectangle_lines(in_rect.x, in_rect.y, in_rect.w, in_rect.h, 1.5, Color::from_rgba(220, 180, 80, 240));
    draw_text("+", in_rect.x + 13.0, in_rect.y + 26.0, 26.0, Color::from_rgba(255, 225, 140, 255));

    // [-] Zoom Out button
    let out_rect = Rect::new(x, y + 42.0, 42.0, 36.0);
    draw_rectangle(out_rect.x, out_rect.y, out_rect.w, out_rect.h, Color::from_rgba(25, 20, 15, 235));
    draw_rectangle_lines(out_rect.x, out_rect.y, out_rect.w, out_rect.h, 1.5, Color::from_rgba(220, 180, 80, 240));
    draw_text("-", out_rect.x + 15.0, out_rect.y + 24.0, 26.0, Color::from_rgba(255, 225, 140, 255));

    // [RST] Reset View button
    let rst_rect = Rect::new(x, y + 84.0, 42.0, 32.0);
    draw_rectangle(rst_rect.x, rst_rect.y, rst_rect.w, rst_rect.h, Color::from_rgba(25, 20, 15, 235));
    draw_rectangle_lines(rst_rect.x, rst_rect.y, rst_rect.w, rst_rect.h, 1.5, Color::from_rgba(220, 180, 80, 240));
    draw_text("RST", rst_rect.x + 6.0, rst_rect.y + 21.0, 13.5, Color::from_rgba(255, 225, 140, 255));

    if is_mouse_button_pressed(MouseButton::Left) {
        let m = mouse_position();
        let m_vec = Vec2::new(m.0, m.1);
        let screen_center = Vec2::new(screen_width() * 0.5, screen_height() * 0.5);

        if in_rect.contains(m_vec) {
            let new_zoom = (ui.camera_zoom * 1.3).clamp(0.35, 4.0);
            ui.camera_pos = screen_center - (screen_center - ui.camera_pos) * (new_zoom / ui.camera_zoom);
            ui.camera_zoom = new_zoom;
        } else if out_rect.contains(m_vec) {
            let new_zoom = (ui.camera_zoom * 0.77).clamp(0.35, 4.0);
            ui.camera_pos = screen_center - (screen_center - ui.camera_pos) * (new_zoom / ui.camera_zoom);
            ui.camera_zoom = new_zoom;
        } else if rst_rect.contains(m_vec) {
            if let Some(prov_idx) = game.selected_province {
                let target = game.provinces[prov_idx].center;
                ui.camera_zoom = 0.95;
                ui.camera_pos = screen_center - target * ui.camera_zoom;
            }
        }
    }
}
