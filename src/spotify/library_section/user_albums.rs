use crate::app::App;
use crate::spotify::auth::get_spotify_client;
use futures::FutureExt;
use futures_util::TryStreamExt;
use rspotify::model::SavedAlbum;
use rspotify::prelude::OAuthClient;
use rspotify::ClientError;
use serde_json::{json, Value};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

/// Fetches a user's saved albums from Spotify
#[tokio::main]
pub async fn user_albums(app: &mut App) -> Result<(), ClientError> {
    // Get a Spotify client using an existing access token (if available).
    let spotify = get_spotify_client(app).await?;

    // Collect all the user's saved albums from Spotify
    let mut albums = Vec::new();
    // Executing the futures sequentially
    let stream = spotify
        .current_user_saved_albums(None)
        .try_for_each(|item| {
            albums.push(item);
            futures::future::ok(())
        })
        .boxed();

    stream.await?;

    save_albums_to_json(app, albums);

    Ok(())
}

/// Saves a vector of saved albums to a JSON file in the Spotify cache directory
fn save_albums_to_json(app: &mut App, albums: Vec<SavedAlbum>) {
    let json_data = json!(albums);

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify");
    path.push("spotify_cache");
    std::fs::create_dir_all(&path).unwrap();
    path.push("user_albums.json");

    let mut file = File::create(&path).unwrap();
    let _ = file.write_all(json_data.to_string().as_bytes());
}

/// Processes the saved albums data stored in the cache file and populates the app's data structures
pub fn process_user_albums(app: &mut App) {
    // Clear any existing user album data in the app before processing
    app.user_album_names.clear();
    app.user_album_links.clear();
    app.user_album_tracks.clear();
    app.user_album_artist_names.clear();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify");
    path.push("spotify_cache");
    path.push("user_albums.json");

    let file = File::open(&path).expect("Failed to open user_albums.json");
    let reader = BufReader::new(file);
    let json_data: Value =
        serde_json::from_reader(reader).expect("Failed to parse user_albums.json");

    // Extract information about each saved album from the JSON data
    if let Value::Array(albums) = json_data {
        for album in albums {
            if let Value::Object(album_obj) = album {
                if let Some(album_info) = album_obj.get("album").and_then(Value::as_object) {
                    if let Some(album_name) = album_info.get("name").and_then(Value::as_str) {
                        app.user_album_names.push(album_name.to_string());
                    }

                    if let Some(external_urls) =
                        album_info.get("external_urls").and_then(Value::as_object)
                    {
                        if let Some(album_link) =
                            external_urls.get("spotify").and_then(Value::as_str)
                        {
                            app.user_album_links.push(album_link.to_string());
                        }
                    }
                    if let Some(artists) = album_info.get("artists").and_then(Value::as_array) {
                        if let Some(first_artist) = artists.first().and_then(Value::as_object) {
                            if let Some(artist_name) =
                                first_artist.get("name").and_then(Value::as_str)
                            {
                                app.user_album_artist_names.push(artist_name.to_string());
                            }
                        }
                    }
                    if let Some(tracks) = album_info.get("tracks").and_then(Value::as_object) {
                        if let Some(total_tracks) = tracks.get("total").and_then(Value::as_u64) {
                            app.user_album_tracks.push(total_tracks as usize);
                        }
                    }
                }
            }
        }
    }
}
