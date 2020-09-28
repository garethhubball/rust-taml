#[derive(Debug, PartialEq)]
struct Parsed<'a, T> {
    result: T,
    rest: &'a str
}

type ParseResult<'a, T> = Result<Parsed<'a, T>, &'a str>;

fn pchar(to_parse: char) -> impl Fn(&str) -> ParseResult<char> {
    move |text | if text.starts_with(to_parse) {
        Ok(Parsed { result: to_parse, rest: text.split_at(1).1 })
    } else {
        Err("Character not found")
    }
}

fn pword(to_parse: &str) -> impl Fn(&str) -> ParseResult<String> + '_ {
    move |text| {
        let parsers = to_parse.chars().map(pchar);
        let acc = Ok(Parsed {result: "".to_string(), rest: text});
        parsers.fold(acc, |acc, parser| {
            let Parsed {result: a_result, rest} = acc?;
            let next = parser(rest);
            let Parsed {result: c_result, rest: c_rest} = next?;
            Ok(Parsed { result: format!("{}{}", a_result, c_result), rest: c_rest })
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::*;

    #[test]
    fn parsing_a_character_works() {
        let expected = Ok(Parsed { result: 'a', rest: "nd" });
        let parser = pchar('a');
        assert_eq!(expected, parser("and"));
    }

    #[test]
    fn an_invalid_character_fails() {
        let expected = Err("Character not found");
        let parser = pchar('b');
        assert_eq!(expected, parser("and"));
    }

    #[test]
    fn parsing_a_word_works() {
        let expected = Ok(Parsed {result: "Hello".to_string(), rest: ", World"});
        let parser = pword("Hello");
        assert_eq!(expected, parser("Hello, World"));
    }

    #[test]
    fn parsing_the_wrong_word_fails() {
        let expected = Err("Character not found");
        let parser = pword("Hello");
        assert_eq!(expected, parser("Hey, World"));
    }

    /*
    macro_rules! parse_thing {
        ($left:ident .>>. $($right:tt)+ ) => {
            $left + parse_thing!($($right)+);
        };
        ($left:ident .<<. $($right:tt)+ ) => {
            $left - parse_thing!($($right)+);
        };
        ($final:ident) => {
            $final;
        }
    }


    #[test]
    #[ignore]
    fn foo() {
        let x = 1;
        let y = 2;
        let z = 3;
        let something = parse_thing! { x .>>. y .>>. z .<<. x };
        assert_eq!(5, something);
    }
    */
}