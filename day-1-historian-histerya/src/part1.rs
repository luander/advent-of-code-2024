pub fn process(input: &str) -> anyhow::Result<u32> {
    let mut left = vec![];
    let mut right = vec![];

    input.lines().for_each(|line| {
        let mut split = line.split("   ");
        let l = split.next().unwrap().parse::<i32>().unwrap();
        let r = split.next().unwrap().parse::<i32>().unwrap();
        let pos = left.binary_search(&l).unwrap_or_else(|e| e);
        left.insert(pos, l);
        let pos = right.binary_search(&r).unwrap_or_else(|e| e);
        right.insert(pos, r);
    });

    let mut sum = 0;

    for i in 0..right.len() {
        sum += (left[i] - right[i]).unsigned_abs();
    }
    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "3   4
4   3
2   5
1   3
3   9
3   3
",
        11
    )]
    fn part_works(#[case] test_input: &str, #[case] expected: u32) -> anyhow::Result<()> {
        let result = process(test_input)?;
        assert_eq!(expected, result);
        Ok(())
    }
}
