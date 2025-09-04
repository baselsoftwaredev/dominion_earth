use super::constants::{color_values, minimap_display, world_dimensions};
use bevy::prelude::*;
use bevy_egui::egui;
use core_sim::Civilization;

pub fn render_world_minimap_window(ctx: &egui::Context, civs: &Query<&Civilization>) {
    egui::Window::new("Minimap")
        .resizable(false)
        .default_size([
            minimap_display::MINIMAP_DEFAULT_WIDTH,
            minimap_display::MINIMAP_DEFAULT_HEIGHT,
        ])
        .show(ctx, |ui| {
            ui.label("Minimap");
            ui.colored_label(
                egui::Color32::GRAY,
                "Map visualization would go here",
            );

            render_civilization_position_visualization(ui, civs);
        });
}

fn render_civilization_position_visualization(
    ui: &mut egui::Ui,
    civs: &Query<&Civilization>,
) {
    let available_display_area = ui.available_size();
    let (response, painter) = ui.allocate_painter(available_display_area, egui::Sense::hover());

    if response.hovered() {
        draw_ocean_background(&painter, response.rect);
        draw_civilization_capital_markers(&painter, response.rect, civs);
    }
}

fn draw_ocean_background(painter: &egui::Painter, display_rectangle: egui::Rect) {
    painter.rect_filled(
        display_rectangle,
        0.0,
        egui::Color32::from_rgb(
            color_values::OCEAN_BACKGROUND_RED,
            color_values::OCEAN_BACKGROUND_GREEN,
            color_values::OCEAN_BACKGROUND_BLUE,
        ),
    );
}

fn draw_civilization_capital_markers(
    painter: &egui::Painter,
    display_rectangle: egui::Rect,
    civs: &Query<&Civilization>,
) {
    let civilizations_collection: Vec<_> = civs.iter().collect();
    for (_civilization_index, civilization) in civilizations_collection
        .iter()
        .enumerate()
        .take(minimap_display::MAXIMUM_CIVILIZATIONS_DISPLAYED)
    {
        if let Some(capital_position) = civilization.capital {
            let capital_marker_position = calculate_capital_marker_position(
                capital_position,
                display_rectangle,
            );

            let civilization_color = convert_civilization_color_to_egui_color(civilization);

            painter.circle_filled(
                capital_marker_position,
                minimap_display::CAPITAL_MARKER_RADIUS,
                civilization_color,
            );
        }
    }
}

fn calculate_capital_marker_position(
    capital_position: core_sim::components::Position,
    display_rectangle: egui::Rect,
) -> egui::Pos2 {
    let normalized_x_position = capital_position.x as f32 / world_dimensions::ASSUMED_WORLD_WIDTH;
    let normalized_y_position = capital_position.y as f32 / world_dimensions::ASSUMED_WORLD_HEIGHT;

    display_rectangle.min
        + egui::Vec2::new(
            normalized_x_position * display_rectangle.width(),
            normalized_y_position * display_rectangle.height(),
        )
}

fn convert_civilization_color_to_egui_color(civilization: &Civilization) -> egui::Color32 {
    egui::Color32::from_rgb(
        (civilization.color[0] * color_values::RGB_COLOR_CONVERSION_FACTOR) as u8,
        (civilization.color[1] * color_values::RGB_COLOR_CONVERSION_FACTOR) as u8,
        (civilization.color[2] * color_values::RGB_COLOR_CONVERSION_FACTOR) as u8,
    )
}
