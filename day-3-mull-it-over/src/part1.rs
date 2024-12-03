use regex::Regex;

pub fn process(input: &str) -> anyhow::Result<u32> {
    let r = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;
    input.lines().for_each(|line| {
        r.captures_iter(line).for_each(|cap| {
            total += cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap();
            dbg!(&cap);
        });
    });

    Ok(total)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        161
    )]
    fn part_works(#[case] test_input: &str, #[case] expected: u32) -> anyhow::Result<()> {
        let result = process(test_input)?;
        assert_eq!(expected, result);
        Ok(())
    }
}
