use crate::core::renderer::{Renderable, Renderer};
use crate::core::Window;
use crate::graphics2d::shapes::ShapeRenderable;

pub struct App<'a> {
    pub window: Box<Window>,
    renderer: Renderer,
    shapes: Vec<ShapeRenderable>,
    render_callback: Option<Box<dyn FnMut() + 'a>>,
}

impl<'a> App<'a> {
    pub fn new(window: Box<Window>) -> Self {
        let renderer = Renderer::new(window.handle());
        Self {
            window,
            renderer,
            shapes: Vec::new(),
            render_callback: None,
        }
    }

    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }

    pub fn add_shape(&mut self, shape: ShapeRenderable) {
        self.shapes.push(shape);
    }

    pub fn shapes(&self) -> &[ShapeRenderable] {
        &self.shapes
    }

    pub fn shapes_mut(&mut self) -> &mut [ShapeRenderable] {
        &mut self.shapes
    }

    pub fn on_render<F>(&mut self, callback: F)
    where
        F: FnMut() + 'a,
    {
        self.render_callback = Some(Box::new(callback));
    }

    pub fn run(mut self) {
        while !self.window.window_should_close() {
            self.window.clear_color();

            if let Some(cb) = self.render_callback.as_mut() {
                cb();
            }

            for shape in &mut self.shapes {
                shape.render(&self.renderer);
            }

            self.window.swap_buffers();
            self.window.poll_events();
        }
    }
}
