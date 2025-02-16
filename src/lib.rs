#![no_std]
#![feature(allocator_api)]
#![warn(clippy::all)]

#[cfg(test)]
#[macro_use]
extern crate std;

extern crate alloc;

pub use tail::*;

mod tail;

#[cfg(test)]
mod test {
    use crate::SplitExtend;

    #[test]
    fn basic() {
        let mut list = vec![0, 1, 2, 3];

        let mut tail = list.tail(1);
        tail.push(4);

        println!("First tail: {:?}", tail);

        let mut tail_2 = tail.tail(3);
        tail_2.extend(5..32);
        println!("Second tail: {:?}", tail_2);

        println!("List: {:?}", list);
    }
}
