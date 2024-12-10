# Web Server

A blazing-fast, concurrent web server built with Rust that manages a music library with efficient genre-based sharding and query caching.

## Features

- 🚀 High-performance concurrent operations using `tokio` and `rayon`
- 💾 Persistent storage with automatic background saves
- 🔍 Fast search capabilities with pre-computed indices
- 📊 Genre-based data sharding for improved query performance
- 💻 RESTful API endpoints
- 🎵 Music library management with play count tracking
- 📝 Query result caching for improved performance

## Technology Stack

- **Warp**: Lightning-fast web framework
- **DashMap**: Thread-safe concurrent HashMap implementation
- **Tokio**: Asynchronous runtime
- **Rayon**: Data-parallelism library
- **Serde**: Serialization/deserialization framework

## API Endpoints

### GET /
Welcome page

### GET /count
Returns the current visit count

### POST /songs/new
Add a new song to the library

Request body:
```json
{
    "title": "Song Title",
    "artist": "Artist Name",
    "genre": "Genre"
}
```

### GET /songs/search
Search for songs with optional filters

Query parameters:
- `title`: Filter by title
- `artist`: Filter by artist
- `genre`: Filter by genre

Example: `/songs/search?artist=Beatles&genre=Rock`

### GET /songs/play/:id
Increment play count for a song and return its details

## Performance Optimizations

1. **Genre-based Sharding**: Data is partitioned by genre for faster queries
2. **Query Caching**: Frequently accessed search results are cached
3. **Parallel Processing**: Uses Rayon for parallel data operations
4. **Efficient Indexing**: Pre-computed lowercase indices for case-insensitive searches
5. **Concurrent Data Structures**: DashMap for thread-safe operations

## Installation

1. Ensure you have Rust installed (1.75+ recommended)
2. Clone the repository
3. Build the project:
```bash
cargo build --release
```

## Running the Server

```bash
cargo run --release
```

The server will start on `localhost:8080`

## Configuration

The project uses the following optimizations in release mode:
- Level 3 optimization for maximum speed
- Link-Time Optimization (LTO) enabled
- Single codegen unit for better optimization
- Panic abort for reduced binary size

## Data Persistence

- Songs are automatically saved to `songs.json` every 10 seconds
- Data is loaded from `songs.json` on startup if available
- Parallel serialization for efficient I/O operations