use crate::*;

#[inline]
pub(crate) fn parse(tokens: Vec<Token>) -> Result<TreeNode,AppError> {
    parse4(&tokens)
}

#[derive(Debug,Clone)]
enum TokenExtended {
    Tok(Token),
    Node(Option<TreeNode>),
}

fn parse4(toks: &[Token]) -> Result<TreeNode,AppError> {
    let mut ext = promote_to_extended_tokens(toks, parse4)?;
    form_node(&mut ext)
}

#[inline]
fn is_in_group(arg: &[char]) -> impl Fn(char) -> bool + '_ {
    move |ch| -> bool {
        arg.iter().any(|&i| i == ch)
    }
}

fn form_node(etoks: &mut [TokenExtended]) -> Result<TreeNode,AppError> {

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
            (Tok(Num(_)),Node(_)) => Equal,
            (Node(_),Tok(Num(_))) => Equal,

            (Tok(Num(_)),_) => Greater,
            (_,Tok(Num(_))) => Less,


            //Less,Greater

            //additive and multiplicative
            (Tok(Op(l)),Tok(Op(r)))
                if is_in_group(ADDITIVE_OPS)(*r) && is_in_group(MULTIPLICATIVE_OPS)(*l) => Greater,
            (Tok(Op(l)),Tok(Op(r)))
                if is_in_group(ADDITIVE_OPS)(*l) && is_in_group(MULTIPLICATIVE_OPS)(*r) => Less,

            //additive and power op
            (Tok(Op(l)),Tok(Op(r)))
                if is_in_group(POWER_OP)(*l) && is_in_group(ADDITIVE_OPS)(*r) => Greater,
            (Tok(Op(l)),Tok(Op(r)))
                if is_in_group(POWER_OP)(*r) && is_in_group(ADDITIVE_OPS)(*l) => Less,

            //power and multiplic.
            (Tok(Op(l)),Tok(Op(r)))
                if is_in_group(MULTIPLICATIVE_OPS)(*l) && is_in_group(POWER_OP)(*r) => Greater,
            (Tok(Op(l)),Tok(Op(r)))
                if is_in_group(MULTIPLICATIVE_OPS)(*r) && is_in_group(POWER_OP)(*l) => Less,

            (_,_) => Equal,

        }
    };

    //iter() doesn't borrow vec to the end of the method, only for the min search.
    if let Some((min,_)) = etoks.iter().enumerate().min_by(|(_,l),(_,r)| cmp_closure(l,r)) {
        println!("next min: \n {:#?}",&etoks[min]);
        let (left,right) = etoks.split_at_mut(min);
        if let Some((curr,right)) = match right {
            [ref mut curr,ref mut right @ ..] => Some((curr,right)),
            _ => None,
        } {
            match curr {
                TokenExtended::Node(nod) => Ok(nod.take().unwrap()),
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
            Err(AppError::LexError("Ill-formed token stream".into()))
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

        if curr_indice >= ranges.len() {
            ret.push(TokenExtended::Tok(toks[i]));
            continue;
        };

        let (left,right) = ranges[curr_indice];

        if i >= left && i <= right {
            if !flag {
                let l = left + 1;
                let r = right - 1;
                ret.push(TokenExtended::Node(Some(parse_fn(&toks[l..=r])?)));
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
                        (Some(_),None) => {curr_interval},
                        (None,Some(_)) => {return Err(AppError::ParseError("Ill-formed braces".into()))},
                        (Some(_),Some(_)) => {return Err(AppError::ParseError("Bad input".into()))},
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
    // use crate::TreeNode;
    // use crate::job as T;
    //
    // #[test]
    // fn t_01() {
    //     let toks = T::lex::lexer("8 + (100^0.5) - 11 * 9").unwrap();
    //     let res = T::parse::promote_to_extended_tokens(&toks,|toks| {
    //         println!("{:?}", toks);
    //         Ok(TreeNode::from_f64(0.00))
    //     });
    //     println!("{:?}",res);
    //     panic!()
    // }
}
