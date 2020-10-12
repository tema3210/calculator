use std::borrow::Cow;

#[derive(Debug,Clone)]
pub enum Msg {
    EnterPressed,
    TextChange(String),
    Output(String),
    Status(String),
}

#[derive(Debug)]
pub enum AppError {
    ParseError(Cow<'static,str>),
    EvalError(Cow<'static,str>),
    LexError(Cow<'static,str>),
    Test(Cow<'static,str>),
}

//9bytes at most
#[derive(Clone,Copy,Debug)]
pub(crate) enum Token {
    Op(char),
    Num(f64),
    Brace{lhs: bool},
}

#[derive(Debug,Clone)]
pub(crate) enum TreeNode {
    Ending(Option<f64>),
    WithChilds{data: Option<char>, children: Option<Box<(TreeNode,TreeNode)>>},
}

mod job;
pub fn perform(inp: String) -> Result<f64,AppError> {
    use job::*;

    println!("{}",&inp);
    let tokens = lex::lexer(&inp)?;
    let tree = parse::parse(&tokens)?;
    eval::eval(tree)
}
