import { eprintln, println } from '@rqft/rust';
import fs from 'node:fs';
import repl from 'node:repl';
import { Interpreter } from './interpreter';
import { Parser } from './parser';
import { RuntimeError } from './runtime_error';
import { Scanner } from './scanner';
import type { Stmt } from './stmt';
import type { Token } from './token';
import { TokenKind } from './token_kind';

export class Everest {
  private static readonly interpreter = new Interpreter();
  public static main(args: Array<string>): void {
    if (args.length > 1) {
      println('usage: eve [script]');
      process.exit(0x40);
    } else if (args.length == 1) {
      this.run_file(args[0] as never);
    } else {
      this.run_prompt();
    }
  }

  private static run_file(path: string): void {
    this.run(fs.readFileSync(path, 'utf-8'));
    if (this.had_error) {
      process.exit(0x41);
    }

    if (this.had_runtime_error) {
      process.exit(0x46);
    }


  }

  private static run_prompt(): void {
    repl.start({
      eval: (cmd) => {
        this.run(cmd);
        this.had_error = false;
      },
    });
  }

  public static run(source: string): void {
    const scanner = new Scanner(source);
    const tokens = scanner.scan_tokens();

    const parser = new Parser(tokens);
    const statements = parser.parse();

    if (this.had_error) { return; }


    this.interpreter.interpret(statements as Array<Stmt>);
  }

  public static error(line: number, message: string): void {
    this.report(line, '', message);
  }

  public static error_with(token: Token, message: string): void {
    if (token.kind === TokenKind.Eof) {
      this.report(token.line, ' at end', message);
    } else {
      this.report(token.line, ' at \'' + token.lexeme + '\'', message);
    }
  }

  private static had_error = false;

  public static report(line: number, where: string, message: string): void {
    println(`[line ${line}] error${where}: ${message}`);
    this.had_error = true;
  }

  private static had_runtime_error = false;
  public static runtime_error(error: RuntimeError): void {
    if (!(error instanceof RuntimeError)) { console.error(error); return; }
    eprintln(error.message + '\n[line ' + error.token.line + ']');
    this.had_runtime_error = true;
  }
}
