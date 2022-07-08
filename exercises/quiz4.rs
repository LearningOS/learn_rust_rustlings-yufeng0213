// quiz4.rs
// This quiz covers the sections:
// - Modules
// - Macros

// Write a macro that passes the quiz! No hints this time, you can do it!

macro_rules! my_macro{
    ($name:expr) => {{
        let s: String = format!("{}{}","Hello ".to_string(),$name);
        s
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_macro_world() {
        assert_eq!(my_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }
}
