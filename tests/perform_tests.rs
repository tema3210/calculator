extern crate calc;

#[test]
fn t_01() {
    let res = calc::perform("( 10 - 9 )*2".to_string());
    match res {
        Ok(num) => {
            assert_eq!(num,2.0);
        },
        Err(e) => {
            println!("{:?}",e);
            panic!()
        }
    }
}
