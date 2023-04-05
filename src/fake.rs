use crate::Error;
extern crate std;
use std::{mem::MaybeUninit};

pub fn getrandom_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    for m in dest {
        m.write(1);
    };
    Ok(())
}