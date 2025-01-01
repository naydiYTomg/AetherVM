mod lexer;
mod utils;
mod parser;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::lexer::lexer::Lexer;
    use crate::utils::stringutils::StringBuilder;
    use super::*;

    #[test]
    fn it_works() {
        let mut input = String::from("$funcdef entry {\n\t$vardef res = 1.0f + 2.0f\n\t}");
        let mut lexer = Lexer::new(input);
        lexer.tokenize().iter().for_each(|x| {
            println!("{}", x)
        });

        assert_eq!(1, 1)
    }
}
