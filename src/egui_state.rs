use egui::{epaint::ClippedShape, Context, FullOutput, Rect, TexturesDelta};

use egui_software_backend::{
    BufferMutRef, BufferRef, CachedPrimitive, ColorFieldOrder, EguiSoftwareRender as Renderer,
};
use smithay_client_toolkit::shm::slot::SlotPool;
use wayland_client::protocol::wl_surface::WlSurface;

pub struct State {
    context: egui::Context,
    input: egui::RawInput,
    renderer: Renderer,
    start_time: std::time::Instant,
    size: Option<Rect>,
}

impl State {
    pub fn new(context: egui::Context) -> Self {
        let input = egui::RawInput {
            focused: true,
            viewport_id: egui::ViewportId::ROOT,
            ..Default::default()
        };

        let renderer = Renderer::new(ColorFieldOrder::Bgra)
            .with_convert_tris_to_rects(true)
            .with_allow_raster_opt(true)
            .with_caching(false);

        // input
        //     .viewports
        //     .entry(egui::ViewportId::ROOT)
        //     .or_default()
        //     .native_pixels_per_point = Some(1.0);

        Self {
            context,
            input,
            renderer,
            start_time: std::time::Instant::now(),
            size: None,
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        let screen_rect = egui::Rect {
            min: egui::Pos2 { x: 0f32, y: 0f32 },
            max: egui::Pos2 {
                x: width as f32,
                y: height as f32,
            },
        };
        self.size = Some(screen_rect);
        println!("set_size: {}x{}", width, height);
        self.input.screen_rect = Some(screen_rect);
    }

    pub(crate) fn get_size(&self) -> (i32, i32) {
        self.size
            .map(|r| (r.width().ceil() as i32, r.height().ceil() as i32))
            .unwrap()
    }

    pub(crate) fn input(&mut self) -> &mut egui::RawInput {
        &mut self.input
    }

    pub fn context(&self) -> &egui::Context {
        &self.context
    }

    pub fn modifiers(&self) -> egui::Modifiers {
        self.input.modifiers
    }

    pub fn push_event(&mut self, event: egui::Event) {
        self.input.events.push(event);
    }

    pub fn process_events(&mut self, run_ui: impl FnMut(&Context)) -> FullOutput {
        // TODO: maybe we need to take input for a certain window / surface?
        self.input.time = Some(self.start_time.elapsed().as_secs_f64());

        let raw_input = self.input.take();
        /* if (&raw_input.events).len() > 0 {
            dbg!(&raw_input.events);
        } */
        self.context.run(raw_input, run_ui)
    }

    pub fn draw(&mut self, full_output: FullOutput, buffer_ref: &mut BufferMutRef) {
        //self.context.set_pixels_per_point(screen_descriptor.pixels_per_point);

        // iterate over viewport outputs
        /* for output in full_output.viewport_output.values() {
            dbg!(&output.repaint_delay);
        } */

        //dbg!(&full_output.);

        // TODO: implement platform output handling
        // this is for things like clipboard support
        //self.state.handle_platform_output(window, full_output.platform_output);

        let clipped_primitives = self
            .context
            .tessellate(full_output.shapes, full_output.pixels_per_point);

        self.renderer.render(
            buffer_ref,
            &clipped_primitives,
            &full_output.textures_delta,
            full_output.pixels_per_point,
        );
    }
}
