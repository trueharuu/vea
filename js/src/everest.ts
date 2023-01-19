import { println } from '@rqft/rust';
import fs from 'node:fs';
import repl from 'node:repl';
import { Scanner } from './scanner';

export class Everest {
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
  }

  private static run_prompt(): void {
    repl.start({
      eval: (cmd) => {
        this.run(cmd);
        this.had_error = false;
      },
    });
  }

  private static run(source: string): void {
    const scanner = new Scanner(source);
    const tokens = scanner.scan_tokens();

    for (const token of tokens) {
      println(String(token));
    }
  }

  public static error(line: number, message: string): void {
    this.report(line, '', message);
  }

  private static had_error = false;

  public static report(line: number, where: string, message: string): void {
    println(`[line ${line}] error${where}: ${message}`);
    this.had_error = true;
  }
}
