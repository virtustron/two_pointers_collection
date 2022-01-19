use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicPtr;

use core::mem::MaybeUninit;


struct TwoHeadVec<T> {
    head_read: AtomicPtr,                 // -> buf1
    head_write: Arc<Mutex<usize>>,        // -> buf2

    buffer_read: *mut T,             
    buffer_write: *mut T,             
    
    capacity_read: usize,            
    capacity_write: usize,    

    length_read: usize,            
    length_write: usize,  
    
    // aproach 1 generation
    generation: CachePadded<AtomicUsize>,     // `CachePadded` - to avoid cache locality.  
    
    _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for TwoHeadVec<T> {}
unsafe impl<T: Sync> Sync for TwoHeadVec<T> {}

impl<T> TwoHeadVec<T> {
    pub fn new(capacity: usize) -> TwoHeadVec<T> {
        assert!(capacity > 0, "capacity must be non-zero");

        head_read = AtomicPtr::default();
        head_write = Arc::new(Mutex::new(0));

        let buffer_read = vec![0; capacity];
        let buffer_read = buffer_read.into_boxed_slice();

        let buffer_write = vec![0; capacity];
        let buffer_write = buffer_write.into_boxed_slice();

        capacity_read = capacity;
        capacity_write = capacity;

        length_read = 0;
        length_write = 0;

        generation = 0;

        TwoHeadVec {
            head_read,
            head_write,
            buffer_read,
            buffer_write,
            capacity_read,
            capacity_write,
            length_read,
            length_write,
            generation,
            _marker: PhantomData,
        }
    }

    pub fn push(&self, index: usize, value: T) -> Result<(), T> {

    /*
    // approach 1

    Mutex  head_write lock() {
        
        

        2. swap `head_read` and `head_write` - atomic operation
          2.1 store head_read   p = head_read.load()
          2.2 head_read.store(head_write)                    // safe for get() due to atomic operation
               generation += 1;  // use atomic increment function
          2.3 head_write = p;

        3. deep copy (content copying) `head_read` -> `head_write`
    

    }  
    */
        assert!(index >= 0, "index must be equal or greater than zero");
        assert!(index < capacity_write, "index must be greater than zero");

        // 1. modify `head_write`
        self.head_write.lock().unwrap();

        length_write
        
        let slot = unsafe { &*self.buffer_write.add(index) };         // shift pointer to `index`
        
        unsafe {
            slot.value.get().write(MaybeUninit::new(value));
        }



        
    }

    

}

/*
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

*/

