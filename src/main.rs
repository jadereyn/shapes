#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

extern crate image;
use eframe::egui::*;
use eframe::epaint::*;
use std::collections::HashMap;
use std::path::Path;

mod gen_util;

const SCREEN_WIDTH: f32 = 850.0;
const SCREEN_HEIGHT: f32 = 600.0;
const SIDE_PANEL_WIDTH: f32 = 200.0;
const PANEL_HEIGHT: f32 = SCREEN_HEIGHT;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(SCREEN_WIDTH, SCREEN_HEIGHT)),
        ..Default::default()
    };
    eframe::run_native(
        "Shapes",
        options,
        Box::new(|_cc| Box::<RandomizedShapeApp>::default()),
    )
}

#[derive(Eq, Hash, PartialEq)]
enum ShapeName {
    Rect,
    Circle,
    Triangle,
    EqTriangle,
    Hexagon,
}

struct ShapeMetaData {
    count: i32,
    max_radius: Option<f32>,
    label_text: String,
}

struct RandomizedShapeApp {
    min_rad: f32,
    metadata: HashMap<ShapeName, ShapeMetaData>,
    shapevec: Vec<Shape>,
    bg_rgba: [u8; 4],
    fullscreen_toggle: bool,
    side_panel_expanded: bool,
    default_filename: String,
    curr_file_num: i32,
    folder_path: Option<String>,
    desired_filename: String,
    desired_filename_exists: bool,
    filename_edit_has_focus: bool,
}

impl Default for RandomizedShapeApp {
    fn default() -> Self {

        let mut md: HashMap<ShapeName, ShapeMetaData> = HashMap::new();

        md.insert(ShapeName::Rect, 
            ShapeMetaData{ 
                count: 3, 
                max_radius: None, 
                label_text: String::from("rectangle") 
            });
        md.insert(ShapeName::Circle, 
            ShapeMetaData{ 
                count: 3, 
                max_radius: Some(PANEL_HEIGHT/4.0), 
                label_text: String::from("circle") 
            });
        md.insert(ShapeName::Triangle, 
            ShapeMetaData{ 
                count: 3, 
                max_radius: None, 
                label_text: String::from("triangle") 
            });
        md.insert(ShapeName::EqTriangle, 
            ShapeMetaData{ 
                count: 3, 
                max_radius: Some(PANEL_HEIGHT/4.0), 
                label_text: String::from("equilateral triangle") 
            });
        md.insert(ShapeName::Hexagon, 
            ShapeMetaData{ 
                count: 3, 
                max_radius: Some(PANEL_HEIGHT/4.0), 
                label_text: String::from("hexagon") 
            });

        Self {
            min_rad: 1.0,
            metadata: md,
            shapevec: Vec::new(),
            bg_rgba: [255, 255, 255, 255],
            fullscreen_toggle: false,
            side_panel_expanded: true,
            default_filename: String::from("shapes"),
            curr_file_num: 0,
            folder_path: None,
            desired_filename: String::from(""),
            desired_filename_exists: false,
            filename_edit_has_focus: false,
        }
    }
}

impl RandomizedShapeApp {
    fn gen_random_shape_vec(&mut self, max_width: f32, max_height: f32) {

        self.shapevec.clear();

        for (name, metadata) in self.metadata.iter() {
            for _ in 0..metadata.count {
                self.shapevec.push(self.gen_random_shape(name, max_width, max_height));
            }
        }

    }

    fn gen_random_shape(&self, shape: &ShapeName, max_width: f32, max_height: f32) -> Shape {

        let color: Color32 = gen_util::gen_random_rgb_color();
    
        match shape {
            ShapeName::Rect => Shape::Rect(RectShape { 
                rect: gen_util::gen_random_rect(max_width, max_height),
                rounding: Rounding::none(),
                fill: color,
                stroke: Stroke::new(1.0, color),
            }),
            ShapeName::Circle => {
                let Some(meta) = self.metadata.get(&shape) else {panic!("Circle shape not added to hashmap.")};
                let rad = match meta.max_radius {
                    Some(r) => r,
                    None => self.min_rad,
                };
                let r = gen_util::gen_random_f32(self.min_rad, rad);
                Shape::Circle(CircleShape {
                    center: gen_util::gen_random_pos2(r, max_width-r, r, max_height-r),
                    radius: r,
                    fill: color,
                    stroke: Stroke::new(1.0, color),
                })
            },
            ShapeName::Triangle => {
                Shape::Path(PathShape {
                    points: gen_util::gen_random_polygon(max_width, max_height, 3),
                    closed: true,
                    fill: color,
                    stroke: Stroke::new(1.0, color),
                })
            },
            ShapeName::EqTriangle => {
                let Some(meta) = self.metadata.get(&shape) else {panic!("EqTriangle shape not added to hashmap.")};
                let rad = match meta.max_radius {
                    Some(r) => r,
                    None => self.min_rad,
                };
                Shape::Path(PathShape {
                    points: gen_util::gen_random_regular_polygon(self.min_rad, rad, max_width, max_height, 3),
                    closed: true,
                    fill: color,
                    stroke: Stroke::new(1.0, color),
                })
            },
            ShapeName::Hexagon => {
                let Some(meta) = self.metadata.get(&shape) else {panic!("Hexagon shape not added to hashmap.")};
                let rad = match meta.max_radius {
                    Some(r) => r,
                    None => self.min_rad,
                };
                Shape::Path(PathShape {
                    points: gen_util::gen_random_regular_polygon(self.min_rad, rad, max_width, max_height, 6),
                    closed: true,
                    fill: color,
                    stroke: Stroke::new(1.0, color),
                })
            },
        } 
    }
}

impl eframe::App for RandomizedShapeApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let my_frame = containers::Frame {
            inner_margin: style::Margin { left: 0., right: 0., top: 0., bottom: 0. },
            outer_margin: style::Margin { left: 0., right: 0., top: 0., bottom: 0. },
            rounding: Rounding { nw: 0.0, ne: 0.0, sw: 0.0, se: 0.0 },
            shadow: Shadow { extrusion: 0.0, color: Color32::YELLOW },
            fill: Color32::from_rgba_unmultiplied(self.bg_rgba[0], self.bg_rgba[1], self.bg_rgba[2], self.bg_rgba[3]),
            stroke: Stroke::new(0.0, Color32::GOLD),
        };

        let ws = _frame.info().window_info.size;

        /* Mouse hover side panel, opted for key instead */
        /* let side_panel_rect = Rect { min: Pos2 { x: ws.x - SIDE_PANEL_WIDTH, y: 0.0 }, max: ws.to_pos2() };
        if let Some(pos_in_panel) = ctx.pointer_latest_pos() {
            self.side_panel_expanded = side_panel_rect.contains(pos_in_panel);
        }
        */

        if !self.filename_edit_has_focus {
            if ctx.input(|i| i.key_pressed(Key::F)) {
                self.fullscreen_toggle = !self.fullscreen_toggle;
                _frame.set_fullscreen(self.fullscreen_toggle);
            }

            if ctx.input(|i| i.key_pressed(Key::P)) {
                self.side_panel_expanded = !self.side_panel_expanded;
            }
    
            if ctx.input(|i| i.key_pressed(Key::G)) {
                self.gen_random_shape_vec(ws.x, ws.y);
            }
    
            if ctx.input(|i| i.key_pressed(Key::S)) {
                _frame.request_screenshot();
            }
        }
        

        SidePanel::right("Settings")
        .exact_width(SIDE_PANEL_WIDTH)
        .show_animated(ctx, self.side_panel_expanded, |ui| {
            ui.label("Settings\n");
            ui.label("Background color:");
            ui.color_edit_button_srgba_unmultiplied(&mut self.bg_rgba);

            for (_, metadata) in self.metadata.iter_mut() {
                ui.label(format!("Number of {}s:", metadata.label_text));
                ui.add(Slider::new(&mut metadata.count, 0..=100));
            }

            for (_, metadata) in self.metadata.iter_mut() {
                if let Some(rad) = metadata.max_radius.as_mut() {
                    ui.label(format!("Max {} radius:", metadata.label_text));
                    ui.add(Slider::new(rad, self.min_rad..=ws.y/2.0));
                }
            }

            if let Some(fp) = &self.folder_path {
                ui.label(format!("\nSave screenshots to: {}", fp));
            } else {
                ui.label(RichText::new("\nPlease select folder to save to: ").color(Color32::RED));
            }
            
            if ui.button("Choose folder...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.folder_path = Some(path.display().to_string());
                }
            }

            if self.desired_filename_exists {
                ui.label(RichText::new("\nFilename already exists! ").color(Color32::RED));
            } else {
                ui.label("\nFilename to save to: ");
            }
            
            let filename_edit_response = ui.add(
                TextEdit::singleline(&mut self.desired_filename).desired_width(SIDE_PANEL_WIDTH - 20.0)
            );

            if filename_edit_response.gained_focus() {
                self.filename_edit_has_focus = true;
            }

            if filename_edit_response.lost_focus() {
                self.filename_edit_has_focus = false;
            }

            ui.label("\nKeyboard controls:");
            ui.label("G - generate new shapes");
            ui.label("F - toggle fullscreen");
            ui.label("P - toggle side panel");
            ui.label("S - save as png");

        });

        CentralPanel::default().frame(my_frame).show(ctx, |ui| {
            for shape in self.shapevec.clone() {
                ui.painter().add(shape);
            }
        });
    }

    fn post_rendering(&mut self, _window_size: [u32; 2], frame: &eframe::Frame) {
        if let Some(screenshot) = frame.screenshot() {
            if let Some(fp) = &self.folder_path {

                let mut file_path;

                if self.desired_filename.len() > 0 {
                    file_path = format!("{}\\{}.png", fp, self.desired_filename);
                    if Path::new(&file_path).exists() {
                        self.desired_filename_exists = true;
                        self.side_panel_expanded = true;
                    } else {
                        self.desired_filename_exists = false;
                    }
                } else {
                    file_path = format!("{}\\{}{}.png", fp, self.default_filename, self.curr_file_num);

                    // check if file path exists, if yes increment curr_file_num, otherwise save to this file path
                    if self.curr_file_num == 0 { // first screenshot since we opened new app instance
                        while Path::new(&file_path).exists() {
                            self.curr_file_num += 1;
                            file_path = format!("{}\\{}{}.png", fp, self.default_filename, self.curr_file_num);
                        }
                    }

                    self.curr_file_num += 1;
                }

                image::save_buffer(
                    file_path,
                    screenshot.as_raw(),
                    screenshot.width() as u32,
                    screenshot.height() as u32,
                    image::ColorType::Rgba8,
                ).unwrap();
                
            } else {
                self.side_panel_expanded = true;
            }
        }
    }
    
}
