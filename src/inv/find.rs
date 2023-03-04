pub fn echo(code: String) {
    println!("src/inv/find: {}", env!("CARGO_PKG_VERSION"));
    log::debug!("src/inv/find: echo() ~> {}", code);
}
