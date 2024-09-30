use std::fs;
use std::io::{self, Write};
use std::process::Command;
// 1year on this is my go to audio tool :)
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
        command.arg(format!("{}/%(title)s.%(ext)s", output_dir));
    }

    command.arg(youtube_link);

    let command_output = command.output().expect("Failed to execute yt-dlp command");

    if command_output.status.success() {
        println!("Download and conversion complete!");
    } else {
        eprintln!("Download and conversion failed!");
        if let Ok(stderr) = String::from_utf8(command_output.stderr) {
            eprintln!("yt-dlp error message:\n{}", stderr);
        }
    }
}

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

    download_and_convert_to_m4a(youtube_link, is_playlist, audio_quality);
}
// the end
