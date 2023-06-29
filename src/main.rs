use std::fs;
use std::io::{self, Write};
use std::process::Command;

fn download_and_convert_to_m4a(youtube_link: &str, is_playlist: bool) {
    let output_dir = "out";
    std::fs::create_dir_all(output_dir).expect("Failed to create output directory");

    let mut command = Command::new("yt-dlp");
    command.args([
        "-x",
        "--audio-format",
        "m4a",
        "--audio-quality",
        "0",
        "--add-metadata",
        "--metadata-from-title",
        "%(artist)s - %(title)s",
        "-o",
    ]);

    if is_playlist {
        command.args([
            &format!("{}/%(playlist_index)s - %(title)s.%(ext)s", output_dir),
            "--yes-playlist",
        ]);
    } else {
        command.arg(&format!("{}/%(title)s.%(ext)s", output_dir));
    }

    command.arg(youtube_link);

    let command_output = command.output().expect("Failed to execute yt-dlp command");

    if command_output.status.success() {
        println!("Download and conversion complete!");

        if is_playlist {
            // Extract playlist name
            let playlist_name = extract_playlist_name(youtube_link);
            let playlist_dir = format!("{}/{}", output_dir, playlist_name);

            // Create the playlist directory
            fs::create_dir(&playlist_dir).expect("Failed to create playlist directory");

            // Move downloaded files into the playlist directory and update metadata
            move_files_to_directory(&playlist_dir);
        }
    } else {
        eprintln!("Download and conversion failed!");
        if let Ok(stderr) = String::from_utf8(command_output.stderr) {
            eprintln!("yt-dlp error message:\n{}", stderr);
        }
    }
}

fn extract_playlist_name(youtube_link: &str) -> String {
    let playlist_info_command = Command::new("yt-dlp")
        .args(["--get-title", "--no-playlist", youtube_link])
        .output()
        .expect("Failed to execute yt-dlp command");

    if playlist_info_command.status.success() {
        let playlist_name = String::from_utf8_lossy(&playlist_info_command.stdout);
        return sanitize_directory_name(&playlist_name);
    }

    "Unknown Playlist".to_owned()
}

fn sanitize_directory_name(name: &str) -> String {
    let invalid_chars = ['\\', '/', ':', '*', '?', '"', '<', '>', '|'];
    let sanitized_name = name
        .chars()
        .filter(|&c| !invalid_chars.contains(&c))
        .collect::<String>();
    sanitized_name.trim().to_owned()
}

fn move_files_to_directory(target_directory: &str) {
    let output_dir = "out";

    let files = fs::read_dir(output_dir)
        .expect("Failed to read output directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|t| t.is_file()).unwrap_or(false));

    for file in files {
        let file_name = file.file_name();
        let file_path = file.path();

        let new_file_path = format!("{}/{}", target_directory, file_name.to_string_lossy());

        fs::rename(&file_path, &new_file_path)
            .unwrap_or_else(|_| eprintln!("Failed to move file: {:?}", file_path));
    }
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

    download_and_convert_to_m4a(youtube_link, is_playlist);
}
