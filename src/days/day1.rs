type Calories = u32;
type ElfSupplies = Vec<Calories>;

pub fn part1(data: String) -> String {
    parse(data)
        .iter()
        .map(&total_elf_calories)
        .max()
        .unwrap()
        .to_string()
}

pub fn part2(data: String) -> String {
    let mut elf_cals = parse(data)
        .iter()
        .map(&total_elf_calories)
        .collect::<Vec<Calories>>();

    elf_cals.sort();

    elf_cals.iter().rev().take(3).sum::<Calories>().to_string()
}

fn parse(data: String) -> Vec<ElfSupplies> {
    let mut elves: Vec<ElfSupplies>;
    let last_elf: ElfSupplies;

    (elves, last_elf) = data
        .split("\n")
        .map(parse_calories)
        .fold((Vec::new(), Vec::new()), accumulate_calories);
    if !last_elf.is_empty() {
        elves.push(last_elf);
    }

    elves
}

fn accumulate_calories(
    state: (Vec<ElfSupplies>, ElfSupplies),
    calories: Option<Calories>,
) -> (Vec<ElfSupplies>, ElfSupplies) {
    let (mut all_elves, mut current_elf) = state;

    match calories {
        Some(cal) => {
            current_elf.push(cal);
        }
        None => {
            all_elves.push(current_elf);
            current_elf = Vec::new()
        }
    };
    (all_elves, current_elf)
}

fn parse_calories(calories: &str) -> Option<Calories> {
    calories.parse::<Calories>().ok()
}

fn total_elf_calories(supplies: &ElfSupplies) -> Calories {
    supplies.iter().sum()
}
