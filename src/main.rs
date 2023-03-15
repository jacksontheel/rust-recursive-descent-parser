mod node;

fn main() {
    let s = "10 + (20 - 30) + (2 - 2 - 2)";
    let parser = node::parser::new_parser(s.chars().collect());

    match parser.parse() {
        Ok(v) => println!("{}", v),
        Err(err) => println!("{}", err)
    }
}