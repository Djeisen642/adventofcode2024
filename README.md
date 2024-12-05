# Advent of Code Rust Solutions

## Project Structure
- `src/main.rs`: Entry point and CLI for running solutions
- `src/solutions/`: Directory containing solutions for each day
- `src/solutions/mod.rs`: Module definition for solutions
- `src/solutions/day*.rs`: Individual day solution files

## Running Solutions
To run a specific day's solution:
```bash
cargo run <day_number>
```

Example:
```bash
cargo run 1  # Runs Day 1 solution
cargo run 2  # Runs Day 2 solution
```

## Adding New Solutions
1. Create a new file `src/solutions/day{N}.rs`
2. Implement `solve()` function with Part 1 and Part 2 solutions
3. Add the day to `src/solutions/mod.rs`
4. Update `src/main.rs` to include the new day in the match statement

## Testing
Run tests for all solutions:
```bash
cargo test
```

## Dependencies
- anyhow: Error handling
- itertools: Iteration utilities
- regex: Regular expression support
