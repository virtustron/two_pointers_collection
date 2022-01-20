use std::ptr;

use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicPtr;
use std::marker::PhantomData;

use core::mem::MaybeUninit;

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
        assert!(self.length_write + 1 < self.capacity, "vector is already full");

        self.head_write.lock().unwrap();
        
        // 1. modify `head_write`
        self.length_write.fetch_add(1, Ordering::SeqCst);
        let slot = unsafe { &*self.buffer_2.add(self.length_write - 1) };
        unsafe { slot.value.get().write(MaybeUninit::new(value)); }
        
        // 2. swap `head_read` and `head_write` and `capacity`, `length` respectively
        let temp_head = self.head_read.load(Ordering::Acquire);        
        self.head_read.store(self.head_write, Ordering::Release);     
        self.generation.fetch_add(1, Ordering::SeqCst);
        self.head_write = temp_head; 
              
        let temp_length = self.length_read.load(Ordering::Acquire);    
        self.length_read.store(self.length_write, Ordering::Release);  
        self.length_write = temp_length; 
                
        //3. deep copy (content copying) `head_read` -> `head_write`
        ptr::copy(self.head_read, self.head_write, self.capacity.load(Ordering::Acquire));

        return self.length_write.load(Ordering::Acquire)
    }  

    pub fn get(&self, index: usize) -> Result<T, ()> {
        // TODO let backoff = Backoff::new();

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
