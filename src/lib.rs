#![no_std]

#![feature(untagged_unions)]

use core::mem;

#[allow(unions_with_drop_fields)]
#[derive(Clone, Copy)]
pub union Slot<T> { pub x: T }

impl<T> Slot<T> {
    #[inline]
    pub fn new() -> Self { unsafe { mem::uninitialized() } }

    #[inline]
    pub unsafe fn as_ref(&self) -> &T { &self.x }

    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { &mut self.x }

    #[inline]
    pub unsafe fn unwrap(self) -> T { self.x }
}

impl<T> Default for Slot<T> {
    #[inline]
    fn default() -> Self { unsafe { mem::uninitialized() } }
}

impl<T> From<T> for Slot<T> {
    #[inline]
    fn from(x: T) -> Self { Slot { x } }
}
