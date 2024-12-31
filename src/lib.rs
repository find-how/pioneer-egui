use std::borrow::Cow;
use deno_core::{Extension, JsRuntime, RuntimeOptions};
use egui::{CentralPanel, Slider};
use egui_wgpu::renderer::ScreenDescriptor;
use egui_winit::State as EguiWinitState;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel};
use tokio_tungstenite::{tungstenite::protocol::Message, accept_async};
use futures_util::{StreamExt, SinkExt};
use winit::{
    event::{Event, WindowEvent, VirtualKeyCode, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub mod events;
pub mod ops;
pub mod state_3d;

// ----- Arg Structs -----
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetLabelArgs {
    pub text: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetSliderArgs {
    pub value: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetInputArgs {
    pub text: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetCheckboxArgs {
    pub id: String,
    pub checked: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetComboBoxArgs {
    pub id: String,
    pub selected: String,
    pub options: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetRadioArgs {
    pub id: String,
    pub selected: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetProgressArgs {
    pub id: String,
    pub value: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Rotate3DArgs {
    pub angle: f32,
}

// ----- App State -----
pub struct EguiApp {
    pub label_text: String,
    pub slider_value: f32,
    pub input_text: String,
    pub checkboxes: std::collections::HashMap<String, bool>,
    pub combo_boxes: std::collections::HashMap<String, (String, Vec<String>)>,
    pub radio_groups: std::collections::HashMap<String, String>,
    pub progress_bars: std::collections::HashMap<String, f32>,
    pub rotation: f32,
    pub is_recording: bool,
    pub recorded_events: Vec<events::RecordedEvent>,
    pub recording_start: Option<std::time::Instant>,
    pub is_playing: bool,
    pub playback_index: usize,
    pub playback_start: Option<std::time::Instant>,
    pub event_sender: UnboundedSender<String>,
}

impl Default for EguiApp {
    fn default() -> Self {
        let (tx, _) = unbounded_channel();
        Self {
            label_text: String::new(),
            slider_value: 0.0,
            input_text: String::new(),
            checkboxes: std::collections::HashMap::new(),
            combo_boxes: std::collections::HashMap::new(),
            radio_groups: std::collections::HashMap::new(),
            progress_bars: std::collections::HashMap::new(),
            rotation: 0.0,
            is_recording: false,
            recorded_events: Vec::new(),
            recording_start: None,
            is_playing: false,
            playback_index: 0,
            playback_start: None,
            event_sender: tx,
        }
    }
}

// ----- Extension Creation -----
pub fn init_ext(sender: UnboundedSender<String>) -> Extension {
    let app = Arc::new(Mutex::new(EguiApp {
        event_sender: sender,
        ..Default::default()
    }));

    Extension {
        name: "pioneer-egui",
        ops: ops::init_ops().into(),
        op_state_fn: Some(Box::new(move |state: &mut deno_core::OpState| {
            state.put(app.clone());
        })),
        ..Default::default()
    }
}

// ---- WEBSOCKET SERVER ----
async fn start_ws_server(tx: UnboundedSender<String>) {
    let addr = "127.0.0.1:9001";
    let try_socket = tokio::net::TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind WebSocket server");
    println!("WebSocket server listening on ws://{}", addr);

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("Incoming TCP connection from: {}", addr);
                let tx = tx.clone();
                tokio::spawn(async move {
                    let ws_stream = match tokio_tungstenite::accept_async(stream).await {
                        Ok(ws_stream) => {
                            println!("WebSocket connection established with: {}", addr);
                            ws_stream
                        }
                        Err(e) => {
                            eprintln!("Error during WebSocket handshake: {}", e);
                            return;
                        }
                    };

                    let (mut write, mut read) = ws_stream.split();

                    // Send a welcome message
                    match write.send(Message::Text("Connected to Pioneer eGUI".to_string())).await {
                        Ok(_) => println!("Welcome message sent to: {}", addr),
                        Err(e) => {
                            eprintln!("Failed to send welcome message: {}", e);
                            return;
                        }
                    }

                    while let Some(message) = read.next().await {
                        match message {
                            Ok(Message::Text(text)) => {
                                println!("Received message from {}: {}", addr, text);
                                if let Err(e) = tx.send(text) {
                                    eprintln!("Failed to forward message: {}", e);
                                    break;
                                }
                            }
                            Ok(Message::Close(_)) => {
                                println!("WebSocket connection closed by client: {}", addr);
                                break;
                            }
                            Err(e) => {
                                eprintln!("WebSocket error from {}: {}", addr, e);
                                break;
                            }
                            _ => {}
                        }
                    }
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }
}

// ---- MAIN RUNTIME ----
pub fn main() {
    println!("Starting Pioneer eGUI Timeline Example");

    // Channel to forward events to Deno
    let (tx, rx) = unbounded_channel();
    println!("Created event channels");

    // Try to kill any existing process using port 9001
    let _ = std::process::Command::new("lsof")
        .args(["-t", "-i:9001"])
        .output()
        .ok()
        .and_then(|output| {
            if !output.stdout.is_empty() {
                let pid = String::from_utf8_lossy(&output.stdout);
                println!("Found existing process on port 9001 (PID: {}), attempting to kill it", pid.trim());
                std::process::Command::new("kill")
                    .arg(pid.trim())
                    .output()
                    .ok()
            } else {
                None
            }
        });

    // Create a separate thread for the WebSocket server
    let tx_ws = tx.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            println!("Starting WebSocket server...");
            start_ws_server(tx_ws).await;
        });
    });

    // Give the WebSocket server a moment to start
    std::thread::sleep(Duration::from_millis(100));
    println!("WebSocket server started and ready for connections");

    // Run the eGUI runtime on the main thread
    println!("Starting eGUI runtime on main thread...");
    run_egui_runtime(rx);
}

pub fn run_egui_runtime(mut rx: UnboundedReceiver<String>) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Pioneer eGUI Timeline Example")
        .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 768.0))
        .build(&event_loop)
        .unwrap();

    // Initialize wgpu for 3D
    let mut state_3d = pollster::block_on(state_3d::State3D::new(&window));

    // Initialize eGUI
    let egui_ctx = egui::Context::default();
    let mut egui_renderer = egui_wgpu::Renderer::new(
        &state_3d.device,
        state_3d.config.format,
        None,
        1,
    );

    // Create app state
    let app = Arc::new(Mutex::new(EguiApp::default()));

    // Deno runtime + extension
    let (tx_ws, _rx_ws) = unbounded_channel();
    let ext = init_ext(tx_ws.clone());
    let mut js_runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![ext],
        ..Default::default()
    });

    let mut egui_state = EguiWinitState::new(&event_loop);
    let mut last_update = Instant::now();

    // The winit event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        // Process any pending events from Deno
        if let Ok(event) = rx.try_recv() {
            println!("Received event from Deno: {}", event);
        }

        match event {
            Event::WindowEvent { event, .. } => {
                if !state_3d.input(&event) {
                    match event {
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }
                        WindowEvent::Resized(new_size) => {
                            state_3d.resize(new_size);
                        }
                        WindowEvent::ScaleFactorChanged { ref new_inner_size, .. } => {
                            state_3d.resize(**new_inner_size);
                        }
                        WindowEvent::KeyboardInput { input, .. } => {
                            if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                                if input.state == ElementState::Pressed {
                                    *control_flow = ControlFlow::Exit;
                                }
                            }
                        }
                        _ => {}
                    }
                }

                // Handle egui input
                let _ = egui_state.on_event(&egui_ctx, &event);
            }
            Event::RedrawRequested(_) => {
                // Limit frame rate
                let now = Instant::now();
                let elapsed = now.duration_since(last_update);
                if elapsed < Duration::from_millis(16) {
                    return;
                }
                last_update = now;

                // eGUI pass
                let mut app = app.lock().unwrap();
                let raw_input = egui_state.take_egui_input(&window);
                egui_ctx.begin_frame(raw_input);

                CentralPanel::default().show(&egui_ctx, |ui| {
                    ui.heading("Pioneer eGUI Timeline Example");
                    ui.label(&app.label_text);

                    // Additional UI controls
                    ui.horizontal(|ui| {
                        if ui.button("Start Recording").clicked() {
                            app.is_recording = true;
                            app.recorded_events.clear();
                            app.recording_start = Some(Instant::now());
                        }
                        if ui.button("Stop Recording").clicked() {
                            app.is_recording = false;
                            app.recording_start = None;
                        }
                        if ui.button("Start Playback").clicked() {
                            app.is_playing = true;
                            app.playback_index = 0;
                            app.playback_start = Some(Instant::now());
                        }
                        if ui.button("Stop Playback").clicked() {
                            app.is_playing = false;
                            app.playback_index = 0;
                            app.playback_start = None;
                        }
                    });

                    ui.add(Slider::new(&mut app.slider_value, 0.0..=100.0).text("Value"));
                    ui.text_edit_singleline(&mut app.input_text);
                });

                let output = egui_ctx.end_frame();
                let paint_jobs = egui_ctx.tessellate(output.shapes);

                // Get the next frame
                let frame = match state_3d.surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(wgpu::SurfaceError::Lost) => {
                        state_3d.resize(state_3d.size);
                        return;
                    }
                    Err(e) => {
                        eprintln!("Failed to acquire next swap chain texture: {:?}", e);
                        return;
                    }
                };

                let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

                // Create encoder
                let mut encoder = state_3d.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

                // Clear pass
                {
                    let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Clear Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                    });
                }

                // Update 3D state
                state_3d.update_uniforms(app.rotation);

                // 3D pass
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("3D Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                    });

                    rpass.set_pipeline(&state_3d.render_pipeline);
                    rpass.set_bind_group(0, &state_3d.uniform_bind_group, &[]);
                    rpass.set_vertex_buffer(0, state_3d.vertex_buffer.slice(..));
                    rpass.set_index_buffer(state_3d.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    rpass.draw_indexed(0..state_3d.num_indices, 0, 0..1);
                }

                // Render eGUI
                let screen_descriptor = ScreenDescriptor {
                    size_in_pixels: [state_3d.size.width, state_3d.size.height],
                    pixels_per_point: window.scale_factor() as f32,
                };

                for (id, image_delta) in &output.textures_delta.set {
                    egui_renderer.update_texture(
                        &state_3d.device,
                        &state_3d.queue,
                        *id,
                        image_delta,
                    );
                }

                egui_renderer.update_buffers(
                    &state_3d.device,
                    &state_3d.queue,
                    &mut encoder,
                    &paint_jobs,
                    &screen_descriptor,
                );

                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("eGUI Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                    });

                    egui_renderer.render(&mut rpass, &paint_jobs, &screen_descriptor);
                }

                // Submit and present
                state_3d.queue.submit(std::iter::once(encoder.finish()));
                frame.present();

                // Step the Deno runtime
                pollster::block_on(js_runtime.run_event_loop(Default::default())).unwrap();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
