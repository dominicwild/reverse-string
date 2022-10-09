pub fn reverse(input: &str) -> String {
    let mut input_chars: Vec<char> = input.chars().collect();
    let mut reverse_chars = vec![];

    for _ in input.chars() {
        let char = input_chars.pop().unwrap();
        reverse_chars.push(char);
    }

    reverse_chars.into_iter().collect()
}
