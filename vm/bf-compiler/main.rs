use std::io::{self, Read, Write, BufWriter};
use std::fs::File;

fn read_all_file(fname: &str) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();

    let mut handle = File::open(fname).expect("cannot open file");
    handle.read_to_end(&mut buf).expect("cannot read from file");

    return buf;
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let lock_out = stdout.lock();
    let mut bufout = BufWriter::new(lock_out);

    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("pass filename as argument");

    let instructions = read_all_file(&filename);
    let mut left_brackets: u64 = 1;
    let mut right_brackets: u64 = 1;

    write!(bufout, "%include \"vm.inc\"\n\n\
                    \tvset r0, 0xFFFF\n\
                    start:\n")?;

    for instruction in instructions {
        match instruction as char {
            '>' => { 
                write!(bufout, "\tvset r2, 1\n\
                                \tvsub r0, r2\n")?; 
            },
            '<' => { 
                write!(bufout, "\tvset r2, 1\n\
                                \tvadd r0, r2\n")?; 
            },
            '+' => {
                write!(bufout, "\tvset r2, 1\n\
                                \tvldb r1, r0\n\
                                \tvadd r1, r2\n\
                                \tvstb r0, r1\n")?; 
            },
            '-' => {
                write!(bufout, "\tvset r2, 1\n\
                                \tvldb r1, r0\n\
                                \tvsub r1, r2\n\
                                \tvstb r0, r1\n")?;
            },
            '.' => {
                write!(bufout, "\tvldb r1, r0\n\
                                \tvoutb 0x20, r1\n")?;
            },
            ',' => {
                write!(bufout, "\tvinb 0x20, r1\n\
                                \tvstb r0, r1\n")?;
            },
            '[' => {
                write!(bufout, "\tvldb r1, r0\n\
                                \tvset r2, 0\n\
                                \tvcmp r1, r2\n")?;
                write!(bufout, "\tvjz rbracket_{}\nlbracket_{}:\n", right_brackets, left_brackets)?;
                left_brackets += 1;
            }
            ']' => {
                write!(bufout, "\tvldb r1, r0\n\
                                \tvset r2, 0\n\
                                \tvcmp r1, r2\n")?;
                write!(bufout, "\tvjnz lbracket_{}\nrbracket_{}:\n", left_brackets-1, right_brackets)?;
                right_brackets += 1;
            }
            c if c.is_whitespace() => continue,
            c   => {
                println!("INVALID INSTRUCTION '{}'", c);
                return Ok(());
            },
        }
    }
    
    write!(bufout, "\tvoff\n")?;

    Ok(())
}
