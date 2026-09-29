#![windows_subsystem = "windows"]

use eframe::egui;
use rand::Rng;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const XOR_KEY: u32 = 129;
const XOR_KEY_U8: u8 = 129;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Language {
    Ru,
    En,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum XorMode {
    Char,
    Byte,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Vec4 {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ItemData {
    #[serde(rename = "spawnId")]
    spawn_id: String,
    id: i32,
    pos: Vec3,
    rot: Vec4,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

struct SaveEditorApp {
    lang: Language,
    file_path: Option<PathBuf>,
    status: String,

    game_data: Option<Value>,
    content_data: Option<Value>,

    xor_mode: XorMode,

    money_input: String,
    playtime_input: String,
    room_name_input: String,

    player_x_input: String,
    player_y_input: String,
    player_z_input: String,

    selected_available_item: usize,
    available_items: Vec<(&'static str, &'static str)>,

    items_list: Vec<ItemData>,
    selected_item_index: Option<usize>,
}

impl SaveEditorApp {
    fn new() -> Self {
        let lang = Self::detect_system_language();
        let available_items = vec![
            ("Pillow", "Pillow"),
            ("Cube", "Cube"),
            ("RTX4080Ti", "RTX4080Ti"),
            ("Projector", "Projector"),
        ];

        let mut app = Self {
            lang,
            file_path: None,
            status: Self::get_text(lang, "ready").to_string(),
            game_data: None,
            content_data: None,
            xor_mode: XorMode::Char,
            money_input: String::new(),
            playtime_input: String::new(),
            room_name_input: String::new(),
            player_x_input: "0".to_string(),
            player_y_input: "0".to_string(),
            player_z_input: "0".to_string(),
            selected_available_item: 0,
            available_items,
            items_list: Vec::new(),
            selected_item_index: None,
        };

        app.apply_language();
        app
    }

    fn detect_system_language() -> Language {
        for arg in std::env::args() {
            let arg_lower = arg.to_lowercase();
            if arg_lower == "--lang=en" || arg_lower == "--en" {
                return Language::En;
            } else if arg_lower == "--lang=ru" || arg_lower == "--ru" {
                return Language::Ru;
            }
        }

        if let Some(locale) = sys_locale::get_locale() {
            if locale.to_lowercase().starts_with("ru") {
                return Language::Ru;
            }
        }

        Language::En
    }

    fn get_text(lang: Language, key: &str) -> &'static str {
        match lang {
            Language::Ru => match key {
                "title" => "PC Simulator Save Editor",
                "open_file" => "Открыть файл",
                "file_not_selected" => "Файл не выбран",
                "game_data" => "GameData",
                "money" => "Деньги",
                "playtime" => "Playtime",
                "room_name" => "Имя сохранения",
                "player_position" => "Позиция игрока (для спавна объектов)",
                "x" => "X",
                "y" => "Y",
                "z" => "Z",
                "add_objects" => "Добавить объекты",
                "select_object" => "Выберите объект",
                "add_object" => "Добавить объект",
                "remove_selected" => "Удалить выбранный",
                "save" => "Сохранить",
                "save_as" => "Сохранить как...",
                "ready" => "Готов к работе",
                "file_loaded" => "Файл загружен успешно",
                "file_load_error" => "Ошибка загрузки файла",
                "object_added" => "Добавлен объект",
                "object_removed" => "Удален объект",
                "save_success" => "Файл сохранён успешно",
                "save_error" => "Ошибка сохранения",
                "open_warning" => "Сначала откройте файл",
                "invalid_format" => "Неверный формат файла",
                "unsupported_format" => "Неподдерживаемый формат файла. Расширение должно содержать \"pc\".",
                _ => "",
            },
            Language::En => match key {
                "title" => "PC Simulator Save Editor",
                "open_file" => "Open file",
                "file_not_selected" => "File not selected",
                "game_data" => "GameData",
                "money" => "Money",
                "playtime" => "Playtime",
                "room_name" => "Save Name",
                "player_position" => "Player Position (for object spawn)",
                "x" => "X",
                "y" => "Y",
                "z" => "Z",
                "add_objects" => "Add Objects",
                "select_object" => "Select object",
                "add_object" => "Add object",
                "remove_selected" => "Remove selected",
                "save" => "Save",
                "save_as" => "Save as...",
                "ready" => "Ready to work",
                "file_loaded" => "File loaded successfully",
                "file_load_error" => "Load error",
                "object_added" => "Object added",
                "object_removed" => "Object removed",
                "save_success" => "File saved successfully",
                "save_error" => "Save error",
                "open_warning" => "Open a file first",
                "invalid_format" => "Invalid file format",
                "unsupported_format" => "Unsupported file format. Extension must contain \"pc\".",
                _ => "",
            },
        }
    }

    fn apply_language(&mut self) {
        self.status = Self::get_text(self.lang, "ready").to_string();
    }

    fn xor_transform_str(text: &str) -> String {
        text.chars()
            .map(|c| {
                let code = (c as u32) ^ XOR_KEY;
                std::char::from_u32(code).unwrap_or(c)
            })
            .collect()
    }

    fn xor_transform_bytes(data: &[u8]) -> Vec<u8> {
        data.iter().map(|b| b ^ XOR_KEY_U8).collect()
    }

    fn open_file(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("All files", &["*"])
            .pick_file()
        {
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();

            if !ext.contains("pc") {
                self.status = Self::get_text(self.lang, "unsupported_format").to_string();
                return;
            }

            self.file_path = Some(path.clone());
            self.load_file();
        }
    }

    fn parse_save_content(text: &str) -> Option<(Value, Value)> {
        let trimmed = text.trim_start_matches(|c: char| {
            c.is_whitespace() || c == '\u{feff}' || c == '\0'
        });

        let start = trimmed.find('{')?;
        let text = &trimmed[start..];

        let mut parts = text.splitn(2, '\n');
        let game_str = parts.next()?;

        let game_json: Value = serde_json::from_str(game_str.trim()).ok()?;
        if !game_json.is_object() {
            return None;
        }

        let has_key = ["coin", "playtime", "roomName"]
            .iter()
            .any(|k| game_json.get(k).is_some());
        if !has_key {
            return None;
        }

        let content_json = if let Some(rest) = parts.next() {
            if rest.trim().is_empty() {
                json!({"itemData": []})
            } else {
                serde_json::from_str(rest.trim()).unwrap_or(json!({"itemData": []}))
            }
        } else {
            json!({"itemData": []})
        };

        Some((game_json, content_json))
    }

    fn load_file(&mut self) {
        let Some(path) = &self.file_path else { return };

        let Ok(mut raw) = fs::read(path) else {
            self.status = Self::get_text(self.lang, "file_load_error").to_string();
            return;
        };

        if raw.starts_with(&[0xEF, 0xBB, 0xBF]) {
            raw.drain(..3);
        }

        let mut variants: Vec<(String, XorMode)> = Vec::new();

        if let Ok(text) = std::str::from_utf8(&raw) {
            variants.push((Self::xor_transform_str(text), XorMode::Char));
        }

        {
            let decoded = Self::xor_transform_bytes(&raw);
            if let Ok(text) = String::from_utf8(decoded) {
                variants.push((text, XorMode::Byte));
            }
        }

        if let Ok(text) = std::str::from_utf8(&raw) {
            variants.push((text.to_string(), XorMode::None));
        }

        let mut found: Option<(Value, Value, XorMode)> = None;
        for (text, mode) in variants {
            if let Some((game, content)) = Self::parse_save_content(&text) {
                found = Some((game, content, mode));
                break;
            }
        }

        let Some((game_json, content_json, mode)) = found else {
            self.status = Self::get_text(self.lang, "invalid_format").to_string();
            return;
        };

        self.xor_mode = mode;

        self.money_input = game_json["coin"].as_i64().unwrap_or(0).to_string();
        self.playtime_input = game_json["playtime"].as_f64().unwrap_or(0.0).to_string();
        self.room_name_input = game_json["roomName"].as_str().unwrap_or("").to_string();

        if let Some(player_data) = content_json.get("playerData") {
            self.player_x_input = player_data["x"].as_f64().unwrap_or(0.0).to_string();
            self.player_y_input = player_data["y"].as_f64().unwrap_or(0.0).to_string();
            self.player_z_input = player_data["z"].as_f64().unwrap_or(0.0).to_string();
        } else {
            self.player_x_input = "0".to_string();
            self.player_y_input = "0".to_string();
            self.player_z_input = "0".to_string();
        }

        self.items_list.clear();
        if let Some(items) = content_json.get("itemData").and_then(|v| v.as_array()) {
            for item_val in items {
                if let Ok(item) = serde_json::from_value::<ItemData>(item_val.clone()) {
                    self.items_list.push(item);
                }
            }
        }

        self.game_data = Some(game_json);
        self.content_data = Some(content_json);
        self.selected_item_index = None;
        self.status = Self::get_text(self.lang, "file_loaded").to_string();
    }

    fn add_item(&mut self) {
        if self.content_data.is_none() {
            self.status = Self::get_text(self.lang, "open_warning").to_string();
            return;
        }

        let spawn_id = self.available_items[self.selected_available_item].1.to_string();
        let item_id: i32 = rand::thread_rng().gen();

        let px = self.player_x_input.parse::<f64>().unwrap_or(0.0);
        let py = self.player_y_input.parse::<f64>().unwrap_or(0.0);
        let pz = self.player_z_input.parse::<f64>().unwrap_or(0.0);

        let new_item = ItemData {
            spawn_id: spawn_id.clone(),
            id: item_id,
            pos: Vec3 { x: px, y: py, z: pz },
            rot: Vec4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            extra: HashMap::new(),
        };

        self.items_list.push(new_item);
        self.status = format!(
            "{}: {} (ID: {})",
            Self::get_text(self.lang, "object_added"),
            spawn_id,
            item_id
        );
    }

    fn remove_item(&mut self) {
        if let Some(idx) = self.selected_item_index {
            if idx < self.items_list.len() {
                let removed = self.items_list.remove(idx);
                self.selected_item_index = None;
                self.status = format!(
                    "{}: {}",
                    Self::get_text(self.lang, "object_removed"),
                    removed.spawn_id
                );
            }
        }
    }

    fn save_file(&mut self) {
        if let Some(path) = self.file_path.clone() {
            self.save_to_file(path);
        } else {
            self.status = Self::get_text(self.lang, "open_warning").to_string();
        }
    }

    fn save_as_file(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("All files", &["*"])
            .set_file_name("save.pc")
            .save_file()
        {
            self.save_to_file(path);
        }
    }

    fn save_to_file(&mut self, path: PathBuf) {
        let (Some(ref mut game_json), Some(ref mut content_json)) =
            (&mut self.game_data, &mut self.content_data)
        else {
            self.status = Self::get_text(self.lang, "open_warning").to_string();
            return;
        };

        let coin = self.money_input.parse::<i64>().unwrap_or(0);
        let playtime = self.playtime_input.parse::<f64>().unwrap_or(0.0);
        game_json["coin"] = json!(coin);
        game_json["playtime"] = json!(playtime);
        game_json["roomName"] = json!(self.room_name_input);

        let items_json = serde_json::to_value(&self.items_list).unwrap_or(json!([]));
        content_json["itemData"] = items_json;

        let game_str = serde_json::to_string(&game_json).unwrap_or_default();
        let content_str = serde_json::to_string(&content_json).unwrap_or_default();
        let combined = format!("{}\n{}", game_str, content_str);

        let write_result = match self.xor_mode {
            XorMode::None => fs::write(&path, combined.as_bytes()),
            XorMode::Char => fs::write(&path, Self::xor_transform_str(&combined).as_bytes()),
            XorMode::Byte => {
                let encrypted = Self::xor_transform_bytes(combined.as_bytes());
                fs::write(&path, encrypted)
            }
        };

        if write_result.is_ok() {
            self.file_path = Some(path.clone());
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("file");
            self.status = format!(
                "{}: {}",
                Self::get_text(self.lang, "save_success"),
                file_name
            );
        } else {
            self.status = Self::get_text(self.lang, "save_error").to_string();
        }
    }
}

impl eframe::App for SaveEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("status_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.status);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(Self::get_text(self.lang, "title"));
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                if ui.button(Self::get_text(self.lang, "open_file")).clicked() {
                    self.open_file();
                }
                let file_name = self
                    .file_path
                    .as_ref()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .unwrap_or(Self::get_text(self.lang, "file_not_selected"));
                ui.label(file_name);
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.label(egui::RichText::new(Self::get_text(self.lang, "game_data")).strong());
                egui::Grid::new("game_data_grid")
                    .num_columns(2)
                    .spacing([10.0, 5.0])
                    .show(ui, |ui| {
                        ui.label(format!("{}:", Self::get_text(self.lang, "money")));
                        ui.text_edit_singleline(&mut self.money_input);
                        ui.end_row();

                        ui.label(format!("{}:", Self::get_text(self.lang, "playtime")));
                        ui.text_edit_singleline(&mut self.playtime_input);
                        ui.end_row();

                        ui.label(format!("{}:", Self::get_text(self.lang, "room_name")));
                        ui.text_edit_singleline(&mut self.room_name_input);
                        ui.end_row();
                    });
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.label(egui::RichText::new(Self::get_text(self.lang, "player_position")).strong());
                ui.horizontal(|ui| {
                    ui.label("X:");
                    ui.add(egui::TextEdit::singleline(&mut self.player_x_input).desired_width(50.0));
                    ui.label("Y:");
                    ui.add(egui::TextEdit::singleline(&mut self.player_y_input).desired_width(50.0));
                    ui.label("Z:");
                    ui.add(egui::TextEdit::singleline(&mut self.player_z_input).desired_width(50.0));
                });
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.label(egui::RichText::new(Self::get_text(self.lang, "add_objects")).strong());
                ui.horizontal(|ui| {
                    ui.label(format!("{}:", Self::get_text(self.lang, "select_object")));

                    let current_selected_name = self.available_items[self.selected_available_item].0;
                    egui::ComboBox::from_id_source("object_combo")
                        .selected_text(current_selected_name)
                        .show_ui(ui, |ui| {
                            for (idx, (name, _)) in self.available_items.iter().enumerate() {
                                ui.selectable_value(&mut self.selected_available_item, idx, *name);
                            }
                        });

                    if ui.button(Self::get_text(self.lang, "add_object")).clicked() {
                        self.add_item();
                    }
                });

                ui.add_space(5.0);

                egui::ScrollArea::vertical()
                    .max_height(120.0)
                    .show(ui, |ui| {
                        for (idx, item) in self.items_list.iter().enumerate() {
                            let label = format!(
                                "{} (ID: {}) [{:.1}, {:.1}, {:.1}]",
                                item.spawn_id, item.id, item.pos.x, item.pos.y, item.pos.z
                            );
                            let is_selected = self.selected_item_index == Some(idx);
                            if ui.selectable_label(is_selected, label).clicked() {
                                self.selected_item_index = Some(idx);
                            }
                        }
                    });

                if ui.button(Self::get_text(self.lang, "remove_selected")).clicked() {
                    self.remove_item();
                }
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button(Self::get_text(self.lang, "save")).clicked() {
                    self.save_file();
                }
                if ui.button(Self::get_text(self.lang, "save_as")).clicked() {
                    self.save_as_file();
                }
            });
        });
    }
}

fn load_icon() -> Option<egui::IconData> {
    let icon_bytes = include_bytes!("icon.ico");
    let image = image::load_from_memory(icon_bytes).ok()?.into_rgba8();
    let (width, height) = image.dimensions();
    Some(egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    })
}

fn main() -> eframe::Result<()> {
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([700.0, 600.0]);

    if let Some(icon) = load_icon() {
        viewport = viewport.with_icon(icon);
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "PC Simulator Save Editor",
        options,
        Box::new(|_cc| Box::new(SaveEditorApp::new())),
    )
}
