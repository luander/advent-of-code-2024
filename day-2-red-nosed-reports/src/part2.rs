use itertools::Itertools;

pub fn process(input: &str) -> anyhow::Result<u32> {
    let mut reports = Vec::new();

    input.lines().for_each(|line| {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|e| e.parse().unwrap())
            .collect();
        reports.push(report);
    });
    let mut safe = 0;
    reports.iter().for_each(|report| {
        let asc = report
            .iter()
            .tuple_windows()
            .all(|(a, b)| a < b && (b - a).abs() <= 3);
        let desc = report
            .iter()
            .tuple_windows()
            .all(|(a, b)| a > b && (a - b).abs() <= 3);

        let mut p2_safe = false;
        if !(asc || desc) {
            report.iter().enumerate().for_each(|(i, _)| {
                let asc2 = report
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| j != i)
                    .tuple_windows()
                    .all(|((_, a), (_, b))| a < b && (b - a).abs() <= 3);
                let desc2 = report
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| j != i)
                    .tuple_windows()
                    .all(|((_, a), (_, b))| a > b && (a - b).abs() <= 3);
                p2_safe = p2_safe || (asc2 || desc2);
            });
        }

        safe += if asc || desc || p2_safe { 1 } else { 0 };
    });

    Ok(safe)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        4
    )]
    fn part_works(#[case] test_input: &str, #[case] expected: u32) -> anyhow::Result<()> {
        let result = process(test_input)?;
        assert_eq!(expected, result);
        Ok(())
    }
}
