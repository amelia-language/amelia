use crate::token::Token;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Node {
    pub token: Token,
    pub children: Vec<Node>,
    pub data: Option<String>,
}

