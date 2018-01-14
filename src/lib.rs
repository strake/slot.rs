#![no_std]

#![feature(untagged_unions)]

use core::mem;

#[allow(unions_with_drop_fields)]
#[derive(Clone, Copy)]
pub union Slot<T> { pub x: T }

impl<T> Slot<T> {
    #[inline]
    pub fn new() -> Self { unsafe { mem::uninitialized() } }
}

impl<T> From<T> for Slot<T> {
    #[inline]
    fn from(x: T) -> Self { Slot { x } }
}
