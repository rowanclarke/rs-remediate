mod document;

use document::parse;

fn main() {
    println!(
        "{:?}",
        parse("<c8f42950 (A)[Front] and (B)[back]>\n<d6fc2934 (A)[Only] one (A)[card]>")
    );
}
