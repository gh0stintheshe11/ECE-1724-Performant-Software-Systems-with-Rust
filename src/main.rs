use serde::{Deserialize, Serialize};
use std::sync::Arc;
use parking_lot::Mutex;
use warp::Filter;

#[derive(Serialize, Deserialize, Clone)]
struct Song {
    id: usize,
    title: String,
    artist: String,
    genre: String,
    play_count: usize,
}

#[derive(Default)]
struct AppState {
    visit_count: Mutex<usize>,
    songs: Mutex<Vec<Song>>,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::default());

    // Basic route
    let index = warp::path::end()
        .map(|| warp::reply::html("Welcome to the Rust-powered web server!"));

    // Visit count
    let visit_count = {
        let state = Arc::clone(&state);
        warp::path("count")
            .map(move || {
                let mut count = state.visit_count.lock();
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
            .map(move |new_song: Song| {
                let mut songs = state.songs.lock();
                let id = songs.len() + 1;
                let song = Song {
                    id,
                    play_count: 0,
                    ..new_song
                };
                songs.push(song.clone());
                warp::reply::json(&song)
            })
    };

    // Search songs
    let search_songs = {
        let state = Arc::clone(&state);
        warp::path!("songs" / "search")
            .and(warp::query::<std::collections::HashMap<String, String>>())
            .map(move |query: std::collections::HashMap<String, String>| {
                let songs = state.songs.lock();
                let results: Vec<_> = songs
                    .iter()
                    .filter(|song| {
                        query.iter().all(|(key, value)| {
                            match key.as_str() {
                                "title" => song.title.to_lowercase().contains(&value.to_lowercase()),
                                "artist" => song.artist.to_lowercase().contains(&value.to_lowercase()),
                                "genre" => song.genre.to_lowercase().contains(&value.to_lowercase()),
                                _ => false,
                            }
                        })
                    })
                    .cloned()
                    .collect();
                warp::reply::json(&results)
            })
    };

    // Play song
    let play_song = {
        let state = Arc::clone(&state);
        warp::path!("songs" / "play" / usize)
            .map(move |id: usize| {
                let mut songs = state.songs.lock();
                if let Some(song) = songs.iter_mut().find(|s| s.id == id) {
                    song.play_count += 1;
                    warp::reply::json(song)
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
}