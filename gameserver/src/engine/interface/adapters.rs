use engine::interface::Input;

// XXX: "make it just work for a while"  implementation!
pub fn str2input(raw_msg: &str) -> Input {
    let mut current = Input::new();
    if raw_msg.contains("w") { current.up = true; }
    if raw_msg.contains("s") { current.down = true; }
    if raw_msg.contains("a") { current.left = true; }
    if raw_msg.contains("d") { current.right = true; }
    return current;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adapter_parses_simple_string() {
        let tested = "w";
        let expected = Input {
            up: true,
            down: false,
            left: false,
            right: false,
            fire: false
        };

        assert_eq!(expected, str2input(tested));
    }

    #[test]
    fn adapter_parses_complex_string() {
        let tested = "dsa\n";
        let expected = Input {
            up: false,
            down: true,
            left: true,
            right: true,
            fire: false
        };

        assert_eq!(expected, str2input(tested));
    }
}
