use std::io::{self, Read, Write, BufReader, BufWriter};
use std::fs::File;

#[derive(Debug)]
struct Brainfuck {
    data:   Vec<u8>,
    ip:     usize,
    dp:     usize,
}

#[derive(Debug)]
enum BrainfuckError {
    InvalidInstruction(char),
    OutOfBounds((usize, usize)),
    MissingRightBracket,
    MissingLeftBracket,
    IOError(io::Error),
}

type BrainfuckResult = Result<Brainfuck, BrainfuckError>;

fn inc_instruction_ptr(bf: Brainfuck) -> BrainfuckResult {
    Ok(Brainfuck{
        data:   bf.data,
        ip:     bf.ip.wrapping_add(1),
        dp:     bf.dp,
    })
}

fn decrement_data_at_ptr(mut bf: Brainfuck) -> BrainfuckResult {
    match bf.data.get_mut(bf.dp) {
        Some(x) => *x = x.wrapping_sub(1),
        None    => return Err(BrainfuckError::OutOfBounds((bf.dp, bf.data.len()))), 
    };

    return inc_instruction_ptr(bf);
}

fn increment_data_at_ptr(mut bf: Brainfuck) -> BrainfuckResult {
    match bf.data.get_mut(bf.dp) {
        Some(x) => *x = x.wrapping_add(1),
        None    => return Err(BrainfuckError::OutOfBounds((bf.dp, bf.data.len()))), 
    };

    return inc_instruction_ptr(bf);
}

fn decrement_data_ptr(bf: Brainfuck) -> BrainfuckResult {
    Ok(Brainfuck{
        data: bf.data,
        ip: bf.ip.wrapping_add(1),
        dp: bf.dp.wrapping_sub(1),
    })
}

fn increment_data_ptr(bf: Brainfuck) -> BrainfuckResult {
    Ok(Brainfuck{
        data: bf.data,
        ip: bf.ip.wrapping_add(1),
        dp: bf.dp.wrapping_add(1),
    })
}

fn write_byte<Out: Write>(bf: Brainfuck, bufout: &mut BufWriter<Out>)-> BrainfuckResult {
    if bf.dp >= bf.data.len() {
        return Err(BrainfuckError::OutOfBounds((bf.dp, bf.data.len())));
    }

    bufout.write_all(&bf.data[bf.dp .. bf.dp+1])
        .map_err(|e| BrainfuckError::IOError(e))?;
    return inc_instruction_ptr(bf);
}


fn read_byte<In: Read>(mut bf: Brainfuck, bufin: &mut BufReader<In>) -> BrainfuckResult {
    if bf.dp >= bf.data.len() {
        return Err(BrainfuckError::OutOfBounds((bf.dp, bf.data.len())));
    }

    bufin.read_exact(&mut bf.data[bf.dp .. bf.dp+1])
        .map_err(|e| BrainfuckError::IOError(e))?;
    return inc_instruction_ptr(bf);
}

fn right_bracket_jump(mut bf: Brainfuck, instructions: &[u8]) -> BrainfuckResult {
    match bf.data.get(bf.dp) {
        Some(0) => return inc_instruction_ptr(bf),
        None    => return Err(BrainfuckError::OutOfBounds((bf.dp, bf.data.len()))),
        _       => { /* continue execution */ },
    }

    loop {
        bf.ip = bf.ip.wrapping_sub(1);
        match instructions.get(bf.ip) {
            Some(91) => break,
            Some(_)  => continue,
            None     => return Err(BrainfuckError::MissingRightBracket),
        }
    }

    return inc_instruction_ptr(bf);
}

fn left_bracket_jump(mut bf: Brainfuck, instructions: &[u8]) -> BrainfuckResult {
    match bf.data.get(bf.dp) {
        Some(0) => { /* continue execution */ },
        None    => return Err(BrainfuckError::OutOfBounds((bf.dp, bf.data.len()))),
        _       => return inc_instruction_ptr(bf),
    }

    loop {
        bf.ip = bf.ip.wrapping_add(1);
        match instructions.get(bf.ip) {
            Some(93) => break,
            Some(_)  => continue,
            None     => return Err(BrainfuckError::MissingLeftBracket),
        }
    }

    return inc_instruction_ptr(bf);
}

fn step<In: Read, Out: Write>(
    instructions: &[u8], bf: Brainfuck, 
    bufin: &mut BufReader<In>, bufout: &mut BufWriter<Out>) ->
    BrainfuckResult
{
    match instructions[bf.ip] as char {
        '>' => increment_data_ptr(bf),
        '<' => decrement_data_ptr(bf),
        '+' => increment_data_at_ptr(bf),
        '-' => decrement_data_at_ptr(bf),
        '.' => write_byte(bf, bufout),
        ',' => read_byte(bf, bufin),
        '[' => left_bracket_jump(bf, instructions),
        ']' => right_bracket_jump(bf, instructions),
        c if c.is_whitespace() => inc_instruction_ptr(bf),
        c => Err(BrainfuckError::InvalidInstruction(c))
    }
}

fn read_all_file(fname: &str) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();

    let mut handle = File::open(fname).expect("cannot open file");
    handle.read_to_end(&mut buf).expect("cannot read from file");

    return buf;
}

fn main() {
    let stdin = io::stdin();
    let lock_in = stdin.lock();
    let mut bufin = BufReader::new(lock_in);

    let stdout = io::stdout();
    let lock_out = stdout.lock();
    let mut bufout = BufWriter::new(lock_out);

    let instructions = read_all_file(&std::env::args().skip(1).next().expect("pass filename as argument"));
    let mut bf = Brainfuck {
        ip: 0,
        dp: 0,
        data: vec![0u8; 16],
    };

    while bf.ip < instructions.len() {
        match step(&instructions, bf, &mut bufin, &mut bufout) {
            Ok(x) => bf = x,
            Err(BrainfuckError::InvalidInstruction(c)) => {
                println!("Invalid instruction '{}'", c);
                break;
            },
            Err(BrainfuckError::OutOfBounds((addr, memorysize))) => {
                println!("Tried to access memory at address {}, while its size is {}", addr, memorysize);
                break;
            },
            Err(BrainfuckError::IOError(e)) => {
                println!("IOError {:?}", e);
                break;
            },
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        }
    }
}

