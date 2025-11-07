# CapScr | A screenshot utility | Rust

### Usage

CapScr works by immediately capturing the whole screen and then allowing you to select the region that you want to capture/edit. 

If you run it normally, it will immediately grab the screen. However, you can run it in daemon mode, where it will essentially wait for you to press print screen before it captures the screen.

There are some shortcuts that you can use to quickly utilize it, such as Ctrl+S to save and Ctrl+C (which you can edit in the application's settings).

### TODOs

[] System tray icon (had a headache with this one)
[] Argument that runs capscr as a daemon in the background
[] Look at how older applications can immediately capture regions just by specifying width, height and coordinates (?)
[] Gif/Video recording options
[] Editing: Basic drawing tools/creating "pointer" lines/text writing/adding captions(whether at the top or bottom)