use rand::prelude::*;

fn print_tree(heap: &mut Vec<i32>) {
    println!("                                    {}                                  ", heap[1]);
    println!("                   {}                                  {}               ", heap[2], heap[3]);
    println!("         {}                 {}                {}               {}       ", heap[2], heap[3], heap[4], heap[5]);
    println!("   {}          {}      {}       {}       {}        {}      {}      {}   ", heap[2], heap[3], heap[4], heap[5], heap[4], heap[4], heap[4], heap[4]);
    println!(" {}  {}      {}  {}  {}  {}   {}  {}   {}  {}    {}  {}  {}  {}  {}  {} ", heap[2], heap[3], heap[3], heap[3], heap[3], heap[3], heap[3], heap[3], heap[3], heap[3], heap[3], heap[3], heap[3], heap[3], heap[3], heap[3]); 

}

fn main() {
    let mut start_vec = vec![];
    let mut rng = rand::thread_rng();

    for _ in 1..30 {
        let mut new_val: i32 = rng.gen();
        while start_vec.contains(&mut new_val) {
            new_val = rng.gen();
        }
        start_vec.push(new_val);
    }

    println!(start_vec);

    println!("Hello, world!");
}
