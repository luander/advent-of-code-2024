use anyhow::Context;
use day_2_red_nosed_reports::part1::process;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = include_str!("../../input.txt");
    let result = process(input).context("process part1")?;
    println!("part1: {}", result);
    Ok(())
}
