use crate::*;

#[inline]
pub(crate) fn parse(tokens: Vec<Token>) -> Result<TreeNode,AppError> {
    Err(AppError::ParseError("Not implemented".into()))
}


#[derive(Debug)]
enum TokenExtended {
    Tok(Token),
    Node(TreeNode),
}

fn promote_to_extended_tokens(toks: Vec<Token>,parse_fn: impl Fn(&[Token])->Result<TreeNode,AppError>) -> Result<Vec<TokenExtended>,AppError> {
    //of braces
    let ranges = process_braces(&toks)?;
    let mut ret = Vec::new();

    //current range of braces
    let mut curr_indice = 0;
    //was it aleady inserted
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
