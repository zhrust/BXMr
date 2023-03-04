pub fn echo(word: String) {
    println!("src/inv/seek: {}", env!("CARGO_PKG_VERSION"));
    log::debug!("src/inv/seek: echo() <- {}", word);
}
