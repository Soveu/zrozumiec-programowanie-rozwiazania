use std::io::{self, Read, Write, BufReader, BufWriter};
use std::fs::File;

fn step<In: Read, Out: Write>(
    instructions: &[u8], data: &mut [u8], mut ip: usize, mut dp: usize, 
    bufin: &mut BufReader<In>, bufout: &mut BufWriter<Out>) ->
    (usize, usize)
{
    match instructions[ip] as char {
        '>' => dp = dp.wrapping_add(1),
        '<' => dp = dp.wrapping_sub(1),
        '+' => data[dp] = data[dp].wrapping_add(1),
        '-' => data[dp] = data[dp].wrapping_sub(1),
        '.' => bufout.write_all(&data[dp .. dp+1]).unwrap(),
        ',' => bufin.read_exact(&mut data[dp .. dp+1]).unwrap(),
        '[' if data[dp] == 0 => {
            while let Some(&x) = instructions.get(ip) {
                if x == ']' as u8 { break; }
                ip = ip.wrapping_add(1);
            }
        },
        ']' if data[dp] != 0 => {
            while let Some(&x) = instructions.get(ip) {
                if x == '[' as u8 { break; }
                ip = ip.wrapping_sub(1);
            }
        },
        _ => {},
    };

    return (ip.wrapping_add(1), dp);
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

    let instructions = read_all_file("hello.bf");
    let mut ip = 0;
    let mut dp = 0;
    let mut data = vec![0u8; 16];

    while ip < instructions.len() {
        let(_ip, _dp) = step(&instructions, &mut data, ip, dp, &mut bufin, &mut bufout);
        ip = _ip;
        dp = _dp;
    }
}

