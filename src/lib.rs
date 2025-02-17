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
        let mut list = vec![3, 1, 2, 0];

        println!("List: {:?}", list);

        let (mut head, mut tail) = <_ as SplitExtend<LocalProvider<_>>>::split_extend(&mut list, 1);

        println!("First head: {:?}", head);
        println!("First tail: {:?}", tail);

        let (mut head_2, mut tail_2): (crate::Head<'_, i32, _>, crate::Tail<'_, i32, _>) =
            tail.split_extend(3);

        println!("Extending list..");
        tail_2.extend(4..32);

        println!("Modifying..");

        head.edit(|slice| slice[0] = 0);
        head_2.edit(|slice| slice[2] = 3);

        // error[E0499]: cannot borrow `tail` as mutable more than once at a time
        // tail.push(1);

        println!("Second head: {:?}", head_2);
        println!("Second tail: {:?}", tail_2);

        println!("First head after modification: {:?}", head);
        println!("Second head after modification: {:?}", head_2);

        println!("List: {:?}", list);

        // error[E0502]: cannot borrow `list` as immutable because it is also borrowed as mutable
        // println!("Second head: {:?}", head_2);
    }
}
