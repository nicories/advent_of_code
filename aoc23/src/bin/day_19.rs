use std::collections::HashMap;

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/19_full.txt"));

#[derive(Debug, Clone)]
enum WorkFlowResult {
    Rejected,
    Accepted,
    Workflow(String),
}
impl WorkFlowResult {
    pub fn from_str(s: &str) -> Self {
        match s {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            _ => Self::Workflow(s.to_string()),
        }
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
enum ConditionOp {
    LessThan,
    GreaterThan,
}
struct Condition {
    operator: ConditionOp,
    operand_a: String,
    operand_b: usize,
}
struct Rule {
    condition: Option<Condition>,
    result: WorkFlowResult,
}

fn process_rules(part: &Part, rules: &Vec<Rule>) -> WorkFlowResult {
    for rule in rules {
        if let Some(condition) = &rule.condition {
            let a = match condition.operand_a.as_str() {
                "x" => part.x,
                "m" => part.m,
                "a" => part.a,
                "s" => part.s,
                _ => panic!("at the disco"),
            };
            match condition.operator {
                ConditionOp::LessThan => {
                    if a < condition.operand_b {
                        return rule.result.clone();
                    }
                }
                ConditionOp::GreaterThan => {
                    if a > condition.operand_b {
                        return rule.result.clone();
                    }
                }
            }
        } else {
            return rule.result.clone();
        }
    }
    panic!("at the disco");
}

fn parse(input: &str) -> (Vec<Part>, HashMap<String, Vec<Rule>>) {
    let (workflow_block, part_block) = input.split_once("\n\n").unwrap();
    let workflows = workflow_block
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once('{').unwrap();
            let mut rules = vec![];
            for rule_str in rest[..rest.len() - 1].split(',') {
                match rule_str {
                    "A" => rules.push(Rule {
                        condition: None,
                        result: WorkFlowResult::Accepted,
                    }),
                    "R" => rules.push(Rule {
                        condition: None,
                        result: WorkFlowResult::Rejected,
                    }),
                    _ if rule_str.contains(':') => {
                        let (condition_str, result_str) = rule_str.split_once(':').unwrap();
                        let result = WorkFlowResult::from_str(result_str);
                        let condition = if condition_str.contains('>') {
                            let operator = ConditionOp::GreaterThan;
                            let (a, b) = condition_str.split_once('>').unwrap();
                            Condition {
                                operator,
                                operand_a: a.to_string(),
                                operand_b: b.parse().unwrap(),
                            }
                        } else {
                            let operator = ConditionOp::LessThan;
                            let (a, b) = condition_str.split_once('<').unwrap();
                            Condition {
                                operator,
                                operand_a: a.to_string(),
                                operand_b: b.parse().unwrap(),
                            }
                        };
                        rules.push(Rule {
                            condition: Some(condition),
                            result: result,
                        });
                    }
                    _ => rules.push(Rule {
                        condition: None,
                        result: WorkFlowResult::Workflow(rule_str.to_string()),
                    }),
                };
            }
            (name.to_string(), rules)
        })
        .collect();
    let parts = part_block
        .lines()
        .map(|line| &line[1..line.len() - 1])
        // ugly
        .map(|line| {
            let mut split = line.split(',').map(|x| x.trim());
            let x = split
                .next()
                .unwrap()
                .split_once('=')
                .unwrap()
                .1
                .parse()
                .unwrap();
            let m = split
                .next()
                .unwrap()
                .split_once('=')
                .unwrap()
                .1
                .parse()
                .unwrap();
            let a = split
                .next()
                .unwrap()
                .split_once('=')
                .unwrap()
                .1
                .parse()
                .unwrap();
            let s = split
                .next()
                .unwrap()
                .split_once('=')
                .unwrap()
                .1
                .parse()
                .unwrap();
            Part { x, m, a, s }
        })
        .collect();
    (parts, workflows)
}

fn part_two(input: &str) -> usize {
    0
}

fn part_one(input: &str) -> usize {
    let (parts, workflows) = parse(input);
    let mut sum = 0;
    for part in parts {
        let mut start = workflows.get("in").unwrap();
        dbg!(&part);
        loop {
            let result = process_rules(&part, &start);
            dbg!(&result);
            match result {
                WorkFlowResult::Rejected => break,
                WorkFlowResult::Accepted => {
                    sum += part.x + part.m + part.a + part.s;
                    break;
                }
                WorkFlowResult::Workflow(name) => start = workflows.get(&name).unwrap(),
            }
        }
    }
    sum
}
fn main() {
    println!("1: {}", part_one(INPUT));
    println!("2: {}", part_two(INPUT));
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_TEST: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/19_test.txt"));

    #[test]
    fn test() {
        assert_eq!(part_one(INPUT_TEST), 19114);
        assert_eq!(part_one(INPUT), 406849);

        // assert_eq!(part_two(INPUT_TEST), 952408144115);
        // assert_eq!(part_two(INPUT), 173152345887206);
    }
}
