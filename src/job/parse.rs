use crate::*;

pub(crate) fn parse(tokens: &[Token]) -> Result<TreeNode,AppError> {
    Err(AppError::ParseError("Not implemented".into()))
}

fn parse1(toks: &[Token])-> Result<TreeNode,AppError> {
    let mut ops: Vec<_> = toks.iter().enumerate().filter_map(|(ind,it)| {
        match it {
            Token::Op(ch) => {
                match ch {
                    '*' | '/' => Some((1,ind)),
                    '+' | '-' => Some((3,ind)),
                    '^' => Some((2,ind)),
                    _ => panic!("Bad op found"),
                }
            },
            _ => None,
        }
    }).collect();
    ops.sort_by(|(r1,_),(r2,_)| r1.cmp(r2).reverse());



    return Err(AppError::ParseError("Not implemented".into()));
}

#[derive(Eq,PartialEq)]
enum OpKind {
    Multiplicative,
    Additive,
    Power,
}
#[derive(Eq)]
struct OpPrior(isize,usize,OpKind);

impl std::cmp::PartialEq<OpPrior> for OpPrior {
    fn eq(&self, rhs: &OpPrior) -> bool {
         self.0 == rhs.0 && self.1 == rhs.1
    }
}
impl std::cmp::Ord for OpPrior {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.partial_cmp(rhs).unwrap()
    }
}
impl std::cmp::PartialOrd<OpPrior> for OpPrior {
    fn partial_cmp(&self, rhs: &OpPrior) -> std::option::Option<std::cmp::Ordering> {
        //self to rhs comparison
        if self.0 == rhs.0 {
            Some(self.1.cmp(&rhs.1))
        } else {
            Some(self.0.cmp(&rhs.0))
        }
    }
}
fn parse2(toks: &[Token])-> Result<TreeNode,AppError> {

    let len = toks.len();
    let mut acc = 0isize;
    let mut ops_p = toks.iter().enumerate().filter_map(|(ind,it)| {
        match it {
            Token::Brace{lhs: true} => {acc+=1;None},
            Token::Brace{lhs: false} => {acc-=1;None},
            Token::Op(ch) => {
                let kind = match ch {
                    '*' | '/' => OpKind::Multiplicative,
                    '+' | '-' => OpKind::Additive,
                    '^' => OpKind::Power,
                    _ => unreachable!("Bad op found"),
                };
                Some(OpPrior(acc,ind,kind))
            }
            _ => None,
        }
    }).collect::<Vec<_>>();
    if acc != 0 {return Err(AppError::ParseError("Ill formed braces".into()));};
    ops_p.sort();



    return Err(AppError::ParseError("Not implemented".into()));
}
