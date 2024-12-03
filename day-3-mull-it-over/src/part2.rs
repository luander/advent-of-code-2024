use regex::Regex;

pub fn process(input: &str) -> anyhow::Result<u32> {
    let match_operation = Regex::new(r"(mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\))").unwrap();
    let mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;
    let mut process = true;
    input.lines().for_each(|line| {
        match_operation.find_iter(line).for_each(|matches| {
            if matches.as_str() == "don't()" {
                process = false;
            }
            if matches.as_str() == "do()" {
                process = true;
            }
            if process {
                if let Some(caps) = mul.captures(matches.as_str()) {
                    total += caps[1].parse::<u32>().unwrap() * caps[2].parse::<u32>().unwrap();
                }
            }
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
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        48
    )]
    fn part_works(#[case] test_input: &str, #[case] expected: u32) -> anyhow::Result<()> {
        let result = process(test_input)?;
        assert_eq!(expected, result);
        Ok(())
    }
}
