use crate::*;

#[inline]
pub(crate) fn parse(tokens: Vec<Token>) -> Result<TreeNode,AppError> {
    parse4(&tokens)
}

#[derive(Debug,Clone)]
enum TokenExtended {
    Tok(Token),
    Node(TreeNode),
}

fn parse4(toks: &[Token]) -> Result<TreeNode,AppError> {
    let ext = promote_to_extended_tokens(toks, parse4)?;
    form_node(&ext)
}

#[inline]
fn is_in_group(arg: &[char]) -> impl Fn(char) -> bool + '_ {
    move |ch| -> bool {
        arg.iter().any(|&i| i == ch)
    }
}

fn form_node(etoks: &[TokenExtended]) -> Result<TreeNode,AppError> {

    const ADDITIVE_OPS: &[char] = &['+','-'];
    const MULTIPLICATIVE_OPS: &[char] = &['*','/'];
    const POWER_OP: &[char] = &['^'];

    //This forms order...
    //TODO:
    let cmp_closure = |left: &TokenExtended,right: &TokenExtended| -> std::cmp::Ordering {
        use TokenExtended::*;
        use Token::*;
        use std::cmp::Ordering::*;
        match (left,right) {
            (Tok(Op(l)),Tok(Op(r)))
                if is_in_group(ADDITIVE_OPS)(*r) && is_in_group(MULTIPLICATIVE_OPS)(*l) => Less,
            (Tok(Op(l)),Tok(Op(r)))
                if is_in_group(ADDITIVE_OPS)(*l) && is_in_group(MULTIPLICATIVE_OPS)(*r) => Greater,

            (Tok(Op(l)),Tok(Op(r)))
                if is_in_group(POWER_OP)(*l) && is_in_group(ADDITIVE_OPS)(*r) => Less,
            (Tok(Op(l)),Tok(Op(r)))
                if is_in_group(POWER_OP)(*r) && is_in_group(ADDITIVE_OPS)(*l) => Greater,

            (Tok(Op(l)),Tok(Op(r)))
                if is_in_group(MULTIPLICATIVE_OPS)(*l) && is_in_group(POWER_OP)(*r) => Less,
            (Tok(Op(l)),Tok(Op(r)))
                if is_in_group(MULTIPLICATIVE_OPS)(*r) && is_in_group(POWER_OP)(*l) => Greater,

            (Tok(Num(_)),Tok(Op(_))) => Less,
            (Tok(Op(_)),Tok(Num(_))) => Greater,

            (Node(_),Tok(Op(_))) => Greater,
            (Tok(Op(_)),Node(_)) => Less,

            (_,_) => Equal,
        }
    };

    //iter() doesn't borrow vec to the end of the method, only for the min search.
    if let Some((min,_)) = etoks.iter().enumerate().min_by(|(_,l),(_,r)| cmp_closure(l,r)) {
        let left = &etoks[0..min];
        if let Some((curr,right)) = match &etoks[min..] {
            [curr,ref right @ ..] => Some((curr,right)),
            _ => None,
        } {
            match curr {
                TokenExtended::Node(nod) => Ok(nod.clone()),
                TokenExtended::Tok(Token::Op(op)) => {
                    let mut ret = TreeNode::from_op(*op);
                    let ch = (form_node(left)?,form_node(right)?);
                    ret.push_chidren(Box::new(ch));
                    Ok(ret)
                },
                TokenExtended::Tok(Token::Num(num)) => Ok(TreeNode::from_f64(*num)),
                TokenExtended::Tok(Token::Brace{lhs: _}) => unreachable!(),
            }
        } else {
            Err(AppError::LexError("".into()))
        }
    } else {
        Err(AppError::ParseError("Empty extended tokens found".into()))
    }
}

fn promote_to_extended_tokens(toks: &[Token],parse_fn: impl Fn(&[Token])->Result<TreeNode,AppError>) -> Result<Vec<TokenExtended>,AppError> {
    //of braces
    let ranges = process_braces(toks)?;
    let mut ret = Vec::new();

    //current range of braces
    let mut curr_indice = 0;
    //was it aleady inserted?
    let mut flag = false;

    for i in 0..toks.len() {
        let (left,right) = ranges[curr_indice];

        if i >= left && i <= right {
            if !flag {
                let l = left + 1;
                let r = right - 1;
                ret.push(TokenExtended::Node(parse_fn(&toks[l..=r])?));
                flag = true;
            }
        } else {
            if flag {
                curr_indice+=1;
                flag = false;
            };
            ret.push(TokenExtended::Tok(toks[i]));
        };

    };

    ret.retain(|el| !matches!(el,TokenExtended::Tok(Token::Brace{lhs: _})));
    Ok(ret)
}


fn process_braces(toks: &[Token]) -> Result<Vec<(usize,usize)>,AppError> {
    let mut res: (Vec<_>,isize,(Option<usize>,Option<usize>)) = toks.iter().enumerate().try_fold(
        (Vec::new(),0isize,(None,None)),
        |(mut vec,mut acc,mut curr_interval),(ind,tok)|{
            match tok {
                Token::Brace{lhs: true} => {
                    acc+=1;
                    curr_interval = match curr_interval {
                        (Some(lft),None) => {curr_interval},
                        (None,Some(rht)) => {return Err(AppError::ParseError("Ill-formed braces".into()))},
                        (Some(lft),Some(rht)) => {return Err(AppError::ParseError("Bad input".into()))},
                        (None,None) => (Some(ind),None),
                    };
                },
                Token::Brace{lhs: false} => {
                    acc-=1;
                    if acc < 0 {return Err(AppError::ParseError("Ill-formed braces".into()))};
                    curr_interval = match curr_interval {
                        (Some(lft),Some(_)) if acc != 0 => (Some(lft),Some(ind)),
                        (Some(lft),Some(_)) if acc == 0 => {
                            vec.push((lft,ind));
                            (None,None)
                        },
                        (Some(_),Some(_)) if acc < 0 => unreachable!(),
                        (None,_) => {return Err(AppError::ParseError("Ill-formed braces".into()))},
                        (lft @ Some(_),None) => (lft,Some(ind)),
                        _ => unreachable!(),
                    };
                },
                _ => {
                    if acc == 0 {
                        curr_interval = if let (Some(lhs),Some(rhs)) = curr_interval {
                            vec.push((lhs,rhs));
                            (None,None)
                        } else {
                            curr_interval
                        };
                    };
                },
            };
            Ok((vec,acc,curr_interval))
    })?;
    if res.1 == 0 {
        if let (Some(lhs),Some(rhs)) = res.2 {
            res.0.push((lhs,rhs));
        };
        Ok(res.0)
    } else {
        Err(AppError::ParseError("Ill-formed braces".into()))
    }
}

#[cfg(test)]
mod braces_tests {
    use crate::job as T;

    //This is helper
    #[allow(unused)]
    fn brace_indices(t: &[crate::Token]) -> Vec<usize> {
        let mut ret = Vec::new();
        for (i,el) in t.iter().enumerate() {
            match el {
                crate::Token::Brace{lhs: _} => ret.push(i),
                _ => {},
            }
        };
        ret
    }

    #[test]
    fn t_01() {
        let toks = T::lex::lexer("( 8 * 10 ) - (100^0.5)").unwrap();
        let br_res = T::parse::process_braces(&toks).unwrap();
        assert_eq!(br_res, vec![(0,4),(6,10)]);
    }
    #[test]
    fn t_02() {
        let toks = T::lex::lexer("( 8 * 10 )").unwrap();
        let br_res = T::parse::process_braces(&toks).unwrap();
        assert_eq!(br_res, vec![(0,4)]);
    }
    #[test]
    fn t_03() {
        let toks = T::lex::lexer("(100^0.5)").unwrap();
        let br_res = T::parse::process_braces(&toks).unwrap();
        assert_eq!(br_res, vec![(0,4)]);
    }
    #[test]
    fn t_04() {
        let toks = T::lex::lexer("( 10 - 3 ) * 17 + (11+-2) / (12 - 7)").unwrap();
        let br_res = T::parse::process_braces(&toks).unwrap();
        assert_eq!(br_res, vec![(0,4),(8,12),(14,18)]);
    }
}

#[cfg(test)]
mod promotion_tests {

}
