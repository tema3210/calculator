#[derive(Debug,Clone)]
pub enum Msg {
    EnterPressed,
    TextChange(String),
    Output(String),
    Status(String),
}

#[derive(Debug)]
pub enum AppError {
    ParseError(String),
    EvalError(String),
    LexError(String),
    Test(String),
}

//9bytes at most
#[derive(Clone,Copy,Debug)]
pub(crate) enum Token {
    Op(char),
    Num(f64),
    Brace{lhs: bool},
}

#[derive(Debug,Clone)]
pub(crate) enum TreeData {
    Op(char),
    Num(f64),
}
#[derive(Debug,Clone)]
pub(crate) enum TreeLeaf {
    Unary(Box<TreeNode>),
    Binary{left: Box<TreeNode>, right: Box<TreeNode>}
}
#[derive(Debug,Clone)]
pub(crate) enum TreeNode {
    Ending{data: Option<TreeData>},
    WithChilds{data: Option<char>, children: Option<TreeLeaf>},
}

mod job;
pub fn perform(inp: String) -> Result<f64,AppError> {
    use job::*;

    println!("{}",&inp);
    let tokens = lex::lexer(inp)?;
    let tree = parse::parse(&tokens)?;
    eval::eval(tree)
}
