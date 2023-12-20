use std::{collections::HashMap, ops::Range};

const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/19_full.txt"));

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
            let v: Vec<usize> = line
                .split(',')
                .map(|x| x.trim())
                .map(|entry| entry.split_once('=').unwrap().1.parse().unwrap())
                .collect();
            let x = v[0];
            let m= v[1];
            let a = v[2];
            let s = v[3];
            Part { x, m, a, s }
        })
        .collect();
    (parts, workflows)
}

#[derive(Debug)]
struct AcceptedRange {
    x_low: usize,
    x_high: usize,
    m_low: usize,
    m_high: usize,
    a_low: usize,
    a_high: usize,
    s_low: usize,
    s_high: usize,
}

impl Default for AcceptedRange {
    fn default() -> Self {
        Self {
            x_low: 1,
            x_high: 4000,
            m_low: 1,
            m_high: 4000,
            a_low: 1,
            a_high: 4000,
            s_low: 1,
            s_high: 4000,
        }
    }
}
// to dodge the condition
fn fix_range_negative(range: &mut AcceptedRange, condition: &Condition) {
    match (condition.operand_a.as_str(), &condition.operator) {
        ("x", ConditionOp::GreaterThan) => range.x_high = range.x_high.min(condition.operand_b),
        ("m", ConditionOp::GreaterThan) => range.m_high = range.m_high.min(condition.operand_b),
        ("a", ConditionOp::GreaterThan) => range.a_high = range.a_high.min(condition.operand_b),
        ("s", ConditionOp::GreaterThan) => range.s_high = range.s_high.min(condition.operand_b),

        ("x", ConditionOp::LessThan) => range.x_low = range.x_low.max(condition.operand_b),
        ("m", ConditionOp::LessThan) => range.m_low = range.m_low.max(condition.operand_b),
        ("a", ConditionOp::LessThan) => range.a_low = range.a_low.max(condition.operand_b),
        ("s", ConditionOp::LessThan) => range.s_low = range.s_low.max(condition.operand_b),
        _ => panic!("at the disco"),
    }
}
// to fulfill the condition
fn fix_range_positive(range: &mut AcceptedRange, condition: &Condition) {
    match (condition.operand_a.as_str(), &condition.operator) {
        ("x", ConditionOp::GreaterThan) => range.x_low = range.x_low.max(condition.operand_b + 1),
        ("m", ConditionOp::GreaterThan) => range.m_low = range.m_low.max(condition.operand_b + 1),
        ("a", ConditionOp::GreaterThan) => range.a_low = range.a_low.max(condition.operand_b + 1),
        ("s", ConditionOp::GreaterThan) => range.s_low = range.s_low.max(condition.operand_b + 1),

        ("x", ConditionOp::LessThan) => range.x_high = range.x_high.min(condition.operand_b - 1),
        ("m", ConditionOp::LessThan) => range.m_high = range.m_high.min(condition.operand_b - 1),
        ("a", ConditionOp::LessThan) => range.a_high = range.a_high.min(condition.operand_b - 1),
        ("s", ConditionOp::LessThan) => range.s_high = range.s_high.min(condition.operand_b - 1),
        _ => panic!("at the disco"),
    }
}
fn part_two(input: &str) -> usize {
    let (_, workflows) = parse(input);
    let mut ranges: Vec<AcceptedRange> = vec![];
    for workflow in &workflows {
        for (i, rule) in workflow.1.iter().enumerate() {
            if rule.result == WorkFlowResult::Accepted {
                let mut range = AcceptedRange::default();
                if let Some(condition) = &rule.condition {
                    fix_range_positive(&mut range, condition);
                }
                // check other conditions
                for j in 0..i {
                    if let Some(condition) = &workflow.1[j].condition {
                        fix_range_negative(&mut range, &condition);
                    }
                }
                let mut previous_name = workflow.0;
                let mut next_workflow = workflows
                    .iter()
                    .filter(|workflow| {
                        workflow.1.iter().any(|rule| match &rule.result {
                            WorkFlowResult::Workflow(name) if name == previous_name => true,
                            _ => false,
                        })
                    })
                    .next()
                    .unwrap();
                loop {
                    for rule in next_workflow.1 {
                        match &rule.result {
                            WorkFlowResult::Workflow(name) if name == previous_name => {
                                if let Some(condition) = &rule.condition {
                                    fix_range_positive(&mut range, condition);
                                }
                                break;
                            }
                            WorkFlowResult::Rejected
                            | WorkFlowResult::Accepted
                            | WorkFlowResult::Workflow(_) => {
                                if let Some(condition) = &rule.condition {
                                    fix_range_negative(&mut range, &condition);
                                }
                            }
                        }
                    }
                    if next_workflow.0 == "in" {
                        break;
                    }
                    previous_name = next_workflow.0;
                    next_workflow = workflows
                        .iter()
                        .filter(|workflow| {
                            workflow.1.iter().any(|rule| match &rule.result {
                                WorkFlowResult::Workflow(name) if name == previous_name => true,
                                _ => false,
                            })
                        })
                        .next()
                        .unwrap();
                }
                ranges.push(range);
            }
        }
    }
    ranges
        .iter()
        .map(|range| {
            (range.x_high - range.x_low + 1)
                * (range.m_high - range.m_low + 1)
                * (range.a_high - range.a_low + 1)
                * (range.s_high - range.s_low + 1)
        })
        .sum()
}

fn part_one(input: &str) -> usize {
    let (parts, workflows) = parse(input);
    let mut sum = 0;
    for part in parts {
        let mut start = workflows.get("in").unwrap();
        loop {
            let result = process_rules(&part, &start);
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

        assert_eq!(part_two(INPUT_TEST), 167409079868000);
        assert_eq!(part_two(INPUT), 138625360533574);
    }
}
