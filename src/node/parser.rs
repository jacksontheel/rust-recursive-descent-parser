use super::add_operator;
use super::sign;
use super::scanner;

pub struct Parser {
    scanner: scanner::Scanner
}

pub fn new_parser(tokens: Vec<char>) -> Parser {
    Parser { scanner: scanner::new_scanner(tokens) }
}

impl Parser {
    pub fn parse(&self) -> Result<i32, &str> {
        let mut scanner = self.scanner.clone();
        self.parse_expr(&mut scanner)
    }

    fn parse_expr(&self, scanner: &mut scanner::Scanner) -> Result<i32, &str> {
        match self.parse_fact(scanner) {
            Ok(fact_result) => {
                let mut value = fact_result;
                while let Ok(add_op) = self.parse_add_operator(scanner) {
                    match add_op {
                        add_operator::AddOperator::Plus => match self.parse_fact(scanner) {
                            Ok(fact_chain_result) => value += fact_chain_result,
                            Err(err) => return Err(err),
                        },
                        add_operator::AddOperator::Minus => match self.parse_fact(scanner) {
                            Ok(fact_chain_result) => value -= fact_chain_result,
                            Err(err) => return Err(err),
                        },
                    }
                }
                return Ok(value);
            },
            Err(err) => Err(err),
        }
    }

    fn parse_fact(&self, scanner: &mut scanner::Scanner) -> Result<i32, &str> {
        let sign = match self.parse_sign(scanner) {
            sign::Sign::Positive => 1,
            sign::Sign::Negative => -1,
        };

        if let Some(n) = scanner.match_number() {
            Ok(n * sign)
        } else if scanner.match_token("(") {
            match self.parse_expr(scanner) {
                Ok(expr_result) => {
                    match scanner.match_token(")") {
                        true => Ok(expr_result * sign),
                        false => Err("Expecting ')'"),
                    }
                },
                Err(err) => Err(err),
            }
        } else {
            Err("Expecting number or start of parenthetical expression")
        }
    }

    fn parse_sign(&self, scanner: &mut scanner::Scanner) -> sign::Sign {
        if scanner.match_token("-") {
            sign::Sign::Negative
        } else {
            sign::Sign::Positive
        }
    }

    fn parse_add_operator(&self, scanner: &mut scanner::Scanner) -> Result<add_operator::AddOperator, &str> {
        if scanner.match_token("+") {
            return Ok(add_operator::AddOperator::Plus);
        }else if scanner.match_token("-") {
            return Ok(add_operator::AddOperator::Minus);
        }
        Err("Expecting either '+' or '-'")
    }
}