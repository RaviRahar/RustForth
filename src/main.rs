use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
enum Word {
    OpPush(i32),
    OpPlus,
    OpMinus,
    OpEqual,
    OpDump,
    OpDup,
    OpGt,
    OpIf(Option<usize>),
    OpEnd(Option<usize>),
    OpElse(Option<usize>),
    OpWhile,
    OpDo(Option<usize>),
}

fn push(num: i32) -> Word {
    Word::OpPush(num)
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

fn dup() -> Word {
    Word::OpDup
}

fn gt() -> Word {
    Word::OpGt
}

fn iff(else_end_idx: Option<usize>) -> Word {
    Word::OpIf(else_end_idx)
}

fn elze(end_idx: Option<usize>) -> Word {
    Word::OpElse(end_idx)
}

fn end(wile_end_idx: Option<usize>) -> Word {
    Word::OpEnd(wile_end_idx)
}

fn wile() -> Word {
    Word::OpWhile
}

fn doo(wile_end_idx: Option<usize>) -> Word {
    Word::OpDo(wile_end_idx)
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

#[derive(Debug, Clone)]
struct Token {
    file_path: String,
    col: usize,
    row: usize,
    word: Word,
}

fn find_word_and_col(line: String) -> Vec<(String, usize)> {
    let line = line.as_str();
    let mut words: Vec<(String, usize)> = Vec::new();

    let mut col_no = 1;
    let mut word_start_col = 1;
    let mut first_space = true;

    let mut word = String::new();
    let total_chars = line.chars().count();
    while col_no <= total_chars {
        let Some(char) = line.chars().nth(col_no - 1) 
            else {println!("Error: lexer error, no character at this index");exit(1)};

        if !char.is_whitespace() {
            word.push(char);
            first_space = true;
            if col_no == total_chars {
                words.push((word.to_owned(), word_start_col));
            }
        } else if first_space == true {
            if !word.trim().is_empty() {
                words.push((word.to_owned(), word_start_col));
            }
            word = "".to_owned();
            word_start_col = col_no;
            first_space = false;
        } else {
            word_start_col = col_no;
        }
        col_no += 1;
    }
    return words;
}

fn lex_file(program_path: &str) -> Vec<Token> {
    let Ok(file) = File::open(program_path) 
        else {println!("Error: unable to open file {program_path}");exit(1)};
    let reader = BufReader::new(file);
    let Ok(lines): Result<Vec<_>,_> = reader.lines().collect() 
                   else {println!("Error: unable to get lines from {program_path}");exit(1)};
    if lines.len() == 0 {
        println!("Error: no lines in file");
        exit(1);
    }
    let mut row_no = 1;
    let mut tokens: Vec<Token> = Vec::new();
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
                    tokens.push(token);
                }
                "-" => {
                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: minus(),
                    };
                    tokens.push(token);
                }
                "." => {
                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: dump(),
                    };
                    tokens.push(token);
                }
                "=" => {
                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: equal(),
                    };
                    tokens.push(token);
                }
                "dup" => {
                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: dup(),
                    };
                    tokens.push(token);
                }
                ">" => {
                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: gt(),
                    };
                    tokens.push(token);
                }
                "if" => {
                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: iff(None),
                    };
                    tokens.push(token);
                }
                "end" => {
                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: end(None),
                    };
                    tokens.push(token);
                }
                "else" => {
                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: elze(None),
                    };
                    tokens.push(token);
                }
                "while" => {
                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: wile(),
                    };
                    tokens.push(token);
                }
                "do" => {
                    let token = Token {
                        file_path: program_path.to_owned(),
                        row: row_no,
                        col: col_no,
                        word: doo(None),
                    };
                    tokens.push(token);
                }
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
                    tokens.push(token);
                }
            }
        }
        row_no += 1;
    }
    return tokens;
}

fn load_program_from_file(program_path: &str) -> Vec<Token> {
    crossreference_blocks(lex_file(program_path))
}

fn crossreference_blocks(program: Vec<Token>) -> Vec<Token> {
    let mut stack: Vec<usize> = Vec::new();
    let mut out_program: Vec<Token> = Vec::new();
    for token_idx in 0..program.len() {
        let token = &program[token_idx];
        match token.word {
            Word::OpIf(_else_end_idx) => {
                stack.push(token_idx);

                out_program.push(Token { ..(*token).clone() });
            }
            Word::OpElse(_end_idx) => {
                let if_idx = handle_stack_empty(stack.pop(), token);
                assert_eq!(program[if_idx].word, Word::OpIf(None));
                out_program[if_idx].word = Word::OpIf(Some(token_idx + 1));
                stack.push(token_idx);

                out_program.push(Token { ..(*token).clone() });
            }
            Word::OpEnd(mut _wile_end_idx) => {
                let block_idx = handle_stack_empty(stack.pop(), token);
                match program[block_idx].word {
                    Word::OpIf(_end_idx) => {
                        out_program[block_idx] = Token {
                            word: Word::OpIf(Some(token_idx)),
                            ..(*token).clone()
                        };
                        out_program.push(Token {
                            word: Word::OpEnd(Some(token_idx + 1)),
                            ..(*token).clone()
                        });
                    }
                    Word::OpElse(_end_idx) => {
                        out_program[block_idx] = Token {
                            word: Word::OpElse(Some(token_idx)),
                            ..(*token).clone()
                        };
                        out_program.push(Token {
                            word: Word::OpEnd(Some(token_idx + 1)),
                            ..(*token).clone()
                        });
                    }
                    Word::OpDo(_wile_idx) => {
                        if let Word::OpDo(Some(wile_idx)) = out_program[block_idx].word {
                            assert_eq!(out_program[wile_idx].word, Word::OpWhile);
                            out_program[block_idx] = Token {
                                word: Word::OpDo(Some(token_idx + 1)),
                                ..(*token).clone()
                            };
                            out_program.push(Token {
                                word: Word::OpEnd(Some(wile_idx)),
                                ..(*token).clone()
                            });
                        };
                    }
                    _ => {
                        println!("Error: end can only close 'if', 'else' or 'do' blocks");
                        exit(1);
                    }
                }
            }
            Word::OpWhile => {
                stack.push(token_idx);

                out_program.push(Token { ..(*token).clone() });
            }
            Word::OpDo(_wile_end_idx) => {
                let wile_end_idx = Some(handle_stack_empty(stack.pop(), token));
                stack.push(token_idx);

                out_program.push(Token {
                    word: Word::OpDo(wile_end_idx),
                    ..(*token).clone()
                });
            }
            _ => {
                out_program.push(Token { ..(*token).clone() });
            }
        }
    }
    return out_program;
}

fn handle_stack_empty<T>(value_in_stack: Option<T>, token: &Token) -> T {
    match value_in_stack {
        None => {
            println!(
                "Error: {}:{}:{}: stack is empty",
                token.file_path, token.row, token.col
            );
            exit(1);
        }
        Some(x) => return x,
    }
}

fn simulate_program(program: Vec<Token>) {
    let mut stack: Vec<i32> = Vec::new();
    let mut token_idx = 0;
    while token_idx < program.len() {
        let token = &program[token_idx];
        match program[token_idx].word {
            Word::OpPush(num) => stack.push(num),
            Word::OpPlus => {
                let a = handle_stack_empty(stack.pop(), token);
                let b = handle_stack_empty(stack.pop(), token);
                stack.push(a + b);
            }
            Word::OpMinus => {
                let a = handle_stack_empty(stack.pop(), token);
                let b = handle_stack_empty(stack.pop(), token);
                stack.push(b - a);
            }
            Word::OpEqual => {
                let a = handle_stack_empty(stack.pop(), token);
                let b = handle_stack_empty(stack.pop(), token);
                stack.push((a == b) as i32);
            }
            Word::OpDump => {
                println!("{}", handle_stack_empty(stack.pop(), token));
            }
            Word::OpDup => {
                let a = handle_stack_empty(stack.pop(), token);
                stack.push(a);
                stack.push(a);
            }
            Word::OpGt => {
                let a = handle_stack_empty(stack.pop(), token);
                let b = handle_stack_empty(stack.pop(), token);
                stack.push((a < b) as i32);
            }
            Word::OpIf(else_end_idx) => {
                let a = handle_stack_empty(stack.pop(), token);
                if a == 0 {
                    let Some(else_end_idx) = else_end_idx 
                            else {println!("Error: 'if' does not have reference to end of block"); exit(1)};
                    token_idx = else_end_idx - 1;
                }
            }
            Word::OpElse(end_idx) => {
                let Some(end_idx) = end_idx 
                        else {println!("Error: 'else' does not have reference to end of block"); exit(1)};
                token_idx = end_idx - 1;
            }
            Word::OpEnd(wile_end_idx) => {
                let Some(wile_end_idx) = wile_end_idx 
                        else {println!("Error: 'end' does not have reference to while block or next instruction"); exit(1)};
                token_idx = wile_end_idx - 1;
            }
            Word::OpWhile => (),
            Word::OpDo(end_idx) => {
                let a = handle_stack_empty(stack.pop(), token);
                if a == 0 {
                    let Some(end_idx) = end_idx
                            else {println!("Error: 'do' does not have reference to end of block"); exit(1)};
                    token_idx = end_idx - 1;
                }
            }
        }
        token_idx += 1;
    }
}

fn compile_program(program: Vec<Token>, output_filename: &str) {
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
    let mut token_idx = 0;
    while token_idx < program.len() {
        let token = &program[token_idx];
        let msg = format!("addr_{}:\n", token_idx);
        out.write_to_file(&msg);
        match token.word {
            Word::OpPush(num) => {
                let msg = format!("    ;; -- push {} --\n", num);
                let inst = format!("    push {}\n", num);
                out.write_to_file(msg.as_str());
                out.write_to_file(inst.as_str());
            }
            Word::OpPlus => {
                out.write_to_file("    ;; -- plus --\n");
                out.write_to_file("    pop rax\n");
                out.write_to_file("    pop rbx\n");
                out.write_to_file("    add rax, rbx\n");
                out.write_to_file("    push rax\n");
            }
            Word::OpMinus => {
                out.write_to_file("    ;; -- minus --\n");
                out.write_to_file("    pop rax\n");
                out.write_to_file("    pop rbx\n");
                out.write_to_file("    sub rbx, rax\n");
                out.write_to_file("    push rbx\n");
            }
            Word::OpEqual => {
                out.write_to_file("    ;; -- equal -- \n");
                out.write_to_file("    mov rcx, 0\n");
                out.write_to_file("    mov rdx, 1\n");
                out.write_to_file("    pop rax\n");
                out.write_to_file("    pop rbx\n");
                out.write_to_file("    cmp rax, rbx\n");
                out.write_to_file("    cmove rcx, rdx\n");
                out.write_to_file("    push rcx\n");
            }
            Word::OpDump => {
                out.write_to_file("    ;; -- dump --\n");
                out.write_to_file("    pop rdi\n");
                out.write_to_file("    call dump\n");
            }
            Word::OpDup => {
                out.write_to_file("    ;; -- dup -- \n");
                out.write_to_file("    pop rax\n");
                out.write_to_file("    push rax\n");
                out.write_to_file("    push rax\n");
            }
            Word::OpGt => {
                out.write_to_file("    ;; -- gt --\n");
                out.write_to_file("    mov rcx, 0\n");
                out.write_to_file("    mov rdx, 1\n");
                out.write_to_file("    pop rbx\n");
                out.write_to_file("    pop rax\n");
                out.write_to_file("    cmp rax, rbx\n");
                out.write_to_file("    cmovg rcx, rdx\n");
                out.write_to_file("    push rcx\n");
            }
            Word::OpIf(else_end_idx) => {
                out.write_to_file("    ;; -- if --\n");
                out.write_to_file("    pop rax\n");
                out.write_to_file("    test rax, rax\n");
                let Some(else_end_idx) = else_end_idx 
                        else {println!("Error: 'if' does not have reference to end of block"); exit(1)};
                let msg = format!("    jz addr_{}\n", else_end_idx);
                out.write_to_file(msg.as_str());
            }
            Word::OpElse(end_idx) => {
                out.write_to_file("    ;; -- else --\n");
                let Some(end_idx) = end_idx 
                    else {println!("Error: 'else' does not have reference to end of block"); exit(1)};
                let msg = format!("    jmp addr_{}\n", end_idx);
                out.write_to_file(msg.as_str());
            }
            Word::OpEnd(wile_end_idx) => {
                let Some(wile_end_idx) = wile_end_idx 
                    else {println!("Error: 'end' does not have reference to while block or next instruction"); exit(1)};
                out.write_to_file("    ;; -- end --\n");
                if (token_idx + 1) != wile_end_idx {
                    let msg = format!("    jmp addr_{}\n", wile_end_idx);
                    out.write_to_file(msg.as_str());
                }
            }
            Word::OpWhile => out.write_to_file("    ;; -- while --\n"),
            Word::OpDo(end_idx) => {
                out.write_to_file("    ;; -- do --\n");
                out.write_to_file("    pop rax\n");
                out.write_to_file("    test rax, rax\n");
                let Some(end_idx) = end_idx
                        else {println!("Error: 'do' does not have reference to end of block"); exit(1)};
                let msg = format!("    jz addr_{}\n", end_idx);
                out.write_to_file(msg.as_str());
            }
        }
        token_idx += 1;
    }

    out.write_fmt(format_args!("addr_{}:\n", program.len()))
        .expect("Error: unable to write to file");
    out.write_to_file("    mov rax, 60\n");
    out.write_to_file("    mov rdi, 0\n");
    out.write_to_file("    syscall\n");
}

trait WriteExt {
    fn write_to_file(&mut self, text: &str);
}

impl WriteExt for File {
    fn write_to_file(&mut self, text: &str) {
        self.write(text.as_bytes())
            .expect("Error: unable to write to output file");
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
