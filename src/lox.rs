use crate::{
    ast_printer::AstPrinter,
    parser::Parser,
    scanner::Scanner,
    token::{Token, TokenKind}, interpreter::{RuntimeError, Interpreter},
};

#[derive(Clone)]
pub struct Lox {
    had_err: bool,
    had_runtime_err: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_err: false, had_runtime_err: false }
    }

    pub fn interpreter(&self) -> Interpreter {
        Interpreter::new(Box::new(self.clone()))
    }

    // pub fn exe(&mut self, argv: &mut Peekable<IntoIter<&str>>) {
    //     // if argv.len() > 1 {
    //     //     println!("Usage: jlox [script]");
    //     //     exit(64);
    //     // } else if argv.len() == 1 {
    //     //     self.run_file(argv.peek().unwrap().to_string());
    //     // } else {
    //     //     self.run_prompt();
    //     // }

    // }

    // fn run_file(&mut self, path: String) {
    //     let bytes = fs::read(Path::new(&path)).unwrap();

    //     self.run(String::from_utf8(bytes).unwrap_or("".to_owned()));

    //     if self.had_err {
    //         exit(65);
    //     }
    // }

    // pub fn run_prompt(&mut self) {
    //     let input = std::io::stdin();

    //     loop {
    //         print!("> ");
    //         let mut line = String::new();
    //         let i = input.read_line(&mut line);

    //         if i.is_err() {
    //             break;
    //         }

    //         self.run(line);
    //         self.had_err = false;
    //     }
    // }

    pub fn run(&mut self, line: String) {
        let mut tokens = Scanner::new(line, self.clone());
        let mut parser = Parser::new(tokens.scan_tokens().to_vec(), self.clone());
        let statements = parser.parse();

        if self.had_err || statements.is_none() {
            return;
        }

        if self.had_runtime_err {
            panic!("had a runtime error, gn")
        }

        self.interpreter().interpret(Ok(statements.unwrap()))
    }

    pub fn error(&mut self, line: usize, message: String) {
        self.report(line, "".to_owned(), message);
    }

    pub fn error_on(&mut self, token: Token, message: String) {
        self.report(
            token.line,
            if token.kind == TokenKind::Eof {
                " at and".to_string()
            } else {
                " at '".to_string() + &token.lexeme + "'"
            },
            message,
        )
    }

    pub fn runtime_err(&mut self, err: RuntimeError) {
        println!("{}\n[line {}]", err.1, err.0.line);
        self.had_runtime_err = true;
    }

    pub fn report(&mut self, line: usize, here: String, message: String) {
        println!("[line {line}] Error{here}: {message}");
        self.had_err = true;
    }
}
