// TODO: Implement all of this in `app.rs`
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


//use tray_icon::{TrayIconBuilder, TrayIcon, Icon};
use image::RgbaImage;
//use std::time::Instant;
use xcap::Monitor;
use eframe::{UserEvent, egui};
use egui::{Color32, Pos2, Rect, StrokeKind, Key};
use chrono::prelude::*;
use rfd::FileDialog;
use winit::{
//    application::ApplicationHandler,
//    window::Window,
    raw_window_handle::{RawWindowHandle, HasWindowHandle},
//    dpi::{LogicalPosition, LogicalSize, Position},
//    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop}, //, ActiveEventLoop},
};


use x11rb::{
    connection::Connection,
    protocol::xproto::{
            ConnectionExt, EventMask, ClientMessageEvent
        },
};

struct CapScrApp {
    screenshot:         RgbaImage,
    restore_fullscreen: bool,
    selection_start:    Pos2,
    selection_end:      Pos2,
    selection:          Rect,
}

impl CapScrApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            restore_fullscreen: false,
            selection_start: Pos2::ZERO,
            selection_end: Pos2::ZERO,
            selection: Rect::ZERO,
            screenshot: CapScrApp::scr_capture()
        }
    }
    // Yeah, I had no luck with creating a system tray icon.
    //pub fn icon_create() -> TrayIcon {
    //    let img = image::open("icon.png").expect("Failed to open icon file");
    //
    //    // Convert it into RGBA8 pixel data
    //    let (width, height) = img.dimensions();
    //    let rgba = img.into_rgba8().into_raw();
    //
    //    // Create a tray_icon::Icon
    //    let icon = Icon::from_rgba(rgba, width, height)
    //        .expect("Failed to create icon");
    //
    //    let tray_icon = TrayIconBuilder::new()
    //        .with_icon(icon)
    //        .with_tooltip("Rust Tray App")
    //        .build()
    //        .unwrap();
    //
    //    tray_icon
    //}
    pub fn _config_load() {
        todo!();
    }

    /*pub fn is_fullscreen(_conn: &RustConnection, window: u32) {

        let (conn, screen_num) = x11rb::connect(None).unwrap();
        let screen = &conn.setup().roots[screen_num];

        // Get atoms
        let atom_fullscreen = conn
            .intern_atom(false, b"_NET_WM_STATE_FULLSCREEN")
            .unwrap()
            .reply()
            .unwrap()
            .atom;

        let atom_net_wm_state = conn
            .intern_atom(false, b"_NET_WM_STATE")
            .unwrap()
            .reply()
            .unwrap()
            .atom;

        // NOTE: The turbofish annotation is required here
        let reply = conn
            .get_property::<RustConnection>(
                false,
                window,
                atom_net_wm_state,
                AtomEnum::ATOM.into(),
                0,
                1024,
            )
            .unwrap()
            .reply()
            .unwrap();

        // Check if fullscreen atom is present
        if let Some(iter) = reply.value32() {
            iter.any(|v| v == atom_fullscreen)
        } else {
            false
        }
        
    }*/


    //pub fn key_listen() {
    //    println!("Listening for key presses. Press ESC to quit.");
    //
    //    if let Err(error) = listen(callback) {
    //        eprintln!("Error: {:?}", error);
    //    }
    //}
    //
    //fn callback(event: Event) {
    //    if let EventType::KeyPress(key) = event.event_type {
    //        println!("Key pressed: {:?}", key);
    //
    //        match key {
    //            Key::Escape => {
    //                println!("ESC detected -- exiting.");
    //                std::process::exit(0);
    //            },
    //            Key::PrintScreen => {
    //                todo!()
    //            },
    //            _ => {}
    //        };
    //    }
    //}

    //fn str_normalized(filename: String) -> String {
    //    filename.replace(['|', '\\', ':', '/'], "")
    //}

    // TODO: Allow for monitor specific capturing (?)
    // TODO: Allow for capturing from all monitors
    pub fn scr_capture() -> RgbaImage {
    //    let start = Instant::now();
        let monitors = Monitor::all().unwrap();

        let primary_monitor = &monitors[0];
        let screenshot = primary_monitor.capture_image().unwrap();
        // TODO: Only print in Debug mode
        //println!("Capture took {:?} .", start.elapsed());

        screenshot
    }

    pub fn save_scr_region(screenshot: &RgbaImage, p1: Pos2, p2: Pos2, viewport_size: egui::Vec2) {
        let image_width  = screenshot.width() as f32;
        let image_height = screenshot.height() as f32;

        // Map egui coordinates to image coordinates
        let crop_min_x = (p1.x * (image_width / viewport_size.x)).round().max(0.0) as u32;
        let crop_min_y = (p1.y * (image_height / viewport_size.y)).round().max(0.0) as u32;
        let crop_max_x = (p2.x * (image_width / viewport_size.x)).round().min(image_width) as u32;
        let crop_max_y = (p2.y * (image_height / viewport_size.y)).round().min(image_height) as u32;

        // Attempt to subtract with overflow: can't go negative with u32 lol
        let mut width  = crop_max_x - crop_min_x;
        let mut height = crop_max_y - crop_min_y;
        if width == 0 && height == 0 {
            width = image_width as u32;
            height = image_height as u32;
        }

        let screenshot_region = image::imageops::crop_imm(screenshot, crop_min_x, crop_min_y, width, height).to_image();
        CapScrApp::save_capture(screenshot_region);
    }

    // TODO: Add a default save location?
    // TODO: Implement formatting: screenshot-[date]-[name convention]
    // TODO: Implement it in a pattern/regex way--to allow editing for end users.
    pub fn save_capture(screenshot: RgbaImage) {
        //let utc: DateTime<Utc> = Utc::now();
        let utc = Local::now();
        let utc_format = utc.format("%Y-%m-%d_%H-%M_%S_capscr.png").to_string();

        let home_dir = home::home_dir().unwrap_or(std::env::current_dir().unwrap());
        if let Some(mut path) = FileDialog::new()
            .set_directory(&home_dir)
            .set_title("Save screenshot as...")
            .add_filter("PNG image", &["png"])
            .add_filter("JPEG image", &["jpg", "jpeg"])
            .set_file_name(&utc_format)
            .save_file() {
                let ext = path
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("png")  // default format: png
                    .to_string();

                let scr_format = match ext.to_lowercase().as_str() {
                    "png" => Some(image::ImageFormat::Png),
                    "jpg" | "jpeg" => Some(image::ImageFormat::Jpeg),
                    "bmp" => Some(image::ImageFormat::Bmp),
                    "tiff" => Some(image::ImageFormat::Tiff),
                    "gif" => Some(image::ImageFormat::Gif),
                    _ => None,
                }.unwrap_or(image::ImageFormat::Png);

                if path.extension().is_none() {
                    path.set_extension(ext);
                }

                screenshot
                    .save_with_format(path, scr_format)
                    .expect("Failed to save image");
        }
    }
}

// TODO: Prevent un-fullscreening
impl eframe::App for CapScrApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        if let Ok(window_handle) = frame.window_handle() {
            let raw = window_handle.as_ref();

            match raw {
                // X11: handle fullscreen
                RawWindowHandle::Xlib(h) => {
                    let window: u32 = (h.window & 0xFFFF_FFFF) as u32;
                    let (conn, screen_num) = x11rb::connect(None).unwrap();
                    let screen = &conn.setup().roots[screen_num];
                    let net_wm_state = 
                        conn.intern_atom(false, b"_NET_WM_STATE").unwrap().reply().unwrap().atom;
                    let net_wm_state_fullscreen = 
                        conn.intern_atom(false, b"_NET_WM_STATE_FULLSCREEN").unwrap().reply().unwrap().atom;
                    let root = screen.root;

                    let event = ClientMessageEvent::new(
                        32,                      // format = 32-bit
                        window,                  // destination window
                        net_wm_state,            // message type
                        [
                            1,                      // _NET_WM_STATE_ADD
                            net_wm_state_fullscreen,
                            0,
                            0,
                            0,
                        ],
                    );

                    let _ = conn.send_event(
                        false,
                        root,
                        EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY,
                        event,
                    );

                    let _ = conn.flush();
                },
                _ => {}
                /*
                RawWindowHandle::Win32(h) => {
                    println!("Win32 handle:   {:?}", h.hwnd);
                }

                RawWindowHandle::Wayland(h) => {
                    println!("Wayland handle: {:?}", h.surface);
                }
                RawWindowHandle::AppKit(h) => {
                    println!("macOS handle:   {:?}", h.ns_view);
                }
                other => {
                    println!("Other raw handle: {:?}", other);
                }
                */
            }
        }

        let scr_color = egui::ColorImage::from_rgba_unmultiplied(
            [self.screenshot.width() as usize, self.screenshot.height() as usize],
            self.screenshot.as_raw(),
        );

        let scr_texture = ctx.load_texture(
            "screenshot",
            scr_color.clone(),
            egui::TextureOptions::NEAREST
        );
        let viewport_rect   = ctx.input(|i| i.viewport_rect());
        let viewport_size   = ctx.input(|i| i.viewport_rect().size()); //content_rect() ?    
        let scr_size        = scr_texture.size_vec2();
        let sized_texture   = egui::load::SizedTexture::new(&scr_texture, scr_size);

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                // TODO: Window gets re-tiled under i3, how do we fix it lol
                // 1. ask i3 to re-tile it (with dirty low level winit code)
                // 2. edit i3 itself? meh
                // 3. use native file dialog -- avoids triggering i3's retile
                if self.restore_fullscreen {
                    //_frame.set_decorated(false);
                    //ctx.send_viewport_cmd(ViewportCommand::Fullscreen(true));
                    self.restore_fullscreen = false;
                }
                //ctx.send_viewport_cmd(ViewportCommand::Visible(false));

                // Quit application
                if ctx.input(|i| i.key_down(Key::Escape)) {
                    std::process::exit(0);
                }

                //ui.image(&scr_texture);
                ui.add(egui::Image::new(sized_texture).fit_to_exact_size(viewport_size));

                let painter = ui.painter();
                // TODO: Allow end-users to change the overlay value
                let overlay_color = Color32::from_rgba_unmultiplied(0, 0, 0, 120);

                // Draw the overlay over the whole screen
                painter.rect_filled(viewport_rect, 0.0, overlay_color);

                // Selection rectangle (e.g. based on mouse drag)
                if ctx.input(|i| i.pointer.primary_down()) { // If left mouse was pressed down..
                    self.selection_start       = ui.input(|i| i.pointer.press_origin().unwrap_or(Pos2::ZERO));
                    self.selection_end         = ui.input(|i| i.pointer.hover_pos().unwrap_or(self.selection_start));
                    self.selection = Rect::from_two_pos(self.selection_start, self.selection_end);
                }

                if self.selection != Rect::ZERO {
                    // Convert crop_rect (pixels) into UV coordinates (0.0..1.0)
                    let uv_rect = Rect::from_min_max(
                        Pos2::new(self.selection.min.x / viewport_size.x, self.selection.min.y / viewport_size.y),
                        Pos2::new(self.selection.max.x / viewport_size.x, self.selection.max.y / viewport_size.y),
                    );

                    // Clear the dark overlay (draws cropped portion of the original image over)
                    painter.add(egui::Shape::image(
                        scr_texture.id(),
                        self.selection,
                        uv_rect,
                        Color32::WHITE,
                    ));
                    painter.rect_stroke(self.selection, 0.0, (1.0, Color32::PURPLE), StrokeKind::Outside);      
                }

                // TODO: Change this shortcut via config file
                if ctx.input(|i| i.modifiers.command && i.key_down(Key::S)) {
                    // TODO: Open filedialog here
                    CapScrApp::save_scr_region(&self.screenshot, self.selection_start, self.selection_end, viewport_size);
                    //restore_fullscreen = true;
                    //std::process::exit(0);
                }
        });
   }
}

fn main() -> Result<(), impl std::error::Error> { //eframe::Result {
    //let _icon = icon_create();
    
    // First "Capture" screen here
    //let screenshot: RgbaImage = scr_capture();

    // TODO: Load key config here
    //key_listen();

    let native_options = eframe::NativeOptions {
        //persist_window: true,
        viewport: egui::ViewportBuilder::default()
            .with_visible(false)
            .with_fullscreen(true)
            .with_resizable(false),
        ..Default::default()
    };

    let eventloop = EventLoop::<UserEvent>::with_user_event().build()?;
    //let eventloop: EventLoop<UserEvent> = EventLoop::new().unwrap();
    //let mut parent_window_id = None;

    eventloop.set_control_flow(ControlFlow::Poll);
    let mut winit_app = eframe::create_native(
        "capscr", //TODO use project name
        native_options,
        Box::new(|cc| Ok(Box::new(CapScrApp::new(cc)))),
        &eventloop,
    );
    eventloop.run_app(&mut winit_app)
}
