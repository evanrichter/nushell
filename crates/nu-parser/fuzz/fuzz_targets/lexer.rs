#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: &[u8]| {
    let _ = nu_parser::lex(input, 0, &[], &[], false);
});
