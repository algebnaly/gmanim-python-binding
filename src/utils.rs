// src/utils.rs
use gmanim_core::mobjects::DrawConfig;
use gmanim_core::Color;

pub fn build_draw_config(
    stroke_width: Option<f32>,
    fill: Option<bool>,
    color: Option<(u8, u8, u8, u8)>,
) -> DrawConfig {
    let mut draw_config = DrawConfig::default();
    if let Some(w) = stroke_width {
        draw_config.stoke_width = w;
    }
    if let Some(f) = fill {
        draw_config.fill = f;
    }
    if let Some(c) = color {
        draw_config.color = Color::new(c.0, c.1, c.2, c.3);
    }
    draw_config
}
