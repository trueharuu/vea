use crate::scanner::Scanner;

#[derive(Copy, Clone)]
pub struct Lox {
    had_err: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_err: false }
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
        let mut tokens = Scanner::new(line, *self);
        println!("{:#?}", tokens.scan_tokens());
    }

    pub fn error(&mut self, line: usize, message: String) {
        self.report(line, "".to_owned(), message);
    }

    pub fn report(&mut self, line: usize, here: String, message: String) {
        println!("[line {line}] Error{here}: {message}");
        self.had_err = true;
    }
}
