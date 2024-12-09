import concurrent.futures
import json
import random
import subprocess
import threading
import time
from dataclasses import dataclass
from datetime import datetime
from typing import Any, Dict

from consts import artists, genres, vocabulary

# Type definitions
SongData = Dict[str, str]
ResultData = Dict[str, Any]

@dataclass
class RequestResult:
    """Class to store request results"""
    index: int
    success: bool
    song: SongData
    response: str
    time_taken: float

# Thread-local storage for random state
thread_local = threading.local()

def get_thread_random() -> random.Random:
    """Get thread-local random number generator"""
    if not hasattr(thread_local, "random"):
        thread_local.random = random.Random()
    return thread_local.random

def generate_random_song() -> SongData:
    """Generate a random song using thread-local random state"""
    rand = get_thread_random()
    num_words = rand.randint(1, 5)
    return {
        "title": " ".join(rand.choices(vocabulary, k=num_words)),
        "artist": rand.choice(artists),
        "genre": rand.choice(genres)
    }

def add_single_song(index: int) -> RequestResult:
    """Add a single song and return the result"""
    song = generate_random_song()
    song_json = json.dumps(song)
    curl_command = [
        "curl",
        "http://localhost:8080/songs/new",
        "--json",
        song_json
    ]

    start_time = time.time()
    try:
        result = subprocess.run(
            curl_command,
            capture_output=True,
            text=True,
            check=True
        )
        time_taken = time.time() - start_time
        return RequestResult(
            index=index,
            success=True,
            song=song,
            response=result.stdout,
            time_taken=time_taken
        )
    except subprocess.CalledProcessError as e:
        return RequestResult(
            index=index,
            success=False,
            song=song,
            response=e.stderr,
            time_taken=0
        )

class ProgressTracker:
    """Thread-safe progress tracker"""
    def __init__(self, total: int):
        self.total = total
        self.current = 0
        self.start_time = datetime.now()
        self.lock = threading.Lock()
        self.success_count = 0
        self.fail_count = 0

    def update(self, success: bool) -> None:
        with self.lock:
            self.current += 1
            if success:
                self.success_count += 1
            else:
                self.fail_count += 1
            self._print_progress()

    def _print_progress(self) -> None:
        elapsed = (datetime.now() - self.start_time).total_seconds()
        rate = self.current / elapsed if elapsed > 0 else 0
        print(f"\rProgress: {self.current}/{self.total} "
              f"({self.current/self.total*100:.1f}%) "
              f"Rate: {rate:.1f} req/s "
              f"Success: {self.success_count} Fail: {self.fail_count}",
              end="", flush=True)

def add_songs_to_server(
    num_songs: int = 100000,
    max_workers: int = 10,
    batch_size: int = 1000
) -> None:
    """Add songs to server using thread pool"""
    progress = ProgressTracker(num_songs)

    with concurrent.futures.ThreadPoolExecutor(max_workers=max_workers) as executor:
        # Process in batches to avoid overwhelming memory
        for batch_start in range(0, num_songs, batch_size):
            batch_end = min(batch_start + batch_size, num_songs)
            futures = {
                executor.submit(add_single_song, i): i
                for i in range(batch_start, batch_end)
            }

            for future in concurrent.futures.as_completed(futures):
                try:
                    result = future.result()
                    progress.update(result.success)
                    if not result.success:
                        print(f"\nFailed to add song {result.index}: {result.response}")
                except Exception as e:
                    print(f"\nError processing request: {e}")
                    progress.update(False)

def main() -> None:
    """Main function with error handling"""
    try:
        random.seed(42)  # Set a random seed for reproducibility
        start_time = time.time()
        add_songs_to_server()
        elapsed = time.time() - start_time
        print(f"\nTotal time: {elapsed:.2f} seconds")
    except KeyboardInterrupt:
        print("\nOperation cancelled by user")
    except Exception as e:
        print(f"\nAn error occurred: {e}")

if __name__ == "__main__":
    main()
