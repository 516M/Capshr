//use tray_icon::{TrayIconBuilder, TrayIcon, Icon};
use image::{GenericImageView};
use rdev::{listen, Event, EventType, Key};

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

pub fn load_config() {
    todo!();
}

fn main() {
    //let _icon = icon_create();

    // TODO: Capture screen here
    // TODO: Load key config here
    println!("Listening for key presses. Press ESC to quit.");

    if let Err(error) = listen(callback) {
        eprintln!("Error: {:?}", error);
    }
}

fn callback(event: Event) {
    if let EventType::KeyPress(key) = event.event_type {
        println!("Key pressed: {:?}", key);

        match key {
            Key::Escape => {
                println!("ESC detected -- exiting.");
                std::process::exit(0);
            },
            Key::PrintScreen => {
                todo!()
            },
            _ => {}
        };
    }
}
