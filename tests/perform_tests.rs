extern crate calc;

#[test]
fn t_01() {
    let res = perform("( 10 - 9 )*2".to_string());
    match res {
        Ok(num) => {
            assert_eq!(num,2);
        },
        Err(e) => {
            panic!(e)
        }
    }
}
