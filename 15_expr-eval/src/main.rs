/*

    "4 + 5" => 9
    "4 + 5 * 4" => 24

*/

#[derive(Debug, PartialEq)]
enum TokenType {
    Number,
    AdditionOp,
    SubtractionOp,
}

#[derive(Debug)]
struct Token {
    value: String,
    kind: TokenType,
}



struct Parser {}

impl Parser {
    fn lex(&self, input: String) -> Vec<Token> {
        let mut input = input.chars().peekable();
        let mut tokens = Vec::new();


        let mut c : char;

        loop {
            match input.next() {
                Some(x) => { c = x }
                None => { break }
            }
            
            // parseDigit
            if c.is_digit(10) {
                let mut value = String::from("");
                while c.is_digit(10) {
                    value.push(c);

                    match input.peek() {
                        Some(x)=> {
                            if !x.is_digit(10) {
                                break;
                            }
                        }
                        None => { break }
                    }
                    
                    match input.next() {
                        Some(x) => { c = x }
                        None => { break }
                    }
                }

                tokens.push(Token{
                    value: value,
                    kind: TokenType::Number,
                });

                continue;
            }

            // parseAdditionOp
            if c == '+' {
                tokens.push(Token{
                    value: c.to_string(),
                    kind: TokenType::AdditionOp,
                });

                continue;
            }

            // parseSubtractionOp
            if c == '-' {
                tokens.push(Token{
                    value: c.to_string(),
                    kind: TokenType::SubtractionOp,
                });

                continue;
            }
        }

        return tokens;
    }
}

#[test]
fn test() {
    let p = Parser{};
    let tokens = p.lex("4+5".into());
    println!("{:?}", tokens);
    assert_eq!(tokens.get(0).expect("missing token 0").kind, TokenType::Number);
    assert_eq!(tokens.get(0).expect("missing token 0").value, "4".to_string());
    assert_eq!(tokens.get(1).expect("missing token 1").kind, TokenType::AdditionOp);
    assert_eq!(tokens.get(1).expect("missing token 1").value, "+".to_string());
    assert_eq!(tokens.get(2).expect("missing token 2").kind, TokenType::Number);
    assert_eq!(tokens.get(2).expect("missing token 2").value, "5".to_string());
}

#[test]
fn test_sub() {
    let p = Parser{};
    let tokens = p.lex("4-56".into());
    println!("{:?}", tokens);
    assert_eq!(tokens.get(0).expect("missing token 0").kind, TokenType::Number);
    assert_eq!(tokens.get(0).expect("missing token 0").value, "4".to_string());
    assert_eq!(tokens.get(1).expect("missing token 1").kind, TokenType::SubtractionOp);
    assert_eq!(tokens.get(1).expect("missing token 1").value, "-".to_string());
    assert_eq!(tokens.get(2).expect("missing token 2").kind, TokenType::Number);
    assert_eq!(tokens.get(2).expect("missing token 2").value, "56".to_string());
}

fn main() {
    println!("Hello, world!");
}
