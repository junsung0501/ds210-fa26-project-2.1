use fixed::FixedSizeArray;
use slow_vec::SlowVec;

// This is a scratch pad for you.
// Feel free to use it to try the different APIs and see if they work.

// Use this code to understand how to use FixedSizeArray.
// Feel free to change it and observe what happens!
fn fixed_sized_array() {
    println!("fixed_sized_array----");
    let mut array: FixedSizeArray<i32> = FixedSizeArray::allocate(2);
    array.put(10, 0);
    array.put(20, 1);
    println!("{array}");  // You can print a FixedSizeArray directly!

    // We can call get many times.
    println!("{}", array.get(0));
    println!("{}", array.get(0));

    // Can we call move many times? try it out and see for yourself.
    println!("{}", array.move_out(0));
    // println!("{}", array.move_out(0));

    // Try other things!
    // What happens if you try to put or get things out of the range?
    // What happens if you try to get or move_out before put?
    // What if you try to get something you had move_out before?
    println!("---------------------");
    println!("");
}

// This part works without any changes.
fn slow_vec_basics() {
    println!("slow_vec_basics------");
    let slow_vec: SlowVec<i32> = SlowVec::from_vec(vec![10, 20, 30]);
    println!("{}", slow_vec);  // should print SlowVec(Fixed[10, 20, 30])

    for i in 0..slow_vec.len() {
        println!("Element at {} is {}", i, slow_vec.get(i));
    }
    println!("---------------------");
    println!("");
}


// This part should work if you implement the push function correctly!
fn slow_vec_push() {
    println!("slow_vec_push--------");
    let mut slow_vec2: SlowVec<i32> = SlowVec::new();
    slow_vec2.push(10);
    println!("{}", slow_vec2);  // should print SlowVec(Fixed[10])
    slow_vec2.push(20);
    println!("{}", slow_vec2);  // should print SlowVec(Fixed[10, 20])
    slow_vec2.push(30);
    println!("{}", slow_vec2);  // should print SlowVec(Fixed[10, 20, 30])
    slow_vec2.clear();
    println!("{}", slow_vec2);  // should print SlowVec(Fixed[])
    println!("---------------------");
    println!("");
}

// This part should work if you implement the remove function correctly!
fn slow_vec_remove() {
    println!("slow_vec_remove------");
    let mut slow_vec3: SlowVec<i32> = SlowVec::from_vec(vec![10, 20, 30, 40]);
    println!("{}", slow_vec3);  // should print SlowVec(Fixed[10, 20, 30, 40])
    slow_vec3.remove(1);
    println!("{}", slow_vec3);  // should print SlowVec(Fixed[10, 30, 40])
    slow_vec3.remove(2);
    println!("{}", slow_vec3);  // should print SlowVec(Fixed[10, 30])
    slow_vec3.remove(0);
    println!("{}", slow_vec3);  // should print SlowVec(Fixed[30])
    slow_vec3.remove(0);
    println!("{}", slow_vec3);  // should print SlowVec(Fixed[])
    println!("---------------------");
    println!("");
}

fn main() {
    // This part works as given.
    slow_vec_basics();

    // This part helps you understand how to use FixedSizeArray
    fixed_sized_array();

    // These parts do not work unless you implement push and remove correctly.
    slow_vec_push();
    slow_vec_remove();
}