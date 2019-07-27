extern crate target_lexicon;

use target_lexicon::Triple;

const HOST: Triple = Triple::host();

fn main() {
    println!(
        "{}",
        HOST.pointer_width()
            .expect("architecture should be known")
            .bytes()
    );
}
