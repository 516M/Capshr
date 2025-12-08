# Capshr | A screenshot utility | Rust

## Usage

Capshr works by immediately capturing the whole screen and then allowing you to select the region that you want to save.

Use Ctrl+S to save your selection as an image, and Ctrl+C to copy the selection into the clipboard. If there is no selection, the whole screen is used.

### TODOs

* <u>README.md (this file)</u>:
- [ ] Insert some demonstration (can be a gif; can be a video, etc..)
- [ ] Insert some examples.
- [ ] Add more sections

* <u>Development:</u>
- [x] Clippy: Skim around and check for potential pitfalls/mistakes.
- [ ] Break down the app to smaller parts and files.

* <u>Release:</u>
- [ ] Argument that runs Capshr in daemon mode (also for X11) in the background -- Run in the background and listen for inputs. e.g. `capshr -d` to run in the background.
- [ ] System tray icon (had a headache with this one)
- [ ] Allow Capshr to capture region by specifying it via CLI -- e.g. `capshr -c -x 320 -y 240 -w 300 -h 300` ... etc
- [ ] Gif/Video recording options. e.g. `capshr -c -g gif` (-c: capture, -g: gui that you can drag around to determine where you want to record on screen)
- [ ] Editing: Basic drawing tools/creating "pointer" lines/text writing/adding captions(whether at the top or bottom)

## License

See the [LICENSE.md](LICENSE.md) file.