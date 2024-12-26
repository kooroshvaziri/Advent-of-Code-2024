use regex::Regex;

fn compute(
    pointer: &usize,
    opcode: &u32,
    operand: &u32,
    reg_a: &mut i64,
    reg_b: &mut i64,
    reg_c: &mut i64,
    output: &mut Vec<u32>,
) -> usize {
    let combo: i64 = match *operand {
        0..=3 => *operand as i64,
        4 => *reg_a,
        5 => *reg_b,
        6 => *reg_c,
        _ => panic!("Invalid instruction codes."),
    };

    match opcode {
        0 => {
            if combo < 0 {
                *reg_a = *reg_a * 2_i64.pow(combo as u32);
            } else {
                *reg_a = *reg_a / 2_i64.pow(combo as u32);
            }
        }
        1 => *reg_b = *reg_b ^ *operand as i64,
        2 => *reg_b = combo.rem_euclid(8),
        3 => {
            if *reg_a != 0 {
                //jump
                compute(pointer, operand, operand, reg_a, reg_b, reg_c, output);
                return *operand as usize;
            }
        }
        4 => *reg_b = *reg_b ^ *reg_c,
        5 => output.push(combo.rem_euclid(8) as u32),
        6 => {
            if combo < 0 {
                *reg_b = *reg_a * 2_i64.pow(combo as u32);
            } else {
                *reg_b = *reg_a / 2_i64.pow(combo as u32);
            }
        }
        7 => {
            if combo < 0 {
                *reg_c = *reg_a * 2_i64.pow(combo as u32);
            } else {
                *reg_c = *reg_a / 2_i64.pow(combo as u32);
            }
        }
        _ => panic!("Invalid instruction codes."),
    }

    return pointer + 2;
}

fn part_1(source: &str) -> (i64, i64, i64, String) {
    let regex = Regex::new(r"^Register A: (?P<register_a>\d+)\nRegister B: (?P<register_b>\d+)\nRegister C: (?P<register_c>\d+)\n\nProgram: (?P<program>.*)").unwrap();

    let (mut reg_a, mut reg_b, mut reg_c, program) = regex
        .captures_iter(source)
        .map(|c| {
            (
                c["register_a"].parse().unwrap_or(0) as i64,
                c["register_b"].parse().unwrap_or(0) as i64,
                c["register_c"].parse().unwrap_or(0) as i64,
                c["program"]
                    .split(',')
                    .filter(|c| c.len() > 0)
                    .map(|d| d.parse::<u32>().unwrap_or(0))
                    .collect::<Vec<u32>>(),
            )
        })
        .last()
        .unwrap_or((0, 0, 0, vec![]));

    let mut output: Vec<u32> = vec![];

    let mut pointer = 0;
    while pointer < program.len() - 1 {
        pointer = compute(
            &pointer,
            &program[pointer],
            &program[pointer + 1],
            &mut reg_a,
            &mut reg_b,
            &mut reg_c,
            &mut output,
        );
    }

    (
        reg_a,
        reg_b,
        reg_c,
        output
            .iter()
            .map(|&n| n.to_string())
            .collect::<Vec<String>>()
            .join(","),
    )
}

fn part_2(source: &str) -> i64 {
    let regex = Regex::new(r"^Register A: (?P<register_a>\d+)\nRegister B: (?P<register_b>\d+)\nRegister C: (?P<register_c>\d+)\n\nProgram: (?P<program>.*)").unwrap();

    let (_reg_a, reg_b, reg_c, program) = regex
        .captures_iter(source)
        .map(|c| {
            (
                c["register_a"].parse().unwrap_or(0) as i64,
                c["register_b"].parse().unwrap_or(0) as i64,
                c["register_c"].parse().unwrap_or(0) as i64,
                c["program"]
                    .split(',')
                    .filter(|c| c.len() > 0)
                    .map(|d| d.parse::<u32>().unwrap_or(0))
                    .collect::<Vec<u32>>(),
            )
        })
        .last()
        .unwrap_or((0, 0, 0, vec![]));

    let size = program.len();
    let mut idx = size - 1;
    let mut final_input: i64 = 0;
    let mut power_vector: Vec<i64> = vec![];
    let mut bins = vec![0; size];
    let mut expect = vec![10; size];

    for i in 0..size {
        power_vector.push(2_i64.pow((i * 3) as u32));
    }

    loop {
        while expect[idx] != program[idx] {
            if bins[idx] < 7 {
                bins[idx] += 1;
            } else {
                //carry over
                bins[idx] = 0;
                expect = vec![10; program.len()];
                idx += 2;
                break;
            }
            final_input = bins.iter().zip(&power_vector).map(|(a, b)| a * b).sum();

            let mut pointer = 0;
            let mut output: Vec<u32> = vec![];
            let mut try_reg_a = final_input;
            let mut try_reg_b = reg_b;
            let mut try_reg_c = reg_c;
            while pointer < program.len() {
                pointer = compute(
                    &pointer,
                    &program[pointer],
                    &program[pointer + 1],
                    &mut try_reg_a,
                    &mut try_reg_b,
                    &mut try_reg_c,
                    &mut output,
                );
            }
            expect = output.clone();
        }
        if idx > 0 {
            idx -= 1;
        } else {
            break;
        }
    }

    final_input
}

fn main() {
    let source = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    assert_eq!(
        (
            0 as i64,
            0 as i64,
            0 as i64,
            "4,6,3,5,6,3,5,2,1,0".to_string()
        ),
        part_1(&source)
    );

    let source = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    assert_eq!(117440, part_2(&source));

    let source = "Register A: 47006051
Register B: 0
Register C: 0

Program: 2,4,1,3,7,5,1,5,0,3,4,3,5,5,3,0";

    assert_eq!(
        (
            0 as i64,
            5 as i64,
            1 as i64,
            "6,2,7,2,3,1,6,0,5".to_string()
        ),
        part_1(&source)
    );

    assert_eq!(236548287712877, part_2(&source));
}
