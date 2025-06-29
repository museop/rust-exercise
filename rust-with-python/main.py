import time

import rust_lib


def sum_primes_python(limit):
    """Calculates the sum of primes up to a given limit in Python."""

    def is_prime(n):
        if n <= 1:
            return False
        for i in range(2, int(n**0.5) + 1):
            if n % i == 0:
                return False
        return True

    total = 0
    for i in range(2, limit + 1):
        if is_prime(i):
            total += i
    return total


if __name__ == "__main__":
    try:
        # 큰 숫자를 입력하면 시간이 꽤 걸릴 수 있습니다. 1,000,000 정도를 추천합니다.
        limit_str = input("Enter a number to sum primes up to: ")
        limit = int(limit_str)

        # --- Rust 버전 ---
        print("\nCalculating with Rust...")
        start_time_rust = time.time()
        result_rust = rust_lib.sum_primes(limit)
        end_time_rust = time.time()
        duration_rust = end_time_rust - start_time_rust

        print(f"[Rust] Sum of primes up to {limit}: {result_rust}")
        print(f"[Rust] Calculation took: {duration_rust:.6f} seconds")

        # --- Python 버전 ---
        print("\nCalculating with Python...")
        start_time_python = time.time()
        result_python = sum_primes_python(limit)
        end_time_python = time.time()
        duration_python = end_time_python - start_time_python

        print(f"\n[Python] Sum of primes up to {limit}: {result_python}")
        print(f"[Python] Calculation took: {duration_python:.6f} seconds")

        # --- 성능 비교 ---
        if duration_rust > 0 and duration_python > 0:
            if duration_python > duration_rust:
                speedup = duration_python / duration_rust
                print(f"\nRust was {speedup:.2f} times faster than Python.")
            else:
                speedup = duration_rust / duration_python
                print(f"\nPython was {speedup:.2f} times faster than Rust.")

    except ValueError:
        print("Invalid input. Please enter an integer.")
