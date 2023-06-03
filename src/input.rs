use console::Term;

pub fn read_line(max_length: usize) -> Result<String, &'static str> {
    let mut input = String::new();
    let len = std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input = input.replace('\n', "");

    if len - 1 <= max_length {
        Ok(input)
    } else {
        Err("Input too long!")
    }
}

pub fn read_keypress() -> char {
    let term = Term::stdout();
    loop {
        let char = Term::read_char(&term);
        return match char {
            Ok(value) => value,
            Err(_) => continue,
        };
    }
}
