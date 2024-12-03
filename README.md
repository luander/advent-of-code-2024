# advent-of-code-2024

### Create project for new day
This project uses cargo-generate to make it easy to create templates for new advent of code days by just running:
```bash
NAME="day-x" make generate
```
### Running a specific day

Each day is divided by part1 and part2 binaries. To get the solutions for a day run:
```bash
cargo run -p day-x-name --bin part1
cargo run -p day-x-name --bin part2
```

### Benchmarking

```bash
cargo bench -p day-x-name
```
```
