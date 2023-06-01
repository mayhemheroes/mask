#![no_main]
use libfuzzer_sys::fuzz_target;
use mask_parser::parser::parse;

fuzz_target!(|maskfile_contents: String| {
    
    let _ = parse(maskfile_contents);
});