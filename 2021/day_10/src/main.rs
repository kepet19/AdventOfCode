fn main() {
    let input = std::fs::read_to_string("input.txt").expect("a valid file");
    dbg!(part1(&input));
    dbg!(part2(&input));
}
fn part1(str: &str) -> usize {
    str.lines()
        .map(|line| {
            let char = check_line(line);
            dbg!(&char);
            points_table(char)
        })
        .sum()
}

fn part2(str: &str) -> usize {
    let mut totals: Vec<usize> = str
        .lines()
        .filter_map(|line| {
            let string = complete_line(line);
            if string.len() == 0 {
                return None;
            }
            Some(string.chars().fold(0, |acc, c| 5 * acc + points_table2(c)))
        })
        .collect();

    totals.sort();
    *totals.get(totals.len() / 2).unwrap()
}
fn points_table2(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn complete_line(line: &str) -> String {
    let mut stack = Vec::new();
    for char in line.chars() {
        match char {
            '(' | '[' | '{' | '<' => stack.push(char),
            ')' | ']' | '}' | '>' => {
                if let Some(match_char) = stack.pop() {
                    if get_open_delimiter(char) != match_char {
                        return "".to_string();
                    }
                }
            }
            _ => unimplemented!(),
        }
    }
    stack
        .into_iter()
        .rev()
        .map(|c| get_closeing_delimiter(c))
        .collect()
}

fn get_closeing_delimiter(c: char) -> char {
    match c {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        _ => unimplemented!(),
    }
}

fn get_open_delimiter(c: char) -> char {
    match c {
        ')' => '(',
        '}' => '{',
        ']' => '[',
        '>' => '<',
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_complete_line() {
        assert_eq!("}}]])})]", complete_line("[({(<(())[]>[[{[]{<()<>>"));
        assert_eq!(")}>]})", complete_line("[(()[<>])]({[<{<<[]>>("));
        assert_eq!("}}>}>))))", complete_line("(((({<>}<{<{<>}{[]{[]{}"));
        assert_eq!("]]}}]}]}>", complete_line("{<[[]]>}<{[{[{[]{()[[[]"));
        assert_eq!("])}>", complete_line("<{([{{}}[<[[[<>{}]]]>[]]"));
    }

    #[test]
    fn test_error_line() {
        assert_eq!(Error::Char(')'), check_line("[[<[([]))<([[{}[[()]]]"));
        assert_eq!(Error::Char(')'), check_line("[<(<(<(<{}))><([]([]()"));
        assert_eq!(Error::Char('>'), check_line("<{([([[(<>()){}]>(<<{{"));
        assert_eq!(Error::Char('}'), check_line("{([(<{}[<>[]}>{[]{[(<()>"));
        assert_eq!(Error::Char(']'), check_line("[{[{({}]{}}([{[{{{}}([]"));
    }

    #[test]
    fn part1_test() {
        assert_eq!(26397, part1(INPUT));
    }

    #[test]
    fn part2_test() {
        assert_eq!(288957, part2(INPUT));
    }

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
}

#[derive(PartialEq)]
enum Error {
    None,
    Char(char),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Char(arg0) => write!(f, "{}", arg0),
        }
    }
}

fn check_line(str: &str) -> Error {
    let mut stack = Vec::new();
    for char in str.chars() {
        match char {
            '(' | '[' | '{' | '<' => stack.push(char),
            ')' | ']' | '}' | '>' => {
                if let Some(match_char) = stack.pop() {
                    if get_open_delimiter(char) != match_char {
                        return Error::Char(char);
                    }
                }
            }
            _ => unimplemented!(),
        }
    }
    Error::None
}

fn points_table(c: Error) -> usize {
    match c {
        Error::Char(')') => 3,
        Error::Char(']') => 57,
        Error::Char('}') => 1197,
        Error::Char('>') => 25137,
        Error::Char(_) => 0,
        Error::None => 0,
    }
}
