use std::fs;
use std::process::Command;
//now its very cozy :)
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
        // Use Rofi to ask for the playlist name
        let playlist_name = get_input_from_rofi("Enter playlist name:");
        let playlist_name = sanitize_directory_name(playlist_name.trim());

        let playlist_dir = format!("{}/{}", output_dir, playlist_name);
        fs::create_dir_all(&playlist_dir).expect("Failed to create playlist directory");

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
    } else {
        eprintln!("Download and conversion failed!");
        if let Ok(stderr) = String::from_utf8(command_output.stderr) {
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

fn get_input_from_rofi(prompt: &str) -> String {
    let rofi_output = Command::new("rofi")
        .args(["-dmenu", "-p", prompt])
        .output()
        .expect("Failed to launch rofi");

    String::from_utf8(rofi_output.stdout)
        .expect("Invalid UTF-8 from rofi")
        .trim()
        .to_string()
}

fn main() {
    // Use Rofi to ask for the YouTube link
    let youtube_link = get_input_from_rofi("Enter YouTube or playlist link:");
    let is_playlist = youtube_link.contains("playlist");

    // Use Rofi to select audio quality
    let audio_quality_input = get_input_from_rofi("Enter audio quality (0 - best, 9 - worst):");
    let audio_quality: u32 = audio_quality_input.trim().parse().unwrap_or(0);

    download_and_convert_to_m4a(&youtube_link, is_playlist, audio_quality);
}
