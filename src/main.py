import subprocess
import os
from mutagen.mp4 import MP4
import tkinter as tk
from tkinter import messagebox

def download_audio(youtube_link):
    output_dir = "out"
    os.makedirs(output_dir, exist_ok=True)

    command = [
        "yt-dlp",
        "-x",
        "--audio-format",
        "m4a",
        "--audio-quality",
        "0",  # Best quality
        "-o",
        f"{output_dir}/%(title)s.%(ext)s",
        youtube_link
    ]
    
    subprocess.run(command, check=True)

def embed_metadata(file_path, artist, track, album):
    audio = MP4(file_path)
    audio["\xa9ART"] = artist
    audio["\xa9nam"] = track
    audio["\xa9alb"] = album
    audio.save()

def on_download():
    youtube_link = link_entry.get()
    if not youtube_link:
        messagebox.showerror("Error", "Please enter a YouTube link.")
        return

    download_audio(youtube_link)

    downloaded_files = os.listdir("out")
    if not downloaded_files:
        messagebox.showerror("Error", "No files downloaded.")
        return

    file_path = os.path.join("out", downloaded_files[0])
    audio = MP4(file_path)

    # Autofill metadata fields
    artist = audio.get("\xa9ART", ["Unknown Artist"])[0]
    track = audio.get("\xa9nam", ["Unknown Track"])[0]
    album = audio.get("\xa9alb", ["Unknown Album"])[0]

    artist_entry.delete(0, tk.END)
    artist_entry.insert(0, artist)
    
    track_entry.delete(0, tk.END)
    track_entry.insert(0, track)
    
    album_entry.delete(0, tk.END)
    album_entry.insert(0, album)

    # Allow user to edit metadata
    if messagebox.askyesno("Edit Metadata", "Would you like to edit the metadata before saving?"):
        artist = artist_entry.get()
        track = track_entry.get()
        album = album_entry.get()
        embed_metadata(file_path, artist, track, album)
        messagebox.showinfo("Success", "Metadata embedded successfully!")
    else:
        messagebox.showinfo("Info", "Metadata not changed.")

# Set up the UI
app = tk.Tk()
app.title("YouTube Audio Downloader")

tk.Label(app, text="YouTube Link:").pack(pady=5)
link_entry = tk.Entry(app, width=50)
link_entry.pack(pady=5)

tk.Label(app, text="Artist Name:").pack(pady=5)
artist_entry = tk.Entry(app, width=50)
artist_entry.pack(pady=5)

tk.Label(app, text="Track Title:").pack(pady=5)
track_entry = tk.Entry(app, width=50)
track_entry.pack(pady=5)

tk.Label(app, text="Album Name:").pack(pady=5)
album_entry = tk.Entry(app, width=50)
album_entry.pack(pady=5)

download_button = tk.Button(app, text="Download and Embed Metadata", command=on_download)
download_button.pack(pady=20)

app.mainloop()
