use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const PART_2: bool = true;

// true => rA, false => rB. yay arbitrary encoding.
#[derive(Debug)]
enum Insn {
    HALF(bool),
    TRIPLE(bool),
    INC(bool),
    JMP(i32),
    JIE(bool, i32),
    JIO(bool, i32)
}

struct Machine {
    a: u32,
    b: u32,
    pc: u32
}

fn is_reg_a(insn: &str) -> bool {
    return &insn[4..5] == "a";
}

fn parse_jump_target(s: &str) -> i32 {
    return s.parse().expect("invalid jump target");
}

fn decode(s: String) -> Insn {
    match &s[0..3] {
        "hlf" => Insn::HALF(is_reg_a(&s)),
        "tpl" => Insn::TRIPLE(is_reg_a(&s)),
        "inc" => Insn::INC(is_reg_a(&s)),
        "jmp" => Insn::JMP(parse_jump_target(&s[4..])),
        "jie" => Insn::JIE(is_reg_a(&s), parse_jump_target(&s[7..])),
        "jio" => Insn::JIO(is_reg_a(&s), parse_jump_target(&s[7..])),
        _ => panic!("bad {}", &s[0..3])
    }
}

pub fn run() {
    let rdr = BufReader::new(File::open("d23_input.txt").expect("Couldn't open input file!"));

    let insns = rdr.lines()
        .map(|r| decode(r.expect("Failure reading line")))
        .collect::<Vec<_>>();

    let mut machine = if PART_2 {
        Machine { a: 1, b: 0, pc: 0 }
    } else {
        Machine { a: 0, b: 0, pc: 0 }
    };

    loop {
        if machine.pc as usize >= insns.len() {
            break;
        }

        match insns[machine.pc as usize] {
            Insn::HALF(a) => { if a { machine.a /= 2; } else { machine.b /= 2; }; machine.pc += 1; },
            Insn::TRIPLE(a) => { if a { machine.a *= 3; } else { machine.b *= 3 }; machine.pc += 1; },
            Insn::INC(a) => { if a { machine.a += 1; } else { machine.b += 1 }; machine.pc += 1; },
            Insn::JMP(offset) => machine.pc = (machine.pc as i32 + offset) as u32,
            Insn::JIE(a, offset) => {
                let val = if a { machine.a } else { machine.b };
                if val % 2 == 0 {
                    machine.pc = (machine.pc as i32 + offset) as u32;
                } else {
                    machine.pc += 1;
                }
            },
            Insn::JIO(a, offset) => {
                let val = if a { machine.a } else { machine.b };
                if val == 1 {
                    machine.pc = (machine.pc as i32 + offset) as u32;
                } else {
                    machine.pc += 1;
                }
            }
        }
    }

    println!("{}", machine.b);
}