use malloc::MALLOC;
use fast_vec::FastVec;
use std::ptr;

// This is a scratch pad for you.
// Feel free to use it to try the different APIs and see if they work.

// Use this code to understand how to use MALLOC and raw pointers.
// Feel free to change it and observe what happens!
fn malloc_and_ptr() {
    // Malloc one value.
    println!("malloc_and_ptr------");
    let ptr1 = unsafe { MALLOC.malloc(size_of::<i32>()) as *mut i32 };
    unsafe {
        ptr::write(ptr1, 10);
    }

    unsafe {
        let value = *ptr1;
        println!("value inside pointer is {value}");
        println!("address of pointer is {ptr1:p}");
    }

    // Malloc many values.
    println!("malloc_and_ptr------");
    let ptr2 = unsafe { MALLOC.malloc(5 * size_of::<i32>()) as *mut i32 };
    for i in 0..5 {
        unsafe {
            ptr::write(ptr2.add(i), 10 * (i as i32));
        }
    }

    for i in 0..5 {
        unsafe {
            let tmp = ptr2.add(i);
            let value = *tmp;
            println!("value at index {i} is {value}");
            println!("address of index {i} is {tmp:p}");
        }
    }
    // What happens if you access data outside the range, e.g. ptr2.add(6)?
    // What happens if you write data outside the range, e.g., at ptr2.add(6)?
    // What happens if you read data from a pointer before writing to it!?
}

// This part will work after you implement get().
fn fast_vec_get() {
    println!("fast_vec_get------");
    let fast_vec: FastVec<i32> = FastVec::from_vec(vec![10, 20, 30]);
    println!("{}", fast_vec);  // should print FastVec[10, 20, 30]

    for i in 0..fast_vec.len() {
        println!("Element at {} is {}", i, fast_vec.get(i));
    }
    println!("---------------------");
    println!("");
}


// This part should work if you implement the push function correctly!
fn fast_vec_push() {
    println!("fast_vec_push--------");
    let mut fast_vec2: FastVec<i32> = FastVec::new();
    fast_vec2.push(10);
    println!("{}", fast_vec2);  // should print FastVec[10]
    fast_vec2.push(20);
    println!("{}", fast_vec2);  // should print FastVec[10, 20]
    fast_vec2.push(30);
    println!("{}", fast_vec2);  // should print FastVec[10, 20, 30]
    fast_vec2.clear();
    println!("{}", fast_vec2);  // should print FastVec[]
    println!("---------------------");
    println!("");
}

// This part should work if you implement the remove function correctly!
fn fast_vec_remove() {
    println!("fast_vec_remove------");
    let mut fast_vec3: FastVec<i32> = FastVec::from_vec(vec![10, 20, 30, 40]);
    println!("{}", fast_vec3);  // should print FastVec[10, 20, 30, 40]
    fast_vec3.remove(1);
    println!("{}", fast_vec3);  // should print FastVec[10, 30, 40]
    fast_vec3.remove(2);
    println!("{}", fast_vec3);  // should print FastVec[10, 30]
    fast_vec3.remove(0);
    println!("{}", fast_vec3);  // should print FastVec[30]
    fast_vec3.remove(0);
    println!("{}", fast_vec3);  // should print FastVec[]
    println!("---------------------");
    println!("");
}

fn main() {
    // This part works as given.
    malloc_and_ptr();

    // These parts do not work unless you implement get, push, and remove correctly.
    fast_vec_get();
    fast_vec_push();
    fast_vec_remove();
}