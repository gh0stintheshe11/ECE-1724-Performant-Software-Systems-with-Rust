import concurrent.futures
import random
import statistics
import subprocess
import threading
import time
import urllib.parse
from typing import Dict, List

from consts import artists, genres, vocabulary

# Server endpoint for search
search_url = "http://localhost:8080/songs/search"

# Thread-local storage for random state
thread_local = threading.local()


def get_thread_random():
    """Get thread-local random number generator"""
    if not hasattr(thread_local, "random"):
        thread_local.random = random.Random()
    return thread_local.random


def generate_random_query() -> Dict[str, str]:
    """Generate random query parameters using thread-local random"""
    rand = get_thread_random()
    query_params = {}
    while not query_params:
        if rand.choice([True, False]):
            num_words = rand.randint(1, 5)
            query_params["title"] = " ".join(
                rand.choices(vocabulary, k=num_words)
            )
        if rand.choice([True, False]):
            query_params["artist"] = rand.choice(artists)
        if rand.choice([True, False]):
            query_params["genre"] = rand.choice(genres)
    return query_params


def execute_query(_: int) -> float:  # Added index parameter (unused)
    """Execute single query and return execution time"""
    query_params = generate_random_query()
    query_string = urllib.parse.urlencode(query_params)
    curl_command = [
        "curl",
        "-s",
        "-o",
        "/dev/null",
        f"{search_url}?{query_string}",
    ]

    start_time = time.time()
    try:
        subprocess.run(curl_command, capture_output=True, text=True, check=True)
        return time.time() - start_time
    except subprocess.CalledProcessError as e:
        print(f"Query failed: {e}")
        return 0


def measure_query_time(num_queries: int, max_workers: int = 10) -> List[float]:
    """Execute queries in parallel and return list of execution times"""
    with concurrent.futures.ThreadPoolExecutor(
        max_workers=max_workers
    ) as executor:
        return list(executor.map(execute_query, range(num_queries)))


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
    print(f"total time: {total_time:.4f} seconds")
    print(f"Successful queries: {len(valid_times)}/{len(times)}")


def main():
    num_queries = 1000
    seeds = [42, 123, 456, 789, 101112]
    all_times = []

    print(
        f"Measuring time over {len(seeds)} runs with {num_queries} queries each..."
    )

    for i, seed in enumerate(seeds, 1):
        random.seed(seed)
        times = measure_query_time(num_queries)
        all_times.extend(times)
        print_statistics(times, i, seed)

    # Print overall statistics
    valid_times = [t for t in all_times if t > 0]
    if valid_times:
        overall_avg = statistics.mean(valid_times)
        print(
            f"\nTotal time across all runs: {overall_avg * len(seeds) * num_queries:.4f} seconds"
        )
        print(f"Total successful queries: {len(valid_times)}/{len(all_times)}")


if __name__ == "__main__":
    main()
