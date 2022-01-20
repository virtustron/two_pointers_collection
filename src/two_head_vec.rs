use std::ptr;

use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicPtr;
use std::marker::PhantomData;

use core::sync::atomic::{self, AtomicUsize, Ordering};

use crossbeam_utils::CachePadded;



pub struct TwoHeadVec<T> {
    head_read: AtomicPtr<T>,               // -> buf1    - the same in-memory representation as a `*mut T`
    head_write: Arc<Mutex<*mut T>>,        // -> buf2

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

        let boxed_buffer_1 = vec![0; capacity].into_boxed_slice();
        let buffer_1 = Box::into_raw(boxed_buffer_1) as *mut T;
        let head_read = AtomicPtr::new(buffer_1);
        
        let boxed_buffer_2 = vec![0; capacity].into_boxed_slice();
        let buffer_2 = Box::into_raw(boxed_buffer_2) as *mut T;
        let head_write = Arc::new(Mutex::new(buffer_2));

        let generation: usize = 0;   // other approach - do count of the references to `head_read`

        TwoHeadVec {
            head_read, 
            head_write,
            buffer_1,
            buffer_2,
            capacity: CachePadded::new(AtomicUsize::new(capacity)),
            length_read: CachePadded::new(AtomicUsize::new(0)),
            length_write: CachePadded::new(AtomicUsize::new(0)),
            generation: CachePadded::new(AtomicUsize::new(0)),
            _marker: PhantomData,
        }
    }

    pub fn push(&self, value: T) -> Result<(), T> {
        let length_write = self.length_write.load(Ordering::Acquire);
        let capacity = self.capacity.load(Ordering::Acquire);
                
        if length_write + 1 < capacity {
            return Err(value);
        }

        let mut mutexed_head_write; 

        match self.head_write.lock() {
            Ok(matched_head_write) => {
                mutexed_head_write = matched_head_write;
            }

            Err(_) => {
                return Err(value);
            }
        }
        
        // 1. modify `head_write`
        length_write += 1;
        unsafe {
            *mutexed_head_write.add(length_write - 1) = value;
        };
                       
        
        // 2. swap `head_read` and `head_write` and `capacity`, `length` respectively
        let temp_head = self.head_read.load(Ordering::Acquire);        
        self.head_read.store(*mutexed_head_write, Ordering::Release);     
        self.generation.fetch_add(1, Ordering::SeqCst);
        *mutexed_head_write = temp_head; 
              
        let temp_length = self.length_read.load(Ordering::Acquire);    
        // TODO after copying lengthes must be equal
        self.length_read.store(length_write, Ordering::Release);  
        self.length_write = CachePadded::new(AtomicUsize::new(temp_length)); 
                
        //3. deep copy (content copying) `head_read` -> `head_write`
        // TODO check length
        ptr::copy(self.head_read.into_inner(), self.head_write, self.capacity.load(Ordering::Acquire));

        //return Ok(self.length_write.load(Ordering::Acquire))
        return Ok();
    }  

    pub fn get(&self, index: usize) -> Result<T, ()> {
        assert!(index > 0, "index must be greater than zero");
        assert!(index < self.length_read.load(Ordering::Acquire), "index must be less than length");

        loop {
            let current_generation = self.generation.load(Ordering::SeqCst);

            let value = unsafe { &*self.head_read.load(Ordering::SeqCst).fetch_add(index) };

            if current_generation == self.generation.load(Ordering::SeqCst) {
                return Some(value)
            }
        }
    }
}


// TODO write tests here

// TODO conditional compilation tests module
mod tests {

}