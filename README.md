Here’s the improved README in full Markdown format, ready to copy:

# yt_m4a

**A simple Rust CLI tool for downloading and converting YouTube audio to M4A using `yt-dlp`.**

## Features

- Download audio from single YouTube videos or entire playlists
- Converts audio to `.m4a` format using `yt-dlp`
- Prompts for playlist folder name and saves tracks with playlist index
- Supports audio quality selection (`0` = best, `9` = worst)
- Adds metadata automatically (from video titles)

## Usage

1. **Run the tool**
   ```bash
   cargo run

2. Follow the prompts:

Paste a YouTube link (video or playlist)

Choose an audio quality (0–9)

If it's a playlist, you'll be prompted to enter a folder name



3. Output

Downloads will be saved in the out/ directory

Playlists are organized into subfolders with numbered tracks




Dependencies

yt-dlp must be installed and available in your system PATH


Example

Enter the YouTube link or playlist link: https://www.youtube.com/playlist?list=PLxyz
Enter audio quality (0 - best, 9 - worst): 2
Enter the playlist name: My Favorite Tracks

Resulting output:

out/My Favorite Tracks/
├── 01 - Track One.m4a
├── 02 - Track Two.m4a
└── ...

To Do

[ ] Reintroduce terminal UI (e.g. using crossterm or tui-rs)

[ ] Allow metadata fields to be autofilled and editable in-terminal


License

MIT

