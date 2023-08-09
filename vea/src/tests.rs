// #[test]
// fn main() {
//     let txt = "if (true) {}";
//     let tokens = super::lex(txt).0.unwrap();
//     let exprs = super::parse(txt, tokens.as_slice()).0.unwrap();

//     let mut env = super::interpreter::Env::default();
//     // let value = super::interpreter::interp(&mut env, exprs.first().unwrap().clone());

//     // dbg!(value);
//     dbg!(super::interpreter::exec(exprs, &mut env)).unwrap();
// }
