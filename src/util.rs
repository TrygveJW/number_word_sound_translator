pub fn strip_newline(inp: String) -> String {
    return match inp.strip_suffix("\n") {
        None => inp,
        Some(strpped) => String::from(strpped),
    };
}
