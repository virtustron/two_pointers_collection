use std::ptr;

use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicPtr;

use core::mem::MaybeUninit;


struct TwoHeadVec<T> {
    head_read: AtomicPtr,                 // -> buf1
    head_write: Arc<Mutex<usize>>,        // -> buf2

    buffer_1: *mut T,             
    buffer_2: *mut T,             
    
    capacity: CachePadded<AtomicUsize>,    

    length_read: CachePadded<AtomicUsize>,            
    length_write: CachePadded<AtomicUsize>,  
    
    // aproach 1 generation
    generation: CachePadded<AtomicUsize>,     // `CachePadded` - to avoid cache locality.  
    
    _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for TwoHeadVec<T> {}
unsafe impl<T: Sync> Sync for TwoHeadVec<T> {}

impl<T> TwoHeadVec<T> {
    pub fn new(capacity: usize) -> TwoHeadVec<T> {
        assert!(capacity > 0, "capacity must be non-zero");

        capacity.store(capacity, Ordering::Relaxed);

        length_read.store(0, Ordering::Relaxed);
        let buffer_1 = vec![0; capacity].into_boxed_slice();
        head_read = AtomicPtr::from(buffer_1);
        
        length_write.store(0, Ordering::Relaxed);
        let buffer_2 = vec![0; capacity].into_boxed_slice();
        head_write = Arc::new(Mutex::new(buffer_2));

        generation = 0;

        TwoHeadVec {
            head_read,
            head_write,
            buffer_1,
            buffer_2,
            capacity,
            length_read,
            length_write,
            generation,
            _marker: PhantomData,
        }
    }

    pub fn push(&self, value: T) -> Result<(), T> {
        assert!(length_write + 1 < capacity, "vector is already full");

        self.head_write.lock().unwrap();
        
        // 1. modify `head_write`
        length_write += 1;
        let slot = unsafe { &*self.buffer_2.add(length_write - 1) };
        unsafe { slot.value.get().write(MaybeUninit::new(value)); }
        
        // 2. swap `head_read` and `head_write` and `capacity`, `length` respectively
        let temp_head = head_read.load(Ordering::Acquire);        
        head_read.store(self.head_write, Ordering::Release);     
        generation.fetch_add(1, Ordering::SeqCst);
        head_write = temp_head; 
              
        let temp_length = length_read.load(Ordering::Acquire);    
        length_read.store(self.length_write, Ordering::Release);  
        length_write = temp_length; 
                
        //3. deep copy (content copying) `head_read` -> `head_write`
        ptr::copy(head_read, head_write, capacity.load(Ordering::Acquire));
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

