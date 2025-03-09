use gilrs::{Gilrs, Event};

use eframe::egui;
use egui::Key;
use std::time::Duration;
use image::{Rgb, RgbImage};
use std::fs;
use imageproc::drawing::draw_antialiased_line_segment_mut;
use imageproc::pixelops::interpolate;

struct DrawingApp {
    strokes: Vec<Option<egui::Pos2>>, // Use None to separate strokes
    canvas_size: egui::Vec2,
    input: Gilrs,
    history: Vec<Vec<Option<egui::Pos2>>>, // Histowy of strokes
    redo_stack: Vec<Vec<Option<egui::Pos2>>>, // Wedo stack
}

impl DrawingApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let input = Gilrs::new().unwrap();
        Self { 
            strokes: Vec::new(),
            canvas_size: egui::vec2(1920 as f32, 1080 as f32),
            input,
            history: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    fn save_drawing(&self) {
        let width = self.canvas_size.x as u32;
        let height = self.canvas_size.y as u32;
        let mut img = RgbImage::from_pixel(width, height, Rgb([255, 255, 255]));
        
        for stroke in self.strokes.windows(2) {
            if let [Some(a), Some(b)] = stroke {
                let (ax, ay) = (a.x as f32, a.y as f32);
                let (bx, by) = (b.x as f32, b.y as f32);
                draw_antialiased_line_segment_mut(&mut img, (ax as i32, ay as i32), (bx as i32, by as i32), Rgb([0, 0, 0]), interpolate);
            }
        }

        let _ = fs::create_dir_all("output");
        img.save("output/drawing.png").unwrap();
    }
}

impl eframe::App for DrawingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Draw on the canvas below:");
            let available_size = ui.available_size();
            self.canvas_size = available_size;
            let response = ui.allocate_response(available_size, egui::Sense::drag());
           
            while let Some(Event {id, event, ..}) = self.input.next_event() {
                println!("Tabletyy");
            }

            if ctx.input(|i| i.key_pressed(Key::C)) {
                println!("Cleared");
                self.strokes.clear();
            }

            if ctx.input(|i| i.key_pressed(Key::Z)) {
                println!("Undone");
                if !self.history.is_empty() {
                    let last_stroke = self.history.pop().unwrap(); // Wemove da wast stwoke fwom histowy
                    self.redo_stack.push(last_stroke); // Save it to da wedo stack
                    self.strokes = self.history.last().cloned().unwrap_or_default(); // Westowe pwevious state               
                }
            }

            if ctx.input(|i| i.key_pressed(Key::Y)) {
                println!("Redone");
                if !self.redo_stack.is_empty() {
                    let last_undone_stroke = self.redo_stack.pop().unwrap(); // Wemove da wast stwoke fwom wedo stack
                    self.history.push(last_undone_stroke.clone()); // Add it back to da histowy
                    self.strokes = last_undone_stroke; // Westowe da stwoke
                }
            }

            if response.dragged() {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    println!("Pointer Position: {:?}", pointer_pos);
                    self.strokes.push(Some(pointer_pos));
                }
            } else if response.drag_released() {
                self.strokes.push(None); // Separate strokes
                self.history.push(self.strokes.clone()); // Save to histowy
            }
            
            let painter = ui.painter();
            painter.rect(
                response.rect,
                0.0,
                egui::Color32::WHITE,
                egui::Stroke::new(2.0, egui::Color32::GRAY),
                egui::StrokeKind::Middle,
            );
            
            for stroke in self.strokes.windows(2) {
                if let [Some(a), Some(b)] = stroke {
                    painter.line_segment([
                        *a, *b
                    ], egui::Stroke::new(5.0, egui::Color32::BLACK));
                }
            }

            if ui.button("Save Drawing").clicked() {
                self.save_drawing();
            }
        });
        
        ctx.request_repaint_after(Duration::from_millis(16));
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Pure Egui Drawing App",
        options,
        Box::new(|cc| Ok(Box::new(DrawingApp::new(cc)))),
    )
}

