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

//! This module provides the `Backend` struct, which sets up and manages the graphical window
//! using the `winit` and `wgpu` libraries, including configuring the window's parameters,
//! enabling a custom font with `egui`, and making the window transparent and unresponsive
//! to mouse events. The Overlay operates on top of the `egui` library.

use super::events::Action;

use winit::platform::windows::WindowExtWindows;

/// The `Backend` struct is used for creating and managing the graphical window.
pub struct Backend {
    pub window: winit::window::Window,
    pub event_loop: winit::event_loop::EventLoop<Action>,
    pub platform: egui_winit_platform::Platform,
    pub surface: wgpu::Surface,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub egui_rpass: egui_wgpu_backend::RenderPass,
}

impl Backend {
    /// Creates a new `Backend` instance. Sets up the window and graphical environment.
    ///
    /// # Arguments
    ///
    /// * `width` - Desired window width.
    /// * `height` - Desired window height.
    /// * `event_loop` - Event loop for handling window events.
    ///
    /// # Returns
    ///
    /// * Self - A new instance of `Backend`.
    pub fn new(width: f32, height: f32, event_loop: winit::event_loop::EventLoop<Action>) -> Self {
        // Creates a new window with specific configurations
        let window = winit::window::WindowBuilder::new()
            .with_decorations(false)
            .with_resizable(false)
            .with_transparent(true)
            .with_always_on_top(true)
            .with_inner_size(winit::dpi::PhysicalSize { width, height })
            .build(&event_loop)
            .unwrap();

        // Positions the window at the top-right of the screen
        window.set_outer_position(winit::dpi::PhysicalPosition::new(
            window.current_monitor().unwrap().size().width - window.inner_size().width - 30,
            30,
        ));

        // Instance creation for WGPU
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(&window) };

        // Request for a compatible adapter
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        // Request for a device and a command queue
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::default(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ))
        .unwrap();

        // Surface configuration
        let size = window.inner_size();
        let surface_format = surface.get_supported_formats(&adapter)[0];
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);

        // Adding a custom font to `egui`
        let mut custom_fonts = egui::FontDefinitions::default();
        let font_data = include_bytes!("fonts/Monospac821 WGL4 BT Bold.ttf");
        custom_fonts.font_data.insert(
            "custom_monospace".to_owned(),
            egui::FontData::from_static(font_data),
        );
        custom_fonts.families.insert(
            egui::FontFamily::Name("MonospaceX".into()),
            vec!["custom_monospace".to_owned()],
        );

        // Platform setup for egui
        let platform =
            egui_winit_platform::Platform::new(egui_winit_platform::PlatformDescriptor {
                physical_width: size.width,
                physical_height: size.height,
                scale_factor: window.scale_factor(),
                font_definitions: custom_fonts,
                style: Default::default(),
            });

        // RenderPass setup for `egui`
        let egui_rpass = egui_wgpu_backend::RenderPass::new(&device, surface_format, 1);

        // Making the window transparent and unresponsive to mouse events
        let hwnd = window.hwnd() as winapi::shared::windef::HWND;
        unsafe {
            let style = winapi::um::winuser::GetWindowLongA(hwnd, winapi::um::winuser::GWL_EXSTYLE);
            winapi::um::winuser::SetWindowLongA(
                hwnd,
                winapi::um::winuser::GWL_EXSTYLE,
                style
                    | winapi::um::winuser::WS_EX_LAYERED as i32
                    | winapi::um::winuser::WS_EX_TRANSPARENT as i32,
            );
        }

        Self {
            window,
            event_loop,
            platform,
            surface,
            surface_config,
            device,
            queue,
            egui_rpass,
        }
    }
}
