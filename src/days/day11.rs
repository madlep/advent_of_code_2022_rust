use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{u128, u32},
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn part1(data: String) -> String {
    let rounds = 20;
    let boredom_factor = 3;
    run(&data, rounds, boredom_factor).to_string()
}

pub fn part2(data: String) -> String {
    let rounds = 10_000;
    let boredom_factor = 1;
    run(&data, rounds, boredom_factor).to_string()
}

fn run(data: &str, rounds: u32, boredom_factor: u32) -> u64 {
    let monkeys = parse(&data);
    let lcm: u128 = monkeys
        .iter()
        .map(|m| m.borrow().test.divisible_by as u128)
        .product();

    for _i in 1..=rounds {
        for monkey in monkeys.iter() {
            monkey
                .borrow_mut()
                .inspect_and_throw_items(&monkeys, boredom_factor, lcm);
        }
    }

    let mut inspection_counts = monkeys
        .iter()
        .map(|m| m.borrow().inspection_count as u64)
        .collect::<Vec<u64>>();
    inspection_counts.sort();
    inspection_counts.reverse();
    inspection_counts[0..=1].iter().product::<u64>()
}

#[derive(Debug, PartialEq)]
struct Monkey {
    items: VecDeque<Item>,
    op: Op,
    test: Test,
    inspection_count: u32,
}
impl Monkey {
    fn new(items: VecDeque<Item>, op: Op, test: Test) -> Self {
        Self {
            items,
            op,
            test,
            inspection_count: 0,
        }
    }

    fn inspect_and_throw_items(
        &mut self,
        monkeys: &Vec<Rc<RefCell<Monkey>>>,
        boredom_factor: u32,
        lcm: u128,
    ) -> () {
        while let Some(item) = self.items.pop_front() {
            self.inspection_count += 1;
            let item = self.op.call(&item, lcm);
            let item = Monkey::bored_with_item(item, boredom_factor);
            let throw_to = self.test.check(&item);
            monkeys[throw_to].borrow_mut().catch(item);
        }
    }

    fn bored_with_item(item: Item, boredom_factor: u32) -> Item {
        if boredom_factor == 1 {
            item
        } else {
            item / (boredom_factor as u128)
        }
    }

    fn catch(&mut self, item: Item) -> () {
        self.items.push_back(item);
    }
}

type MonkeyId = usize;

type Item = u128;

#[derive(Debug, PartialEq)]
enum Op {
    Mult(u32),
    Plus(u32),
    Sq,
}

impl Op {
    fn call(&self, item: &Item, lcm: u128) -> Item {
        let calc = match self {
            Op::Mult(n) => item * (*n as u128),
            Op::Plus(n) => item + (*n as u128),
            Op::Sq => item * item,
        };
        calc % lcm
    }
}

#[derive(Debug, PartialEq)]
struct Test {
    divisible_by: u32,
    true_throw: MonkeyId,
    false_throw: MonkeyId,
}

impl Test {
    fn new(divisible_by: u32, true_throw: MonkeyId, false_throw: MonkeyId) -> Self {
        Self {
            divisible_by,
            true_throw,
            false_throw,
        }
    }

    fn check(&self, item: &Item) -> MonkeyId {
        if item % (self.divisible_by as u128) == 0 {
            self.true_throw
        } else {
            self.false_throw
        }
    }
}

fn parse(input: &str) -> Vec<Rc<RefCell<Monkey>>> {
    let (_rest, monkeys) = monkeys_parser(input).unwrap();
    monkeys
}

fn monkeys_parser(input: &str) -> IResult<&str, Vec<Rc<RefCell<Monkey>>>> {
    separated_list0(tag("\n\n"), monkey_parser)(input)
}
fn monkey_parser(input: &str) -> IResult<&str, Rc<RefCell<Monkey>>> {
    let header = preceded(tag("Monkey "), terminated(u32, tag(":\n")));
    let p = preceded(
        header,
        tuple((
            terminated(items_parser, tag("\n")),
            terminated(operation_parser, tag("\n")),
            test_parser,
        )),
    );
    map(p, |(items, op, test)| {
        Rc::new(RefCell::new(Monkey::new(items.into(), op, test)))
    })(input)
}

fn items_parser(input: &str) -> IResult<&str, Vec<Item>> {
    let mut p = preceded(tag("  Starting items: "), separated_list0(tag(", "), u128));
    p(input)
}

fn operation_parser(input: &str) -> IResult<&str, Op> {
    let mult = map(preceded(tag("* "), u32), |num| Op::Mult(num));
    let plus = map(preceded(tag("+ "), u32), |num| Op::Plus(num));
    let sq = map(tag("* old"), |_| Op::Sq);
    preceded(tag("  Operation: new = old "), alt((mult, plus, sq)))(input)
}

fn test_parser(input: &str) -> IResult<&str, Test> {
    let check = terminated(test_check_parser, tag("\n"));
    let clauses = separated_pair(
        test_clause_parser(true),
        tag("\n"),
        test_clause_parser(false),
    );
    let mut p = map(tuple((check, clauses)), |(div, (t, f))| {
        Test::new(div, t, f)
    });
    p(input)
}

fn test_check_parser(input: &str) -> IResult<&str, u32> {
    preceded(tag("  Test: divisible by "), u32)(input)
}

fn test_clause_parser(test_result: bool) -> impl FnMut(&str) -> IResult<&str, MonkeyId> {
    move |input| {
        let bool_parser = if test_result {
            tag("true")
        } else {
            tag("false")
        };

        let prefix = tuple((tag("    If "), bool_parser, tag(": throw to monkey ")));
        map(preceded(prefix, u32), |id| id as usize)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn it_parses_input() {
        let expected = vec![
            Monkey::new(vec![79, 98].into(), Op::Mult(19), Test::new(23, 2, 3)),
            Monkey::new(
                vec![54, 65, 75, 74].into(),
                Op::Plus(6),
                Test::new(19, 2, 0),
            ),
            Monkey::new(vec![79, 60, 97].into(), Op::Sq, Test::new(13, 1, 3)),
            Monkey::new(vec![74].into(), Op::Plus(3), Test::new(17, 0, 1)),
        ];
        let parsed = parse(DATA);
        assert!(parsed[0].borrow().eq(&expected[0]));
        assert!(parsed[1].borrow().eq(&expected[1]));
        assert!(parsed[2].borrow().eq(&expected[2]));
    }

    #[test]
    fn it_parses_items() {
        let starting_items = "  Starting items: 54, 65, 75, 74";
        assert_eq!(
            items_parser(starting_items).unwrap(),
            ("", vec![54, 65, 75, 74])
        );
    }

    #[test]
    fn it_parses_operations() {
        let mult = "  Operation: new = old * 19";
        assert_eq!(operation_parser(mult).unwrap(), ("", Op::Mult(19)));

        let plus = "  Operation: new = old + 6";
        assert_eq!(operation_parser(plus).unwrap(), ("", Op::Plus(6)));

        let sq = "  Operation: new = old * old";
        assert_eq!(operation_parser(sq).unwrap(), ("", Op::Sq));
    }

    #[test]
    fn it_parses_test_clause() {
        let t = "    If true: throw to monkey 1";
        assert_eq!(test_clause_parser(true)(t).unwrap(), ("", 1));

        let f = "    If false: throw to monkey 3";
        assert_eq!(test_clause_parser(false)(f).unwrap(), ("", 3));
    }

    #[test]
    fn it_parses_test() {
        let data = "  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";
        assert_eq!(test_parser(data).unwrap(), ("", Test::new(23, 2, 3)))
    }

    #[test]
    fn it_parses_monkey() {
        let data = "\
Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3
";

        let expected = Monkey::new(vec![79, 60, 97].into(), Op::Sq, Test::new(13, 1, 3));
        let (_rest, monkey) = monkey_parser(data).unwrap();
        assert!(monkey.borrow().eq(&expected));
    }
}
