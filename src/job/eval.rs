use crate::*;

pub(crate) fn eval_entry(tree: TreeNode) -> Result<f64,AppError> {
    // println!("eval got {:#?}",&tree );
    eval(tree)
}

pub(crate) fn eval(tree: TreeNode) -> Result<f64,AppError> {
    match tree {
        TreeNode::Ending(Some(num)) => Ok(num),
        TreeNode::Ending(None) => Err(AppError::EvalError("Ill formed ending".into())),
        TreeNode::WithChilds{data: Some(op),children: Some(ch)} => {
            let (left,right) = *ch;

            let f = move |fst: f64,sec: f64| -> Result<f64,AppError> {
                match op {
                    '*' => Ok(fst * sec),
                    '/' => Ok(fst / sec),
                    '+' => Ok(fst + sec),
                    '-' => Ok(fst - sec),
                    '^' => Ok(fst.powf(sec)),
                    _ => Err(AppError::EvalError("Bad op found".into()))
                }
            };

            f(eval(left)?,eval(right)?)
        },
        TreeNode::WithChilds{data: _, children: _} => {
            Err(AppError::EvalError("Ill formed tree".into()))
        },
    }
}

#[cfg(test)]
mod evaluator_tests {
    pub use crate::*;

    #[test]
    fn t_01() {
        let mut tree = TreeNode::from_op('+');
        tree.push_chidren(Box::new((TreeNode::from_f64(10.0),TreeNode::from_f64(10.0))));
        match job::eval::eval(tree) {
            Ok(num) => assert_eq!(num,20.0),
            Err(e) => {
                panic!(e)
            },

        }
    }
    #[test]
    fn t_02() {
        let mut tree = TreeNode::from_op('+');
        tree.push_chidren(Box::new(({
            let mut tree = TreeNode::from_op('^');
            tree.push_chidren(Box::new((TreeNode::from_f64(2.0),TreeNode::from_f64(2.0))));
            tree
        },TreeNode::from_f64(10.0))));
        match job::eval::eval(tree) {
            Ok(num) => assert_eq!(num,14.0),
            Err(e) => {
                panic!(e)
            },

        }
    }
}
