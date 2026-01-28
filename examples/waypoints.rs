//! Waypoints example using Camera2D projection with WGS84 coordinates.
//!
//! Each waypoint is defined in WGS84 (longitude, latitude) and projected
//! to screen coordinates via Mercator + Camera2D. Waypoints are rendered
//! as small triangles with labels using ShapeRenderable.
//!
//! - Scroll wheel: zoom in/out (zooms toward cursor)

extern crate wilhelm_renderer;

use std::cell::Cell;
use wilhelm_renderer::core::{
    App, Camera2D, Color, Projection, Vec2, Window,
    wgs84_to_mercator,
};
use wilhelm_renderer::graphics2d::shapes::{ShapeKind, ShapeRenderable, ShapeStyle, Text, Triangle};

thread_local! {
    static CAMERA_CENTER: Cell<(f32, f32)> = Cell::new((0.0, 0.0));
    static CAMERA_SCALE: Cell<f32> = Cell::new(1.0);
    static MOUSE_POS: Cell<(f64, f64)> = Cell::new((0.0, 0.0));
}

const FONT_PATH: &str = "fonts/DejaVuSans.ttf";
const FONT_SIZE: u32 = 11;

fn main() {
    let waypoint_data: &[(f32, f32, &str)] = &[
        (6.1432, 46.2044, "Geneva"),
        (6.6323, 46.5197, "Lausanne"),
        (7.4474, 46.9480, "Bern"),
        (8.2457, 46.8959, "Sarnen"),
        (8.5417, 47.3769, "Zurich"),
        (9.8355, 46.4908, "St-Moritz"),
    ];

    let mut window = Window::new(
        "Waypoints - WGS84 Projection",
        800, 600,
        Color::from_rgb(0.07, 0.13, 0.17),
    );

    // Convert waypoints to Mercator positions (state)
    let mercator_positions: Vec<Vec2> = waypoint_data
        .iter()
        .map(|(lon, lat, _)| {
            let m = wgs84_to_mercator(Vec2::new(*lon, *lat));
            Vec2::new(m.x, -m.y)
        })
        .collect();

    // Compute bounding box for initial camera view
    let min_x = mercator_positions.iter().map(|p| p.x).fold(f32::MAX, f32::min);
    let max_x = mercator_positions.iter().map(|p| p.x).fold(f32::MIN, f32::max);
    let min_y = mercator_positions.iter().map(|p| p.y).fold(f32::MAX, f32::min);
    let max_y = mercator_positions.iter().map(|p| p.y).fold(f32::MIN, f32::max);

    let center = Vec2::new((min_x + max_x) / 2.0, (min_y + max_y) / 2.0);
    let range_x = max_x - min_x;
    let range_y = max_y - min_y;
    let initial_scale = (700.0 / range_x).min(500.0 / range_y);

    CAMERA_CENTER.with(|c| c.set((center.x, center.y)));
    CAMERA_SCALE.with(|s| s.set(initial_scale));

    // Scroll to zoom at cursor
    window.on_scroll(move |_, y_offset| {
        let zoom_factor = if y_offset > 0.0 { 1.1 } else { 1.0 / 1.1 };
        let mouse_pos = MOUSE_POS.with(|m| m.get());
        let center = CAMERA_CENTER.with(|c| c.get());
        let scale = CAMERA_SCALE.with(|s| s.get());

        let mut camera = Camera2D::new(
            Vec2::new(center.0, center.1),
            scale,
            Vec2::new(800.0, 600.0),
        );
        camera.zoom_at(zoom_factor, Vec2::new(mouse_pos.0 as f32, mouse_pos.1 as f32));

        let new_scale = camera.scale().clamp(initial_scale * 0.01, initial_scale * 100.0);
        camera.set_scale(new_scale);

        CAMERA_CENTER.with(|c| c.set((camera.center().x, camera.center().y)));
        CAMERA_SCALE.with(|s| s.set(camera.scale()));
    });

    window.on_cursor_position(move |x, y| {
        MOUSE_POS.with(|m| m.set((x, y)));
    });

    let mut app = App::new(window);

    // Build shapes: [marker0, label0, marker1, label1, ...] -- 2 per waypoint
    let color = Color::from_rgb(0.2, 0.6, 1.0);
    let triangle = Triangle::new([(-4.0, 3.0), (4.0, 3.0), (0.0, -5.0)]);

    let mut shapes = Vec::with_capacity(waypoint_data.len() * 2);
    for (_lon, _lat, name) in waypoint_data {
        shapes.push(ShapeRenderable::from_shape(
            0.0, 0.0,
            ShapeKind::Triangle(triangle.clone()),
            ShapeStyle::fill(color),
        ));
        shapes.push(ShapeRenderable::from_shape(
            0.0, 0.0,
            ShapeKind::Text(Text::new(*name, FONT_PATH, FONT_SIZE)),
            ShapeStyle::fill(color),
        ));
    }
    app.add_shapes(shapes);

    app.on_pre_render(move |shapes, _renderer| {
        let center = CAMERA_CENTER.with(|c| c.get());
        let scale = CAMERA_SCALE.with(|s| s.get());

        let camera = Camera2D::new(
            Vec2::new(center.0, center.1),
            scale,
            Vec2::new(800.0, 600.0),
        );

        for (i, mercator) in mercator_positions.iter().enumerate() {
            let screen_pos = camera.world_to_screen(*mercator);
            // marker at index 2*i
            shapes[2 * i].set_position(screen_pos.x, screen_pos.y);
            // label at index 2*i + 1
            shapes[2 * i + 1].set_position(
                screen_pos.x + 8.0,
                screen_pos.y - (FONT_SIZE as f32) / 2.0,
            );
        }
    });

    println!("Waypoints - WGS84 Projection");
    println!("  Scroll: zoom in/out (zooms toward cursor)");
    println!();
    println!("Waypoints: Geneva, Lausanne, Bern, Sarnen, Zurich, St-Moritz");

    app.run();
}
