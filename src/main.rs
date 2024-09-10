extern crate gtk;
use gtk::prelude::*;
use gtk::{Box, Button, ComboBoxText, Entry, Label, Orientation, Window, WindowType};
use std::cell::RefCell;
use std::fs;
use std::process::{Command, Output};
use std::rc::Rc;

fn download_and_convert_to_m4a(youtube_link: &str, is_playlist: bool, audio_quality: u32) {
    let output_dir = "out";
    if let Err(e) = std::fs::create_dir_all(output_dir) {
        eprintln!("Failed to create output directory: {}", e);
        return;
    }

    let mut command = Command::new("yt-dlp");
    command.args([
        "-x",
        "--audio-format",
        "m4a",
        "--audio-quality",
        &audio_quality.to_string(),
        "--add-metadata",
        "--metadata-from-title",
        "%(artist)s - %(title)s",
        "-o",
    ]);

    if is_playlist {
        let playlist_name = sanitize_directory_name("My Playlist"); // Dummy name for now
        let playlist_dir = format!("{}/{}", output_dir, playlist_name);

        if let Err(e) = fs::create_dir_all(&playlist_dir) {
            eprintln!("Failed to create playlist directory: {}", e);
            return;
        }

        command.args([
            &format!("{}/%(playlist_index)s - %(title)s.%(ext)s", playlist_dir),
            "--yes-playlist",
        ]);
    } else {
        command.arg(&format!("{}/%(title)s.%(ext)s", output_dir));
    }

    match command.output() {
        Ok(output) => handle_command_output(output),
        Err(e) => eprintln!("Failed to execute yt-dlp command: {}", e),
    }
}

fn handle_command_output(output: Output) {
    if output.status.success() {
        println!("Download and conversion complete!");
    } else {
        eprintln!("Download and conversion failed!");
        if let Ok(stderr) = String::from_utf8(output.stderr) {
            eprintln!("yt-dlp error message:\n{}", stderr);
        }
    }
}

fn sanitize_directory_name(name: &str) -> String {
    let invalid_chars = ['\\', '/', ':', '*', '?', '"', '<', '>', '|'];
    name.chars()
        .filter(|&c| !invalid_chars.contains(&c))
        .collect::<String>()
        .trim()
        .to_owned()
}

fn main() {
    if gtk::init().is_err() {
        eprintln!("Failed to initialize GTK.");
        return;
    }

    // Create the main window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("YouTube to M4A Downloader");
    window.set_default_size(350, 150);

    // Create a vertical box to hold all widgets
    let vbox = Box::new(Orientation::Vertical, 10);

    // YouTube link input
    let label_link = Label::new(Some("Enter YouTube link or playlist link:"));
    let entry_link = Rc::new(RefCell::new(Entry::new()));

    // Playlist toggle input (ComboBox)
    let label_playlist = Label::new(Some("Is this a playlist?"));
    let playlist_combo = Rc::new(RefCell::new(ComboBoxText::new()));
    playlist_combo.borrow().append_text("No");
    playlist_combo.borrow().append_text("Yes");
    playlist_combo.borrow().set_active(Some(0));

    // Audio quality input
    let label_quality = Label::new(Some("Select audio quality (0 - best, 9 - worst):"));
    let entry_quality = Rc::new(RefCell::new(Entry::new()));
    entry_quality.borrow().set_text("0");

    // Button to start the download
    let button = Button::with_label("Download and Convert");

    // Layout
    vbox.pack_start(&label_link, false, false, 0);
    vbox.pack_start(&*entry_link.borrow(), false, false, 0);
    vbox.pack_start(&label_playlist, false, false, 0);
    vbox.pack_start(&*playlist_combo.borrow(), false, false, 0);
    vbox.pack_start(&label_quality, false, false, 0);
    vbox.pack_start(&*entry_quality.borrow(), false, false, 0);
    vbox.pack_start(&button, false, false, 0);

    // Add vbox to the window
    window.add(&vbox);

    // Button click event
    let entry_link_clone = Rc::clone(&entry_link);
    let playlist_combo_clone = Rc::clone(&playlist_combo);
    let entry_quality_clone = Rc::clone(&entry_quality);

    button.connect_clicked(move |_| {
        let youtube_link = entry_link_clone.borrow().text().to_string();
        let is_playlist = playlist_combo_clone
            .borrow()
            .active_text()
            .unwrap()
            .to_string()
            == "Yes";
        let audio_quality: u32 = entry_quality_clone.borrow().text().parse().unwrap_or(0);

        if !youtube_link.is_empty() {
            println!("Downloading and converting: {}", youtube_link);
            download_and_convert_to_m4a(&youtube_link, is_playlist, audio_quality);
        } else {
            println!("Please enter a valid YouTube link.");
        }
    });

    // Show all widgets
    window.show_all();

    // Close the application when the window is closed
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
