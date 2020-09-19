use crate::*;

pub(crate) fn parse(tokens: &[Token]) -> Result<TreeNode,AppError> {
    use Token::*;
    let sz = tokens.len();
    let mut brace_count = 0; //count of brace pairs
    let mut acc = 0isize;

    println!("tokens: {:?}\n",tokens);

    #[derive(Debug,Clone,Copy)]
    enum Rank {
        Rank(isize),
        Brace{lhs: bool},
    }

    let storage = |i: usize,store: &mut ()| -> (Option<usize>,Option<usize>) {

        (None,None)
    };

    let tokens_ranks = tokens.iter().enumerate().try_fold((Vec::with_capacity(sz),()),|(mut vec,store),(ind,it)| {
        let ret = match it {
            Brace{lhs: true} => {
                acc+=1;
                brace_count+=1;
                Rank::Brace{lhs: true}
            },
            Brace{lhs: false} => {
                acc-=1;
                if acc < 0 {return Err(AppError::ParseError("Bad brace formation".to_string()))};
                Rank::Brace{lhs: false}
            },
            _ => {
                Rank::Rank(acc)
            }
        };
        vec.push(ret);
        Ok((vec,store))
    })?.0.into_boxed_slice();
    if acc != 0 {return Err(AppError::ParseError("Bad brace formation".to_string()))};

    println!("ranks: {:?}\n",tokens_ranks);

    #[derive(Clone,Debug)]
    enum Partial<'a> {
        Tok(&'a Token),
        Dat(TreeNode),
        BraceLeft,
        BraceRight,
    }

    let braces_indices = {

    };

    println!("indices: {:?}\n",braces_indices);

    Ok(TreeNode::Ending{data: Some(TreeData::Num(1.0))})
    // Err(AppError::ParseError("Not implemented".to_string()))
}
