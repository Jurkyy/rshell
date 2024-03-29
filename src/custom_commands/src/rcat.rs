use std::fs::*;
use std::io::*;
use std::path::*;

const BUFFER_SIZE: usize = 16 * 1024;

struct Flags {
    line_no: isize,
    filename: bool,
    number: bool,
}

fn _help() {
    print!(concat!(
        "Usage: ",
        env!("CARGO_PKG_NAME"),
        " [OPTIONS]... [FILE]...\n\n",
        "OPTIONS:\n",
        "  -f, --filenames, --filename   display filenames\n",
        "  -n, --number                  number all output lines \n",
        "  -?, --help                    display this help and exit\n"
    ));
}

fn _error(path: &Path, message: &Error) {
    let text: String = message.to_string();
    eprintln!("{}: {}: {}", env!("CARGO_PKG_NAME"), path.display(), text);
    std::io::stderr().flush().unwrap();
}

pub fn rcat(args: &[&str]) {
    let mut file_names = Vec::new();
    let mut flags = Flags {
        line_no: 1,
        filename: false,
        number: false,
    };
    for arg in args {
        if arg.starts_with('-') && arg.len() > 1 {
            if *arg == "-f" || *arg == "--filenames" || *arg == "--filename" {
                flags.filename = true;
            } else if *arg == "-n" || *arg == "--number" {
                flags.number = true;
            } else if *arg == "-h" || *arg == "--help" {
                _help();
            } else {
                println!("unrecognized option -- '{}'", arg);
            }
        } else {
            file_names.push(arg.to_string())
        }
    }

    let mut buffer = [0u8; BUFFER_SIZE];

    for file_name in file_names {
        let mut bytes = 0;
        let mut byte = [0];
        let mut last = [10];
        let mut _count = 0;
        let _path = Path::new(&file_name);
        flags.line_no = 1;
        match File::open(_path) {
            Ok(ref mut file) => {
                while {
                    match file.read(&mut buffer) {
                        Ok(_bytes) => _count = _bytes,
                        Err(_err) => {
                            _error(_path, &_err);
                            _count = 0
                        }
                    }
                    _count
                } > 0
                {
                    if flags.filename && bytes == 0 {
                        eprintln!("{}", _path.display())
                    }
                    bytes += _count;
                    // 0..count instead of u8, to avoid needless memory allocations.
                    for index in 0.._count {
                        byte[0] = buffer[index];
                        if last[0] == 10 {
                            if flags.number {
                                print!("{0:4}\t", flags.line_no);
                            }
                            flags.line_no += 1;
                        }
                        print!("{}", byte[0] as char);
                        std::io::stdout().flush().unwrap();

                        last = byte;
                    }
                }
                println!();
            }
            Err(_message) => _error(_path, &_message),
        }
    }
}
