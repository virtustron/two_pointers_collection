use std::sync::atomic::AtomicPtr;


use std::sync::Arc;
use std::sync::Mutex;

struct TwoHeadVec<T> {
    head_read: AtomicPtr,            // -> buf1
    head_write: Mutex<usize>,        // -> buf2

    buffer_1: *mut T,             
    buffer_2: *mut T,             
    
    capacity_1: usize,            
    capacity_2: usize,    
    
    // aproach 1 generation
    generation: CachePadded<AtomicUsize>,     // `CachePadded` - to avoid cache locality.   
}

unsafe impl<T: Send> Sync for TwoHeadVec<T> {}
unsafe impl<T: Send> Send for TwoHeadVec<T> {}

impl<T> TwoHeadVec<T> {
    pub fn new(capacity: usize) -> TwoHeadVec<T> {
        assert!(capacity > 0, "capacity must be non-zero");

        head_read = AtomicPtr::default();
        head_write = Mutex::new(0);

        let buffer_1 = vec![0; capacity];
        let buffer_1 = buffer_1.into_boxed_slice();

        let buffer_2 = vec![0; capacity];
        let buffer_2 = buffer_2.into_boxed_slice();

        capacity_1 = capacity;
        capacity_2 = capacity;

        generation = 0;

        TwoHeadVec {
            head_read,
            head_write,
            buffer_1,
            buffer_2,
            capacity_1,
            capacity_2,
            generation,
        }
    }



    

}

new() {

    /*
    // approach 1
    
    generation = 0
    */
}


push() {

    /*
    // approach 1

    Mutex  head_write lock() {
        
        1. modify `head_write`

        2. swap `head_read` and `head_write` - atomic operation
          2.1 store head_read   p = head_read.load()
          2.2 head_read.store(head_write)                    // safe for get() due to atomic operation
               generation += 1;  // use atomic increment function
          2.3 head_write = p;

        3. deep copy (content copying) `head_read` -> `head_write`
    

    }  
    */

}


get() {
    /*
    // approach 1

    loop {
        let gen = generation 

        read from  `head_read`

        if gen == generation {
            return readed from  `head_read`
        }
    }
    */


    // aproach 2 - count references to `head_read`
}

  

}



