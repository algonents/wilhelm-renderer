//! Example demonstrating Camera2D for pan and zoom.
//!
//! - Scroll wheel: zoom in/out (zooms toward cursor)
//!
//! Shapes are defined in world coordinates and transformed to screen
//! coordinates using the camera projection. Shape SIZES stay constant
//! in screen pixels (like map markers/waypoints) - only POSITIONS change
//! with zoom. When zoomed out, shapes cluster together; when zoomed in,
//! they spread apart.
//!
//! This example is production-ready: ShapeRenderables are created once
//! and their positions are updated each frame via set_position().

extern crate wilhelm_renderer;

use std::cell::Cell;
use wilhelm_renderer::core::{App, Camera2D, Color, Projection, Vec2, Window};
use wilhelm_renderer::graphics2d::shapes::{Circle, Rectangle, ShapeKind, ShapeRenderable, ShapeStyle};

thread_local! {
    static CAMERA_CENTER: Cell<(f32, f32)> = Cell::new((0.0, 0.0));
    static CAMERA_SCALE: Cell<f32> = Cell::new(1.0);
    static MOUSE_POS: Cell<(f64, f64)> = Cell::new((0.0, 0.0));
}

fn make_shape(shape: ShapeKind, color: Color) -> ShapeRenderable {
    ShapeRenderable::from_shape(0.0, 0.0, shape, ShapeStyle::fill(color))
}

fn main() {
    let mut window = Window::new("Camera2D Example", 800, 600, Color::from_rgb(0.1, 0.1, 0.15));

    // Handle scroll for zoom
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

        let new_scale = camera.scale().clamp(0.1, 50.0);
        camera.set_scale(new_scale);

        CAMERA_CENTER.with(|c| c.set((camera.center().x, camera.center().y)));
        CAMERA_SCALE.with(|s| s.set(camera.scale()));

        println!(
            "scale: {:.2}, center: ({:.1}, {:.1})",
            camera.scale(),
            camera.center().x,
            camera.center().y
        );
    });

    window.on_cursor_position(move |x, y| {
        MOUSE_POS.with(|m| m.set((x, y)));
    });

    let mut app = App::new(window);

    // World positions (state) -- parallel to the shapes added below
    let world_positions: Vec<Vec2> = vec![
        // Grid of circles
        Vec2::new(0.0, 0.0),
        Vec2::new(100.0, 0.0),
        Vec2::new(200.0, 0.0),
        Vec2::new(-100.0, 0.0),
        Vec2::new(-200.0, 0.0),
        Vec2::new(0.0, 100.0),
        Vec2::new(0.0, -100.0),
        Vec2::new(0.0, 200.0),
        Vec2::new(0.0, -200.0),
        // Rectangles at corners
        Vec2::new(150.0, 150.0),
        Vec2::new(-150.0, -150.0),
        Vec2::new(-150.0, 150.0),
        Vec2::new(150.0, -150.0),
        // Origin marker
        Vec2::new(0.0, 0.0),
    ];

    // Shapes (view) -- parallel to world_positions
    app.add_shapes(vec![
        make_shape(ShapeKind::Circle(Circle::new(30.0)), Color::from_rgb(1.0, 0.3, 0.3)),
        make_shape(ShapeKind::Circle(Circle::new(30.0)), Color::from_rgb(0.3, 1.0, 0.3)),
        make_shape(ShapeKind::Circle(Circle::new(30.0)), Color::from_rgb(0.3, 0.3, 1.0)),
        make_shape(ShapeKind::Circle(Circle::new(30.0)), Color::from_rgb(1.0, 1.0, 0.3)),
        make_shape(ShapeKind::Circle(Circle::new(30.0)), Color::from_rgb(1.0, 0.3, 1.0)),
        make_shape(ShapeKind::Circle(Circle::new(30.0)), Color::from_rgb(0.3, 1.0, 1.0)),
        make_shape(ShapeKind::Circle(Circle::new(30.0)), Color::from_rgb(1.0, 0.6, 0.3)),
        make_shape(ShapeKind::Circle(Circle::new(30.0)), Color::from_rgb(0.6, 0.3, 1.0)),
        make_shape(ShapeKind::Circle(Circle::new(30.0)), Color::from_rgb(0.3, 0.6, 0.3)),
        make_shape(ShapeKind::Rectangle(Rectangle::new(80.0, 50.0)), Color::from_rgb(0.8, 0.4, 0.2)),
        make_shape(ShapeKind::Rectangle(Rectangle::new(60.0, 90.0)), Color::from_rgb(0.2, 0.4, 0.8)),
        make_shape(ShapeKind::Rectangle(Rectangle::new(70.0, 70.0)), Color::from_rgb(0.4, 0.8, 0.4)),
        make_shape(ShapeKind::Rectangle(Rectangle::new(50.0, 80.0)), Color::from_rgb(0.8, 0.8, 0.2)),
        // Origin marker
        make_shape(ShapeKind::Circle(Circle::new(5.0)), Color::white()),
    ]);

    app.on_pre_render(move |shapes, _renderer| {
        let center = CAMERA_CENTER.with(|c| c.get());
        let scale = CAMERA_SCALE.with(|s| s.get());

        let camera = Camera2D::new(
            Vec2::new(center.0, center.1),
            scale,
            Vec2::new(800.0, 600.0),
        );

        for (shape, world_pos) in shapes.iter_mut().zip(world_positions.iter()) {
            let screen_pos = camera.world_to_screen(*world_pos);
            shape.set_position(screen_pos.x, screen_pos.y);
        }
    });

    println!("Camera2D Example");
    println!("  Scroll: zoom in/out (zooms toward cursor)");
    println!("");
    println!("Shapes are in world coordinates, camera transforms to screen.");
    println!("Shape sizes stay constant; only positions change with zoom.");

    app.run();
}
