use std::env;
use std::vec::Vec;

#[derive(Debug)]
enum Lexeme {
    Operation(char),
    Number(String),
    Parenthesis(char),
}


struct Lexemes<'a> {
    data : &'a str,
    curs : usize,
}

impl<'a> Lexemes<'a> {

    fn lex_it(&mut self) -> Option<Lexeme> {

        let c = match self.data.chars().nth(self.curs) {
            None => return None,
            Some(item) => item,
        };

        self.curs += 1;

        match c {
            tok @ '(' | tok @ ')' => Some(Lexeme::Parenthesis(tok)),
            tok @ '+' |
            tok @ '-' |
            tok @ '*' |
            tok @ '/' |
            tok @ '%'             => Some(Lexeme::Operation(tok)),
            tok @ '0' ... '9'     =>
                Some(Lexeme::Number(tok.to_string() + match self.lex_it() {
                    Some(Lexeme::Number(ref num)) => num,
                    None                          => "",
                    _                             => {self.curs -= 1; ""}
                })),
            ' '                    => self.lex_it(),
            _                      => None,
        }
    }

}

impl<'a> Iterator for Lexemes<'a> {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Lexeme> {
        self.lex_it()
    }
}

fn lexeme(s : &str) -> Lexemes {
    Lexemes { data : s, curs : 0 }
}

fn take_num(lex : &Lexeme) -> i32 {
    match lex {
        &Lexeme::Number(ref s) => s.parse::<i32>().unwrap(),
        _ => panic!("lexeme is not a number")
    }
}

fn compute_stack(stack : &mut Vec<Lexeme>, eat_parenth : bool) {
    if stack.is_empty() {
        return;
    }

    while stack.len() > 1 {
        let left = take_num(&stack.pop().unwrap());
        let middle = stack.pop().unwrap();

        match middle {
            Lexeme::Parenthesis('(') => {
                if !eat_parenth {
                    stack.push(middle);
                }
                stack.push(Lexeme::Number(left.to_string()));
                break
            },
            _ => ()
        }

        let right = take_num(&stack.pop().unwrap());

        let res = match middle {
            Lexeme::Operation('+') => right + left,
            Lexeme::Operation('-') => right - left,
            Lexeme::Operation('*') => right * left,
            Lexeme::Operation('/') => right / left,
            Lexeme::Operation('%') => right % left,
            _ => 0,
        };

        /* match middle {
            Lexeme::Operation('+') =>  */
        stack.push(Lexeme::Number(res.to_string()));
    }
}

fn compute(expr : &str) -> i32 {
    
    let mut stack : Vec<Lexeme> = Vec::new();

    for item in lexeme(expr) {

        match item {
            l @ Lexeme::Number(_)          => stack.push(l),
            l @ Lexeme::Operation('+') |
                l @ Lexeme::Operation('-') =>
                    {compute_stack(&mut stack, false); stack.push(l)},

            l @ Lexeme::Parenthesis('(')   => {
                    stack.push(l);
                    stack.push(Lexeme::Number("0".to_string()));
                    stack.push(Lexeme::Operation('+'));
            },
                Lexeme::Parenthesis(')') => compute_stack(&mut stack, true),
            l @ Lexeme::Operation(_)   => stack.push(l),
            _ => (),
        };
    }

    while stack.len() > 1 {
        compute_stack(&mut stack, true);
    }
    return take_num(&stack[0]);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        println!("{}", compute(&args[1]));
    } else {
        println!("bad usage");
    }
}
