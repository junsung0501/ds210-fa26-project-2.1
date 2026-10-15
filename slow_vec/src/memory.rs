use slow_vec::SlowVec;

use tracker::Tracker;

fn main() {
    println!("Case 1------------------");
    let mut tracker = Tracker::new();

    let mut slow_vec = SlowVec::new();
    slow_vec.push(tracker.track(10));
    println!("{}", slow_vec);
    tracker.print_status();

    slow_vec.remove(0);
    println!("{}", slow_vec);
    tracker.print_status();
    println!("End of case 1-----------");



    println!("Case 2------------------");
    let mut tracker2 = Tracker::new();

    let mut slow_vec = SlowVec::new();
    slow_vec.push(tracker2.track(10));
    slow_vec.push(tracker2.track(20));
    slow_vec.push(tracker2.track(30));
    println!("{}", slow_vec);
    tracker2.print_status();

    drop(slow_vec);
    tracker2.print_status();
    println!("End of case 2-----------");
}
