use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use std::process::Command;
use std::env;

#[derive(Debug)]
enum Word {
    OpPush(i32),
    OpPlus,
    OpMinus,
    OpEqual,
    OpDump,
    OpIf,
    OpEnd,
    OpElse,
    OpDup,
    OpGt,
    OpWhile,
    OpDo,
}

fn push(x: i32) -> Word {
    Word::OpPush(x)
}
fn plus() -> Word {
    Word::OpPlus
}

fn minus() -> Word {
    Word::OpMinus
}

fn equal() -> Word {
    Word::OpEqual
}

fn dump() -> Word {
    Word::OpDump
}

fn iff() -> Word {
    Word::OpIf
}

fn end() -> Word {
    Word::OpEnd
}

fn elze() -> Word {
    Word::OpElse
}

fn dup() -> Word {
    Word::OpDup
}

fn gt() -> Word {
    Word::OpGt
}

fn wile() -> Word {
    Word::OpWhile
}

fn doo() -> Word {
    Word::OpDo
}

fn usage(compiler_name: &str) {
    println!("Usage: %s <SUBCOMMAND> [ARGS] {compiler_name}");
    println!("SUBCOMMANDS:");
    println!("    sim <file>       Simulate the program");
    println!("    com <file>       Compile the program");
    println!("    help             Print this help to stdout and exit with 0 code");
}

fn cmd_echoed(cmd: Vec<&str>) {
    println!("CMD: {:?}", cmd);
    Command::new(&cmd[0])
        .args(&cmd[1..])
        .spawn()
        .expect("Error: {cmd} failed to execute");
}

#[derive(Debug)]
struct Token {
    file_path: String,
    col: i32,
    row: i32,
    word: Word,
}

fn find_word_and_col(line: String) -> Vec<(String, i32)> {
    let line = line.as_str();
    let mut words: Vec<(String, i32)> = Vec::new();

    let mut col_no: i32 = 1;
    let mut word_start_col: i32 = 1;
    let mut first_space = true;

    let mut word = String::new();
    while col_no <= line.chars().count() as i32 {
        let Some(char) = line.chars().nth(col_no as usize - 1) 
            else {println!("Error: lexer error, no character at this index");exit(1)};

        if !char.is_whitespace() {
            word.push(char);
            first_space = true;
        }
        else if first_space == true {
            words.push((word.to_owned(), word_start_col));
            word = "".to_owned();
            word_start_col = col_no;
            first_space=false;
        }
        else {
            word_start_col = col_no;
        }
        col_no+=1;

    }
    return words;
}

fn lex_file(program_path: &str) -> Result<Vec<Result<Token, Box<dyn Error>>>, Box<dyn Error>> {
    let Ok(file) = File::open(program_path) 
        else {println!("Error: unable to open file {program_path}");exit(1)};
    let reader = BufReader::new(file);
    let Ok(lines): Result<Vec<_>,_> = reader.lines().collect() 
                   else {println!("Error: unable to get lines from {program_path}");exit(1)};
    if lines.len() == 0 {
        return Err("Error: no lines in file".into());
    }
    let mut row_no: i32 = 1;
    let mut tokens: Vec<Result<Token, Box<dyn Error>>> = Vec::new();
    for line in lines {
        let words = find_word_and_col(line);
        for (word, col_no) in words {
            match word.as_str() {
                "+" => {
                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: plus(),
                    };
                    tokens.push(Ok(token));
                },
                "-" => {
                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: minus(),
                    };
                    tokens.push(Ok(token));
                },
                "." => {
                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: dump(),
                    };
                    tokens.push(Ok(token));
                },
                "=" => {
                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: equal(),
                    };
                    tokens.push(Ok(token));
                },
                "if" => {
                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: iff(),
                    };
                    tokens.push(Ok(token));
                },
                "end" => {
                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: end(),
                    };
                    tokens.push(Ok(token));
                },
                "else" => {
                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: elze(),
                    };
                    tokens.push(Ok(token));
                },
                "dup" => {
                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: dup(),
                    };
                    tokens.push(Ok(token));
                },
                ">" => {
                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: gt(),
                    };
                    tokens.push(Ok(token));
                },
                "while" => {
                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: wile(),
                    };
                    tokens.push(Ok(token));
                },
                "do" => {
                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: doo(),
                    };
                    tokens.push(Ok(token));
                },
                _ => {
                    let number = match word.parse::<i32>() {
                        Ok(number) => number,
                        Err(err) => {
                            println!("Error: {program_path}:{row_no}:{col_no}: {word} {err}");
                            exit(1);
                        }
                    };

                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: push(number),
                    };
                    tokens.push(Ok(token));
                },
            }
        }
        row_no += 1;
    }
    return Ok(tokens);
}

fn load_program_from_file(program_path: &str) -> Vec<Result<Token, Box<dyn Error>>> {
    match lex_file(program_path) {
        Ok(vec_token) => {
            return vec_token;
        }
        Err(err) => {
            println!("Error: {}", err);
            exit(1);
        }
    }
}

// fn crossreference_blocks(
//     program: Vec<Result<Word, Box<dyn Error>>>,
// ) -> Vec<Result<Word, Box<dyn Error>>> {
//     !unimplemented!();
// }

fn handle_stack_empty<T>(value_in_stack: Option<T>, token: &Token) -> T {
    match value_in_stack {
        None => {
            println!(
                "Error: {}:{}:{}: stack is empty",
                token.file_path, token.col, token.row
            );
            exit(1);
        }
        Some(x) => return x,
    }
}

fn simulate_program(program: Vec<Result<Token, Box<dyn Error>>>) {
    let mut stack: Vec<i32> = Vec::new();
    for token_res in program {
        match token_res {
            Ok(token) => match token.word {
                Word::OpPush(x) => stack.push(x),
                Word::OpPlus => {
                    let a = handle_stack_empty(stack.pop(), &token);
                    let b = handle_stack_empty(stack.pop(), &token);
                    stack.push(a + b);
                }
                Word::OpMinus => {
                    let a = handle_stack_empty(stack.pop(), &token);
                    let b = handle_stack_empty(stack.pop(), &token);
                    stack.push(b - a);
                }
                Word::OpEqual => {
                    let a = handle_stack_empty(stack.pop(), &token);
                    let b = handle_stack_empty(stack.pop(), &token);
                    stack.push((a == b) as i32);
                }
                Word::OpDump => {
                    println!("{}", handle_stack_empty(stack.pop(), &token));
                }
                Word::OpDup => {
                    let a = handle_stack_empty(stack.pop(), &token);
                    stack.push(a);
                    stack.push(a);
                }
                Word::OpGt => {
                    let a = handle_stack_empty(stack.pop(), &token);
                    let b = handle_stack_empty(stack.pop(), &token);
                    stack.push((a < b) as i32);
                }
                Word::OpIf => println!("hi"),
                Word::OpEnd => println!("hi"),
                Word::OpElse => println!("hi"),
                Word::OpWhile => println!("hi"),
                Word::OpDo => println!("hi"),
            },
            Err(err) => println!("{err:?}"),
        }
    }
}

fn compile_program(program: Vec<Result<Token, Box<dyn Error>>>, output_filename: &str) {
    // Generates assembly file
    let Ok(mut out) = File::options().create(true).write(true).open(output_filename) else {println!("Error: Unable to open file {output_filename}"); exit(1)};
        out.write_to_file("BITS 64\n");
        out.write_to_file("segment .text\n");
        out.write_to_file("dump:\n");
        out.write_to_file("    mov     r9, -3689348814741910323\n");
        out.write_to_file("    sub     rsp, 40\n");
        out.write_to_file("    mov     BYTE [rsp+31], 10\n");
        out.write_to_file("    lea     rcx, [rsp+30]\n");
        out.write_to_file(".L2:\n");
        out.write_to_file("    mov     rax, rdi\n");
        out.write_to_file("    lea     r8, [rsp+32]\n");
        out.write_to_file("    mul     r9\n");
        out.write_to_file("    mov     rax, rdi\n");
        out.write_to_file("    sub     r8, rcx\n");
        out.write_to_file("    shr     rdx, 3\n");
        out.write_to_file("    lea     rsi, [rdx+rdx*4]\n");
        out.write_to_file("    add     rsi, rsi\n");
        out.write_to_file("    sub     rax, rsi\n");
        out.write_to_file("    add     eax, 48\n");
        out.write_to_file("    mov     BYTE [rcx], al\n");
        out.write_to_file("    mov     rax, rdi\n");
        out.write_to_file("    mov     rdi, rdx\n");
        out.write_to_file("    mov     rdx, rcx\n");
        out.write_to_file("    sub     rcx, 1\n");
        out.write_to_file("    cmp     rax, 9\n");
        out.write_to_file("    ja      .L2\n");
        out.write_to_file("    lea     rax, [rsp+32]\n");
        out.write_to_file("    mov     edi, 1\n");
        out.write_to_file("    sub     rdx, rax\n");
        out.write_to_file("    xor     eax, eax\n");
        out.write_to_file("    lea     rsi, [rsp+32+rdx]\n");
        out.write_to_file("    mov     rdx, r8\n");
        out.write_to_file("    mov     rax, 1\n");
        out.write_to_file("    syscall\n");
        out.write_to_file("    add     rsp, 40\n");
        out.write_to_file("    ret\n");
        out.write_to_file("global _start\n");
        out.write_to_file("_start:\n");
    for token_res in &program {
        match token_res {
            Ok(token) => match token.word {
                Word::OpPush(x) => {
                    let msg = format!("    ;; -- push {} --\n", x);
                    let inst = format!("    push {}\n", x);
                    out.write_to_file(msg.as_str());
                    out.write_to_file(inst.as_str());
                },
                Word::OpPlus => {
                    out.write_to_file("    ;; -- plus --\n");
                    out.write_to_file("    pop rax\n");
                    out.write_to_file("    pop rbx\n");
                    out.write_to_file("    add rax, rbx\n");
                    out.write_to_file("    push rax\n");
                },
                Word::OpMinus => {
                    out.write_to_file("    ;; -- minus --\n");
                    out.write_to_file("    pop rax\n");
                    out.write_to_file("    pop rbx\n");
                    out.write_to_file("    sub rbx, rax\n");
                    out.write_to_file("    push rbx\n");
                },
                Word::OpEqual => {
                    out.write_to_file("    ;; -- equal -- \n");
                    out.write_to_file("    mov rcx, 0\n");
                    out.write_to_file("    mov rdx, 1\n");
                    out.write_to_file("    pop rax\n");
                    out.write_to_file("    pop rbx\n");
                    out.write_to_file("    cmp rax, rbx\n");
                    out.write_to_file("    cmove rcx, rdx\n");
                    out.write_to_file("    push rcx\n");
                },
                Word::OpDump => {
                    out.write_to_file("    ;; -- dump --\n");
                    out.write_to_file("    pop rdi\n");
                    out.write_to_file("    call dump\n");
                },
                Word::OpDup => {
                    out.write_to_file("    ;; -- dup -- \n");
                    out.write_to_file("    pop rax\n");
                    out.write_to_file("    push rax\n");
                    out.write_to_file("    push rax\n");
                },
                Word::OpGt => {
                    out.write_to_file("    ;; -- dup -- \n");
                    out.write_to_file("    pop rax\n");
                    out.write_to_file("    push rax\n");
                    out.write_to_file("    push rax\n");
                },
                Word::OpIf => (),
                Word::OpEnd => (),
                Word::OpElse => (),
                Word::OpWhile => (),
                Word::OpDo => (),
            }
            Err(err) => println!("{err:?}"),
        }};

        out.write_fmt(format_args!("addr_{}:\n", program.len())).expect("Error: unable to write to file");
        out.write_to_file("    mov rax, 60\n");
        out.write_to_file("    mov rdi, 0\n");
        out.write_to_file("    syscall\n");
}

trait WriteExt {
    fn write_to_file(&mut self, text: &str);
}

impl WriteExt for File {
    fn write_to_file(&mut self, text: &str) {
        self.write(text.as_bytes()).expect("Error: unable to write to output file");
    }
}

fn main() {
    let mut args: VecDeque<String> = VecDeque::from(env::args().collect::<Vec<String>>());
    assert!(args.len() >= 1, "No. of arguments should be greater than 1");

    let Some(compiler_path) = args.pop_front() 
        else {println!("Error: No compiler path provided");exit(1)};
    let compiler_path = compiler_path.as_str();
    let compiler_name = Path::new(compiler_path).file_name();
    let Some(compiler_name) = compiler_name 
        else {println!("Error: error getting compiler name from path provided");exit(1)};
    let compiler_name = compiler_name.to_str();
    let Some(compiler_name) = compiler_name 
        else {println!("Error: error converting compiler name to string");exit(1)};

    let Some(subcommand) = args.pop_front() 
        else {println!("Error: provide atleast one subcommand"); exit(1)};
    match subcommand.as_str() {
        "-s" | "sim" | "simulate" | "--simulate" => {
            let Some(program_path) = args.pop_front() 
                else {println!("Error: provide file for compilation"); exit(1)};
            let program_path = program_path.as_str();
            let program_path = Path::new(program_path);
            let Some(program_extension) = program_path.extension() 
                else {println!("Error: cannot get extension of file");exit(1)};
            let Some(program_extension) = program_extension.to_str() 
                else {println!("Error: cannot convert file extension to string");exit(1)};
            if program_extension != "rf" {
                println!("Error: not a forth file. Input forth file to compile");
                exit(1);
            }

            let Some(program_path) = program_path.to_str() 
                else {println!("Error: cannot convert file path to string"); exit(1)};
            let program = load_program_from_file(program_path);
            simulate_program(program);
        }
        "-c" | "com" | "compile" | "--compile" => {
            let Some(program_path) = args.pop_front() 
                else {println!("Error: provide file for compilation"); exit(1)};
            let program_path = program_path.as_str();
            let program_path = Path::new(program_path);
            let Some(program_stem) = program_path.file_stem() 
                else {println!("Error: cannot get base name of file");exit(1)};
            let Some(program_stem) = program_stem.to_str() 
                else {println!("Error: cannot convert base name of file to string");exit(1)};
            let output_asm_name = program_stem.to_owned() + &".asm".to_owned();
            let output_obj_name = program_stem.to_owned() + &".o".to_owned();
            let Some(program_extension) = program_path.extension() 
                else {println!("Error: cannot get extension of file");exit(1)};
            let Some(program_extension) = program_extension.to_str() 
                else {println!("Error: cannot convert file extension to string");exit(1)};
            if program_extension != "rf" {
                println!("Error: not a rusty forth file. Input forth file to compile");
                exit(1);
            }
            println!("Info: Generating {}", output_asm_name);
            let Some(program_path) = program_path.to_str() 
                else {println!("Error: cannot convert file path to string"); exit(1)};
            let program = load_program_from_file(program_path);
            compile_program(program, output_asm_name.as_str());
            cmd_echoed(vec!["nasm", "-felf64", output_asm_name.as_str()]);
            cmd_echoed(vec!["ld", "-o", program_stem, output_obj_name.as_str()]);
        }
        "-h" | "help" | "--help" => {
            usage(compiler_name);
            exit(0);
        }
        _ => {
            usage(compiler_name);
            println!("Error: unknown subcommand {}", subcommand);
            exit(1);
        }
    }
}
