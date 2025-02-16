#![no_std]
#![feature(allocator_api)]

#[cfg(test)]
#[macro_use]
extern crate std;

extern crate alloc;

pub use head::*;
pub use provider::*;
pub use split_extend::*;
pub use tail::*;

mod head;
mod provider;
mod split_extend;
mod tail;

fn index_out_of_range() -> ! {
    panic!("Index out of range.");
}

#[cfg(test)]
mod test {
    use crate::{LocalProvider, SplitExtend};

    #[test]
    fn basic() {
        let mut list = vec![-159, 1, 2, 3];

        println!("List: {:?}", list);

        let (mut head, mut tail) = <_ as SplitExtend<LocalProvider<_>>>::split_extend(&mut list, 1);

        println!("First head: {:?}", head);
        println!("First tail: {:?}", tail);

        let (head_2, mut tail_2): (crate::Head<'_, i32, _>, crate::Tail<'_, i32, _>) =
            tail.split_extend(3);

        println!("Extending list..");
        tail_2.extend(4..32);

        println!("Second head: {:?}", head_2);
        println!("Second tail: {:?}", tail_2);

        head.map(|slice| slice[0] = 0);

        println!("First head after modification: {:?}", head);

        println!("List: {:?}", list);

        // Uncomment to get:
        // error[E0502]: cannot borrow `list` as immutable because it is also borrowed as mutable
        // println!("Second head: {:?}", head_2);
    }
}
