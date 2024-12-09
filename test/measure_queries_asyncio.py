import asyncio
import random
import statistics
import time
import urllib.parse
from typing import Dict, List

import aiohttp
from consts import artists, genres, vocabulary

# Server endpoint for search
search_url = "http://localhost:8080/songs/search"

def generate_random_query() -> Dict[str, str]:
    """Generate random query parameters"""
    query_params = {}
    while not query_params:
        if random.choice([True, False]):
            num_words = random.randint(1, 5)
            query_params["title"] = " ".join(random.choices(vocabulary, k=num_words))
        if random.choice([True, False]):
            query_params["artist"] = random.choice(artists)
        if random.choice([True, False]):
            query_params["genre"] = random.choice(genres)
    return query_params

async def execute_query(session: aiohttp.ClientSession) -> float:
    """Execute single query and return execution time"""
    query_params = generate_random_query()
    query_string = urllib.parse.urlencode(query_params)
    url = f"{search_url}?{query_string}"

    start_time = time.time()
    try:
        async with session.get(url) as response:
            await response.read()
        return time.time() - start_time
    except Exception as e:
        print(f"Query failed: {e}")
        return 0

async def measure_query_time(num_queries: int, max_concurrent: int = 10) -> List[float]:
    """Execute queries with concurrency limit and return execution times"""
    async with aiohttp.ClientSession() as session:
        semaphore = asyncio.Semaphore(max_concurrent)

        async def bounded_query():
            async with semaphore:
                return await execute_query(session)

        tasks = [bounded_query() for _ in range(num_queries)]
        return await asyncio.gather(*tasks)

def print_statistics(times: List[float], run_num: int, seed: int):
    """Print statistics for a single run"""
    valid_times = [t for t in times if t > 0]
    if not valid_times:
        print(f"Run {run_num} with seed {seed}: All queries failed")
        return

    avg_time = statistics.mean(valid_times)
    median_time = statistics.median(valid_times)
    total_time = sum(valid_times)

    print(f"\nRun {run_num} with random seed {seed}")
    print(f"Average time: {avg_time:.4f} seconds")
    print(f"Median time: {median_time:.4f} seconds")
    print(f"Total time: {total_time:.4f} seconds")
    print(f"Successful queries: {len(valid_times)}/{len(times)}")

async def main():
    num_queries = 1000
    seeds = [42, 123, 456, 789, 101112]
    all_times = []

    print(f"Measuring time over {len(seeds)} runs with {num_queries} queries each...")

    for i, seed in enumerate(seeds, 1):
        random.seed(seed)
        times = await measure_query_time(num_queries)
        all_times.extend(times)
        print_statistics(times, i, seed)

    valid_times = [t for t in all_times if t > 0]
    if valid_times:
        overall_avg = statistics.mean(valid_times)
        print(f"\nTotal time across all runs: {overall_avg * len(seeds) * num_queries:.4f} seconds")
        print(f"Total successful queries: {len(valid_times)}/{len(all_times)}")

if __name__ == "__main__":
    asyncio.run(main())
