// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// As always, there are hints if you execute `rustlings hint iterators2`!


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    let mut s = "".to_string();
    match c.next() {
        None => {String::new()},
        Some(first) => {
            s.push((first as u8 - 32) as char);
            s.push_str(&input[1..input.len()]);
            s
        },  
    }
    
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let s = words.iter();
    let mut v: Vec<String> = Vec::new();

    for val in s{
        v.push(capitalize_first(*val));        
    }
    v
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let s = words.iter();
    let mut st = String::new();
 
     for val in s{
         if *val == " "{
             st.push_str(*val);
         }else{
             st.push_str(&capitalize_first(*val));
         }
     }
    st
 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
