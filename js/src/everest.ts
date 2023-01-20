import { Parser } from './parser';
import type { RuntimeError } from './runtime_error';
import { Scanner } from './scanner';
import type { Token } from './token';
import { TokenKind } from './token_kind';

export class Everest {
  // private static readonly interpreter = new Interpreter();
  private static had_error = false;
  private static had_runtime_error = false;
  public static run(source: string): void {
    const scanner = new Scanner(source);
    const tokens = scanner.scan_tokens();

    const parser = new Parser(tokens);

    const statements = parser.parse();

    if (this.had_error) {
      return process.exit(64);
    }

    // const resolver = new Resolver(this.interpreter);
    // resolver.resolve(statements);

    // eslint-disable-next-line @typescript-eslint/no-unnecessary-condition
    if (this.had_error) {
      return process.exit(64);
    }

    // this.interpreter.interpret(statements);

    if (this.had_runtime_error) {
      process.exit(70);
    }
  }

  public static error(line: number, message: string): void {
    this.report(line, '', message);
  }

  private static report(line: number, where: string, message: string): void {
    console.error(`[line ${line}] error${where}: ${message}`);
    this.had_error = true;
  }

  public static error_with(token: Token, message: string): void {
    if (token.kind === TokenKind.Eof) {
      this.report(token.line, ' at end', message);
    } else {
      this.report(token.line, `at '${token.lexeme}'`, message);
    }
  }

  public static runtime_error(error: RuntimeError): void {
    console.error(`${error.message}\n[line ${error.token.line}]`);
    this.had_runtime_error = true;
  }
}
