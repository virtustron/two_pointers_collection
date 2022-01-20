mod two_head_vec;

use crate::two_head_vec::TwoHeadVec;

fn main() {
    println!("Hello, world!");

    let v = TwoHeadVec::new(2);

    assert_eq!(v.push('a'), Ok(()));
    assert_eq!(v.push('b'), Ok(()));

    assert_eq!(v.get(0), Ok('a'));
    assert_eq!(v.get(1), Ok('b'));
}


