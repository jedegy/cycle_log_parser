// Copyright (c) 2023
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

mod backend;
mod blocks;
pub mod events;

use blocks::{log, server, time};

/// The main component responsible for overlay display, request handling, and calling display functions
/// for other widgets (blocks).
pub struct Overlay {
    width: f32,
    height: f32,
    state: std::sync::Arc<crate::state::StateHolder>,
    server_block: server::Server,
    event_block: log::Log,
    time_block: time::Time,
}

impl Overlay {
    /// Creates a new instance of Overlay.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the overlay window.
    /// * `height` - The height of the overlay window.
    /// * `state` - The shared state holder.
    ///
    /// # Returns
    ///
    /// A new `Overlay` instance.
    pub fn new(width: f32, height: f32, state: std::sync::Arc<crate::state::StateHolder>) -> Self {
        Self {
            width,
            height,
            state,
            server_block: server::Server::default(),
            event_block: log::Log::default(),
            time_block: time::Time::default(),
        }
    }

    /// Renders the user interface of the overlay.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The `egui` context for rendering.
    ///
    /// # Returns
    ///
    /// * None
    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("The Cycle: Overlay")
            .title_bar(false)
            .resizable(false)
            .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::new(0.0, 0.0))
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                // Create new egui frame
                egui::Frame::none().show(ui, |ui| {
                    // Display server widget
                    self.server_block.show(ui);
                    // Display time widget
                    self.time_block.show(ui);
                    // Display events widget
                    self.event_block.show(ui);
                });
            });
    }

    /// Runs the overlay and event loop.
    ///
    /// # Arguments
    ///
    /// * `self` - The `Overlay` instance.
    /// * `event_loop` - The `winit` event loop.
    ///
    /// # Returns
    ///
    /// * None
    pub async fn run(mut self, event_loop: winit::event_loop::EventLoop<events::Action>) {
        let mut backend = backend::Backend::new(self.width, self.height, event_loop);

        let start_time = std::time::Instant::now();
        backend.event_loop.run(move |event, _, control_flow| {
            // Handle `winit` with custom events
            backend.platform.handle_event(&event);
            match event {
                winit::event::Event::RedrawRequested(..) => {
                    backend
                        .platform
                        .update_time(start_time.elapsed().as_secs_f64());

                    let output_frame = match backend.surface.get_current_texture() {
                        Ok(frame) => frame,
                        Err(wgpu::SurfaceError::Outdated) => {
                            // This error occurs when the app is minimized on Windows.
                            // Silently return here to prevent spamming the console with:
                            // "The underlying surface has changed, and therefore the swap chain must be updated"
                            return;
                        }
                        Err(e) => {
                            eprintln!("Dropped frame with error: {}", e);
                            return;
                        }
                    };
                    let output_view = output_frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    // Begin to draw the UI frame
                    backend.platform.begin_frame();

                    // Draw the overlay
                    self.show(&backend.platform.context());

                    // End the UI frame. We could now handle the output and draw the UI with the backend
                    let full_output = backend.platform.end_frame(Some(&backend.window));
                    let paint_jobs = backend.platform.context().tessellate(full_output.shapes);

                    let mut encoder =
                        backend
                            .device
                            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: Some("encoder"),
                            });

                    // Upload all resources for the GPU
                    let screen_descriptor = egui_wgpu_backend::ScreenDescriptor {
                        physical_width: backend.surface_config.width,
                        physical_height: backend.surface_config.height,
                        scale_factor: backend.window.scale_factor() as f32,
                    };
                    let t_delta: egui::TexturesDelta = full_output.textures_delta;
                    backend
                        .egui_rpass
                        .add_textures(&backend.device, &backend.queue, &t_delta)
                        .expect("add texture ok");
                    backend.egui_rpass.update_buffers(
                        &backend.device,
                        &backend.queue,
                        &paint_jobs,
                        &screen_descriptor,
                    );

                    // Record all render passes
                    backend
                        .egui_rpass
                        .execute(
                            &mut encoder,
                            &output_view,
                            &paint_jobs,
                            &screen_descriptor,
                            Some(wgpu::Color::TRANSPARENT),
                        )
                        .unwrap();
                    // Submit the commands
                    backend.queue.submit(std::iter::once(encoder.finish()));

                    // Redraw `egui`
                    output_frame.present();

                    backend
                        .egui_rpass
                        .remove_textures(t_delta)
                        .expect("remove texture ok");
                }
                winit::event::Event::MainEventsCleared => {
                    backend.window.request_redraw();
                }
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                    }
                    _ => {}
                },
                winit::event::Event::UserEvent(events::Action::TotalPlayerCountUpdate(event)) => {
                    // Update total number of players in the sever widget
                    self.server_block.total_players = event.players;
                }
                winit::event::Event::UserEvent(events::Action::NearPlayerCountUpdate(event)) => {
                    // Update near number of players in the sever widget
                    self.server_block.near_players = event.players;
                }
                winit::event::Event::UserEvent(events::Action::EvacShipCalled(event)) => {
                    // Post event in the event log widget with timer
                    self.event_block.post(Box::new(event));
                }
                winit::event::Event::UserEvent(events::Action::MeteorsEvent(event)) => {
                    // Post event in the event log widget with timer
                    self.event_block.post(Box::new(event));
                }
                winit::event::Event::UserEvent(events::Action::PlayerEscaped(event)) => {
                    // Post event in the event log widget with timer
                    self.event_block.post(Box::new(event));
                }
                winit::event::Event::UserEvent(events::Action::PlayerDead(event)) => {
                    // Post event in the event log widget with timer
                    self.event_block.post(Box::new(event));
                }
                winit::event::Event::UserEvent(events::Action::UpdateState(event)) => {
                    // If the general state has been updated, we call the appropriate functions in each widget
                    self.server_block
                        .on_state_update(event.game.clone(), self.state.clone());
                    self.time_block.on_state_update(event.game.clone());
                    self.event_block
                        .on_state_update(event.game, self.state.clone());
                }
                _ => (),
            }
        });
    }
}

/// Displays a labeled message with specified color, font family, and font size.
///
/// # Arguments
///
/// * `ui` - The `egui` Ui context.
/// * `message` - The message to display.
/// * `color` - The color of the message.
/// * `font_family` - The font family to use.
/// * `font_size` - The font size to use.
///
/// # Returns
///
/// * None
fn show_label(
    ui: &mut egui::Ui,
    message: String,
    color: egui::Color32,
    font_family: egui::FontFamily,
    font_size: f32,
) {
    ui.label(
        (egui::RichText::new(message).color(color)).font(egui::FontId::new(font_size, font_family)),
    );
}
