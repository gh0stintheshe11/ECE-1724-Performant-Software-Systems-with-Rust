use serde::{Deserialize, Serialize};
use warp::Filter;
use dashmap::DashMap;
use std::{sync::Arc, fs, path::Path};
use rayon::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
struct Song {
    id: usize,
    title: String,
    artist: String,
    genre: String,
    play_count: usize,
    index: SongIndex, // Precomputed lowercase indices for search
}

#[derive(Serialize, Deserialize, Clone)]
struct SongIndex {
    title: String,
    artist: String,
    genre: String,
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
    music_library: DashMap<String, DashMap<usize, Song>>, // Genre-based sharding
    next_song_id: DashMap<String, usize>,
    query_cache: DashMap<String, Vec<Song>>, // Query cache
}

const DATA_FILE: &str = "songs.json";

fn load_data() -> DashMap<String, DashMap<usize, Song>> {
    let map = DashMap::new();

    if Path::new(DATA_FILE).exists() {
        match fs::read_to_string(DATA_FILE) {
            Ok(data) => match serde_json::from_str::<Vec<Song>>(&data) {
                Ok(songs) => {
                    for song in songs {
                        let genre_map = map.entry(song.index.genre.clone())
                            .or_insert_with(DashMap::new);
                        genre_map.insert(song.id, song);
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

fn save_data(library: &DashMap<String, DashMap<usize, Song>>) {
    let all_songs: Vec<Song> = library
        .iter()
        .flat_map(|shard| {
            shard
                .value()
                .iter()
                .map(|entry| entry.value().clone())
                .collect::<Vec<_>>() // Collect inner DashMap entries into a Vec
        })
        .collect(); // Collect all shards into a single Vec

    match serde_json::to_string_pretty(&all_songs) {
        Ok(json) => {
            if let Err(e) = fs::write(DATA_FILE, json) {
                eprintln!("Error writing to songs.json: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error serializing songs: {}", e);
        }
    }
}

fn matches_query(song: &Song, query: &std::collections::HashMap<String, String>) -> bool {
    query.iter().all(|(key, value)| {
        let lower_value = value.to_lowercase();
        match key.as_str() {
            "title" => song.index.title.contains(&lower_value),
            "artist" => song.index.artist.contains(&lower_value),
            "genre" => song.index.genre.contains(&lower_value),
            _ => false,
        }
    })
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        visit_count: DashMap::new(),
        music_library: load_data(),
        next_song_id: DashMap::new(),
        query_cache: DashMap::new(),
    });

    // Background task to save data every 10 seconds
    let state_clone = Arc::clone(&state);
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));
        loop {
            interval.tick().await;
            save_data(&state_clone.music_library);
        }
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
                let mut id = state.next_song_id.entry("next_id".to_string()).or_insert(1);
                let song = Song {
                    id: *id,
                    title: new_song.title.clone(),
                    artist: new_song.artist.clone(),
                    genre: new_song.genre.clone(),
                    play_count: 0,
                    index: SongIndex {
                        title: new_song.title.to_lowercase(),
                        artist: new_song.artist.to_lowercase(),
                        genre: new_song.genre.to_lowercase(),
                    },
                };
                *id += 1; // Increment for the next song

                // Insert the song into the appropriate genre shard
                let genre_map = state
                    .music_library
                    .entry(song.index.genre.clone())
                    .or_insert_with(DashMap::new);
                genre_map.insert(song.id, song.clone());

                warp::reply::json(&song) // Respond with the created song
            })
    };

    // Search songs
    let search_songs = {
        let state = Arc::clone(&state);
        warp::path!("songs" / "search")
            .and(warp::query::<std::collections::HashMap<String, String>>())
            .map(move |query: std::collections::HashMap<String, String>| {
                let cache_key = serde_json::to_string(&query).unwrap();
                if let Some(cached_result) = state.query_cache.get(&cache_key) {
                    return warp::reply::json(&*cached_result);
                }

                let mut results = Vec::new();
                if let Some(genre) = query.get("genre") {
                    if let Some(shard) = state.music_library.get(genre) {
                        results.extend(shard.iter().filter_map(|entry| {
                            let song = entry.value();
                            if matches_query(song, &query) {
                                Some(song.clone())
                            } else {
                                None
                            }
                        }));
                    }
                } else {
                    for shard in state.music_library.iter() {
                        results.extend(shard.value().iter().filter_map(|entry| {
                            let song = entry.value();
                            if matches_query(song, &query) {
                                Some(song.clone())
                            } else {
                                None
                            }
                        }));
                    }
                }

                state.query_cache.insert(cache_key, results.clone());
                warp::reply::json(&results)
            })
    };

    // Play song
    let play_song = {
        let state = Arc::clone(&state);
        warp::path!("songs" / "play" / usize)
            .map(move |id: usize| {
                for shard in state.music_library.iter() {
                    if let Some(mut song) = shard.value().get_mut(&id) {
                        song.play_count += 1;
                        return warp::reply::json(&*song);
                    }
                }
                warp::reply::json(&serde_json::json!({ "error": "Song not found" }))
            })
    };

    // Combine routes
    let routes = warp::get()
        .and(index.or(visit_count).or(search_songs).or(play_song))
        .or(add_song);

    println!("The server is currently listening on localhost:8080.");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}