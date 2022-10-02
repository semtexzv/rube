// #![feature(alloc_system)]
// extern crate alloc_system;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use std::sync::Arc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    println!("Started");
    for i in 0 .. 1 {
        // println!("Iter");
        let mut tree = irbtree::RBTree::new();
        for i in 0 .. 5000_000 {
            tree.insert(i, i + 1, Arc::new(i));
        }
        let mut data = tree
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<_>>();
        let mut tree = irbtree::RBTree::from_iter(data);
        println!("{:?} => :{:?}", tree.len(), 0); // tree.into_iter().collect::<Vec<_>>().into_iter().map(|v| v.1).max());
    }
}
