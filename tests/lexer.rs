extern crate samson;

use samson::lexer;

#[test]
fn test_lex() {
    let str = String::from(".");
    let tokens_res = lexer::tokenize(&str);
    assert!(tokens_res.is_ok());

    let tokens = tokens_res.unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], lexer::Token::Meta);
}

#[test]
fn test_select_all() {
    let str = String::from("SELECT * FROM users");
    let token_opt = lexer::tokenize(&str);
    assert!(token_opt.is_ok());

    let tokens = token_opt.unwrap();
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0], lexer::Token::SelectKey);
}
