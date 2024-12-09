use serde::{Deserialize, Serialize};
use warp::Filter;
use dashmap::DashMap;
use std::{sync::Arc, fs, path::Path};

#[derive(Serialize, Deserialize, Clone)]
struct Song {
    id: usize,
    title: String,
    artist: String,
    genre: String,
    play_count: usize,
}

#[derive(Deserialize)]
struct NewSong {
    title: String,
    artist: String,
    genre: String,
}

#[derive(Default)]
struct AppState {
    visit_count: DashMap<String, usize>,
    music_library: DashMap<usize, Song>,
    next_song_id: DashMap<String, usize>,
}

const DATA_FILE: &str = "songs.json";

fn load_data() -> DashMap<usize, Song> {
    let map = DashMap::new();

    if Path::new(DATA_FILE).exists() {
        match fs::read_to_string(DATA_FILE) {
            Ok(data) => match serde_json::from_str::<Vec<Song>>(&data) {
                Ok(songs) => {
                    for song in songs {
                        map.insert(song.id, song);
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing songs.json: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Error reading songs.json: {}", e);
            }
        }
    }

    map
}

fn save_data(library: &DashMap<usize, Song>) {
    let songs: Vec<_> = library.iter().map(|entry| entry.clone()).collect();
    let json = serde_json::to_string_pretty(&songs).unwrap();
    fs::write(DATA_FILE, json).unwrap();
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        visit_count: DashMap::new(),
        music_library: load_data(),
        next_song_id: DashMap::new(),
    });

    // Basic route
    let index = warp::path::end()
        .map(|| warp::reply::html("Welcome to the Rust-powered web server!"));

    // Visit count
    let visit_count = {
        let state = Arc::clone(&state);
        warp::path("count")
            .map(move || {
                let mut count = state
                    .visit_count
                    .entry("count".to_string())
                    .or_insert(0);
                *count += 1;
                format!("Visit count: {}", *count)
            })
    };

    // Add song
    let add_song = {
        let state = Arc::clone(&state);
        warp::path!("songs" / "new")
            .and(warp::post())
            .and(warp::body::json())
            .map(move |new_song: NewSong| {
                // Generate a new unique ID for the song
                let mut id = state.next_song_id.entry("next_id".to_string()).or_insert(1);
                let song = Song {
                    id: *id,
                    title: new_song.title,
                    artist: new_song.artist,
                    genre: new_song.genre,
                    play_count: 0,
                };
                *id += 1; // Increment for the next song
    
                // Insert the new song into the library
                state.music_library.insert(song.id, song.clone());
                warp::reply::json(&song) // Respond with the created song
            })
    };

    // Search songs
    let search_songs = {
        let state = Arc::clone(&state);
        warp::path!("songs" / "search")
            .and(warp::query::<std::collections::HashMap<String, String>>())
            .map(move |query: std::collections::HashMap<String, String>| {
                let results: Vec<_> = state
                    .music_library
                    .iter()
                    .filter(|entry| {
                        let song = entry.value();
                        query.iter().all(|(key, value)| {
                            match key.as_str() {
                                "title" => song.title.contains(value),
                                "artist" => song.artist.contains(value),
                                "genre" => song.genre.contains(value),
                                _ => false,
                            }
                        })
                    })
                    .map(|entry| entry.clone())
                    .collect();
                warp::reply::json(&results)
            })
    };

    // Play song
    let play_song = {
        let state = Arc::clone(&state);
        warp::path!("songs" / "play" / usize)
            .map(move |id: usize| {
                if let Some(mut song) = state.music_library.get_mut(&id) {
                    song.play_count += 1;
                    warp::reply::json(&*song)
                } else {
                    warp::reply::json(&serde_json::json!({ "error": "Song not found" }))
                }
            })
    };

    // Combine routes
    let routes = warp::get().and(index.or(visit_count).or(search_songs).or(play_song))
        .or(add_song);

    println!("The server is currently listening on localhost:8080.");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

    // Save data before exiting
    save_data(&state.music_library);
}