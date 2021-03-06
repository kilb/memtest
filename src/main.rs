use std::{ptr, mem};
use libc::{mmap, munmap, c_void};
use libc::{PROT_READ, PROT_WRITE, MAP_PRIVATE, MAP_ANONYMOUS, MAP_HUGETLB, MAP_HUGE_1GB};
unsafe fn mmap_malloc(len: usize) -> Vec<u8>
{
    let ptr = mmap(ptr::null_mut(), len, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS|MAP_HUGETLB|MAP_HUGE_1GB, -1,0);
    if ptr == 0xffffffffffffffff as *mut c_void{
        println!("Error!");
    }
    println!("{:?}", ptr);
    Vec::from_raw_parts(
        ptr as *mut u8,
        len,
        len,
    )
}

fn main() {
    unsafe {
        let mut a = mmap_malloc(1024*1024*1024*2);
        println!("{}", mem::align_of_val(&a));
        munmap(a.as_mut_ptr() as *mut c_void, a.capacity());
        mem::forget(a);
    }
}
