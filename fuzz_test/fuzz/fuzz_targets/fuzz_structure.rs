#![no_main]
use url::Rgb;

libfuzzer_sys::fuzz_target!(|color: Rgb| {
    let hsl = color.clone();
    let mut rgb = hsl;

    // This should be true for all RGB -> HSL -> RGB conversions!
    assert_eq!(color, rgb);
});
