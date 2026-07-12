use nih_plug::prelude::*;
use nih_plug_egui::{egui, EguiState};
use std::f32::consts::PI;
use std::sync::Arc;
use crate::GradientKnobParams;

const START_DEG: f32 = 120.0;
const END_DEG: f32 = 420.0;
const SWEEP_DEG: f32 = 300.0;
const DEAD_START: f32 = 60.0;
const DEAD_END: f32 = 120.0;

fn lerp_color(t: f32) -> egui::Color32 {
    let t = t.clamp(0.0, 1.0);
    if t <= 0.5 {
        let k = t * 2.0;
        egui::Color32::from_rgb((34.0 + (250.0-34.0)*k) as u8, (197.0 + (204.0-197.0)*k) as u8, (94.0 + (21.0-94.0)*k) as u8)
    } else {
        let k = (t-0.5)*2.0;
        egui::Color32::from_rgb((250.0 + (220.0-250.0)*k) as u8, (204.0 + (38.0-204.0)*k) as u8, (21.0 + (38.0-21.0)*k) as u8)
    }
}
fn angle_to_pos(c: egui::Pos2, r: f32, a_rad: f32) -> egui::Pos2 { egui::Pos2::new(c.x + a_rad.cos()*r, c.y + a_rad.sin()*r) }
fn arc_points(c: egui::Pos2, r: f32, s_rad: f32, e_rad: f32, n: usize) -> Vec<egui::Pos2> {
    (0..=n).map(|i|{ let t = i as f32 / n as f32; angle_to_pos(c, r, s_rad + (e_rad - s_rad)*t)}).collect()
}
fn rounded_line(painter: &egui::Painter, p1: egui::Pos2, p2: egui::Pos2, w: f32, col: egui::Color32){
    painter.line_segment([p1,p2], egui::Stroke::new(w, col));
    painter.circle_filled(p1, w*0.5, col);
    painter.circle_filled(p2, w*0.5, col);
}
fn map_deg_to_val(deg: f32) -> f32 {
    if deg >= START_DEG { (deg - START_DEG)/SWEEP_DEG * 100.0 }
    else if deg <= DEAD_START { (deg + 240.0)/SWEEP_DEG * 100.0 }
    else { 0.0 }
}

#[derive(Default)]
struct UiState { current: f32, initialized: bool }

pub fn create_editor(
    params: Arc<GradientKnobParams>,
    egui_state: Arc<EguiState>,
) -> Option<Box<dyn Editor>> {
    nih_plug_egui::create_egui_editor(
        egui_state,
        params.clone(),
        |_, _| {},
        move |egui_ctx, setter, state| {
            let target_plain = state.params.value.modulated_plain_value();

            let mut ui_state = egui_ctx.data_mut(|d| {
                d.get_temp_mut_or_default::<UiState>(egui::Id::new("knob_state")).clone()
            });
            // we need manual clone because UiState is not egui storable by default, so reinsert
            // init
            if!ui_state.initialized {
                ui_state.current = target_plain;
                ui_state.initialized = true;
            }

            let dt = egui_ctx.input(|i| i.unstable_dt).clamp(0.0, 0.05);
            let lerp = 1.0 - (-dt * 18.0_f32).exp();
            ui_state.current += (target_plain - ui_state.current) * lerp;
            if (target_plain - ui_state.current).abs() < 0.01 { ui_state.current = target_plain; }

            // keyboard
            if egui_ctx.input(|i| i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::ArrowUp)) {
                let nv = (target_plain + 2.0).clamp(0.0,100.0);
                setter.begin_set_parameter(&state.params.value);
                setter.set_parameter(&state.params.value, nv);
                setter.end_set_parameter(&state.params.value);
            }
            if egui_ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::ArrowDown)) {
                let nv = (target_plain - 2.0).clamp(0.0,100.0);
                setter.begin_set_parameter(&state.params.value);
                setter.set_parameter(&state.params.value, nv);
                setter.end_set_parameter(&state.params.value);
            }

            egui::CentralPanel::default()
               .frame(egui::Frame { fill: egui::Color32::from_rgb(14,14,18), inner_margin: egui::Margin::symmetric(20.0,20.0),..Default::default() })
               .show(egui_ctx, |ui| {
                ui.vertical_centered(|ui|{
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("Circular Gradient Knob").size(18.0).color(egui::Color32::from_gray(200)).strong());
                    ui.add_space(6.0);

                    let desired = egui::Vec2::splat(300.0_f32);
                    let (rect, resp) = ui.allocate_exact_size(desired, egui::Sense::click_and_drag());
                    let center = rect.center();
                    let radius = 92.0_f32;
                    let sw = 16.0_f32;
                    let tick_r = 120.0_f32;
                    let start_rad = START_DEG*PI/180.0;
                    let end_rad = END_DEG*PI/180.0;
                    let sweep_rad = SWEEP_DEG*PI/180.0;
                    let cur_t = ui_state.current/100.0;
                    let cur_rad = start_rad + cur_t * sweep_rad;
                    let cur_col = lerp_color(cur_t);

                    if resp.dragged() || resp.clicked() {
                        if let Some(p) = resp.interact_pointer_pos() {
                            let v = p - center;
                            let mut deg = v.y.atan2(v.x).to_degrees();
                            if deg < 0.0 { deg += 360.0; }
                            if!(deg > DEAD_START && deg < DEAD_END) {
                                let nv = map_deg_to_val(deg).clamp(0.0,100.0);
                                if (nv - target_plain).abs() < 70.0 {
                                    setter.begin_set_parameter(&state.params.value);
                                    setter.set_parameter(&state.params.value, nv);
                                    setter.end_set_parameter(&state.params.value);
                                }
                            }
                        }
                    }

                    let painter = ui.painter_at(rect);
                    painter.circle_filled(center, radius+20.0, egui::Color32::from_rgba_unmultiplied(cur_col.r(),cur_col.g(),cur_col.b(),14));

                    for i in 0..=40 {
                        let t = i as f32/40.0;
                        let rad = (START_DEG + t*SWEEP_DEG)*PI/180.0;
                        let major = i%10==0;
                        let mid = i%5==0;
                        let len = if major {16.0_f32} else if mid {11.0} else {7.0};
                        let w = if major {2.8_f32} else {1.6_f32};
                        let dist = (t-cur_t).abs();
                        let scale = if dist<0.10 {1.0+(0.10-dist)/0.10*0.6} else {1.0};
                        let col = if t <= cur_t+0.001 { lerp_color(t) } else { egui::Color32::from_gray(95) };
                        let p1 = angle_to_pos(center, tick_r, rad);
                        let p2 = angle_to_pos(center, tick_r+len*scale, rad);
                        rounded_line(&painter, p1, p2, w*scale, col);
                    }

                    let grey_col = egui::Color32::from_rgb(48,52,62);
                    let full_pts = arc_points(center, radius, start_rad, end_rad, 64);
                    painter.add(egui::Shape::Path(egui::epaint::PathShape{ points: full_pts, closed:false, fill:egui::Color32::TRANSPARENT, stroke:egui::Stroke::new(sw,grey_col)}));

                    if cur_t > 0.001 {
                        let steps = 100;
                        for s in 0..steps {
                            let t0 = s as f32/steps as f32 * cur_t;
                            let t1 = (s+1) as f32/steps as f32 * cur_t;
                            let a0 = start_rad + t0*sweep_rad;
                            let a1 = start_rad + t1*sweep_rad;
                            let mid = (t0+t1)*0.5;
                            painter.line_segment([angle_to_pos(center,radius,a0), angle_to_pos(center,radius,a1)], egui::Stroke::new(sw, lerp_color(mid)));
                        }
                        painter.circle_filled(angle_to_pos(center,radius,start_rad), sw*0.5, lerp_color(0.0));
                        painter.circle_filled(angle_to_pos(center,radius,cur_rad), sw*0.5, cur_col);
                    } else {
                        painter.circle_filled(angle_to_pos(center,radius,start_rad), sw*0.5, grey_col);
                        painter.circle_filled(angle_to_pos(center,radius,end_rad), sw*0.5, grey_col);
                    }

                    let knob_pos = angle_to_pos(center, radius, cur_rad);
                    painter.circle_filled(knob_pos, 17.0, egui::Color32::from_black_alpha(90));
                    painter.circle_filled(knob_pos, 13.5, egui::Color32::WHITE);
                    painter.circle_stroke(knob_pos, 13.5, egui::Stroke::new(3.2_f32, cur_col));
                    painter.circle_filled(knob_pos, 3.0, cur_col);

                    painter.text(center, egui::Align2::CENTER_CENTER, format!("{:.0}%", ui_state.current), egui::FontId::proportional(40.0), egui::Color32::WHITE);
                    let sub = egui::Pos2::new(center.x, center.y+34.0);
                    painter.text(sub, egui::Align2::CENTER_CENTER, "drag or arrow keys", egui::FontId::proportional(12.0), egui::Color32::from_gray(130));

                    ui.add_space(18.0);
                    ui.horizontal(|ui|{
                        ui.label(egui::RichText::new(format!("Value: {:.0}", ui_state.current)).color(egui::Color32::from_gray(160)));
                        let (bar_rect, _) = ui.allocate_exact_size(egui::Vec2::new(160.0,6.0), egui::Sense::hover());
                        let mut fill_rect = bar_rect;
                        fill_rect.max.x = bar_rect.min.x + bar_rect.width()*cur_t;
                        let up = ui.painter();
                        up.rect_filled(bar_rect, 3.0, egui::Color32::from_rgb(36,40,50));
                        up.rect_filled(fill_rect, 3.0, cur_col);
                    });

                    // save state back
                    egui_ctx.data_mut(|d| d.insert_temp(egui::Id::new("knob_state"), ui_state));
                    egui_ctx.request_repaint();
                });
            });
        },
    )
}