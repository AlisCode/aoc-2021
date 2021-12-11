#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

#[aoc(day10, part1)]
fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .filter_map(|l| match do_parse_line(&l) {
            ParseResult::Corrupted(c) => Some(illegal_score(c)),
            ParseResult::Incomplete(_) => None,
            ParseResult::Complete => None,
        })
        .sum()
}

#[aoc(day10, part2)]
fn part2(input: &[String]) -> u64 {
    let mut scores: Vec<u64> = input
        .iter()
        .filter_map(|l| match do_parse_line(&l) {
            ParseResult::Corrupted(_) => None,
            ParseResult::Incomplete(stack) => {
                let score = stack
                    .into_iter()
                    .rev()
                    .fold(0, |total, c| total * 5 + completion_score(c));
                Some(score)
            }
            ParseResult::Complete => None,
        })
        .collect();
    scores.sort();
    scores[scores.len() / 2]
}

fn completion_score(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("Unknown char"),
    }
}

fn illegal_score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unknown char"),
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ParseResult {
    Complete,
    Incomplete(Vec<char>),
    Corrupted(char),
}

fn parse_line(input: &str, mut stack: Vec<char>) -> ParseResult {
    if input.len() == 0 {
        return match stack.len() {
            0 => ParseResult::Complete,
            _ => ParseResult::Incomplete(stack),
        };
    }
    let c = input.chars().next().unwrap(); // safe because input.len() > 0
    match c {
        '(' | '[' | '{' | '<' => {
            stack.push(c);
            parse_line(&input[1..], stack)
        }
        ')' if stack.iter().rev().peekable().peek() == Some(&&'(') => {
            stack.pop();
            parse_line(&input[1..], stack)
        }
        ']' if stack.iter().rev().peekable().peek() == Some(&&'[') => {
            stack.pop();
            parse_line(&input[1..], stack)
        }
        '>' if stack.iter().rev().peekable().peek() == Some(&&'<') => {
            stack.pop();
            parse_line(&input[1..], stack)
        }
        '}' if stack.iter().rev().peekable().peek() == Some(&&'{') => {
            stack.pop();
            parse_line(&input[1..], stack)
        }
        _ => ParseResult::Corrupted(c),
    }
}

fn do_parse_line(input: &str) -> ParseResult {
    let stack = Vec::new();
    parse_line(input, stack)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn valid_chunks() {
        assert_eq!(do_parse_line("()"), ParseResult::Complete);
        assert_eq!(do_parse_line("[]"), ParseResult::Complete);
        assert_eq!(do_parse_line("([])"), ParseResult::Complete);
        assert_eq!(do_parse_line("{()()()}"), ParseResult::Complete);
        assert_eq!(do_parse_line("<([{}])>"), ParseResult::Complete);
        assert_eq!(do_parse_line("[<>({}){}[([])<>]]"), ParseResult::Complete);
        assert_eq!(do_parse_line("(((((((((())))))))))"), ParseResult::Complete);
    }

    #[test]
    fn incomplete_chunks() {
        assert_eq!(
            do_parse_line("[(()[<>])]({[<{<<[]>>("),
            ParseResult::Incomplete(vec!['(', '{', '[', '<', '{', '(',])
        );
        assert_eq!(
            do_parse_line("{<(<{[[([[(<[(<>[])]<{()}<()[]>>><{<{}[]><[]>}(<<>{}><()()>)>)]]{((({{<><>}{{}{}}}{<[]()><{}<>>})<[{("),
            ParseResult::Incomplete(vec![ '{', '<', '(', '<', '{', '[', '[', '(', '{', '(', '(', '<', '[', '{', '(' ])
        );
    }

    #[test]
    fn corrupted_chunks() {
        assert_eq!(
            do_parse_line("{([(<{}[<>[]}>{[]{[(<()>"),
            ParseResult::Corrupted('}'),
        );
    }

    #[test]
    fn should_work() {
        let data = parse(INPUT);
        assert_eq!(part1(&data), 26397);
        assert_eq!(part2(&data), 288957);
    }
}
