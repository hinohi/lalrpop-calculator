use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn term() {
        assert_eq!(grammar::ExprParser::new().parse("22").unwrap(), 22);
        assert_eq!(grammar::ExprParser::new().parse("(23)").unwrap(), 23);
        assert_eq!(grammar::ExprParser::new().parse("((((24))))").unwrap(), 24);
        assert!(grammar::ExprParser::new().parse("((22)").is_err());
    }

    #[test]
    fn expr() {
        assert_eq!(grammar::ExprParser::new().parse("2 + 3 * 4"), Ok(14));
        assert_eq!(grammar::ExprParser::new().parse("6 / 2 * 3"), Ok(9));
    }
}
