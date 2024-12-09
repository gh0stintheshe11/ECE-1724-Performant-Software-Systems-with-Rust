import asyncio
import random
import time
from dataclasses import dataclass
from datetime import datetime
from typing import Any, Dict

import aiohttp
from consts import artists, genres, vocabulary

SongData = Dict[str, str]
ResultData = Dict[str, Any]


@dataclass
class RequestResult:
    index: int
    success: bool
    song: SongData
    response: str
    time_taken: float


def generate_random_song() -> SongData:
    num_words = random.randint(1, 5)
    return {
        "title": " ".join(random.choices(vocabulary, k=num_words)),
        "artist": random.choice(artists),
        "genre": random.choice(genres),
    }


async def add_single_song(
    session: aiohttp.ClientSession, index: int
) -> RequestResult:
    song = generate_random_song()
    start_time = time.time()

    try:
        async with session.post(
            "http://localhost:8080/songs/new", json=song
        ) as response:
            response_text = await response.text()
            time_taken = time.time() - start_time

            # Add detailed logging
            if response.status != 200 and response.status != 201:
                print(f"\nRequest {index} failed:")
                print(f"Status: {response.status}")
                print(f"Response: {response_text}")
                print(f"Headers: {dict(response.headers)}")

            return RequestResult(
                index=index,
                success=response.status in (200, 201),
                song=song,
                response=response_text,
                time_taken=time_taken,
            )
    except Exception as e:
        print(f"\nRequest {index} failed with exception:")
        print(f"Error: {str(e)}")
        return RequestResult(
            index=index, success=False, song=song, response=str(e), time_taken=0
        )


class ProgressTracker:
    def __init__(self, total: int):
        self.total = total
        self.current = 0
        self.start_time = datetime.now()
        self.success_count = 0
        self.fail_count = 0
        self._lock = asyncio.Lock()

    async def update(self, success: bool) -> None:
        async with self._lock:
            self.current += 1
            if success:
                self.success_count += 1
            else:
                self.fail_count += 1
            self._print_progress()

    def _print_progress(self) -> None:
        elapsed = (datetime.now() - self.start_time).total_seconds()
        rate = self.current / elapsed if elapsed > 0 else 0
        print(
            f"\rProgress: {self.current}/{self.total} "
            f"({self.current/self.total*100:.1f}%) "
            f"Rate: {rate:.1f} req/s "
            f"Success: {self.success_count} Fail: {self.fail_count}",
            end="",
            flush=True,
        )


async def add_songs_to_server(
    num_songs: int = 100000, max_concurrent: int = 10, batch_size: int = 1000
) -> None:
    progress = ProgressTracker(num_songs)
    semaphore = asyncio.Semaphore(max_concurrent)

    async with aiohttp.ClientSession() as session:
        for batch_start in range(0, num_songs, batch_size):
            batch_end = min(batch_start + batch_size, num_songs)

            async def process_song(index: int):
                async with semaphore:
                    result = await add_single_song(session, index)
                    await progress.update(result.success)
                    if not result.success:
                        print(
                            f"\nFailed to add song {result.index}: {result.response}"
                        )
                    return result

            batch_tasks = [
                process_song(i) for i in range(batch_start, batch_end)
            ]
            await asyncio.gather(*batch_tasks)


async def main() -> None:
    try:
        random.seed(42)
        start_time = time.time()
        await add_songs_to_server()
        elapsed = time.time() - start_time
        print(f"\nTotal time: {elapsed:.2f} seconds")
    except KeyboardInterrupt:
        print("\nOperation cancelled by user")
    except Exception as e:
        print(f"\nAn error occurred: {e}")


if __name__ == "__main__":
    asyncio.run(main())
