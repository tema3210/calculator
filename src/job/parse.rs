use crate::*;

#[inline]
pub(crate) fn parse(tokens: Vec<Token>) -> Result<TreeNode,AppError> {
    Err(AppError::ParseError("Not implemented".into()))
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
                _ => {},
            };
            Ok((vec,acc,curr_interval))
    }
    )?;
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
mod tests {
    use crate::job as T;

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
}
