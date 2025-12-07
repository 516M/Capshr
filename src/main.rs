// TODO: Maybe break down the app to smaller parts
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::borrow::Cow;
use std::path::PathBuf;

use image::ImageBuffer;
use arboard::{ImageData, Clipboard};
//use clipboard_rs::{Clipboard, ClipboardContext, RustImageData};

//use tray_icon::{TrayIconBuilder, TrayIcon, Icon};
use image::RgbaImage;
use xcap::Monitor;
use eframe::{UserEvent, egui};
use egui_file_dialog::{DialogState, FileDialog, FileDialogConfig};
//use egui_file_dialog::*;
use egui::{Color32, Pos2, Rect, StrokeKind, Key};
//use egui::util::poll_promise::Promise;
use chrono::prelude::*;
//use rfd::FileDialog;
//use rfd::AsyncFileDialog;
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

struct App {
    // Responsible for keeping the file dialog open + filedialog access
    file_dialog: FileDialog,
    show_dialog:        bool,

    // The screenshot itself
    screenshot:         RgbaImage,

    // Used by x11rb to make the app fullscreen.
    restore_fullscreen: bool,

    // To determine if a region was already drawn.
    drawn_rect:         bool,

    // Used by the app to make a selection on the screenshot.
    selection_start:    Pos2,
    selection_end:      Pos2,
    selection:          Rect,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            file_dialog: App::create_save_dialog(),
            show_dialog: false,
            restore_fullscreen: false,
            drawn_rect: false,
            selection_start: Pos2::ZERO,
            selection_end: Pos2::ZERO,
            selection: Rect::ZERO,
            screenshot: App::capture_screenshot()
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
    pub fn capture_screenshot() -> RgbaImage {
    //    let start = Instant::now();
        let monitors = Monitor::all().unwrap();

        let primary_monitor = &monitors[0];
        let screenshot = primary_monitor.capture_image().unwrap();
        // TODO: Only print in Debug mode
        //println!("Capture took {:?} .", start.elapsed());

        screenshot
    }

    pub fn get_screenshot_region(&self, p1: Pos2, p2: Pos2, viewport_size: egui::Vec2) -> ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>> {
        let image_width  = self.screenshot.width() as f32;
        let image_height = self.screenshot.height() as f32;

        // Map egui coordinates to image coordinates
        let crop_min_x = (p1.x * (image_width / viewport_size.x)).round().max(0.0) as u32;
        let crop_min_y = (p1.y * (image_height / viewport_size.y)).round().max(0.0) as u32;
        let crop_max_x = (p2.x * (image_width / viewport_size.x)).round().min(image_width) as u32;
        let crop_max_y = (p2.y * (image_height / viewport_size.y)).round().min(image_height) as u32;

        // Checking where the starting point is and where the ending point is.
        // Could use some clean up and optimization.
        let mut point_x = crop_min_x;
        let mut width: u32 = match crop_max_x > crop_min_x {
            true => {
                (crop_max_x as i32 - crop_min_x as i32).abs() as u32
            },
            false => {
                point_x = crop_max_x;
                (crop_min_x as i32 - crop_max_x as i32).abs() as u32
            }
        };
        let mut point_y = crop_min_y;
        let mut height: u32 = match crop_max_y > crop_min_y {
            true => {
                (crop_max_y as i32 - crop_min_y as i32).abs() as u32
            },
            false => {
                point_y = crop_max_y;
                (crop_min_y as i32 - crop_max_y as i32).abs() as u32
            }
        };

        // If the width and the height are 0, just grab the whole screen
        if width == 0 && height == 0 {
            width = image_width as u32;
            height = image_height as u32;
        }

        let screenshot_region = image::imageops::crop_imm(&self.screenshot, point_x, point_y, width, height)
            .to_image();

        return screenshot_region;
    }

    pub fn create_save_dialog() -> FileDialog {
        // TODO: Point to Desktop by default -- but make this configurable.
        let home_dir = home::home_dir().unwrap_or(std::env::current_dir().unwrap());
        //let desktop_dir = home_dir.

        // TODO: Have a configurable format here.
        // TODO: Add a configurable default save location?
        // TODO: Implement formatting: screenshot-[date]-[name convention]
        // TODO: Implement it in a pattern/regex way--to allow editing for end users.
        let utc = Local::now();
        let appname = env!("CARGO_PKG_NAME").to_string();
        let utc_format = utc.format(&(
            "%Y-%m-%d_%H-%M_%S_".to_string()+
            &appname+
            ".png" // NOTE: Remember to change this as well, if changing config.
        )).to_string();

        let mut config = FileDialogConfig {
            default_file_name: utc_format,
            initial_directory: PathBuf::from(&home_dir),
            resizable: false,
            show_new_folder_button: true,
            show_search: true,
            show_path_edit_button: true,
            default_save_extension: Some("PNG files".to_string()),
            ..Default::default()
        };

        // TODO: Add support for other file types: jpg, gif, mp4..
        config = config
            .add_save_extension("PNG files", "png");
            //.add_save_extension("JPG files", "jpg");

        
        let fd = FileDialog::with_config(config.clone())
            .id("file-dialog-a")
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]);

        fd
    }

    // TODO: Check if successfully copied to clipboard?
    fn save_screenshot_clipboard(&self, screenshot_buffer: ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>> ) {
        // TODO: Add OS checks -- X11 handles things differently from windows, etc etc..

        // TODO (X11/xorg): Daemon mode  -- Support acting as a clipboard manager for X11.
        // If you run in daemon mode, things that are copied are preserved similar to
        // a clipboard manager.
        //      Remember that in X11, things are not copied until pasting is required --
        //      That means if you copy something from an application and quit it before pasting,
        //      attempts to paste will yield nothing (because the application holding 
        //      whatever you wanted to copy is gone).
        let mut clipboard = Clipboard::new().unwrap();
        let screenshot_data = ImageData {
            width: screenshot_buffer.width() as usize,
            height: screenshot_buffer.height() as usize,
            bytes: Cow::from(screenshot_buffer.as_raw()),
        };

        clipboard.set_image(screenshot_data).unwrap();
        
        // TODO: Configure copy time
        // A small delay to let content be copied to the clipboard.
        std::thread::sleep(std::time::Duration::from_millis(30));
    }

    pub fn save_screenshot_file(&self, screenshot_buffer: ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>, path: &std::path::Path) {
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("png")  // TODO: Config: remember to change the default format
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
            path.with_extension(ext);
        }

        screenshot_buffer
            .save_with_format(path, scr_format)
            .expect("Failed to save image");
    }
}

// TODO: Find a way to swallow events that attempt to quit fullscreen
// You can still exit fullscreen in i3, but you'll see it for a brief moment
// before it goes back to fullscreen.
impl eframe::App for App {
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
                    //let net_wm_bypass_compositor = 
                    //    conn.intern_atom(false, b"_NET_WM_BYPASS_COMPOSITOR").unwrap().reply().unwrap().atom;
                    let root = screen.root;

                    let event = ClientMessageEvent::new(
                        32,                      // format = 32-bit
                        window,                  // destination window
                        net_wm_state,            // message type
                        [
                            1,                      // _NET_WM_STATE_ADD
                            net_wm_state_fullscreen,
                            1,
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
                // NOTE: Fixed the window not being fullscreen for i3, but
                // we are keeping this here to see what happens with other
                // systems (e.g. windows, or another graphical environment.)
                if self.restore_fullscreen {
                    //_frame.set_decorated(false);
                    //ctx.send_viewport_cmd(ViewportCommand::Fullscreen(true));
                    self.restore_fullscreen = false;
                }
                //ctx.send_viewport_cmd(ViewportCommand::Visible(false));

                // Quit application
                if ctx.input(|i| i.key_pressed(Key::Escape) && !self.drawn_rect && !self.show_dialog) {
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
                if !self.show_dialog {
                    if ctx.input(|i| i.pointer.primary_down()) { // If left mouse was pressed down..
                        self.selection_start       = ui.input(|i| i.pointer.press_origin().unwrap_or(Pos2::ZERO));
                        self.selection_end         = ui.input(|i| i.pointer.hover_pos().unwrap_or(self.selection_start));
                        self.selection = Rect::from_two_pos(self.selection_start, self.selection_end);
                        self.drawn_rect = true;
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
                }

                // TODO: Change this shortcut via config file
                // Save screenshot as file
                if ctx.input(|i| i.modifiers.command && i.key_pressed(Key::S) && !self.show_dialog) {
                    //std::process::exit(0);
                    self.show_dialog = true;

                    self.file_dialog = App::create_save_dialog();
                    // Initiates the file dialog that will save the file
                    self.file_dialog.save_file();
                
                } else if ctx.input(|inp| inp.events.iter().any(|ev| matches!(ev, egui::Event::Copy)) && !self.show_dialog) { 
                // TODO: Egui swallows common shortcuts such as Ctrl+C, Ctrl+V, Ctrl+U, etc. See: egui::Event
                // TODO: That means that you won't be able to check for inputs like ctrl + C.
                // TODO: Change this shortcut via config file
                // TODO: Make sure that when checking for events like Ctrl+C, you check for common events (see above)
                // Copy screenshot to clipboard
                //if ctx.input(|i| i.modifiers.command && i.key_pressed(Key::C) && !self.show_dialog) {

                    // TODO: Configure whether to quit after copying to clipboard
                    // TODO: Copy to clipboard here
                    let screenshot_region = self.get_screenshot_region(self.selection_start, self.selection_end, viewport_size);
                    self.save_screenshot_clipboard(screenshot_region);
                    std::process::exit(0);
                }

                // Draw the "save file" dialog here
                if self.show_dialog {
                    if ctx.input(|i| i.key_pressed(Key::Escape)) {
                        self.show_dialog = false;
                    } else {
                        // Shows the dialog
                        self.file_dialog.update(ctx);

                        // Checks what the user picked after pressing OK
                        if let Some(path) = self.file_dialog.take_picked() {
                            //println!("Selected file: {:?}", path);
                            let screenshot_region = self.get_screenshot_region(self.selection_start, self.selection_end, viewport_size);
                            self.save_screenshot_file(screenshot_region, &path);
                            self.show_dialog = false;

                            // TODO: Configure whether to quit after saving
                            std::process::exit(0);
                        } else {

                            // Check for the dialog's current state
                            match self.file_dialog.state() {
                                DialogState::Closed | DialogState::Cancelled => {
                                    self.show_dialog = false;
                                },
                                DialogState::Open => {},
                                DialogState::Picked(_) => {},
                                DialogState::PickedMultiple(_) => {}
                            }
                        }
                        /*
                        let scr_region = App::get_scr_region(&self.screenshot, self.selection_start, self.selection_end, viewport_size);
                        let dialog_res = App::save_screenshot(ui, scr_region);
                        if dialog_res {
                            self.show_dialog = false;
                        }*/
                        //App::save_capture(ui, scr_region);
                    }
                } else if self.drawn_rect {
                    if ctx.input(|i| i.key_pressed(Key::Escape)) {
                        self.drawn_rect = false;
                        self.selection_start = Pos2::ZERO;
                        self.selection_end = Pos2::ZERO;
                        self.selection = Rect::ZERO;
                    } 
                }
        });
   }
}

fn main() -> Result<(), impl std::error::Error> { //eframe::Result {
    //let _icon = icon_create();
    
    // First "Capture" screen here
    //let screenshot: RgbaImage = capture_screenshot();

    // TODO: Load key config here
    //key_listen();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_visible(false)
            .with_decorations(false)
            .with_fullscreen(true)
            .with_window_level(egui::WindowLevel::Normal)
            .with_resizable(false),
        ..Default::default()
    };

    let eventloop = EventLoop::<UserEvent>::with_user_event().build()?;
    //let eventloop: EventLoop<UserEvent> = EventLoop::new().unwrap();
    //let mut parent_window_id = None;

    eventloop.set_control_flow(ControlFlow::Poll);
    let mut winit_app = eframe::create_native(
        env!("CARGO_PKG_NAME"), //TODO use project name
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
            Ok(Box::new(App::new(cc)
        ))}),
        &eventloop,
    );
    eventloop.run_app(&mut winit_app)
}
