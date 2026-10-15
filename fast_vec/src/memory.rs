use fast_vec::FastVec;

use malloc::MALLOC;
use tracker::Tracker;

fn main() {
    println!("Case 1------------------");
    let mut tracker = Tracker::new();

    let mut fast_vec = FastVec::new();
    fast_vec.push(tracker.track(10));
    println!("{}", fast_vec);
    tracker.print_status();

    fast_vec.remove(0);
    println!("{}", fast_vec);
    tracker.print_status();
    println!("Transcript of allocations {:?}", MALLOC.state().transcript());
    println!("End of case 1-----------");

    println!("Case 2------------------");
    MALLOC.clear();
    let mut tracker2 = Tracker::new();

    let mut fast_vec = FastVec::new();
    fast_vec.push(tracker2.track(10));
    fast_vec.push(tracker2.track(20));
    fast_vec.push(tracker2.track(30));
    println!("{}", fast_vec);
    tracker2.print_status();

    fast_vec.clear();
    tracker2.print_status();
    println!("Transcript of allocations {:?}", MALLOC.state().transcript());
    println!("End of case 2-----------");
}
