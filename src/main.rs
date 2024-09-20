use std::fs;
use std::io::{self, Write};
use std::process::Command;

// Function to embed metadata using ffmpeg
fn embed_metadata(file_path: &str, artist: &str, track: &str, album: &str) {
    let mut command = Command::new("ffmpeg");
    command.args([
        "-i",
        file_path,
        "-metadata",
        &format!("artist={}", artist),
        "-metadata",
        &format!("title={}", track),
        "-metadata",
        &format!("album={}", album),
        "-codec",
        "copy",                                      // Don't re-encode, just copy the file
        &format!("{}_with_metadata.m4a", file_path), // Save to a new file with embedded metadata
    ]);

    let output = command.output().expect("Failed to execute ffmpeg command");

    if output.status.success() {
        println!("Metadata embedded successfully into {}!", file_path);
    } else {
        eprintln!("Embedding metadata failed!");
        if let Ok(stderr) = String::from_utf8(output.stderr) {
            eprintln!("ffmpeg error message:\n{}", stderr);
        }
    }
}

// Function to download and convert YouTube audio to m4a
fn download_and_convert_to_m4a(youtube_link: &str, is_playlist: bool, audio_quality: u32) {
    let output_dir = "out";
    std::fs::create_dir_all(output_dir).expect("Failed to create output directory");

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
        // Ask for the playlist name
        print!("Enter the playlist name: ");
        io::stdout().flush().unwrap();
        let mut playlist_name = String::new();
        io::stdin()
            .read_line(&mut playlist_name)
            .expect("Failed to read input");

        let playlist_name = sanitize_directory_name(playlist_name.trim());

        let playlist_dir = format!("{}/{}", output_dir, playlist_name);
        fs::create_dir_all(&playlist_dir).expect("Failed to create playlist directory");

        // Update the command arguments with playlist name appended to file names
        command.args([
            &format!("{}/%(playlist_index)s - %(title)s.%(ext)s", playlist_dir),
            "--yes-playlist",
        ]);
    } else {
        command.arg(&format!("{}/%(title)s.%(ext)s", output_dir));
    }

    command.arg(youtube_link);

    let command_output = command.output().expect("Failed to execute yt-dlp command");

    if command_output.status.success() {
        println!("Download and conversion complete!");

        // Extract the output file path and auto-generate metadata for embedding
        let downloaded_file = format!("{}/%(title)s.m4a", output_dir);
        let artist = "Default Artist"; // Replace with metadata retrieval logic if needed
        let track = "Default Track";
        let album = "Default Album";

        // Embed metadata after downloading
        embed_metadata(&downloaded_file, artist, track, album);
    } else {
        eprintln!("Download and conversion failed!");
        if let Ok(stderr) = String::from_utf8(command_output.stderr) {
            eprintln!("yt-dlp error message:\n{}", stderr);
        }
    }
}

// Function to sanitize the playlist or file name by removing invalid characters
fn sanitize_directory_name(name: &str) -> String {
    let invalid_chars = ['\\', '/', ':', '*', '?', '"', '<', '>', '|'];
    let sanitized_name = name
        .chars()
        .filter(|&c| !invalid_chars.contains(&c))
        .collect::<String>();
    sanitized_name.trim().to_owned()
}

fn main() {
    let mut youtube_link = String::new();

    print!("Enter the YouTube link or playlist link: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut youtube_link)
        .expect("Failed to read input");

    let youtube_link = youtube_link.trim();

    let is_playlist = youtube_link.contains("playlist");

    // Ask for audio quality
    let mut audio_quality_input = String::new();
    print!("Enter audio quality (0 - best, 9 - worst): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut audio_quality_input)
        .expect("Failed to read input");

    let audio_quality: u32 = audio_quality_input.trim().parse().unwrap_or(0);

    // Download and convert to m4a, then embed metadata
    download_and_convert_to_m4a(youtube_link, is_playlist, audio_quality);
}
