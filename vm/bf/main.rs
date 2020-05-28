use std::io::{self, Read, Write, BufReader, BufWriter};
use std::fs::File;

/* std::io::BufReader<std::io::StdinLock<'_>> */

fn read_all_file(fname: &str) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();

    let mut handle = File::open(fname).expect("cannot open file");
    handle.read_to_end(&mut buf).expect("cannot read from file");

    return buf;
}

struct Brainfuck<'a> {
    instructions:   Vec<u8>,
    data:           Vec<u8>,
    ip:             usize,
    dp:             usize,
    bufin:          BufReader<std::io::StdinLock<'a>>,
    bufout:         BufWriter<std::io::StdoutLock<'a>>,
}

impl<'a> Brainfuck<'a> {
    fn step(&mut self) {
        let x = self.instructions[self.ip] as char;

        match x {
            '>' => self.dp = self.dp.wrapping_add(1),
            '<' => self.dp = self.dp.wrapping_sub(1),
            '+' => self.data[self.dp] = self.data[self.dp].wrapping_add(1),
            '-' => self.data[self.dp] = self.data[self.dp].wrapping_sub(1),
            '.' => self.bufout.write_all(&self.data[self.dp .. self.dp+1]).unwrap(),
            ',' => self.bufin.read_exact(&mut self.data[self.dp .. self.dp+1]).unwrap(),
            _ => {},
        };

        if x == '[' && self.data[self.dp] == 0 {
            while self.ip < self.instructions.len() && self.instructions[self.ip] as char != ']' {
                self.ip = self.ip.wrapping_add(1);
            }
        }

        if x == ']' && self.data[self.dp] != 0 {
            while self.ip < self.instructions.len() && self.instructions[self.ip] as char != '[' {
                self.ip = self.ip.wrapping_sub(1);
            }
        }
        
        self.ip = self.ip.wrapping_add(1);
    }

    fn run(&mut self) {
        while self.ip < self.instructions.len() {
            self.step();
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lock_in = stdin.lock();

    let stdout = io::stdout();
    let lock_out = stdout.lock();

    let mut bf = Brainfuck{
        instructions: read_all_file("hello.bf"),
        ip: 0,
        dp: 0,
        data: vec![0; 16],
        bufin: BufReader::new(lock_in),
        bufout: BufWriter::new(lock_out),
    };

    bf.run();
}
