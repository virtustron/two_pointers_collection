use std::thread;
use std::sync::Arc;

use crate::two_head_vec::TwoHeadVec;


#[test]
fn push_two_elements_succesful() {
    let v = TwoHeadVec::new(2);

    assert_eq!(v.push('a'), Ok(()));
    assert_eq!(v.push('b'), Ok(()));
}

#[test]
fn push_two_elements_multithread() {
    const ELEMENTS_COUNT: usize = 10;
        
    let vector = Arc::new(TwoHeadVec::new(ELEMENTS_COUNT));

    let mut handles = vec![];

    for _ in 0..ELEMENTS_COUNT {
        let temp_vector = vector.clone();

        let handle = thread::spawn(move || {
            assert_eq!(temp_vector.push('a'), Ok(()));
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }


    let final_vector_result = Arc::try_unwrap(vector);
    
    match final_vector_result {
        Ok(final_vector) => {
            for i in 0..ELEMENTS_COUNT {
                assert_eq!(final_vector.get(i), Ok('a'));               
            }
        }

        Err(final_vector) => {
            for i in 0..ELEMENTS_COUNT {
                assert_eq!(final_vector.get(i), Ok('a'));               
            }
        }
    }    
}