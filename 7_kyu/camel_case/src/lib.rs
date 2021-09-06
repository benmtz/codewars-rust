#[cfg(test)]
mod tests {

    // Write simple .camelCase method (camel_case function in PHP, CamelCase in C# or camelCase in Java) for strings.
    // All words must have their first letter capitalized without spaces.

    // For instance:
    
    // camel_case("hello case"); // => "HelloCase"
    // camel_case("camel case word"); // => "CamelCaseWord"

    fn camel_case(str: &str) -> String {
        let str2: String = str.to_string();
        str2.split_whitespace()
          .map(|word| {
            let w = word;
            let first_letter = match w.get(..1) {
              Some(letter) => letter.to_uppercase(),
              None => "".to_string() 
            };
            let word_end = match w.get(1..) {
              Some(letter) => String::from(letter),
              None => "".to_string()
            };
            first_letter + &word_end
          })
          .collect()
    }

    // https://www.codewars.com/kata/587731fda577b3d1b0001196/solutions/rust

    // More concise from solutions
    // fn camel_case(str: &str) -> String {
    //   str.split_whitespace()
    //       .map(|s| [&s[..1].to_uppercase(), &s[1..]].join(""))
    //       .collect()
    // }

    #[test]
    fn sample_test() {
      assert_eq!(camel_case("test case"), "TestCase");
      assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
      assert_eq!(camel_case("say hello "), "SayHello");
      assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
      assert_eq!(camel_case(""), "");
    }
}
