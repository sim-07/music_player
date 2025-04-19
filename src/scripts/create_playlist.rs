use std::fmt::format;
use std::fs::File;
use std::path::Path;

pub fn create_playlist(playlist_name: &str) {
    let file = format!("playlists/{}.csv", playlist_name);
    let path = Path::new(&file);

    File::create(path).expect("Error creating playlist");
}