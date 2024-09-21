import os
import subprocess
from mutagen.easymp4 import EasyMP4
import yt_dlp

def download_audio(youtube_link, output_dir="out", audio_quality="128"):
    # Create output directory if it doesn't exist
    os.makedirs(output_dir, exist_ok=True)

    ydl_opts = {
        'format': 'bestaudio/best',
        'outtmpl': f'{output_dir}/%(title)s.%(ext)s',
        'postprocessors': [{
            'key': 'FFmpegExtractAudio',
            'preferredcodec': 'm4a',
            'preferredquality': audio_quality
        }],
        'addmetadata': True
    }

    with yt_dlp.YoutubeDL(ydl_opts) as ydl:
        result = ydl.download([youtube_link])

    # Get the downloaded filename
    return ydl.prepare_filename(ydl.extract_info(youtube_link))

def embed_metadata(file_path, artist, track, album):
    # Load the downloaded audio file
    audio_file = EasyMP4(file_path)

    # Set metadata
    audio_file['artist'] = artist
    audio_file['title'] = track
    audio_file['album'] = album

    # Save the file with metadata
    audio_file.save()

def main():
    youtube_link = input("Enter the YouTube link: ").strip()

    artist = input("Artist: ").strip()
    track = input("Track: ").strip()
    album = input("Album: ").strip()

    print("Downloading audio...")
    downloaded_file = download_audio(youtube_link)

    print(f"Downloaded: {downloaded_file}")
    
    # Add metadata
    embed_metadata(downloaded_file)
