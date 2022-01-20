use crate::two_head_vec::TwoHeadVec;


#[test]
fn push_two_elements_succesful() {
    let v = TwoHeadVec::new(2);

    assert_eq!(v.push('a'), Ok(()));
    assert_eq!(v.push('b'), Ok(()));
}