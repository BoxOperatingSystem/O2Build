use std::slice::Iter;

use super::function::{parse_abi, parse_function_definition};
use crate::{
    abi::Abi,
    ast::Node,
    match_token, next_token,
    token::{Keyword, Token},
};

pub fn parse_definition(public: bool, it: &mut Iter<Token>) -> Result<Node, Option<Token>> {
    let token = next_token!(it, return Err(None));

    match token {
        Token::Keyword(_, Keyword::Function) => {
            parse_function_definition(public, Abi::default(), it)
        }
        Token::Keyword(_, Keyword::Abi) => {
            let abi = parse_abi(it)?;
            match_token!(it.next(), Token::Keyword(_, Keyword::Function), Ok(()))?;

            parse_function_definition(public, abi, it)
        }
        _ => Err(Some(token.clone())),
    }
}
