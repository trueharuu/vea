#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let (Some(tokens), out) = vea::lex(data) else { return };
    if !out.is_empty() {
        return;
    }

    let (Some(ast), out) = vea::parse(data, &tokens) else { return };
    if !out.is_empty() {
        return;
    }

    let _ = vea::interp(data, &tokens, ast);
});
