pub fn process(input: &str) -> anyhow::Result<u32> {
    let result: u32 = input
        .trim()
        .split(',')
        .map(|seq| {
            seq.chars()
                .scan(0, |state, c| {
                    *state = ((*state + c as u32) * 17) % 256;
                    Some(*state)
                })
                .last()
                .unwrap()
        })
        .sum();
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("HASH", 52)]
    #[case("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", 1320)]
    fn part_works(#[case] test_input: &str, #[case] expected: u32) -> anyhow::Result<()> {
        let result = process(test_input)?;
        assert_eq!(expected, result);
        Ok(())
    }
}
